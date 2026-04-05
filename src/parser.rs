//! Dispatch parsing by command type.
//!
//! Corresponds to the Dart parser in [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter).

use crate::command::{NexstarCommand, PassThroughCommand};
use crate::constants::NexstarCommandType;
use crate::response::{
    EchoResponse, GetDateResponse, GetGpsLatitudeResponse, GetGpsLongitudeResponse,
    GetGpsTimeResponse, GetGpsYearResponse, GetLocationResponse, GetModelResponse,
    GetPositionResponse, GetTimeResponse, GetTrackingModeResponse, GetVersionResponse,
    IsAlignmentCompleteResponse, IsGotoInProgressResponse, IsGpsLinkedResponse,
    NexstarParsedResponse, PassThroughResponse, VoidResponse,
};

/// Parse a `#`-terminated reply into [`crate::NexstarParsedResponse`] using `cmd`’s opcode.
///
/// Prefer [`NexstarCommand::parse_response`] when you already hold a [`NexstarCommand`].
#[must_use]
pub fn parse_response(cmd: &NexstarCommand, response: &[u8]) -> NexstarParsedResponse {
    match cmd.command_type() {
        NexstarCommandType::GetRaDec
        | NexstarCommandType::GetPreciseRaDec
        | NexstarCommandType::GetAzmAlt
        | NexstarCommandType::GetPreciseAzmAlt => {
            NexstarParsedResponse::Position(GetPositionResponse::new(cmd, response))
        }
        NexstarCommandType::GetTrackingMode => {
            NexstarParsedResponse::TrackingMode(GetTrackingModeResponse::new(cmd, response))
        }
        NexstarCommandType::GetLocation => {
            NexstarParsedResponse::Location(GetLocationResponse::new(cmd, response))
        }
        NexstarCommandType::GetTime => NexstarParsedResponse::Time(GetTimeResponse::new(cmd, response)),
        NexstarCommandType::IsGpsLinked => {
            NexstarParsedResponse::GpsLinked(IsGpsLinkedResponse::new(cmd, response))
        }
        NexstarCommandType::GetLatitude => {
            NexstarParsedResponse::GpsLatitude(GetGpsLatitudeResponse::new(cmd, response))
        }
        NexstarCommandType::GetLongitude => {
            NexstarParsedResponse::GpsLongitude(GetGpsLongitudeResponse::new(cmd, response))
        }
        NexstarCommandType::GetDate | NexstarCommandType::GetRtcDate => {
            NexstarParsedResponse::GpsDate(GetDateResponse::new(cmd, response))
        }
        NexstarCommandType::GetYear | NexstarCommandType::GetRtcYear => {
            NexstarParsedResponse::GpsYear(GetGpsYearResponse::new(cmd, response))
        }
        NexstarCommandType::GetGpsTime | NexstarCommandType::GetRtcTime => {
            NexstarParsedResponse::GpsTime(GetGpsTimeResponse::new(cmd, response))
        }
        NexstarCommandType::GetVersion | NexstarCommandType::GetDeviceVersion => {
            NexstarParsedResponse::Version(GetVersionResponse::new(cmd, response))
        }
        NexstarCommandType::GetModel => {
            NexstarParsedResponse::Model(GetModelResponse::new(cmd, response))
        }
        NexstarCommandType::Echo => NexstarParsedResponse::Echo(EchoResponse::new(cmd, response)),
        NexstarCommandType::IsAlignmentComplete => NexstarParsedResponse::AlignmentComplete(
            IsAlignmentCompleteResponse::new(cmd, response),
        ),
        NexstarCommandType::IsGotoInProgress => NexstarParsedResponse::GotoInProgress(
            IsGotoInProgressResponse::new(cmd, response),
        ),
        NexstarCommandType::PassThrough => {
            let pass = PassThroughCommand::new(cmd.arguments().to_vec());
            NexstarParsedResponse::PassThrough(PassThroughResponse::new(&pass, response))
        }
        _ => NexstarParsedResponse::Void(VoidResponse::new(cmd, response)),
    }
}

impl crate::command::NexstarCommand {
    /// Parse a `#`-terminated response from the mount for this command’s opcode.
    ///
    /// See [`crate::parse_response`] and [`crate::NexstarParsedResponse`].
    #[must_use]
    pub fn parse_response(&self, response: &[u8]) -> NexstarParsedResponse {
        parse_response(self, response)
    }
}

impl crate::command::PassThroughCommand {
    /// Same as [`NexstarCommand::parse_response`] for the underlying pass-through frame.
    #[must_use]
    pub fn parse_response(&self, response: &[u8]) -> NexstarParsedResponse {
        parse_response(self.inner(), response)
    }
}
