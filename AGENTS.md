# OpenHanse - AGENTS.md

This document outlines important information for agents working on the OpenHanse project, including the source of truth for various aspects of the project and its connected repositories.

## Repositories

OpenHanse consists of multiple repositories that serve different purposes:

- `openhanse`: This repository is like a general project management repository.
- `openhanse-apple`: This repository is containing the source code for the Apple gateway app (iOS, iPadOS and macOS).
- `openhanse-android`: This repository is containing the source code for the Android gateway app.
- `openhanse-windows`: This repository is containing the source code for the Windows gateway app.
- `openhanse-linux`: This repository is containing the source code for the Linux gateway app.
- `openhanse-server`: This repository is containing the source code for the OpenHanse server components acting as connection point for the gateway apps.

## Source Of Truth

- Project vision: `openhanse/CONTEXT.md`

## Working Rules

- Prefer simple solutions and avoid unnecessary complexity or over-engineering
- Prefer minimal changes
- Do not introduce new dependencies without approval

## Technical Stack

- **Rust** for the server components, common backend code, and shared libraries
- **Swift** for the Apple gateway app utilizing a web-based user interface and Rust for shared logic
- **Kotlin** for the Android gateway app utilizing a web-based user interface and Rust for shared logic
- **C#** for the Windows gateway app utilizing a web-based user interface and Rust for shared logic
- **C++** for the Linux gateway app utilizing a web-based user interface and Rust for shared logic