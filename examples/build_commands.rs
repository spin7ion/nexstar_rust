//! Build NexStar command bytes (e.g. to send over a serial port).
//!
//! Run: `cargo run --example build_commands`

use nexstar_lib::{
    build_get_ra_dec_command, build_get_version_command, build_goto_ra_dec_command,
    build_set_tracking_mode_command, NexstarTrackingMode,
};

fn main() {
    let goto = build_goto_ra_dec_command(10.0, 88.0, false);
    println!(
        "Goto RA/Dec: {}",
        String::from_utf8_lossy(&goto.command_data())
    );

    let get_pos = build_get_ra_dec_command(false);
    println!("Get RA/Dec: {}", String::from_utf8_lossy(&get_pos.command_data()));

    let version = build_get_version_command();
    println!(
        "Firmware query: {}",
        String::from_utf8_lossy(&version.command_data())
    );

    let track = build_set_tracking_mode_command(NexstarTrackingMode::AltAz);
    println!("Set tracking Alt-Az: {:?}", track.command_data());
}
