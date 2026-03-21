# OpenHanse / Source/openhanse-cli

Rust flavor of the OpenHanse CLI reference client.

This crate now lives in the main `openhanse` repository under `Source/openhanse-cli` and is intended to mirror the Phase 1 command surface:

- interactive chat mode with a local message receiver
- hub mode through `--peer-mode hub`
- combined peer mode through `--peer-mode both`
- `/lookup`
- `/connect`
- `/inbox`
- `/quit`

## Design Goal

This implementation should prove that the shared Rust crates are shaped well for real gateway clients:

- `openhanse-core` owns the shared runtime, including models, gateway behavior, and hub capabilities
- `openhanse-gui` builds on top of that core for the shared REST, web UI, and native host integration surface
- `openhanse-cli` stays focused on CLI orchestration, terminal interaction, and presenting runtime state

## Usage

Run two peers in separate terminals:

```bash
cargo run -- --id gateway-a --target gateway-b
cargo run -- --id gateway-b --target gateway-a
```

The client will:

- start the shared gateway runtime
- expose a local `/message` receiver through that runtime
- register the peer with the hub
- keep heartbeats alive
- print incoming messages live
- send messages through the shared runtime, which currently uses `direct_tcp` when a credible HTTP direct path exists and otherwise falls back to relay in `auto` mode

Defaults:

- server: `http://127.0.0.1:8080`
- host: auto-detected local IPv4 for outbound traffic, with fallback to `127.0.0.1`
- communication mode: `auto`
- ports: stable defaults derived from `--id`, with `gateway-a` using `17441` and `gateway-b` using `17442`

Optional flags:

- `--peer-mode <gateway|hub|both>`
- `--server <url>`
- `--host <host>`
- `--port <port>`
- `--communication-mode <auto|direct|relay>`
- `--display-name <name>`
- `--device-key <key>`
- `--heartbeat-interval-secs <seconds>`

Use `--communication-mode relay` to disable direct advertising and force relay-backed messaging for cross-network tests. Use `--communication-mode direct` to force strict `direct_tcp` delivery for diagnostics.

To run the binary as a standalone hub process:

```bash
cargo run --release -- --id hub --peer-mode hub --server http://0.0.0.0:8080
```

## Current Limitation

`direct_udp` discovery candidates are collected as groundwork for later hole punching, but actual delivery still uses `direct_tcp` or relay today. NAT-separated peers may therefore still fall back to relay in many real-world scenarios.
