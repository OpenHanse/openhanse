# OpenHanse / Roadmap / Phase 2 - Direct Connection Optimization / MVP Stage 2

This document defines the second MVP stage for OpenHanse development. Phase 2 builds on the reliable relay-backed communication from MVP Stage 1 and focuses on reducing relay dependence and hosting cost.

The goal of this stage is:

- improve how often direct communication succeeds in real-world topologies
- reduce relay traffic and infrastructure cost
- keep relay as the fallback safety net when direct connectivity is not possible

MVP Stage 2 should start only after MVP Stage 1 has proven reliable bidirectional communication between an iPhone on mobile data and a MacBook inside a private LAN through a Linux-hosted hub.

## Proposed Direction

Phase 2 should keep the same product promise as Stage 1 while improving efficiency:

- direct delivery becomes the stronger engineering focus
- relay remains the guaranteed fallback path
- transport choices should be guided by measurable reductions in relay usage

The primary question in this stage is no longer "can both devices communicate at all?" but "how often can they communicate directly without losing reliability?"

## Expected Work Areas

### Reachability and connection decisions

- improve how gateways decide whether to advertise direct addresses
- improve how the hub decides when to return a direct connection versus relay
- distinguish friendly same-LAN or publicly reachable cases from clearly relay-only cases

### NAT traversal research and experiments

- evaluate which NAT-separated scenarios are worth targeting first
- research practical hole-punching approaches that fit the OpenHanse architecture
- validate whether the targeted direct-connect improvements are realistic for mobile and residential networks

### Direct transport improvements

- explore more NAT-friendly direct transports where appropriate
- evaluate whether HTTP remains sufficient or whether a different direct transport would be beneficial
- keep the direct transport design compatible with relay fallback

### Instrumentation and cost awareness

- measure direct connection success versus relay usage
- track which network topologies succeed directly and which still require relay
- use those measurements to guide prioritization and estimate hosting cost impact

## Non-Goals

MVP Stage 2 should explicitly not aim for:

- removing relay fallback
- replacing the proven Stage 1 relay path before direct connectivity is validated
- broad production hardening unrelated to direct-connect improvement
- solving every NAT topology from the start

## Acceptance Criteria

- Direct delivery works in more real-world topologies than MVP Stage 1.
- Relay usage decreases measurably for supported scenarios.
- Failed direct attempts fall back automatically to the working relay path.
- The system can report or measure whether delivery succeeded directly or through relay.
- The additional direct-connect complexity is justified by real reductions in relay dependence.
