//! Protocol enums and constants.
//!
//! Logic matches the Dart types in [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarCommandType {
    GetRaDec,
    GetPreciseRaDec,
    GetAzmAlt,
    GetPreciseAzmAlt,
    GotoRaDec,
    GotoPreciseRaDec,
    GotoAzmAlt,
    GotoPreciseAzmAlt,
    SyncRaDec,
    SyncPreciseRaDec,
    GetTrackingMode,
    SetTrackingMode,
    SlewRate,
    GetLocation,
    SetLocation,
    GetTime,
    SetTime,
    IsGpsLinked,
    GetLatitude,
    GetLongitude,
    GetDate,
    GetYear,
    GetGpsTime,
    GetRtcDate,
    GetRtcYear,
    GetRtcTime,
    SetRtcDate,
    SetRtcYear,
    SetRtcTime,
    GetVersion,
    GetDeviceVersion,
    GetModel,
    Echo,
    IsAlignmentComplete,
    IsGotoInProgress,
    CancelGoto,
    PassThrough,
}

impl NexstarCommandType {
    /// First byte of the command (ASCII) sent to the mount.
    pub const fn first_char(self) -> u8 {
        match self {
            Self::GetRaDec => b'E',
            Self::GetPreciseRaDec => b'e',
            Self::GetAzmAlt => b'Z',
            Self::GetPreciseAzmAlt => b'z',
            Self::GotoRaDec => b'R',
            Self::GotoPreciseRaDec => b'r',
            Self::GotoAzmAlt => b'B',
            Self::GotoPreciseAzmAlt => b'b',
            Self::SyncRaDec => b'S',
            Self::SyncPreciseRaDec => b's',
            Self::GetTrackingMode => b't',
            Self::SetTrackingMode => b'T',
            Self::SlewRate => b'P',
            Self::GetLocation => b'w',
            Self::SetLocation => b'W',
            Self::GetTime => b'h',
            Self::SetTime => b'H',
            Self::IsGpsLinked
            | Self::GetLatitude
            | Self::GetLongitude
            | Self::GetDate
            | Self::GetYear
            | Self::GetGpsTime
            | Self::GetRtcDate
            | Self::GetRtcYear
            | Self::GetRtcTime
            | Self::SetRtcDate
            | Self::SetRtcYear
            | Self::SetRtcTime
            | Self::GetDeviceVersion => b'P',
            Self::GetVersion => b'V',
            Self::GetModel => b'm',
            Self::Echo => b'K',
            Self::IsAlignmentComplete => b'J',
            Self::IsGotoInProgress => b'L',
            Self::CancelGoto => b'M',
            Self::PassThrough => b'P',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NexstarTrackingMode {
    Off = 0,
    AltAz = 1,
    EqNorth = 2,
    EqSouth = 3,
}

impl NexstarTrackingMode {
    #[must_use]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarRate {
    Variable,
    Fixed,
}

impl NexstarRate {
    pub const fn byte(self) -> u8 {
        match self {
            Self::Variable => 0x3,
            Self::Fixed => 0x2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarFixedRate {
    Sidereal,
    Solar,
    Lunar,
}

impl NexstarFixedRate {
    pub const fn byte(self) -> u8 {
        match self {
            Self::Sidereal => 0xff,
            Self::Solar => 0xfe,
            Self::Lunar => 0xfd,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarDirection {
    Positive,
    Negative,
}

impl NexstarDirection {
    pub const fn byte(self) -> u8 {
        match self {
            Self::Positive => 6,
            Self::Negative => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarAxis {
    Ra,
    Dec,
    Azm,
    Alt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarDevices {
    MotorFocus,
    MotorAzmRa,
    MotorAltDec,
    Gps,
    Rtc,
}

impl NexstarDevices {
    pub const fn id(self) -> u8 {
        match self {
            Self::MotorFocus => 12,
            Self::MotorAzmRa => 16,
            Self::MotorAltDec => 17,
            Self::Gps => 176,
            Self::Rtc => 178,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarMotorMsg {
    McGetPosition,
    McGotoFast,
    McSetPosition,
    McSetPosGuiderate,
    McSetNegGuiderate,
    McLevelStart,
    McPecRecordStart,
    McPecPlayback,
    McSetPosBacklash,
    McSetNegBacklash,
    McLevelDone,
    McSlewDone,
    McUnkhown,
    McPecRecordDone,
    McPecRecordStop,
    McGotoSlow,
    McAtIndex,
    McSeekIndex,
    McMovePos,
    McMoveNeg,
    McEnableCordwrap,
    McDisableCordwrap,
    McSetCordwrapPos,
    McPollCordwrap,
    McGetCordwrapPos,
    McGetPosBacklash,
    McGetNegBacklash,
    McSetAutoguideRate,
    McGetAutoguideRate,
    McProgramEnter,
    McProgramInit,
    McProgramData,
    McProgramEnd,
    McGetApproach,
    McSetApproach,
    McGetVer,
}

impl NexstarMotorMsg {
    pub const fn id(self) -> u8 {
        match self {
            Self::McGetPosition => 0x01,
            Self::McGotoFast => 0x02,
            Self::McSetPosition => 0x04,
            Self::McSetPosGuiderate => 0x06,
            Self::McSetNegGuiderate => 0x07,
            Self::McLevelStart => 0x0b,
            Self::McPecRecordStart => 0x0c,
            Self::McPecPlayback => 0x0d,
            Self::McSetPosBacklash => 0x10,
            Self::McSetNegBacklash => 0x11,
            Self::McLevelDone => 0x12,
            Self::McSlewDone => 0x13,
            Self::McUnkhown => 0x14,
            Self::McPecRecordDone => 0x15,
            Self::McPecRecordStop => 0x16,
            Self::McGotoSlow => 0x17,
            Self::McAtIndex => 0x18,
            Self::McSeekIndex => 0x19,
            Self::McMovePos => 0x24,
            Self::McMoveNeg => 0x25,
            Self::McEnableCordwrap => 0x38,
            Self::McDisableCordwrap => 0x39,
            Self::McSetCordwrapPos => 0x3a,
            Self::McPollCordwrap => 0x3b,
            Self::McGetCordwrapPos => 0x3c,
            Self::McGetPosBacklash => 0x40,
            Self::McGetNegBacklash => 0x41,
            Self::McSetAutoguideRate => 0x46,
            Self::McGetAutoguideRate => 0x47,
            Self::McProgramEnter => 0x81,
            Self::McProgramInit => 0x82,
            Self::McProgramData => 0x83,
            Self::McProgramEnd => 0x84,
            Self::McGetApproach => 0xfc,
            Self::McSetApproach => 0xfd,
            Self::McGetVer => 0xfe,
        }
    }

    /// Expected response payload length (excluding terminator `#`), per protocol notes in Dart.
    pub const fn resp_len_bytes(self) -> u8 {
        match self {
            Self::McGetPosition => 3,
            Self::McGotoFast => 3,
            Self::McSetPosition => 0,
            Self::McSetPosGuiderate => 0,
            Self::McSetNegGuiderate => 0,
            Self::McLevelStart => 0,
            Self::McPecRecordStart => 0,
            Self::McPecPlayback => 0,
            Self::McSetPosBacklash => 0,
            Self::McSetNegBacklash => 0,
            Self::McLevelDone => 1,
            Self::McSlewDone => 1,
            Self::McUnkhown => 1,
            Self::McPecRecordDone => 1,
            Self::McPecRecordStop => 0,
            Self::McGotoSlow => 0,
            Self::McAtIndex => 1,
            Self::McSeekIndex => 0,
            Self::McMovePos => 0,
            Self::McMoveNeg => 0,
            Self::McEnableCordwrap => 0,
            Self::McDisableCordwrap => 0,
            Self::McSetCordwrapPos => 0,
            Self::McPollCordwrap => 1,
            Self::McGetCordwrapPos => 3,
            Self::McGetPosBacklash => 1,
            Self::McGetNegBacklash => 1,
            Self::McSetAutoguideRate => 0,
            Self::McGetAutoguideRate => 1,
            Self::McProgramEnter => 1,
            Self::McProgramInit => 1,
            Self::McProgramData => 1,
            Self::McProgramEnd => 1,
            Self::McGetApproach => 1,
            Self::McSetApproach => 0,
            Self::McGetVer => 2,
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Some(match id {
            0x01 => Self::McGetPosition,
            0x02 => Self::McGotoFast,
            0x04 => Self::McSetPosition,
            0x06 => Self::McSetPosGuiderate,
            0x07 => Self::McSetNegGuiderate,
            0x0b => Self::McLevelStart,
            0x0c => Self::McPecRecordStart,
            0x0d => Self::McPecPlayback,
            0x10 => Self::McSetPosBacklash,
            0x11 => Self::McSetNegBacklash,
            0x12 => Self::McLevelDone,
            0x13 => Self::McSlewDone,
            0x14 => Self::McUnkhown,
            0x15 => Self::McPecRecordDone,
            0x16 => Self::McPecRecordStop,
            0x17 => Self::McGotoSlow,
            0x18 => Self::McAtIndex,
            0x19 => Self::McSeekIndex,
            0x24 => Self::McMovePos,
            0x25 => Self::McMoveNeg,
            0x38 => Self::McEnableCordwrap,
            0x39 => Self::McDisableCordwrap,
            0x3a => Self::McSetCordwrapPos,
            0x3b => Self::McPollCordwrap,
            0x3c => Self::McGetCordwrapPos,
            0x40 => Self::McGetPosBacklash,
            0x41 => Self::McGetNegBacklash,
            0x46 => Self::McSetAutoguideRate,
            0x47 => Self::McGetAutoguideRate,
            0x81 => Self::McProgramEnter,
            0x82 => Self::McProgramInit,
            0x83 => Self::McProgramData,
            0x84 => Self::McProgramEnd,
            0xfc => Self::McGetApproach,
            0xfd => Self::McSetApproach,
            0xfe => Self::McGetVer,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarCardinalDirections {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NexstarNs {
    North = 0,
    South = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NexstarEw {
    East = 0,
    West = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NexstarModel {
    Unknown,
    GpsSeries,
    ISeries,
    ISeriesSe,
    Cge,
    AdvancedGt,
    Slt,
    Cpc,
    Gt,
    FourFiveSe,
    SixEightSe,
}

impl NexstarModel {
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => Self::GpsSeries,
            3 => Self::ISeries,
            4 => Self::ISeriesSe,
            5 => Self::Cge,
            6 => Self::AdvancedGt,
            7 => Self::Slt,
            9 => Self::Cpc,
            10 => Self::Gt,
            11 => Self::FourFiveSe,
            12 => Self::SixEightSe,
            _ => Self::Unknown,
        }
    }
}
