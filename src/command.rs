//! Command envelope: opcode + payload.
//!
//! Mirrors `NexstarCommand` / pass-through types in [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter).
//!
//! Build with [`crate::factory`] helpers or [`NexstarCommand::new`], then send [`NexstarCommand::command_data`].
//! Incoming bytes are decoded with [`NexstarCommand::parse_response`] (see [`crate::NexstarParsedResponse`]).

use crate::constants::NexstarCommandType;

/// One NexStar command: ASCII opcode byte plus opaque arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NexstarCommand {
    command_type: NexstarCommandType,
    arguments: Vec<u8>,
}

impl NexstarCommand {
    #[must_use]
    pub fn new(command_type: NexstarCommandType, arguments: Vec<u8>) -> Self {
        Self {
            command_type,
            arguments,
        }
    }

    #[must_use]
    pub fn command_type(&self) -> NexstarCommandType {
        self.command_type
    }

    pub fn set_command_type(&mut self, command_type: NexstarCommandType) {
        self.command_type = command_type;
    }

    #[must_use]
    pub fn arguments(&self) -> &[u8] {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<u8> {
        &mut self.arguments
    }

    /// Full bytes to send: `[opcode, ...arguments]`.
    #[must_use]
    pub fn command_data(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.arguments.len());
        out.push(self.command_type.first_char());
        out.extend_from_slice(&self.arguments);
        out
    }
}

/// Pass-through (`P`) command: length, destination device, motor message id, three payload bytes, expected response length.
///
/// Built with [`crate::build_pass_through_command`] or [`crate::build_direct_motor_command`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassThroughCommand {
    inner: NexstarCommand,
}

impl PassThroughCommand {
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            inner: NexstarCommand::new(NexstarCommandType::PassThrough, data),
        }
    }

    #[must_use]
    pub fn inner(&self) -> &NexstarCommand {
        &self.inner
    }

    #[must_use]
    pub fn into_inner(self) -> NexstarCommand {
        self.inner
    }

    #[must_use]
    pub fn command_data(&self) -> Vec<u8> {
        self.inner.command_data()
    }

    #[must_use]
    pub fn msg_len(&self) -> Option<u8> {
        self.inner.arguments.first().copied()
    }

    #[must_use]
    pub fn dest_id(&self) -> Option<u8> {
        self.inner.arguments.get(1).copied()
    }

    #[must_use]
    pub fn msg_id(&self) -> Option<u8> {
        self.inner.arguments.get(2).copied()
    }

    /// The three fixed payload bytes (padded).
    #[must_use]
    pub fn data_triple(&self) -> Option<[u8; 3]> {
        if self.inner.arguments.len() < 6 {
            return None;
        }
        Some([
            self.inner.arguments[3],
            self.inner.arguments[4],
            self.inner.arguments[5],
        ])
    }

    /// Expected response payload length (excluding `#`), stored as last byte of arguments.
    #[must_use]
    pub fn response_bytes_len(&self) -> Option<u8> {
        self.inner.arguments.last().copied()
    }
}
