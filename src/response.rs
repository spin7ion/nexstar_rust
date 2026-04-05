//! Parsed responses from the mount.
//!
//! Structured like the Dart responses in [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter).

use crate::command::{NexstarCommand, PassThroughCommand};
use crate::constants::{
    NexstarCommandType, NexstarEw, NexstarModel, NexstarMotorMsg, NexstarNs, NexstarTrackingMode,
};
use crate::utils::string_to_double_rad;

const HASH: u8 = b'#';

#[derive(Debug, Clone, PartialEq)]
pub struct VoidResponse {
    success: bool,
}

impl VoidResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        let success = response.len() == 1 && response[0] == HASH;
        Self { success }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetPositionResponse {
    success: bool,
    ra_azm: f64,
    dec_alt: f64,
}

impl GetPositionResponse {
    #[must_use]
    pub fn new(cmd: &NexstarCommand, response: &[u8]) -> Self {
        let Ok(s) = std::str::from_utf8(response) else {
            return Self {
                success: false,
                ra_azm: 0.0,
                dec_alt: 0.0,
            };
        };
        if !s.ends_with('#') {
            return Self {
                success: false,
                ra_azm: 0.0,
                dec_alt: 0.0,
            };
        }
        let precise = matches!(
            cmd.command_type(),
            NexstarCommandType::GetPreciseRaDec | NexstarCommandType::GetPreciseAzmAlt
        );
        let body = s.trim_end_matches('#');
        let parts: Vec<&str> = body.split(',').collect();
        if parts.len() != 2 {
            return Self {
                success: false,
                ra_azm: 0.0,
                dec_alt: 0.0,
            };
        }
        Self {
            success: true,
            ra_azm: string_to_double_rad(parts[0], precise),
            dec_alt: string_to_double_rad(parts[1], precise),
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn ra_azm(&self) -> f64 {
        self.ra_azm
    }

    #[must_use]
    pub const fn dec_alt(&self) -> f64 {
        self.dec_alt
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTrackingModeResponse {
    success: bool,
    mode: NexstarTrackingMode,
}

impl GetTrackingModeResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        let Ok(s) = std::str::from_utf8(response) else {
            return Self {
                success: false,
                mode: NexstarTrackingMode::Off,
            };
        };
        if !s.ends_with('#') || s.len() != 2 {
            return Self {
                success: false,
                mode: NexstarTrackingMode::Off,
            };
        }
        let mode = match s.as_bytes()[0] {
            b'0' => NexstarTrackingMode::Off,
            b'1' => NexstarTrackingMode::AltAz,
            b'2' => NexstarTrackingMode::EqNorth,
            b'3' => NexstarTrackingMode::EqSouth,
            _ => {
                return Self {
                    success: false,
                    mode: NexstarTrackingMode::Off,
                };
            }
        };
        Self {
            success: true,
            mode,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn mode(&self) -> NexstarTrackingMode {
        self.mode
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLocationResponse {
    success: bool,
    latitude_deg: u8,
    latitude_min: u8,
    latitude_sec: u8,
    direction_ns: NexstarNs,
    longitude_deg: u8,
    longitude_min: u8,
    longitude_sec: u8,
    direction_ew: NexstarEw,
}

impl GetLocationResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 9 || *response.last().unwrap_or(&0) != HASH {
            return Self {
                success: false,
                latitude_deg: 0,
                latitude_min: 0,
                latitude_sec: 0,
                direction_ns: NexstarNs::North,
                longitude_deg: 0,
                longitude_min: 0,
                longitude_sec: 0,
                direction_ew: NexstarEw::East,
            };
        }
        Self {
            success: true,
            latitude_deg: response[0],
            latitude_min: response[1],
            latitude_sec: response[2],
            direction_ns: if response[3] == 0 {
                NexstarNs::North
            } else {
                NexstarNs::South
            },
            longitude_deg: response[4],
            longitude_min: response[5],
            longitude_sec: response[6],
            direction_ew: if response[7] == 0 {
                NexstarEw::East
            } else {
                NexstarEw::West
            },
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn latitude_deg(&self) -> u8 {
        self.latitude_deg
    }

    #[must_use]
    pub const fn latitude_min(&self) -> u8 {
        self.latitude_min
    }

    #[must_use]
    pub const fn latitude_sec(&self) -> u8 {
        self.latitude_sec
    }

    #[must_use]
    pub const fn direction_ns(&self) -> NexstarNs {
        self.direction_ns
    }

    #[must_use]
    pub const fn longitude_deg(&self) -> u8 {
        self.longitude_deg
    }

    #[must_use]
    pub const fn longitude_min(&self) -> u8 {
        self.longitude_min
    }

    #[must_use]
    pub const fn longitude_sec(&self) -> u8 {
        self.longitude_sec
    }

    #[must_use]
    pub const fn direction_ew(&self) -> NexstarEw {
        self.direction_ew
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetTimeResponse {
    success: bool,
    h: u8,
    m: u8,
    s: u8,
    month: u8,
    day: u8,
    year: u8,
    timezone: u8,
    dst: u8,
}

impl GetTimeResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 9 || *response.last().unwrap_or(&0) != HASH {
            return Self {
                success: false,
                h: 0,
                m: 0,
                s: 0,
                month: 0,
                day: 0,
                year: 0,
                timezone: 0,
                dst: 0,
            };
        }
        Self {
            success: true,
            h: response[0],
            m: response[1],
            s: response[2],
            month: response[3],
            day: response[4],
            year: response[5],
            timezone: response[6],
            dst: response[7],
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn h(&self) -> u8 {
        self.h
    }

    #[must_use]
    pub const fn m(&self) -> u8 {
        self.m
    }

    #[must_use]
    pub const fn s(&self) -> u8 {
        self.s
    }

    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }

    #[must_use]
    pub const fn year(&self) -> u8 {
        self.year
    }

    #[must_use]
    pub const fn timezone(&self) -> u8 {
        self.timezone
    }

    #[must_use]
    pub const fn dst(&self) -> u8 {
        self.dst
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsGpsLinkedResponse {
    success: bool,
    linked: bool,
}

impl IsGpsLinkedResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 2 || response[1] != HASH {
            return Self {
                success: false,
                linked: false,
            };
        }
        Self {
            success: true,
            linked: response[0] == b'1',
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn linked(&self) -> bool {
        self.linked
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetGpsLatitudeResponse {
    success: bool,
    latitude: f64,
}

impl GetGpsLatitudeResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 4 || response[3] != HASH {
            return Self {
                success: false,
                latitude: 0.0,
            };
        }
        let v =
            u32::from(response[0]) << 16 | u32::from(response[1]) << 8 | u32::from(response[2]);
        let latitude = f64::from(v) * 360.0 / f64::from(1u32 << 24);
        Self {
            success: true,
            latitude,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn latitude(&self) -> f64 {
        self.latitude
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetGpsLongitudeResponse {
    success: bool,
    longitude: f64,
}

impl GetGpsLongitudeResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 4 || response[3] != HASH {
            return Self {
                success: false,
                longitude: 0.0,
            };
        }
        let v =
            u32::from(response[0]) << 16 | u32::from(response[1]) << 8 | u32::from(response[2]);
        let longitude = f64::from(v) * 360.0 / f64::from(1u32 << 24);
        Self {
            success: true,
            longitude,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn longitude(&self) -> f64 {
        self.longitude
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetGpsDateResponse {
    success: bool,
    month: u8,
    day: u8,
}

impl GetGpsDateResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 3 || response[2] != HASH {
            return Self {
                success: false,
                month: 0,
                day: 0,
            };
        }
        Self {
            success: true,
            month: response[0],
            day: response[1],
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn month(&self) -> u8 {
        self.month
    }

    #[must_use]
    pub const fn day(&self) -> u8 {
        self.day
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetGpsYearResponse {
    success: bool,
    year: u16,
}

impl GetGpsYearResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 3 || response[2] != HASH {
            return Self {
                success: false,
                year: 0,
            };
        }
        let year = u16::from(response[0]) * 256 + u16::from(response[1]);
        Self {
            success: true,
            year,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn year(&self) -> u16 {
        self.year
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetGpsTimeResponse {
    success: bool,
    h: u8,
    m: u8,
    s: u8,
}

impl GetGpsTimeResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 4 || response[3] != HASH {
            return Self {
                success: false,
                h: 0,
                m: 0,
                s: 0,
            };
        }
        Self {
            success: true,
            h: response[0],
            m: response[1],
            s: response[2],
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn h(&self) -> u8 {
        self.h
    }

    #[must_use]
    pub const fn m(&self) -> u8 {
        self.m
    }

    #[must_use]
    pub const fn s(&self) -> u8 {
        self.s
    }
}

pub type GetDateResponse = GetGpsDateResponse;
pub type GetYearResponse = GetGpsYearResponse;
pub type GetRtcTimeResponse = GetGpsTimeResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetVersionResponse {
    success: bool,
    version: String,
}

impl GetVersionResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 3 || response[2] != HASH {
            return Self {
                success: false,
                version: String::new(),
            };
        }
        let version = format!("{}.{}", response[0], response[1]);
        Self {
            success: true,
            version,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl std::fmt::Display for GetVersionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.version.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetModelResponse {
    success: bool,
    model: NexstarModel,
}

impl GetModelResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 2 || response[1] != HASH {
            return Self {
                success: false,
                model: NexstarModel::Unknown,
            };
        }
        Self {
            success: true,
            model: NexstarModel::from_code(response[0]),
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn model(&self) -> NexstarModel {
        self.model
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoResponse {
    success: bool,
    echo: String,
}

impl EchoResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 2 || response[1] != HASH {
            return Self {
                success: false,
                echo: String::new(),
            };
        }
        let echo = char::from_u32(u32::from(response[0]))
            .map(|c| c.to_string())
            .unwrap_or_default();
        Self { success: true, echo }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub fn echo(&self) -> &str {
        &self.echo
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsAlignmentCompleteResponse {
    success: bool,
    complete: bool,
}

impl IsAlignmentCompleteResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 2 || response[1] != HASH {
            return Self {
                success: false,
                complete: false,
            };
        }
        Self {
            success: true,
            complete: response[0] == 1,
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn complete(&self) -> bool {
        self.complete
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsGotoInProgressResponse {
    success: bool,
    in_progress: bool,
}

impl IsGotoInProgressResponse {
    #[must_use]
    pub fn new(_cmd: &NexstarCommand, response: &[u8]) -> Self {
        if response.len() != 2 || response[1] != HASH {
            return Self {
                success: false,
                in_progress: false,
            };
        }
        Self {
            success: true,
            in_progress: response[0] == b'1',
        }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub const fn in_progress(&self) -> bool {
        self.in_progress
    }
}

/// Decoded payload for selected pass-through motor messages.
#[derive(Debug, Clone, PartialEq)]
pub enum PassThroughPayload {
    Raw(Vec<u8>),
    Approach(bool),
    LevelDone(bool),
    SlewDone(bool),
    PecRecordDone(bool),
    FirmwareVersion(String),
    AtIndex(bool),
    AutoguideRate(u8),
    PollCordwrap(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassThroughResponse {
    success: bool,
    data: Option<PassThroughPayload>,
}

impl PassThroughResponse {
    #[must_use]
    pub fn new(cmd: &PassThroughCommand, response: &[u8]) -> Self {
        let Some(awaited_len) = cmd.response_bytes_len() else {
            return Self {
                success: false,
                data: None,
            };
        };
        let hash = response.iter().position(|&b| b == HASH);
        let Some(hash_idx) = hash else {
            return Self {
                success: false,
                data: None,
            };
        };
        let success = i32::from(awaited_len) + 1 == hash_idx as i32;
        let msg_id = cmd.msg_id();
        let Some(msg_id) = msg_id else {
            return Self {
                success,
                data: None,
            };
        };
        let Some(msg) = NexstarMotorMsg::from_id(msg_id) else {
            return Self {
                success,
                data: None,
            };
        };
        let payload = if hash_idx > 0 {
            &response[..hash_idx]
        } else {
            &[]
        };
        let data = match msg {
            NexstarMotorMsg::McGetPosition => Some(PassThroughPayload::Raw(payload.to_vec())),
            NexstarMotorMsg::McGetApproach => Some(PassThroughPayload::Approach(
                payload.first() == Some(&1),
            )),
            NexstarMotorMsg::McLevelDone => Some(PassThroughPayload::LevelDone(
                payload.first() == Some(&0xff),
            )),
            NexstarMotorMsg::McSlewDone => Some(PassThroughPayload::SlewDone(
                payload.first() == Some(&0xff),
            )),
            NexstarMotorMsg::McPecRecordDone => Some(PassThroughPayload::PecRecordDone(
                payload.first() == Some(&0xff),
            )),
            NexstarMotorMsg::McAtIndex => Some(PassThroughPayload::AtIndex(
                payload.first() == Some(&0xff),
            )),
            NexstarMotorMsg::McPollCordwrap => Some(PassThroughPayload::PollCordwrap(
                payload.first() == Some(&0xff),
            )),
            NexstarMotorMsg::McGetVer => {
                if payload.len() >= 2 {
                    Some(PassThroughPayload::FirmwareVersion(format!(
                        "{}.{}",
                        payload[0], payload[1]
                    )))
                } else {
                    None
                }
            }
            NexstarMotorMsg::McGetAutoguideRate => payload
                .first()
                .copied()
                .map(PassThroughPayload::AutoguideRate),
            _ => None,
        };
        Self { success, data }
    }

    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    #[must_use]
    pub fn data(&self) -> Option<&PassThroughPayload> {
        self.data.as_ref()
    }
}

/// Result of [`NexstarCommand::parse_response`](crate::command::NexstarCommand::parse_response).
#[derive(Debug, Clone, PartialEq)]
pub enum NexstarParsedResponse {
    Void(VoidResponse),
    Position(GetPositionResponse),
    TrackingMode(GetTrackingModeResponse),
    Location(GetLocationResponse),
    Time(GetTimeResponse),
    GpsLinked(IsGpsLinkedResponse),
    GpsLatitude(GetGpsLatitudeResponse),
    GpsLongitude(GetGpsLongitudeResponse),
    GpsDate(GetGpsDateResponse),
    GpsYear(GetGpsYearResponse),
    GpsTime(GetGpsTimeResponse),
    Version(GetVersionResponse),
    Model(GetModelResponse),
    Echo(EchoResponse),
    AlignmentComplete(IsAlignmentCompleteResponse),
    GotoInProgress(IsGotoInProgressResponse),
    PassThrough(PassThroughResponse),
}
