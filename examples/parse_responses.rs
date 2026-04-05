//! Parse `#`-terminated replies from the mount into structured values.
//!
//! Run: `cargo run --example parse_responses`

use nexstar_lib::{
    build_get_ra_dec_command, build_get_version_command, NexstarParsedResponse,
};

fn main() {
    let cmd = build_get_version_command();
    let reply = cmd.parse_response(&[4, 21, b'#']);
    match reply {
        NexstarParsedResponse::Version(v) if v.success() => {
            println!("Controller version: {}", v.version());
        }
        other => println!("Unexpected: {other:?}"),
    }

    let pos = build_get_ra_dec_command(false);
    let reply = pos.parse_response(b"071C,3E94#");
    match reply {
        NexstarParsedResponse::Position(p) if p.success() => {
            println!(
                "RA/Azm ≈ {:.4}°, Dec/Alt ≈ {:.4}°",
                p.ra_azm(),
                p.dec_alt()
            );
        }
        other => println!("Unexpected: {other:?}"),
    }
}
