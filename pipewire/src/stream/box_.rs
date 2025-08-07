// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use crate::{core::Core, error::Error, properties::PropertiesBox};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ptr,
};

use super::Stream;

pub struct StreamBox<'c> {
    ptr: ptr::NonNull<pw_sys::pw_stream>,
    core: PhantomData<&'c Core>,
}

impl<'c> StreamBox<'c> {
    /// Create a [`StreamBox`]
    ///
    /// Initialises a new stream with the given `name` and `properties`.
    pub fn new(
        core: &'c Core,
        name: &str,
        properties: PropertiesBox,
    ) -> Result<StreamBox<'c>, Error> {
        let name = CString::new(name).expect("Invalid byte in stream name");

        let c_str = name.as_c_str();
        StreamBox::new_cstr(core, c_str, properties)
    }

    /// Initialises a new stream with the given `name` as C String and `properties`.
    pub fn new_cstr(
        core: &'c Core,
        name: &CStr,
        properties: PropertiesBox,
    ) -> Result<StreamBox<'c>, Error> {
        unsafe {
            let stream =
                pw_sys::pw_stream_new(core.as_raw_ptr(), name.as_ptr(), properties.into_raw());
            let stream = ptr::NonNull::new(stream).ok_or(Error::CreationFailed)?;

            Ok(Self::from_raw(stream))
        }
    }

    pub unsafe fn from_raw(raw: ptr::NonNull<pw_sys::pw_stream>) -> StreamBox<'c> {
        Self {
            ptr: raw,
            core: PhantomData,
        }
    }

    pub fn into_raw(self) -> ptr::NonNull<pw_sys::pw_stream> {
        std::mem::ManuallyDrop::new(self).ptr
    }
}

impl<'c> std::ops::Deref for StreamBox<'c> {
    type Target = Stream;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.cast().as_ref() }
    }
}

impl<'c> std::fmt::Debug for StreamBox<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamBox")
            .field("name", &self.name())
            .field("state", &self.state())
            .field("node-id", &self.node_id())
            .field("properties", &self.properties())
            .finish()
    }
}

impl<'c> std::ops::Drop for StreamBox<'c> {
    fn drop(&mut self) {
        unsafe { pw_sys::pw_stream_destroy(self.as_raw_ptr()) }
    }
}
