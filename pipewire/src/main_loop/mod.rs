// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use crate::loop_::Loop;

mod box_;
pub use box_::*;
mod rc;
pub use rc::*;

#[repr(transparent)]
pub struct MainLoop(pw_sys::pw_main_loop);

impl MainLoop {
    pub fn as_raw(&self) -> &pw_sys::pw_main_loop {
        &self.0
    }

    pub fn as_raw_ptr(&self) -> *mut pw_sys::pw_main_loop {
        std::ptr::addr_of!(self.0).cast_mut()
    }

    pub fn loop_(&self) -> &Loop {
        unsafe {
            let pw_loop = pw_sys::pw_main_loop_get_loop(self.as_raw_ptr());
            // FIXME: Make sure pw_loop is not null
            &*(pw_loop.cast::<Loop>())
        }
    }

    pub fn run(&self) {
        unsafe {
            pw_sys::pw_main_loop_run(self.as_raw_ptr());
        }
    }

    pub fn quit(&self) {
        unsafe {
            pw_sys::pw_main_loop_quit(self.as_raw_ptr());
        }
    }
}

impl std::convert::AsRef<Loop> for MainLoop {
    fn as_ref(&self) -> &Loop {
        self.loop_()
    }
}
