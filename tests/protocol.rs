//! Golden vectors for command bytes and response parsing.

use nexstar_lib::{
    build_cancel_goto_command, build_direct_motor_command, build_get_azm_alt_command,
    build_get_device_version, build_get_gps_linked_command, build_get_gps_time_command,
    build_get_latitude_command, build_get_location_command, build_get_longitude_command,
    build_get_model_command, build_get_ra_dec_command, build_get_time_command,
    build_get_tracking_mode_command, build_get_version_command, build_goto_azm_alt_command,
    build_goto_ra_dec_command, build_pass_through_command, build_set_time_command,
    build_set_tracking_mode_command, build_slew_command, build_sync_ra_dec_command,
    NexstarAxis, NexstarCommand, NexstarCommandType, NexstarDevices, NexstarDirection,
    NexstarModel, NexstarMotorMsg, NexstarParsedResponse, NexstarRate, NexstarTrackingMode,
    PassThroughPayload,
};

#[test]
fn goto_ra_dec_golden_bytes() {
    let cmd = build_goto_ra_dec_command(10.0, 88.0, false);
    // 10°→1820→0x071C; 88°→round(88*65536/360)=16020→0x3E94
    assert_eq!(cmd.command_data(), b"R071C,3E94#".to_vec());
}

#[test]
fn sync_ra_dec_golden_bytes() {
    let cmd = build_sync_ra_dec_command(10.0, 88.0, false);
    assert_eq!(cmd.command_data(), b"S071C,3E94#".to_vec());
}

#[test]
fn goto_azm_alt_uses_b_opcode() {
    let cmd = build_goto_azm_alt_command(10.0, 88.0, false);
    assert_eq!(cmd.command_type(), NexstarCommandType::GotoAzmAlt);
    assert_eq!(cmd.command_data(), b"B071C,3E94#".to_vec());
}

#[test]
fn get_queries_have_no_payload() {
    assert_eq!(build_get_ra_dec_command(false).command_data(), b"E");
    assert_eq!(build_get_ra_dec_command(true).command_data(), b"e");
    assert_eq!(build_get_azm_alt_command(false).command_data(), b"Z");
    assert_eq!(build_get_azm_alt_command(true).command_data(), b"z");
    assert_eq!(build_get_tracking_mode_command().command_data(), b"t");
    assert_eq!(build_get_location_command().command_data(), b"w");
    assert_eq!(build_get_time_command().command_data(), b"h");
    assert_eq!(build_get_version_command().command_data(), b"V");
    assert_eq!(build_get_model_command().command_data(), b"m");
    assert_eq!(build_cancel_goto_command().command_data(), b"M");
}

#[test]
fn set_tracking_mode_is_capital_t_with_mode_byte() {
    let cmd = build_set_tracking_mode_command(NexstarTrackingMode::AltAz);
    assert_eq!(cmd.command_type(), NexstarCommandType::SetTrackingMode);
    assert_eq!(cmd.command_data(), [b'T', 1]);
}

#[test]
fn set_time_negative_timezone_wraps_to_u8() {
    let cmd = build_set_time_command(12, 30, 0, 4, 5, 25, -5, 0);
    assert_eq!(
        cmd.command_data(),
        vec![b'H', 12, 30, 0, 4, 5, 25, 251, 0]
    );
}

#[test]
fn slew_variable_rate_encodes_tracking_bytes() {
    let cmd = build_slew_command(
        NexstarRate::Variable,
        NexstarAxis::Ra,
        NexstarDirection::Positive,
        256,
    );
    assert_eq!(
        cmd.command_data(),
        vec![b'P', 0x03, 16, 6, 4, 0, 0, 0]
    );
}

#[test]
fn gps_pass_through_prefix_matches_documented_templates() {
    assert_eq!(
        build_get_gps_linked_command().command_data(),
        vec![b'P', 1, 176, 55, 0, 0, 0, 1]
    );
    assert_eq!(
        build_get_latitude_command().command_data(),
        vec![b'P', 1, 176, 1, 0, 0, 0, 3]
    );
    assert_eq!(
        build_get_longitude_command().command_data(),
        vec![b'P', 1, 176, 2, 0, 0, 0, 3]
    );
    assert_eq!(
        build_get_gps_time_command().command_data(),
        vec![b'P', 1, 176, 51, 0, 0, 0, 3]
    );
}

#[test]
fn get_device_version_pass_through() {
    let cmd = build_get_device_version(NexstarDevices::MotorAzmRa);
    assert_eq!(
        cmd.command_data(),
        vec![b'P', 1, 16, 254, 0, 0, 0, 2]
    );
}

#[test]
fn direct_motor_command_matches_dart_fixture() {
    let slew_cmd = build_direct_motor_command(
        NexstarDevices::MotorAzmRa,
        NexstarMotorMsg::McSetPosGuiderate,
        vec![2, 88],
    );
    assert_eq!(
        slew_cmd.command_data(),
        vec![b'P', 3, 16, 6, 2, 88, 0, 0]
    );
}

#[test]
fn parse_void_ack() {
    let cmd = build_cancel_goto_command();
    match cmd.parse_response(b"#") {
        NexstarParsedResponse::Void(v) => assert!(v.success()),
        other => panic!("expected Void, got {other:?}"),
    }
}

#[test]
fn parse_get_position_ra_dec() {
    let cmd = build_get_ra_dec_command(false);
    let r = cmd.parse_response(b"071C,3E94#");
    match r {
        NexstarParsedResponse::Position(p) => {
            assert!(p.success());
            assert!((p.ra_azm() - 10.0).abs() < 0.01);
            assert!((p.dec_alt() - 88.0).abs() < 0.01);
        }
        other => panic!("expected Position, got {other:?}"),
    }
}

#[test]
fn parse_tracking_mode() {
    let cmd = build_get_tracking_mode_command();
    match cmd.parse_response(b"2#") {
        NexstarParsedResponse::TrackingMode(t) => {
            assert!(t.success());
            assert_eq!(t.mode(), NexstarTrackingMode::EqNorth);
        }
        other => panic!("expected TrackingMode, got {other:?}"),
    }
}

#[test]
fn parse_location_and_time() {
    let loc_cmd = build_get_location_command();
    let loc = vec![10, 20, 30, 0, 40, 50, 60, 1, b'#'];
    match loc_cmd.parse_response(&loc) {
        NexstarParsedResponse::Location(l) => {
            assert!(l.success());
            assert_eq!(l.latitude_deg(), 10);
            assert_eq!(l.longitude_deg(), 40);
        }
        other => panic!("expected Location, got {other:?}"),
    }

    let time_cmd = build_get_time_command();
    let t = vec![14, 30, 45, 6, 15, 24, 200, 1, b'#'];
    match time_cmd.parse_response(&t) {
        NexstarParsedResponse::Time(tm) => {
            assert!(tm.success());
            assert_eq!(tm.h(), 14);
            assert_eq!(tm.month(), 6);
            assert_eq!(tm.timezone(), 200);
        }
        other => panic!("expected Time, got {other:?}"),
    }
}

#[test]
fn parse_gps_linked_and_model() {
    let cmd = build_get_gps_linked_command();
    match cmd.parse_response(b"1#") {
        NexstarParsedResponse::GpsLinked(g) => {
            assert!(g.success());
            assert!(g.linked());
        }
        other => panic!("expected GpsLinked, got {other:?}"),
    }

    let model_cmd = build_get_model_command();
    match model_cmd.parse_response(&[7, b'#']) {
        NexstarParsedResponse::Model(m) => {
            assert!(m.success());
            assert_eq!(m.model(), NexstarModel::Slt);
        }
        other => panic!("expected Model, got {other:?}"),
    }
}

#[test]
fn parse_gps_latitude_longitude_degrees() {
    // 24-bit fraction: degrees = value * 360 / 2^24 — half turn => 0x800000 => 180°
    let lat_cmd = build_get_latitude_command();
    match lat_cmd.parse_response(&[0x80, 0x00, 0x00, b'#']) {
        NexstarParsedResponse::GpsLatitude(l) => {
            assert!(l.success());
            assert!((l.latitude() - 180.0).abs() < 1e-6);
        }
        other => panic!("expected GpsLatitude, got {other:?}"),
    }

    let lon_cmd = build_get_longitude_command();
    match lon_cmd.parse_response(&[0x40, 0x00, 0x00, b'#']) {
        NexstarParsedResponse::GpsLongitude(l) => {
            assert!(l.success());
            assert!((l.longitude() - 90.0).abs() < 1e-6);
        }
        other => panic!("expected GpsLongitude, got {other:?}"),
    }
}

#[test]
fn parse_version_response() {
    let cmd = build_get_version_command();
    match cmd.parse_response(&[1, 2, b'#']) {
        NexstarParsedResponse::Version(v) => {
            assert!(v.success());
            assert_eq!(v.version(), "1.2");
        }
        other => panic!("expected Version, got {other:?}"),
    }
}

#[test]
fn pass_through_parses_motor_payloads() {
    // `success` is true when `awaited_len + 1 == index('#')` (matches Dart).
    let args = vec![3, 16, 0x01, 0xAA, 0xBB, 0xCC, 1];
    let pt = build_pass_through_command(args);
    let raw = pt.parse_response(&[0xAA, 0xBB, b'#']);
    match raw {
        NexstarParsedResponse::PassThrough(p) => {
            assert!(p.success());
            assert_eq!(
                p.data(),
                Some(&PassThroughPayload::Raw(vec![0xAA, 0xBB]))
            );
        }
        other => panic!("expected PassThrough, got {other:?}"),
    }
}

#[test]
fn command_setters_roundtrip_type() {
    let mut cmd = NexstarCommand::new(NexstarCommandType::Echo, Vec::new());
    assert_eq!(cmd.command_type(), NexstarCommandType::Echo);
    cmd.set_command_type(NexstarCommandType::GetModel);
    assert_eq!(cmd.command_type(), NexstarCommandType::GetModel);
}
