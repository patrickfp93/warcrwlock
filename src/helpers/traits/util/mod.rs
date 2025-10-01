use helpers::{access_methods, get_ident_headers};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, TraitItem};

mod helpers;

use crate::helpers::to_token_stream;

mod generator_method;
mod type_return;

const MUTEX_GUARD_SUFFIX: &'static str = "MutGuard";
const REF_GUARD_SUFFIX: &'static str = "RefGuard";

pub fn has_mutation(item_trait: &ItemTrait) -> bool {
    // Iterate through the items in the trait definition
    for item in &item_trait.items {
        // Check if the item is a method
        if let TraitItem::Fn(method) = item {
            // Check if the method has a mutable receiver
            if method
                .sig
                .receiver()
                .map_or(false, |receiver| receiver.mutability.is_some())
            {
                return true;
            }
        }
    }
    false
}

pub fn ref_abstract_module(item_trait: &ItemTrait) -> TokenStream {
    let (ident_trait, ident_abs_module, ident_abs_structure, vis_trait) = get_ident_headers(item_trait);

    quote! {
       //#vis_trait mod #ident_abs_module {

            use std::{ops::Deref, sync::Arc};

            //use super::#ident_trait;

            pub struct #ident_abs_structure {
                base: Arc<dyn #ident_trait>,
            }
            //Implementations
            impl Deref for #ident_abs_structure {
                type Target = dyn #ident_trait;

                fn deref(&self) -> &Self::Target {
                    self.base.as_ref()
                }
            }
            impl AsRef<dyn #ident_trait + 'static> for #ident_abs_structure {
                fn as_ref(&self) -> &'static dyn #ident_trait {
                    let reply = self.base.as_ref() as *const dyn #ident_trait;
                    unsafe{&(*reply)}
                }
            }

            ///Traits Implementations
            impl<T: #ident_trait + 'static> From<T> for #ident_abs_structure {
                fn from(value: T) -> Self {
                    let base = Arc::new(value);
                    let base = base as Arc<dyn #ident_trait>;
                    Self {
                        base
                    }
                }
            }

            impl Clone for #ident_abs_structure {
                fn clone(&self) -> Self {
                    Self {
                        base: self.base.clone(),
                    }
                }
            }

            unsafe impl Send for #ident_abs_structure {}

            unsafe impl Sync for #ident_abs_structure {}
        //}
    }
}

pub fn mut_abstract_module(item_trait: &ItemTrait) -> TokenStream {

    let (ident_trait, ident_abs_module, ident_abs_structure, vis_trait) = get_ident_headers(item_trait);

    let ident_flag = to_token_stream(format!("{}Flag", ident_trait));
    let ident_box = to_token_stream(format!("{}Box", ident_trait));
    let ident_ref_guard = to_token_stream(format!("{}{}", ident_trait, REF_GUARD_SUFFIX));
    let ident_mut_guard = to_token_stream(format!("{}{}", ident_trait, MUTEX_GUARD_SUFFIX));
    let access_methods = access_methods(item_trait, ident_trait);


    let doc_read = format!(
        "Provides read access to the underlying `{}` implementation.\n\
         This method acquires a read lock on the internal flag, ensuring\n\
         that the trait object can be accessed safely in a multithreaded context.\n\
         # Returns\n\
         A `{}` that allows read access to the `{}` object.\n\
         # Panics\n\
         Panics if the lock is poisoned (i.e., if another thread has panicked\n\
         while holding the lock).",
        ident_trait, ident_ref_guard, ident_trait
    );


    let reply = quote! {
            use std::{
                any::TypeId,
                mem::transmute,
                ops::{Deref, DerefMut},
                sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
            };
            #vis_trait struct #ident_abs_structure {
                flag: Arc<RwLock<TypeId>>,
                arc_base: Arc<dyn #ident_trait>,
                base: *mut dyn #ident_trait,
            }
            //Implementations
            impl #ident_abs_structure {
                #[doc = #doc_read]
                pub fn read(&self) -> #ident_ref_guard<dyn #ident_trait> {
                    let flag = self.flag.read().unwrap();
                    #ident_ref_guard ::new(self.base, flag)
                }

                /// Attempts to acquire a read lock on the underlying `#ident_trait ` implementation.
                ///
                /// This method attempts to acquire a read lock on the internal flag, ensuring
                /// that the trait object can be accessed safely in a multithreaded context.
                /// If the lock is successfully acquired, it returns a `#ident_ref_guard`
                /// that allows read access to the `#ident_trait ` object. If the lock is poisoned,
                /// it returns an error.
                ///
                /// # Returns
                ///
                /// A `Result` containing a `#ident_ref_guard` if the lock is successfully
                /// acquired, or a `PoisonError` if the lock is poisoned.
                ///
                /// # Errors
                ///
                /// This method will return an `Err` if the lock is poisoned, i.e., if another
                /// thread has panicked while holding the lock.
                ///
                pub fn try_read(&self) -> Result<#ident_ref_guard<dyn #ident_trait>, PoisonError<RwLockReadGuard<TypeId>>> {
                    let possible_flag = self.flag.read();
                    match possible_flag {
                        Ok(flag) => Ok(#ident_ref_guard::new(self.base, flag)),
                        Err(err) => Err(err),
                    }
                }

                /// Attempts to acquire a write lock on the underlying `#ident_trait ` implementation.
                ///
                /// This method attempts to acquire a write lock on the internal flag, ensuring
                /// that the trait object can be modified safely in a multithreaded context.
                /// If the lock is successfully acquired, it returns a `#ident_mut_guard`
                /// that allows to write access to the `#ident_trait ` object. If the lock is poisoned,
                /// it returns an error.
                ///
                /// # Returns
                ///
                /// A `Result` containing a `#ident_mut_guard` if the lock is successfully
                /// acquired, or a `PoisonError` if the lock is poisoned.
                ///
                /// # Errors
                ///
                /// This method will return an `Err` if the lock is poisoned, i.e., if another
                /// thread has panicked while holding the lock.
                ///
                pub fn try_write(&mut self) -> Result<#ident_mut_guard<dyn #ident_trait>, PoisonError<RwLockWriteGuard<TypeId>>> {
                    let possible_flag = self.flag.write();
                    match possible_flag {
                        Ok(flag) => Ok(#ident_mut_guard::new(self.base, flag)),
                        Err(err) => Err(err),
                    }
                }

                /// Provides write access to the underlying `#ident_trait ` implementation.
                ///
                /// This method acquires a write lock on the internal flag, ensuring
                /// that the trait object can be modified safely in a multithreaded context.
                ///
                /// # Returns
                ///
                /// A `#ident_mut_guard` that allows to write access to the `#ident_trait ` object.
                ///
                /// # Panics
                ///
                /// Panics if the lock is poisoned (i.e., if another thread has panicked
                /// while holding the lock).
                pub fn write(&mut self) -> #ident_mut_guard<dyn #ident_trait> {
                    let flag = self.flag.write().unwrap();
                    #ident_mut_guard::new(self.base, flag)
                }

                #access_methods
            }

            ///Traits Implementations

            impl<T: #ident_trait + 'static> From<T> for #ident_abs_structure {
                fn from(value: T) -> Self {
                    let flag = Arc::new(RwLock::new(TypeId::of::<T>()));
                    let arc_base: Arc<dyn #ident_trait> = Arc::new(value);
                    let base = arc_base.as_ref() as *const dyn #ident_trait as *mut dyn #ident_trait;
                    Self {
                        flag,
                        arc_base,
                        base,
                    }
                }
            }

           impl Clone for #ident_abs_structure {
                fn clone(&self) -> Self {
                    Self {
                        flag: self.flag.clone(),
                        base: self.base,
                        arc_base: self.arc_base.clone(),
                    }
                }
            }

            unsafe impl Send for #ident_abs_structure {}

            unsafe impl Sync for #ident_abs_structure {}

            //GUARDS
            #vis_trait struct #ident_ref_guard<'a, T : ?Sized> {
                _guard: RwLockReadGuard<'a, TypeId>,
                reference: *const T,
            }

            impl<'a, T : ?Sized> #ident_ref_guard<'a, T> {
                fn new(ptr: *const T, guard: RwLockReadGuard<'a,TypeId>) -> Self {
                    Self {
                        _guard: guard,
                        reference: ptr,
                    }
                }
            }

            impl<'a, T : ?Sized> Deref for #ident_ref_guard<'a, T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    unsafe { &*self.reference }
                }
            }

            #vis_trait struct #ident_mut_guard<'a, T : ?Sized> {
                _guard: RwLockWriteGuard<'a, TypeId>,
                reference: *mut T,
            }

            impl<'a, T : ?Sized> #ident_mut_guard<'a, T> {
                fn new(ptr: *mut T, guard: RwLockWriteGuard<'a, TypeId>) -> Self {
                    Self {
                        _guard: guard,
                        reference: ptr,
                    }
                }
            }

            impl<'a, T : ?Sized> Deref for #ident_mut_guard<'a, T> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    unsafe { &*self.reference }
                }
            }

            impl<'a, T : ?Sized> DerefMut for #ident_mut_guard<'a, T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { &mut *self.reference }
                }
            }
    };
    reply

}
