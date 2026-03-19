# OpenHanse / Roadmap / Phase 1 - Central Hub MVP / MVP Stage 1

This document defines the steps for the first phase of OpenHanse development. Phase 1 is MVP Stage 1: the first trustworthy real-world OpenHanse outcome.

The concrete end-to-end goal for this stage is:

- an iPhone running the OpenHanse Apple app on a mobile connection
- a MacBook running the OpenHanse CLI inside a private LAN
- a Linux server running the OpenHanse hub on a public address
- successful bidirectional text communication between the iPhone and the MacBook
- relay through the hub as the guaranteed success path
- direct peer-to-peer delivery preferred when it is obviously reachable, but not required for this stage

The concrete MVP example for this phase is a very simple chat flow:

- each gateway can send a text message
- each gateway can receive a text message
- the iPhone and MacBook can exchange messages in both directions across different networks
- hub relay must work reliably for the target NAT-separated scenario
- direct peer-to-peer delivery remains a preferred optimization, not a dependency for MVP Stage 1

The goal of this phase is to prove reliable communication in the target real-world setup before investing in more advanced NAT traversal or broader platform rollout.

## Proposed OpenHanse MVP

Phase 1 should be optimized for open-source sustainability while still delivering a reliable first MVP:

- prefer direct peer-to-peer delivery when a peer is clearly reachable
- keep the hub responsible for coordination and fallback, not for carrying all traffic by default in the long term
- guarantee successful communication through relay fallback for the target real-world setup
- defer hole punching and more advanced NAT traversal until a later stage

The first feature used to prove this model should be a minimal chat flow between two gateways. The key success case is not just "two local peers can chat", but "a mobile iPhone can reach a MacBook in a private LAN, and the MacBook can reply back, through a Linux-hosted hub".

Build one Rust hub application with two logical responsibilities:

- `rendezvous`: peer registration, heartbeats, peer lookup, connection negotiation, and liveness tracking
- `relay`: forwarding chat payloads for sessions that cannot be established directly

Build one simple shared gateway runtime with two logical responsibilities:

- a minimal command-oriented or web-based way to send and inspect text messages
- a networking layer that can both accept direct peer messages and use the central hub for coordination or relay

This runtime is now implemented as a small set of shared Rust crates:

- `openhanse-protocol`: shared wire models
- `openhanse-gateway-core`: shared OpenHanse client runtime
- `openhanse-gateway-cli`: reference terminal client using the core runtime
- `openhanse-gateway-web`: shared REST, web UI, and C ABI gateway built on the core runtime

The first native host app using the shared web gateway is:

- `openhanse-apple`: Apple gateway app through the C ABI exposed by `openhanse-gateway-web`

The CLI client should live in its own repository:

- `openhanse-cli`: protocol test client and reference CLI for the MVP

This keeps the Phase 1 runtime focused on protocol validation without coupling the networking logic to any single UI. Native GUI clients for Apple, Windows, Linux, and Android can build on top of that shared runtime in their own repositories later.

The MVP should use in-memory state only.

- Online peers are tracked in memory with expiry based on heartbeats.
- Active relay sessions are tracked in memory and removed after completion or timeout.
- A hub restart is allowed to lose state, and clients are expected to re-register.

The MVP should assume a trust-based environment.

- Peers are known devices or known members of a trusted group.
- The system is not designed as an anonymous public network.
- The control plane may validate device identity, but full account management is out of scope.

The communication strategy for MVP Stage 1 should be:

- direct peer-to-peer attempt first when a clearly usable direct address exists
- relay through the hub as the guaranteed path for the target iPhone-mobile <-> MacBook-LAN setup
- simple reachability-based connection negotiation included in this stage
- hole punching and more advanced NAT strategies explicitly deferred

The intended deployment picture for Phase 1 should be:

- `openhanse-hub` runs on a Linux server with public network reachability
- `openhanse-apple` runs on an iPhone and remains usable from a mobile network
- `openhanse-cli` runs on a MacBook inside a home or office LAN
- the hub coordinates discovery and, when needed, relays traffic between both devices

This deployment picture is now also the verified relay-backed reference setup for Phase 1:

- `openhanse-hub` running on a public Linux server
- `openhanse-apple` running on iPhone over 5G in relay mode
- `openhanse-gateway-cli` running in relay mode against the same public hub
- successful bidirectional text messaging through the hub relay path

## Initial Interface Decisions

The first control-plane design should be organized around these concepts:

- `PeerId`: stable logical identity of a gateway or peer
- `DeviceKey`: key material used to identify a device and support trust decisions
- `PresenceLease`: registration plus expiry window maintained by heartbeats
- `ConnectRequest`: a request from one peer to reach another peer
- `ConnectDecision`: the hub response telling both peers whether to attempt direct connection or use relay
- `RelaySessionId`: identifier used to pair both sides of one relay-backed session

The MVP hub should support these operations at a planning level:

- peer register or refresh presence
- heartbeat to keep a peer online
- lookup or negotiation request for a target peer
- direct-attempt instruction when both peers appear clearly reachable enough
- relay-required instruction with a relay session token when direct setup is not suitable
- relay session attach, message forwarding, and receive polling for paired peers

The MVP gateway client runtime should support these operations at a planning level:

- register itself with the hub and keep its presence alive
- expose a basic messaging endpoint for direct peer delivery
- send a text message to a target peer through a thin client shell such as the CLI or Apple app
- attempt direct delivery when the hub returns reachable peer information
- attach to a relay session when direct delivery is not possible
- receive relay-backed messages through the same runtime and inbox flow used for direct messages

### Basic State Transitions

Peer presence state:

- `offline` -> `registered` when a peer completes registration with the hub
- `registered` -> `online` after the first successful heartbeat refresh
- `registered` or `online` -> `offline` when the presence lease expires or the hub restarts
- `offline` -> `registered` again when the peer re-registers after expiry or restart

Relay session state:

- `created` when the hub returns a relay-backed `ConnectDecision`
- `source_attached` or `target_attached` when either peer attaches first
- `paired` when both peers have attached to the same relay session
- `active` while queued relay messages are pending for either side
- `finished` once both peers have attached and all pending relay queues are drained
- `expired` if the session stays idle long enough to miss the relay timeout before completion

These transitions are intentionally simple for MVP Stage 1. The hub keeps all presence and relay session state in memory, uses timeouts as the main recovery mechanism, and expects peers to reconnect or recreate sessions when state is lost.

The first transport assumptions should be:

- use an HTTP-friendly control-plane protocol for registration and connect negotiation
- use an HTTP-friendly relay transport for chat payloads in MVP Stage 1
- keep a reliable fallback available for NAT-separated peers
- defer more advanced direct transports and hole-punching protocols

For the chat example, the transport split should be:

- hub coordination over HTTP or HTTPS
- direct message delivery between gateways when reachable
- hub relay when direct delivery is not possible or not appropriate

Wire format, final schema details, and full cryptographic protocol design should remain open until the implementation matures further.

## Implementation Checklist

### Phase 1.0: Message flow sketch

- [x] Write the control-plane message sketch for registration, heartbeat, connect negotiation, and relay pairing.
- [x] Define basic peer and relay session state transitions.
- [x] Draw a simple sequence diagram for direct-first chat delivery with relay fallback.
- [x] Sketch the gateway message endpoint shape for receiving a text message.

### Phase 1.1: Rendezvous basics

- [x] Implement peer registration and heartbeat handling.
- [x] Store online peers in memory with expiry timestamps.
- [x] Support peer lookup and basic liveness checks.
- [x] Return enough connection metadata for gateways to attempt direct delivery when appropriate.

### Phase 1.2: Connection negotiation

- [x] Add `ConnectRequest` handling between two peers.
- [x] Return a `ConnectDecision` that prefers direct setup when conditions look acceptable.
- [x] Add timeouts and cleanup for incomplete negotiations.
- [x] Support the chat use case where one gateway wants to deliver a text message to another.

### Phase 1.3: Relay-backed delivery

- [x] Implement the CLI client direct messaging endpoint and direct send attempt.
- [x] Move the reusable gateway runtime into the shared `openhanse-gateway-core` crate.
- [x] Split the host-facing REST, web UI, and C ABI layer into `openhanse-gateway-web`.
- [x] Prove the shared runtime through both `openhanse-gateway-cli` and the first Apple app shell.
- [x] Implement relay session transport using `RelaySessionId`.
- [x] Allow both peers to attach to the same relay session.
- [x] Forward chat payloads between both peers once paired.
- [x] Add a relay receive loop to the shared gateway runtime.
- [x] Surface relay-backed messages through the same inbox and event flow used for direct messages.
- [x] Fall back to relay automatically when direct delivery fails or is not possible.
- [x] Validate the primary target scenario: iPhone on mobile data reaches a MacBook inside a private LAN through the Linux-hosted hub.
- [x] Validate reverse communication: the MacBook can send a message back to the iPhone in the same setup.

### Phase 1.4: Minimal hardening for MVP Stage 1

- [x] Add enough logs and status information to understand whether a message used direct delivery or relay.
- [x] Improve cleanup for stale peers, abandoned negotiations, and idle relay sessions.
- [x] Document the canonical deployment and validation flow for the Linux hub, MacBook CLI, and iPhone app.

## Canonical Deployment And Validation Flow

The canonical Phase 1 deployment is:

- `openhanse-hub` on a public Linux server reachable over TCP
- `openhanse-apple` on iPhone with the hub URL pointing at that public server
- `openhanse-gateway-cli` on a MacBook using the same hub URL

Recommended validation order:

1. Start the hub and confirm `GET /health` works from another machine.
2. Start the iPhone app and confirm it registers successfully with the hub.
3. Start the Rust CLI and confirm it registers successfully with the same hub.
4. Set both clients to `relay` mode first and verify bidirectional text messaging.
5. Switch one or both clients to `direct` mode only when a real direct address is expected to be reachable.
6. Use the client status view and event log to confirm whether a message used `direct` or `relay`.

Operational expectations for Phase 1:

- Relay mode is the reference configuration for NAT-separated or cross-network tests.
- Direct mode is a preferred optimization for clearly reachable peers, not a requirement for success.
- If the hub restarts, clients recover by registering again and creating fresh relay sessions as needed.

## Non-Goals

MVP Stage 1 should explicitly not aim for:

- full RustDesk NAT traversal parity
- hole punching or advanced NAT traversal
- anonymous public participation
- clustered or highly available deployment
- durable persistence
- complete account management
- polished production-ready native GUI clients for macOS, Windows, Linux, or Android
- marketplace, app distribution, or broader OpenHanse application-layer features

## Acceptance Scenarios

- An iPhone on a mobile connection can send a message to a MacBook inside a private LAN through the hub.
- A MacBook inside a private LAN can send a message back to an iPhone on a mobile connection through the hub.
- The same relay-backed setup also works with `openhanse-hub` on a public Linux server, `openhanse-apple` on 5G in relay mode, and `openhanse-gateway-cli` in relay mode.
- The target NAT-separated setup succeeds without depending on hole punching.
- When two peers are clearly directly reachable, a direct peer-to-peer delivery attempt is still preferred.
- Two gateways behind NAT or on different networks can still exchange messages through the hub.
- Peer presence expires automatically after missed heartbeats.
- A relay session only pairs the intended two peers.
- After hub restart, in-memory state is lost and peers recover by registering again.
- The sending and receiving clients show text messages regardless of whether the message used a direct path or relay.
