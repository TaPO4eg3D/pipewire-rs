// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::ptr::addr_of;

use crate::{constants::ID_INVALID, pod::SpaTypes};

#[repr(transparent)]
pub struct Command(spa_sys::spa_command);

impl Command {
    pub fn into_raw(self) -> spa_sys::spa_command {
        self.0
    }

    pub fn from_raw(raw: spa_sys::spa_command) -> Self {
        Self(raw)
    }

    pub fn as_raw_ptr(&self) -> *mut spa_sys::spa_command {
        addr_of!(self.0).cast_mut()
    }

    pub fn type_(&self) -> SpaTypes {
        unsafe { SpaTypes::from_raw(spa_sys::spa_command_type(self.as_raw_ptr())) }
    }

    pub fn id(&self, type_: SpaTypes) -> Result<u32, ()> {
        let id = unsafe { spa_sys::spa_command_id(self.as_raw_ptr(), type_.as_raw()) };

        if id == ID_INVALID {
            Err(())
        } else {
            Ok(id)
        }
    }

    pub fn init(type_: SpaTypes, id: u32) -> Self {
        Self(unsafe { spa_sys::spa_command_init(type_.as_raw(), id) })
    }
}
