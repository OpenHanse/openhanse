# OpenHanse / Roadmap

This document contains a high-level roadmap and actionable plan for the OpenHanse project.

## Phase 1: Central Server MVP And Shared Gateway Layers

The first phase of OpenHanse development focuses on building a central server application that implements rendezvous and relay functionality, plus a shared set of Rust gateway layers that can be hosted by both CLI and native apps. The concrete MVP example remains a simple chat app with direct peer-to-peer delivery preferred and server relay used as a fallback.

## Phase 2: Cross-Platform Gateway Hosts

The second phase will expand the gateway applications to other platforms, including iOS, Android, Windows, and Linux. This will involve building thin platform hosts around the shared gateway web/runtime layers while adapting lifecycle, packaging, and platform-specific integration points as needed.

## Phase 3: Hardening

In this phase OpenHanse will move from an MVP to a more production-ready state. This will include hardening the server and gateway applications as well as providing better documentation and operational guidance. A first release for the platform gateway apps on the corresponding app stores will be targeted in this phase.

## Phase 4 And Beyond

Future phases will be guided by learnings from the initial implementation and user feedback.
