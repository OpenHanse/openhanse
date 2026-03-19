# OpenHanse / Roadmap / Phase 2 - Direct Communication Through NAT / MVP Stage 2

This document defines the second MVP stage for OpenHanse development. Phase 2 builds on the reliable relay-backed communication from MVP Stage 1 and shifts the engineering focus toward reducing or avoiding hub relay load by making direct communication succeed in more real-world NAT-separated scenarios.

The final target outcome for this stage is:

- `openhanse-hub` remains publicly reachable and available as the coordination and fallback service
- `openhanse-apple` can run on an iPhone over 5G
- `openhanse-gateway-cli` can run on a MacBook or other machine inside a private local network
- both peers can still fall back to relay when needed
- the preferred and measured success path for the target setup becomes direct communication rather than relay

The concrete final measurement for Phase 2 is:

- direct communication between `openhanse-apple` on a 5G connection and `openhanse-gateway-cli` in a local network

MVP Stage 2 should start only after MVP Stage 1 has already proven reliable bidirectional relay-backed communication for the same topology.

## Phase 2 Goal

The goal of this stage is:

- improve how often direct communication succeeds behind NAT
- reduce relay traffic and therefore reduce ongoing server load and hosting cost
- preserve the Stage 1 relay path as the guaranteed fallback
- make transport choice observable so direct success can be measured rather than assumed

The main question in this stage is no longer "can both devices communicate at all?" but:

- "can both devices communicate directly in the target NAT-separated setup?"
- "when direct communication fails, does the system recover cleanly through relay?"
- "does the direct-connect complexity earn its keep by reducing real relay usage?"

## Stage Promise

Phase 2 should keep the Stage 1 user promise intact:

- communication must remain reliable
- relay must remain available as the safety net
- the hub must still coordinate discovery, liveness, and negotiation
- direct communication should become the preferred success path for supported topologies

This means Phase 2 is an optimization and reachability stage, not a rewrite of the architecture proven in Phase 1.

## Proposed Direction

Phase 2 should move the system from "relay-first in practice" toward "direct-first in practice" for supported topologies.

That requires four improvements working together:

- better reachability discovery
- better connection decisions
- a more NAT-friendly direct transport strategy
- better measurement of what actually happened in the field

The target outcome is not full RustDesk-style parity across every NAT type. The target outcome is a narrower but real win: the known iPhone-over-5G to local-network CLI scenario can succeed directly often enough to materially reduce relay dependence.

## Core Work Areas

### 1. Reachability Discovery

Gateways should stop relying only on manually configured or guessed local addresses.

Expected direction:

- discover the outward-facing address information that matters for direct communication
- distinguish local bind addresses from advertised reachability information
- detect when a peer is clearly relay-only
- avoid advertising stale or obviously unusable direct addresses

This area exists to replace the current MVP-stage assumption that a local Wi-Fi IP is a useful direct hint.

### 2. Connection Negotiation

The hub and gateway runtime should make better direct-versus-relay decisions.

Expected direction:

- attach richer reachability metadata to presence records
- improve how the hub decides whether to return direct information or a relay session
- support direct-attempt strategies that are informed by real reachability evidence
- keep negotiation simple enough that failure still falls back safely to relay

The direct path should become intentional rather than optimistic guesswork.

### 3. NAT Traversal And Direct Transport

Phase 2 should introduce the first practical NAT traversal work for OpenHanse.

Expected direction:

- evaluate which direct transport is the best fit for Phase 2
- research and implement a limited hole-punching strategy if it fits the architecture
- target the specific mobile-to-home-network scenario first instead of trying to solve every topology
- keep direct transport compatible with the existing rendezvous and relay model

This stage may still remain partial in NAT coverage as long as the target topology is improved meaningfully and safely.

#### To Be Decided

- `STUN-like reachability discovery`
  This would let gateways learn how they appear from outside their local network and advertise more realistic reachability information than a private LAN IP alone. It is the most natural first step if OpenHanse wants better direct-connect decisions without immediately committing to a full hole-punching design.

- `Direct TCP coordination`
  This would keep the direct path conceptually close to the current HTTP and TCP-based MVP by improving how peers attempt direct TCP connections after hub negotiation. It may be simpler to integrate with the existing Phase 1 transport model, but it may also be less effective than UDP-based techniques for harder NAT scenarios.

- `UDP-based hole-punching path`
  This would move closer to the approach used by systems such as RustDesk for NAT traversal. It offers the strongest long-term path toward direct communication behind NAT, but it also introduces the most protocol, implementation, and operational complexity, so it needs deliberate evaluation before becoming the primary Phase 2 direction.

### 4. Measurement And Operational Cost

Phase 2 should produce evidence, not just theory.

Expected direction:

- record whether each delivery used direct or relay
- track direct success rate for targeted scenarios
- track fallback rate from failed direct attempts to relay
- measure relay traffic reduction for the target topology

Without this measurement, there is no reliable way to say whether Phase 2 succeeded.

## Phase 2 Architecture Intent

The architecture from Phase 1 should remain intact:

- `openhanse-hub` remains the public rendezvous and relay service
- `openhanse-protocol` remains the shared wire contract
- `openhanse-gateway-core` remains the shared gateway runtime for transport logic
- `openhanse-gateway-cli` remains the reference terminal client
- `openhanse-gateway-web` remains the host-facing web, REST, and C ABI layer
- `openhanse-apple` remains the first native host application on top of the shared runtime

Phase 2 should extend this architecture rather than bypass it. In particular:

- NAT traversal logic should live in the shared Rust gateway layers, not in app-specific UI code
- direct-versus-relay metrics should come from shared runtime behavior
- the Apple app and Rust CLI should continue to share the same transport decisions and runtime semantics

## Implementation Checklist

### Phase 2.0: Direct-connect design

- [ ] Define the direct-connect success criteria for the target Apple 5G <-> local-network CLI scenario.
- [ ] Choose the initial direct transport strategy for Phase 2.
- [ ] Define what new reachability metadata peers should advertise to the hub.
- [ ] Define the updated direct-versus-relay negotiation flow.

### Phase 2.1: Reachability discovery

- [ ] Implement a better model for advertised direct reachability than a raw local LAN IP.
- [ ] Distinguish between bind address, local network address, and externally useful reachability hints.
- [ ] Prevent obviously unusable direct advertisements from being preferred.
- [ ] Keep relay-only mode available for diagnostics and fallback testing.

### Phase 2.2: NAT traversal and direct transport

- [ ] Implement the first Phase 2 direct-connect mechanism aimed at NAT-separated peers.
- [ ] Support the target topology of iPhone on 5G reaching a peer inside a private local network.
- [ ] Keep the direct path integrated with the shared gateway runtime.
- [ ] Preserve automatic fallback to relay when direct establishment fails.

### Phase 2.3: Observability and measurement

- [ ] Record direct-attempt success and failure outcomes in shared runtime status or logs.
- [ ] Expose enough runtime information to verify whether the final path was direct or relay.
- [ ] Measure relay reduction for the target topology.
- [ ] Capture enough data to compare Stage 1 relay behavior with Stage 2 direct behavior.

### Phase 2.4: End-to-end validation

- [ ] Validate direct communication from `openhanse-apple` on 5G to `openhanse-gateway-cli` in a local network.
- [ ] Validate reverse direct communication from `openhanse-gateway-cli` in a local network back to `openhanse-apple` on 5G.
- [ ] Validate that failed direct attempts still fall back cleanly to relay.
- [ ] Document the canonical deployment and validation flow for the Phase 2 target topology.

## Suggested Validation Sequence

The recommended validation order for Phase 2 should be:

1. Keep the public Linux hub setup from Phase 1.
2. Confirm the Phase 1 relay path still works unchanged.
3. Enable the new direct-connect capability for both peers.
4. Attempt direct communication from Apple on 5G to the CLI inside a private network.
5. Attempt reverse direct communication from the CLI back to Apple on 5G.
6. Confirm whether the runtime reports `direct` for the final successful delivery path.
7. Intentionally break or disable the direct path and confirm clean fallback to relay.

This sequence ensures Phase 2 improves the system without breaking the Phase 1 guarantee.

## Non-Goals

MVP Stage 2 should explicitly not aim for:

- removing relay fallback
- replacing the proven Stage 1 hub role
- solving every NAT topology from the start
- broad production hardening unrelated to direct communication
- cluster, multi-region, or enterprise-scale hub deployment
- polished platform features unrelated to direct transport improvement

It is acceptable for Phase 2 to support only a narrower set of direct-connect scenarios as long as:

- the target topology is improved clearly
- the fallback remains reliable
- the relay cost benefit is measurable

## Acceptance Criteria

- Direct communication can succeed between `openhanse-apple` on 5G and `openhanse-gateway-cli` in a local network.
- Reverse direct communication can also succeed for the same topology.
- Failed direct attempts fall back automatically to the proven relay path.
- The runtime can report whether a message was delivered directly or through relay.
- Relay usage decreases measurably for the target topology compared with Phase 1 behavior.
- The added direct-connect complexity is justified by real reduction in relay dependence and server load.
