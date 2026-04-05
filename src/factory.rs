//! High-level command builders.
//!
//! API shape follows [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter) `NexstarCommandFactory`.

use crate::command::{NexstarCommand, PassThroughCommand};
use crate::constants::{
    NexstarAxis, NexstarCommandType, NexstarDevices, NexstarDirection, NexstarEw, NexstarMotorMsg,
    NexstarNs, NexstarRate, NexstarTrackingMode,
};
use crate::utils::{
    convert_degrees_to_nexstar, convert_degrees_to_precise_nexstar, int_to_string_rad,
    tracking_rate_to_bytes,
};

#[must_use]
pub fn build_command(command_type: NexstarCommandType, arguments: Vec<u8>) -> NexstarCommand {
    NexstarCommand::new(command_type, arguments)
}

#[must_use]
pub fn build_get_ra_dec_command(precise: bool) -> NexstarCommand {
    let t = if precise {
        NexstarCommandType::GetPreciseRaDec
    } else {
        NexstarCommandType::GetRaDec
    };
    NexstarCommand::new(t, Vec::new())
}

#[must_use]
pub fn build_get_azm_alt_command(precise: bool) -> NexstarCommand {
    let t = if precise {
        NexstarCommandType::GetPreciseAzmAlt
    } else {
        NexstarCommandType::GetAzmAlt
    };
    NexstarCommand::new(t, Vec::new())
}

#[must_use]
pub fn build_goto_ra_dec_command(ra: f64, dec: f64, precise: bool) -> NexstarCommand {
    let ra_int = if precise {
        convert_degrees_to_precise_nexstar(ra)
    } else {
        convert_degrees_to_nexstar(ra)
    };
    let dec_int = if precise {
        convert_degrees_to_precise_nexstar(dec)
    } else {
        convert_degrees_to_nexstar(dec)
    };
    let args = format!(
        "{},{}#",
        int_to_string_rad(ra_int, precise),
        int_to_string_rad(dec_int, precise)
    );
    let t = if precise {
        NexstarCommandType::GotoPreciseRaDec
    } else {
        NexstarCommandType::GotoRaDec
    };
    NexstarCommand::new(t, args.into_bytes())
}

#[must_use]
pub fn build_goto_azm_alt_command(azm: f64, alt: f64, precise: bool) -> NexstarCommand {
    let azm_int = if precise {
        convert_degrees_to_precise_nexstar(azm)
    } else {
        convert_degrees_to_nexstar(azm)
    };
    let alt_int = if precise {
        convert_degrees_to_precise_nexstar(alt)
    } else {
        convert_degrees_to_nexstar(alt)
    };
    let args = format!(
        "{},{}#",
        int_to_string_rad(azm_int, precise),
        int_to_string_rad(alt_int, precise)
    );
    let t = if precise {
        NexstarCommandType::GotoPreciseAzmAlt
    } else {
        NexstarCommandType::GotoAzmAlt
    };
    NexstarCommand::new(t, args.into_bytes())
}

#[must_use]
pub fn build_sync_ra_dec_command(ra: f64, dec: f64, precise: bool) -> NexstarCommand {
    let ra_int = if precise {
        convert_degrees_to_precise_nexstar(ra)
    } else {
        convert_degrees_to_nexstar(ra)
    };
    let dec_int = if precise {
        convert_degrees_to_precise_nexstar(dec)
    } else {
        convert_degrees_to_nexstar(dec)
    };
    let args = format!(
        "{},{}#",
        int_to_string_rad(ra_int, precise),
        int_to_string_rad(dec_int, precise)
    );
    let t = if precise {
        NexstarCommandType::SyncPreciseRaDec
    } else {
        NexstarCommandType::SyncRaDec
    };
    NexstarCommand::new(t, args.into_bytes())
}

#[must_use]
pub fn build_get_tracking_mode_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::GetTrackingMode, Vec::new())
}

#[must_use]
pub fn build_set_tracking_mode_command(mode: NexstarTrackingMode) -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::SetTrackingMode,
        vec![mode.to_byte()],
    )
}

#[must_use]
pub fn build_slew_command(
    rate: NexstarRate,
    axis: NexstarAxis,
    direction: NexstarDirection,
    rate_value: i32,
) -> NexstarCommand {
    let mut arguments = vec![0u8; 7];
    arguments[0] = rate.byte();
    arguments[1] = match axis {
        NexstarAxis::Azm | NexstarAxis::Ra => NexstarDevices::MotorAzmRa.id(),
        NexstarAxis::Alt | NexstarAxis::Dec => NexstarDevices::MotorAltDec.id(),
    };
    arguments[2] = direction.byte();
    if rate == NexstarRate::Variable {
        let b = tracking_rate_to_bytes(rate_value);
        arguments[3] = b[0];
        arguments[4] = b[1];
    } else {
        arguments[3] = rate_value.clamp(0, 255) as u8;
        arguments[4] = 0;
    }
    arguments[5] = 0;
    arguments[6] = 0;
    NexstarCommand::new(NexstarCommandType::SlewRate, arguments)
}

#[must_use]
pub fn build_get_location_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::GetLocation, Vec::new())
}

#[must_use]
#[allow(clippy::too_many_arguments)] // Mirrors the Dart factory API.
pub fn build_set_location_dms_command(
    latitude_deg: u8,
    latitude_min: u8,
    latitude_sec: u8,
    longitude_deg: u8,
    longitude_min: u8,
    longitude_sec: u8,
    direction_ns: NexstarNs,
    direction_ew: NexstarEw,
) -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::SetLocation,
        vec![
            latitude_deg,
            latitude_min,
            latitude_sec,
            direction_ns as u8,
            longitude_deg,
            longitude_min,
            longitude_sec,
            direction_ew as u8,
        ],
    )
}

#[must_use]
pub fn build_get_time_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::GetTime, Vec::new())
}

#[must_use]
#[allow(clippy::too_many_arguments)] // Mirrors the Dart factory API.
pub fn build_set_time_command(
    h: u8,
    m: u8,
    s: u8,
    month: u8,
    day: u8,
    year: u8,
    timezone: i8,
    dst: u8,
) -> NexstarCommand {
    let tz = if timezone < 0 {
        (256i16 + i16::from(timezone)) as u8
    } else {
        timezone as u8
    };
    NexstarCommand::new(
        NexstarCommandType::SetTime,
        vec![h, m, s, month, day, year, tz, dst],
    )
}

#[must_use]
pub fn build_get_gps_linked_command() -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::IsGpsLinked,
        vec![1, NexstarDevices::Gps.id(), 55, 0, 0, 0, 1],
    )
}

#[must_use]
pub fn build_get_latitude_command() -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::GetLatitude,
        vec![1, NexstarDevices::Gps.id(), 1, 0, 0, 0, 3],
    )
}

#[must_use]
pub fn build_get_longitude_command() -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::GetLongitude,
        vec![1, NexstarDevices::Gps.id(), 2, 0, 0, 0, 3],
    )
}

#[must_use]
pub fn build_get_gps_time_command() -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::GetGpsTime,
        vec![1, NexstarDevices::Gps.id(), 51, 0, 0, 0, 3],
    )
}

#[must_use]
pub fn build_get_version_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::GetVersion, Vec::new())
}

#[must_use]
pub fn build_get_device_version(device: NexstarDevices) -> NexstarCommand {
    NexstarCommand::new(
        NexstarCommandType::GetDeviceVersion,
        vec![1, device.id(), 254, 0, 0, 0, 2],
    )
}

#[must_use]
pub fn build_get_model_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::GetModel, Vec::new())
}

#[must_use]
pub fn build_cancel_goto_command() -> NexstarCommand {
    NexstarCommand::new(NexstarCommandType::CancelGoto, Vec::new())
}

#[must_use]
pub fn build_pass_through_command(args: Vec<u8>) -> PassThroughCommand {
    PassThroughCommand::new(args)
}

#[must_use]
pub fn build_direct_motor_command(
    mot_id: NexstarDevices,
    msg: NexstarMotorMsg,
    mut data: Vec<u8>,
) -> PassThroughCommand {
    let payload_len = data.len();
    let len = payload_len + 1;
    if data.len() < 3 {
        data.resize(3, 0);
    } else if data.len() > 3 {
        data.truncate(3);
    }
    let mut cmd = Vec::with_capacity(7);
    cmd.push(len as u8);
    cmd.push(mot_id.id());
    cmd.push(msg.id());
    cmd.extend_from_slice(&data);
    cmd.push(msg.resp_len_bytes());
    build_pass_through_command(cmd)
}
