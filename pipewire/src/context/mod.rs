// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::ptr;

use crate::{
    core::CoreBox,
    properties::{Properties, PropertiesRef},
    Error,
};

mod box_;
pub use box_::*;
mod rc;
pub use rc::*;

#[repr(transparent)]
pub struct Context(pw_sys::pw_context);

impl Context {
    pub fn as_raw(&self) -> &pw_sys::pw_context {
        &self.0
    }

    pub fn as_raw_ptr(&self) -> *mut pw_sys::pw_context {
        std::ptr::addr_of!(self.0).cast_mut()
    }

    pub fn properties(&self) -> &PropertiesRef {
        unsafe {
            let props = pw_sys::pw_context_get_properties(self.as_raw_ptr());
            let props = ptr::NonNull::new(props.cast_mut()).expect("context properties is NULL");
            props.cast().as_ref()
        }
    }

    pub fn update_properties(&self, properties: &spa::utils::dict::DictRef) {
        unsafe {
            pw_sys::pw_context_update_properties(self.as_raw_ptr(), properties.as_raw_ptr());
        }
    }

    pub fn connect<'c>(&'c self, properties: Option<Properties>) -> Result<CoreBox<'c>, Error> {
        let properties = properties.map_or(ptr::null_mut(), |p| p.into_raw());

        unsafe {
            let core = pw_sys::pw_context_connect(self.as_raw_ptr(), properties, 0);
            let ptr = ptr::NonNull::new(core).ok_or(Error::CreationFailed)?;

            Ok(CoreBox::from_raw(ptr))
        }
    }
}
