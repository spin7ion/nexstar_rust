# nexstar_lib

[![docs.rs](https://docs.rs/nexstar_lib/badge.svg)](https://docs.rs/nexstar_lib)
[![crates.io](https://img.shields.io/crates/v/nexstar_lib.svg)](https://crates.io/crates/nexstar_lib)

Rust helpers for the **Celestron NexStar** handset serial protocol: build command frames and parse `#`-terminated responses.

This crate is a **Rust port** of the Dart/Flutter NexStar protocol library [**spin7ion/nexstar_flutter**](https://github.com/spin7ion/nexstar_flutter) (command builders and response parsing). Behavior follows that project, with a few protocol fixes noted in the source where the original had clear mistakes.

**API reference (this crate on [Docs.rs](https://docs.rs/)):** [nexstar_lib on docs.rs](https://docs.rs/nexstar_lib/latest/nexstar_lib/)

This crate handles **encoding and decoding only**. It does not open a serial port; wire those bytes up with your platform’s UART/USB stack (for example [`serialport`](https://crates.io/crates/serialport) on desktop).

## Quick start

Add the crate to your project:

```toml
[dependencies]
nexstar_lib = "0.1"
```

Build bytes and parse a reply:

```rust
use nexstar_lib::{build_get_version_command, NexstarParsedResponse};

let v = build_get_version_command();
let reply_from_mount = [4u8, 21, b'#'];
match v.parse_response(&reply_from_mount) {
    NexstarParsedResponse::Version(r) if r.success() => {
        assert_eq!(r.version(), "4.21");
    }
    _ => panic!("expected version"),
}
```

## Examples

| Example            | What it shows                          |
|--------------------|----------------------------------------|
| `build_commands`   | Common command strings / byte vectors  |
| `parse_responses`  | Turning sample replies into structs    |

```bash
cargo run --example build_commands
cargo run --example parse_responses
```

## API overview

- **`factory`** — `build_*` functions for goto, sync, slew, GPS pass-through, etc.
- **[`NexstarCommand::parse_response`](https://docs.rs/nexstar_lib/latest/nexstar_lib/struct.NexstarCommand.html#method.parse_response)** — dispatch to [`NexstarParsedResponse`](https://docs.rs/nexstar_lib/latest/nexstar_lib/enum.NexstarParsedResponse.html) (position, version, void ack, …).
- **`utils`** — degree ↔ NexStar fraction, slew rate bytes, DMS helpers.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

## Publishing (maintainers)

See the Cargo Book: [Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html). Before the first upload, set `repository` (and optionally `homepage`) in `Cargo.toml`, run `cargo publish --dry-run`, then `cargo login` and `cargo publish`.
