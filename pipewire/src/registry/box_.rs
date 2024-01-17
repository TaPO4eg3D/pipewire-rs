// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::{marker::PhantomData, ptr};

use crate::core::Core;

use super::Registry;

#[derive(Debug)]
pub struct RegistryBox<'c> {
    ptr: ptr::NonNull<pw_sys::pw_registry>,
    core: PhantomData<&'c Core>,
}

impl<'c> RegistryBox<'c> {
    pub unsafe fn from_raw(ptr: ptr::NonNull<pw_sys::pw_registry>) -> Self {
        Self {
            ptr,
            core: PhantomData,
        }
    }
}

impl<'c> std::ops::Deref for RegistryBox<'c> {
    type Target = Registry;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.cast::<Registry>().as_ref() }
    }
}

impl<'c> AsRef<Registry> for RegistryBox<'c> {
    fn as_ref(&self) -> &Registry {
        &*self
    }
}

impl<'c> Drop for RegistryBox<'c> {
    fn drop(&mut self) {
        unsafe {
            pw_sys::pw_proxy_destroy(self.as_raw_ptr().cast());
        }
    }
}
