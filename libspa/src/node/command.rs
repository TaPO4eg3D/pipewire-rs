// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use crate::{constants::ID_INVALID, pod::command::Command as PodCommand};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Command(spa_sys::spa_node_command);

impl Command {
    pub const SUSPEND: Self = Self(spa_sys::SPA_NODE_COMMAND_Suspend);
    pub const PAUSE: Self = Self(spa_sys::SPA_NODE_COMMAND_Pause);
    pub const START: Self = Self(spa_sys::SPA_NODE_COMMAND_Start);
    pub const ENABLE: Self = Self(spa_sys::SPA_NODE_COMMAND_Enable);
    pub const DISABLE: Self = Self(spa_sys::SPA_NODE_COMMAND_Disable);
    pub const FLUSH: Self = Self(spa_sys::SPA_NODE_COMMAND_Flush);
    pub const DRAIN: Self = Self(spa_sys::SPA_NODE_COMMAND_Drain);
    pub const MARKER: Self = Self(spa_sys::SPA_NODE_COMMAND_Marker);
    pub const PARAM_BEGIN: Self = Self(spa_sys::SPA_NODE_COMMAND_ParamBegin);
    pub const PARAM_END: Self = Self(spa_sys::SPA_NODE_COMMAND_ParamEnd);
    pub const REQUEST_PROCESS: Self = Self(spa_sys::SPA_NODE_COMMAND_RequestProcess);

    pub fn as_raw(&self) -> spa_sys::spa_node_command {
        self.0
    }

    pub fn from_raw(raw: spa_sys::spa_node_command) -> Self {
        Self(raw)
    }

    pub fn id(command: &PodCommand) -> Result<u32, ()> {
        let id = unsafe { spa_sys::spa_node_command_id(command.as_raw_ptr()) };

        if id == ID_INVALID {
            Err(())
        } else {
            Ok(id)
        }
    }

    pub fn init(self) -> PodCommand {
        PodCommand::from_raw(unsafe { spa_sys::spa_node_command_init(self.as_raw()) })
    }
}
