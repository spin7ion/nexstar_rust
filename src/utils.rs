//! Angle and binary helpers.
//!
//! Same conversions as `NexstarUtils` in [**nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter).

use crate::constants::NexstarCardinalDirections;

/// Converts an ASCII hex fraction from the mount into degrees (0–360).
#[must_use]
pub fn string_to_double_rad(s: &str, precise: bool) -> f64 {
    let v = u32::from_str_radix(s.trim(), 16).unwrap_or(0);
    if precise {
        f64::from(v) * 360.0 / 4_294_967_296.0
    } else {
        f64::from((v & 0xFFFF) as u16) * 360.0 / 65_536.0
    }
}

#[must_use]
pub fn int_to_string_rad(value: u32, precise: bool) -> String {
    if precise {
        format!("{:08X}", value)
    } else {
        format!("{:04X}", (value & 0xFFFF) as u16)
    }
}

/// Converts user degrees to NexStar 16-bit fraction (wrapping `round` like Dart `int`).
#[must_use]
pub fn convert_degrees_to_nexstar(deg: f64) -> u32 {
    let r = (deg * 65_536.0 / 360.0).round() as i32;
    r as u32
}

#[must_use]
pub fn convert_degrees_to_precise_nexstar(deg: f64) -> u32 {
    let r = (deg * 4_294_967_296.0 / 360.0).round() as i64;
    r as u32
}

#[must_use]
pub fn tracking_rate_to_bytes(tracking_rate: i32) -> [u8; 2] {
    let scaled = tracking_rate.saturating_mul(4);
    let hi = (scaled / 256) as u8;
    let lo = (scaled % 256) as u8;
    [hi, lo]
}

#[must_use]
pub fn dms_to_deg(dms: [i32; 3], direction: NexstarCardinalDirections) -> f64 {
    let mut deg = f64::from(dms[0]) + f64::from(dms[1]) / 60.0 + f64::from(dms[2]) / 3600.0;
    if matches!(
        direction,
        NexstarCardinalDirections::South | NexstarCardinalDirections::West
    ) {
        deg *= -1.0;
    }
    deg
}

#[must_use]
pub fn deg_to_dms(deg: f64) -> [i32; 3] {
    let d = deg.trunc() as i32;
    let m_float = (deg - f64::from(d)) * 60.0;
    let m = m_float.trunc() as i32;
    let s_float = (m_float - f64::from(m)) * 60.0;
    let s = s_float.trunc() as i32;
    [d, m, s]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::NexstarCardinalDirections;

    #[test]
    fn int_to_string_rad_standard_and_precise() {
        assert_eq!(int_to_string_rad(0x0714, false), "0714");
        assert_eq!(int_to_string_rad(0x1234ABCD, true), "1234ABCD");
        assert_eq!(int_to_string_rad(0xFFFF_0001, false), "0001");
        assert_eq!(int_to_string_rad(0xFFFF, false), "FFFF");
    }

    #[test]
    fn string_to_double_rad_roundtrip_16_bit() {
        let n = convert_degrees_to_nexstar(10.0_f64);
        let s = int_to_string_rad(n, false);
        let back = string_to_double_rad(&s, false);
        let expected = f64::from((n & 0xFFFF) as u16) * 360.0 / 65_536.0;
        assert!(
            (back - expected).abs() < 1e-12,
            "hex {s}: got {back}, expected {expected}"
        );
    }

    #[test]
    fn string_to_double_rad_precise_matches_scale() {
        let s = "000F4240";
        let d = string_to_double_rad(s, true);
        let expected = 0x000F_4240u32 as f64 * 360.0 / 4_294_967_296.0;
        assert!((d - expected).abs() < 1e-12);
    }

    #[test]
    fn tracking_rate_to_bytes_matches_dart_formula() {
        assert_eq!(tracking_rate_to_bytes(0), [0, 0]);
        assert_eq!(tracking_rate_to_bytes(256), [4, 0]);
        let scaled = 99_i32.saturating_mul(4);
        assert_eq!(tracking_rate_to_bytes(99), [(scaled / 256) as u8, (scaled % 256) as u8]);
    }

    #[test]
    fn dms_to_deg_north_east_positive() {
        let d = dms_to_deg([10, 20, 30], NexstarCardinalDirections::North);
        assert!((d - (10.0 + 20.0 / 60.0 + 30.0 / 3600.0)).abs() < 1e-9);
    }

    #[test]
    fn dms_to_deg_south_west_negative() {
        let d = dms_to_deg([1, 0, 0], NexstarCardinalDirections::South);
        assert!((d + 1.0).abs() < 1e-9);
        let d = dms_to_deg([1, 0, 0], NexstarCardinalDirections::West);
        assert!((d + 1.0).abs() < 1e-9);
    }

    #[test]
    fn deg_to_dms_roundtrip_simple() {
        let dms = deg_to_dms(12.5);
        assert_eq!(dms, [12, 30, 0]);
    }
}
