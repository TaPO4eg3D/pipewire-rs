// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::{marker::PhantomData, ops::Deref};

use crate::context::Context;

use super::Core;

#[derive(Debug)]
pub struct CoreBox<'c> {
    ptr: std::ptr::NonNull<pw_sys::pw_core>,
    context: PhantomData<&'c Context>,
}

impl<'c> CoreBox<'c> {
    pub unsafe fn from_raw(raw: std::ptr::NonNull<pw_sys::pw_core>) -> CoreBox<'c> {
        Self {
            ptr: raw,
            context: PhantomData,
        }
    }

    pub fn into_raw(self) -> std::ptr::NonNull<pw_sys::pw_core> {
        std::mem::ManuallyDrop::new(self).ptr
    }
}

impl<'c> std::ops::Deref for CoreBox<'c> {
    type Target = Core;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.cast::<Core>().as_ref() }
    }
}

impl<'c> AsRef<Core> for CoreBox<'c> {
    fn as_ref(&self) -> &Core {
        self.deref()
    }
}

impl<'c> std::ops::Drop for CoreBox<'c> {
    fn drop(&mut self) {
        unsafe {
            pw_sys::pw_core_disconnect(self.as_raw_ptr());
        }
    }
}
