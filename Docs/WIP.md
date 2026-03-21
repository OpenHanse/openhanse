# OpenHanse / Docs / WIP

This document is the active work-in-progress roadmap for OpenHanse.

The old Phase 1 milestone is complete enough to stop treating it as the current planning document. The codebase has already been refactored into the new shared structure:

- `openhanse-core`
- `openhanse-cli`
- `openhanse-gui`
- `openhanse-apple`

The current work should therefore focus on the next meaningful engineering step inside that structure rather than continuing to plan around the removed split repositories.

## Current Baseline

OpenHanse currently has:

- a shared Rust core that combines protocol models, gateway behavior, and hub behavior
- a CLI that can run in `gateway`, `hub`, or `both` mode
- a GUI layer that can start the unified peer runtime and expose the web UI and C ABI
- an Apple host app on top of the GUI layer
- relay-backed communication as the reliable fallback path
- direct delivery as the preferred optimization when reachability looks credible

This means the current baseline is no longer "build the first relay-backed MVP". That part is already proven well enough to move on.

## Current Goal

The current goal is:

- move from optimistic direct attempts toward explicit NAT-aware connection setup
- build the remaining pieces required for full UDP hole punching
- improve how often direct communication succeeds in real NAT-separated scenarios
- reduce dependence on relay traffic where it is safe and worthwhile
- keep relay as the guaranteed recovery and fallback path
- keep the shared `PeerMode` architecture intact while improving the runtime behavior

The target validation scenario remains:

- `openhanse-apple` on iPhone over 5G
- `openhanse-cli` on a MacBook or another machine inside a private local network
- direct communication preferred
- relay fallback still reliable

## Active Architecture

The active architecture is now:

- `openhanse-core` as the shared runtime
- `openhanse-cli` as the terminal and service-style peer wrapper
- `openhanse-gui` as the host-facing UI, REST, and C ABI layer
- `openhanse-apple` as the thin native host app

Within that architecture:

- `PeerMode::Gateway` runs only gateway behavior
- `PeerMode::Hub` runs only hub behavior
- `PeerMode::Both` runs both roles in one executable

This architecture should now be treated as the source of truth for implementation work.

## Main Work Areas

### 1. Direct Reachability Quality

The runtime should become better at deciding which direct addresses are actually useful and which ones are only local or observational hints.

Work in this area includes:

- improving advertised reachability data
- separating local bind addresses from externally useful direct hints
- reducing obviously wrong or stale direct advertisements
- avoiding false-positive direct TCP assumptions based only on hub-observed registration source addresses
- keeping relay-only operation available for diagnostics

### 2. Connection Decision Quality

The hub and gateway parts of the shared runtime should make better direct-versus-relay decisions.

Work in this area includes:

- enriching peer reachability metadata
- improving negotiation logic in the shared core
- making NAT behavior part of the direct-versus-relay decision
- making direct attempts intentional instead of optimistic guesswork
- preserving clean relay fallback

### 3. NAT Traversal

OpenHanse should now begin practical NAT traversal work for the target topology.

Work in this area includes:

- keeping UDP discovery as the basis for NAT analysis
- classifying NAT behavior instead of only discovering one observed address
- exchanging credible UDP candidates through the hub
- coordinating simultaneous UDP punch attempts between peers
- adding a real direct UDP transport after connection setup succeeds
- deciding retry timing, fallback timing, and relay handoff rules
- targeting the Apple-over-5G to LAN-peer scenario first

### 4. Full Hole Punching Path

The next major milestone is no longer "just better direct connect". It is a complete hole-punching path that can be attempted intentionally and then abandoned cleanly when conditions are wrong.

Work in this area includes:

- keeping the current NAT classification step as preflight input
- defining the message flow for punch coordination through the hub
- exchanging source and target UDP candidates with timestamps and attempt windows
- opening and maintaining the local UDP socket that will later carry peer traffic
- promoting a successful punch into an active direct UDP session
- falling back to relay when punch coordination expires or fails

### 4. PeerMode Product Completion

`PeerMode` now exists in the runtime, but product integration is not fully finished.

Work in this area includes:

- making hub-only usage a first-class CLI deployment flow
- improving GUI behavior when running in `hub` mode
- improving GUI behavior when running in `both` mode
- documenting deployment flows for each peer mode

### 5. Observability And Measurement

OpenHanse should produce evidence for whether direct-connect work is actually helping.

Work in this area includes:

- surfacing NAT classification in status and logs
- surfacing whether delivery used direct or relay
- recording when direct was skipped because of symmetric NAT
- recording punch-attempt success, timeout, and fallback outcomes
- recording direct attempt success and fallback outcomes
- measuring relay reduction for the target topology
- keeping enough runtime information to debug field failures

## Active Checklist

### Runtime

- [x] Improve the reachability model used for direct advertisement with NAT behavior metadata.
- [x] Distinguish clearly between local bind addresses and externally useful direct hints.
- [x] Improve direct-versus-relay decision logic in the shared runtime so symmetric NAT does not attempt direct first.
- [ ] Preserve clean automatic relay fallback for all direct failure cases.

### NAT Work

- [x] Choose the first explicit NAT traversal strategy for the target topology: UDP-based hole punching with relay fallback.
- [x] Implement the first NAT preflight step: dual UDP discovery probes for NAT classification.
- [ ] Register and expose NAT classification cleanly in all runtime surfaces.
- [ ] Exchange UDP candidates between peers through the hub.
- [ ] Define and implement the first explicit punch-coordination flow.
- [ ] Add a real direct UDP message transport after successful punch setup.
- [ ] Decide and implement retry timing and timeout rules for punch attempts.
- [ ] Implement explicit relay fallback after punch failure or expiry.
- [ ] Validate the target Apple 5G to LAN peer scenario.
- [ ] Validate the reverse direction from LAN peer back to Apple over 5G.

### PeerMode

- [ ] Make `PeerMode::Hub` feel complete as a deployable service flow.
- [ ] Make `PeerMode::Both` feel complete as a combined peer flow.
- [ ] Adjust GUI behavior so hub-only mode does not expose misleading gateway-only actions.
- [ ] Document recommended usage patterns for `gateway`, `hub`, and `both`.

### Deployment And Docs

- [ ] Update remaining roadmap text that still refers to removed Phase 1 repository structure.
- [ ] Document the canonical hub deployment flow using `openhanse-cli --peer-mode hub`.
- [ ] Document the canonical peer deployment flow using the unified core runtime.
- [ ] Tighten Linux deployment examples around the current CLI-based hub flow.

### Measurement

- [ ] Report NAT behavior clearly in status and logs.
- [ ] Report final delivery mode clearly in status and logs.
- [ ] Record when direct was skipped because NAT classification made it non-credible.
- [ ] Record punch attempt start, success, timeout, and fallback.
- [ ] Record failed direct attempts that fall back to relay.
- [ ] Measure direct success rate in the target topology.
- [ ] Measure relay usage reduction against the current baseline.

## Non-Goals For The Current WIP Window

- rebuilding the architecture from scratch again
- reintroducing separate standalone hub or CLI repositories
- removing relay fallback
- solving every NAT topology immediately
- building full ICE/TURN/STUN parity before proving the first OpenHanse-specific punch flow
- broad production hardening unrelated to the current communication model
- advanced platform expansion before the shared runtime behavior is better proven

## Done Recently

- [x] move the main Rust crates into the `openhanse` repository
- [x] create a single `openhanse-core`
- [x] replace the old split `Source/Rust` structure
- [x] add `PeerMode` to the shared runtime
- [x] wire `PeerMode` into CLI and GUI
- [x] remove the obsolete standalone `openhanse-cli` and `openhanse-hub` repositories
- [x] stop treating hub-observed public TCP source addresses as credible direct TCP reachability
- [x] add dual UDP discovery probes to classify NAT behavior as predictable or symmetric

## Success Criteria For The Next Step

The next WIP cycle should be considered successful if:

- the unified runtime remains stable
- deployment through `openhanse-cli --peer-mode hub` is the documented and normal hub path
- NAT classification is visible and trustworthy during field testing
- the runtime can coordinate at least one explicit UDP hole-punch attempt path
- direct UDP communication succeeds more often in the target NAT-separated scenario
- relay remains reliable when direct delivery does not work
- the runtime can show whether the real-world outcome was direct or relay
