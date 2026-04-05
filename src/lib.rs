#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod command;
pub mod constants;
pub mod factory;
pub mod parser;
pub mod response;
pub mod utils;

pub use command::{NexstarCommand, PassThroughCommand};
pub use constants::*;
pub use factory::*;
pub use parser::parse_response;
pub use response::{
    EchoResponse, GetDateResponse, GetGpsLatitudeResponse, GetGpsLongitudeResponse,
    GetGpsTimeResponse, GetGpsYearResponse, GetLocationResponse, GetModelResponse,
    GetPositionResponse, GetRtcTimeResponse, GetTimeResponse, GetTrackingModeResponse,
    GetVersionResponse, IsAlignmentCompleteResponse, IsGotoInProgressResponse, IsGpsLinkedResponse,
    NexstarParsedResponse, PassThroughPayload, PassThroughResponse, VoidResponse,
};
pub use utils::{
    convert_degrees_to_nexstar, convert_degrees_to_precise_nexstar, deg_to_dms, dms_to_deg,
    int_to_string_rad, string_to_double_rad, tracking_rate_to_bytes,
};
