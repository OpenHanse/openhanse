# OpenHanse / Roadmap / Phase 1 - Central Server MVP And CLI Gateway

This document defines the steps for the first phase of OpenHanse development, which focuses on building a central server application that implements rendezvous and relay functionality, as well as a simple CLI gateway client that can connect to the server and establish peer-to-peer communication with other gateways.

The concrete MVP example for this phase is a very simple chat flow:

- each gateway can send a text message
- each gateway has a basic messaging endpoint for receiving a message
- both gateways should be able to exchange messages across different networks
- direct peer-to-peer should be preferred whenever possible
- relay through the server must work as a fallback when direct communication is not possible

The goal of this phase is to prove the communication model and establish a working MVP before expanding to other platforms or adding more advanced features.

## Proposed OpenHanse MVP

Phase 1 should be optimized for open-source sustainability:

- prefer direct peer-to-peer delivery to reduce central server load and bandwidth cost
- keep the server responsible for coordination and fallback, not for carrying all traffic by default
- guarantee successful communication through relay fallback even when both peers are behind NAT or on different networks

The first feature used to prove this model should be a minimal chat flow between two gateways.

Build one Rust server application with two logical responsibilities:

- `rendezvous`: peer registration, heartbeats, peer lookup, connection negotiation, and liveness tracking
- `relay`: byte forwarding for sessions that cannot be established directly

Build one simple CLI gateway client with two logical responsibilities:

- a minimal command-oriented way to send and inspect text messages
- a networking layer that can both accept direct peer messages and use the central server for coordination or relay

The CLI client should live in its own repository:

- `openhanse-cli`: protocol test client and reference CLI for the MVP

This keeps the Phase 1 client focused on protocol validation without coupling it to any specific GUI platform. Native GUI clients for Apple, Windows, Linux, and Android can follow later in their own repositories.

The MVP should use in-memory state only.

- Online peers are tracked in memory with expiry based on heartbeats.
- Active relay sessions are tracked in memory and removed after completion or timeout.
- A server restart is allowed to lose state, and clients are expected to re-register.

The MVP should assume a trust-based environment.

- Peers are known devices or known members of a trusted group.
- The system is not designed as an anonymous public network.
- The control plane may validate device identity, but full account management is out of scope.

The communication strategy for v1 should be:

- direct peer-to-peer attempt first
- TCP relay fallback second
- NAT-friendly connection negotiation included in the MVP
- more advanced NAT strategies and optimizations deferred until later

## Initial Interface Decisions

The first control-plane design should be organized around these concepts:

- `PeerId`: stable logical identity of a gateway or peer
- `DeviceKey`: key material used to identify a device and support trust decisions
- `PresenceLease`: registration plus expiry window maintained by heartbeats
- `ConnectRequest`: a request from one peer to reach another peer
- `ConnectDecision`: the server response telling both peers whether to attempt direct connection or use relay
- `RelaySessionId`: identifier used to pair both sides of one relay-backed session

The MVP server should support these operations at a planning level:

- peer register or refresh presence
- heartbeat to keep a peer online
- lookup or negotiation request for a target peer
- direct-attempt instruction when both peers appear reachable enough
- relay-required instruction with a relay session token when direct setup is not suitable

The MVP gateway client should support these operations at a planning level:

- register itself with the server and keep its presence alive
- expose a basic messaging endpoint for direct peer delivery
- send a text message to a target peer through the CLI
- attempt direct delivery when the server returns reachable peer information
- attach to a relay session when direct delivery is not possible

The first transport assumptions should be:

- use an HTTP-friendly control-plane protocol for registration and connect negotiation
- allow NAT-aware direct connection negotiation where practical
- keep a reliable HTTPS/TCP fallback available
- use TCP relay for the first implementation

For the chat example, the transport split should be:

- server coordination over HTTP or HTTPS
- direct message delivery between gateways when reachable
- server relay when direct delivery fails or is not possible

Wire format, final schema details, and full cryptographic protocol design should remain open until the server repository is created.

## Implementation Checklist

### Phase 1.0: Message flow sketch

- [ ] Write the control-plane message sketch for registration, heartbeat, connect negotiation, and relay pairing.
- [ ] Define basic peer and relay session state transitions.
- [ ] Draw a simple sequence diagram for direct-first chat delivery with relay fallback.
- [ ] Sketch the gateway message endpoint shape for receiving a text message.

### Phase 1.1: Rendezvous basics

- [ ] Implement peer registration and heartbeat handling.
- [ ] Store online peers in memory with expiry timestamps.
- [ ] Support peer lookup and basic liveness checks.
- [ ] Return enough connection metadata for gateways to attempt direct delivery.

### Phase 1.2: Connection negotiation

- [ ] Add `ConnectRequest` handling between two peers.
- [ ] Return a `ConnectDecision` that prefers direct setup when conditions look acceptable.
- [ ] Add timeouts and cleanup for incomplete negotiations.
- [ ] Support the chat use case where one gateway wants to deliver a text message to another.

### Phase 1.3: Direct delivery and relay fallback

- [ ] Implement the CLI client direct messaging endpoint and direct send attempt.
- [ ] Implement relay session creation and pairing using `RelaySessionId`.
- [ ] Allow both peers to attach to the same relay session.
- [ ] Forward chat payloads between both peers once paired.
- [ ] Fall back to relay automatically when direct delivery fails.

### Phase 1.4: Hardening

- [ ] Add structured logs, metrics, and timeout handling.
- [ ] Improve cleanup for stale peers, abandoned negotiations, and idle relay sessions.
- [ ] Measure how often delivery succeeds directly versus through relay so hosting cost can be evaluated.

## Non-Goals

The MVP should explicitly not aim for:

- full RustDesk NAT traversal parity
- anonymous public participation
- clustered or highly available deployment
- durable persistence
- complete account management
- native GUI clients for macOS, Windows, Linux, or Android
- marketplace, app distribution, or broader OpenHanse application-layer features

## Acceptance Scenarios

- Two CLI gateways register successfully and remain visible while heartbeats continue.
- A text message sent from one CLI gateway to another prefers a direct peer-to-peer delivery attempt when both peers appear reachable.
- The server instructs relay fallback when direct setup fails or is unsuitable.
- Two gateways behind NAT or on different networks can still exchange messages through the server.
- Peer presence expires automatically after missed heartbeats.
- A relay session only pairs the intended two peers.
- After server restart, in-memory state is lost and peers recover by registering again.
- The CLI output shows sent and received text messages regardless of whether the message used a direct path or relay.
