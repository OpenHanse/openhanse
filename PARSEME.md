# OpenHanse - PARSEME.md

This document is written for agentic workers who need a compact overview of the OpenHanse project structure, source of truth, working rules, and technical stack. Use it as a navigation guide before making changes across the OpenHanse repositories.

## Key Facts

- Project: OpenHanse
- Goal: Explore technical and economic opportunities for an open, independent, and distributed software ecosystem.
- Domain: https://openhanse.org (https://openhanse.com is a redirect)

## Project Structure

OpenHanse consists of a small set of repositories with different roles. The main Rust runtime now lives in the `openhanse` repository itself.

- `openhanse`: Project-level repository for planning, documentation, shared vision, and the shared Rust crates under `Source/openhanse-core`, `Source/openhanse-cli`, and `Source/openhanse-gui`.
- `openhanse-apple`: Source code for the Apple gateway app on iOS, iPadOS, and macOS.
- `openhanse-android`: Planned repository for the Android gateway app. Not started yet.
- `openhanse-windows`: Planned repository for the Windows gateway app. Not started yet.
- `openhanse-linux`: Planned repository for the Linux gateway app. Not started yet.

### Source Of Truth

Repositories may contain a shared set of documentation files. These files are the primary source of truth for contributors and agents:

- `README.md`: General overview of the repository, its purpose, and how it fits into OpenHanse. Written for human readers.
- `PARSEME.md`: Structured summary for agentic readers, focused on project layout, source-of-truth files, and technical direction.
- `CONTEXT.md`: Deeper explanation of the problem space, design goals, and overall vision.
- `INSPIRATIONS.md`: Related projects, technologies, and ideas that influenced the repository and project direction.

## Contributing

Contributions to OpenHanse are welcome. Before making changes:

- Read the relevant `README.md`, `PARSEME.md`, `CONTEXT.md`, and `INSPIRATIONS.md` files for the repository you are changing.
- Provide proactive and constructive feedback when you see risks, unclear assumptions, or simpler alternatives.
- Prefer simple, common solutions over complex ones.
- Avoid solutions that tie the project unnecessarily to a specific platform, technology, or company.
- Do not introduce new dependencies without approval.

### Technical Stack

The OpenHanse technical stack is centered on platform-agnostic technologies to preserve portability and keep contribution paths simple. Primary technologies include:

- **Rust** for the business logic, core components, and shared libraries
- **HTML/CSS/JavaScript** for user interfaces

Platform-specific code should stay minimal and mainly serve as a host for the shared Rust and web-based layers. Examples:

- **Swift** for Apple platforms, to host a `WKWebView` and bridge to Rust code
- **Kotlin** for Android, to host a `WebView` and bridge to Rust code
- **C#** for Windows, to host a web-based user interface and bridge to Rust code
