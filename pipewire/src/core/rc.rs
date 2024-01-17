// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::{
    ops::Deref,
    ptr,
    rc::{Rc, Weak},
};

use super::{Core, CoreBox};

#[derive(Debug)]
struct CoreRcInner {
    core: CoreBox<'static>,
    // Store the context here, so that the context is not dropped before the core,
    // which may lead to undefined behaviour. Rusts drop order of struct fields
    // (from top to bottom) ensures that this is always destroyed _after_ the core.
    _context: crate::context::ContextRc,
}

#[derive(Debug, Clone)]
pub struct CoreRc {
    inner: Rc<CoreRcInner>,
}

impl CoreRc {
    pub fn from_raw(
        ptr: ptr::NonNull<pw_sys::pw_core>,
        context: crate::context::ContextRc,
    ) -> Self {
        let core = unsafe { CoreBox::from_raw(ptr) };

        Self {
            inner: Rc::new(CoreRcInner {
                core,
                _context: context,
            }),
        }
    }

    pub fn downgrade(&self) -> CoreWeak {
        let weak = Rc::downgrade(&self.inner);
        CoreWeak { weak }
    }
}

impl Deref for CoreRc {
    type Target = Core;

    fn deref(&self) -> &Self::Target {
        self.inner.core.deref()
    }
}

impl AsRef<Core> for CoreRc {
    fn as_ref(&self) -> &Core {
        self.deref()
    }
}

pub struct CoreWeak {
    weak: Weak<CoreRcInner>,
}

impl CoreWeak {
    pub fn upgrade(&self) -> Option<CoreRc> {
        self.weak.upgrade().map(|inner| CoreRc { inner })
    }
}
