# OpenHanse / Roadmap

This document contains a high-level roadmap and actionable plan for the OpenHanse project.

## Phase 1: Central Hub MVP / MVP Stage 1

The first phase of OpenHanse development focuses on the first trustworthy real-world MVP: reliable bidirectional communication between an iPhone on mobile data and a MacBook in a private LAN through a publicly reachable Linux hub. Shared Rust gateway layers continue to power both the CLI and native host apps. Relay through the hub is the guaranteed path, while direct peer-to-peer delivery remains preferred when clearly reachable.

## Phase 2: Direct Connection Optimization / MVP Stage 2

The second phase builds on the reliable relay-backed MVP by improving how often direct communication succeeds behind NAT. This phase focuses on reducing relay usage and hosting cost through better reachability decisions, NAT traversal work, and direct-connect improvements, while keeping relay as the fallback safety net. The target validation scenario is direct communication between `openhanse-apple` on 5G and `openhanse-gateway-cli` inside a private local network.

## Phase 3: Hardening

In this phase OpenHanse will move from the staged MVP work toward a more production-ready state. This will include hardening the server and gateway applications as well as providing better documentation and operational guidance. A first release for the platform gateway apps on the corresponding app stores can be targeted once the earlier MVP stages are proven.

## Phase 4 And Beyond

Future phases will be guided by learnings from the initial implementation and user feedback.

## Future Ideas

- [SSH And Peer Service Addons](./FutureIdeas-SSH-And-Peer-Services.md): a short note about treating SSH as an optional peer service addon on supported platforms such as Linux and macOS, alongside the higher-level gateway features.
