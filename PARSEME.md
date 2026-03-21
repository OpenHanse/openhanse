# OpenHanse - PARSEME.md

This document is written for agentic workers (e.g., AI agents) who want to understand the project structure, source of truth, working rules, and technical stack for contributing to OpenHanse. It serves as a starting guide for navigating the various repositories and understanding the core principles that govern the project.

## Key Facts

- Project: OpenHanse
- Goal: Exploring technical and economic opportunities for an open, independent, and distributed software ecosystem.
- Domain: https://openhanse.org (https://openhanse.com is a redirect)

## Project Structure

As a technical project, OpenHanse consists of multiple repositories that serve different purposes. Each repository has its own focus and codebase, but they all contribute to the overall vision of OpenHanse.

- `openhanse`: This repository is like a general project management repository.
- `openhanse-apple`: This repository is containing the source code for the Apple gateway app (iOS, iPadOS and macOS).
- `openhanse-android`: This repository is containing the source code for the Android gateway app. (Not started yet, but planned for the future.)
- `openhanse-windows`: This repository is containing the source code for the Windows gateway app. (Not started yet, but planned for the future.)
- `openhanse-linux`: This repository is containing the source code for the Linux gateway app. (Not started yet, but planned for the future.)
- `openhanse-hub`: This repository is containing the source code for the OpenHanse hub components acting as connection point for the gateway apps.

### Source Of Truth

Each repository has can have a set of common files containing important information about the respective repository and the project as a whole. These files include:

- `README.md`: A general overview of the repository, its purpose, and how it fits into the overall project. Written for human readers.
- `PARSEME.md`: A version of the README.md that is specifically formatted for agentic readers, containing structured information about the repository and its role in the project.
- `CONTEXT.md`: A more detailed exploration of the problem space, design goals, and the vision. This file provides deeper insights into the repository's purpose and how it contributes to the overall project vision.
- `INSPIRATIONS.md`: A collection of related projects, technologies, and ideas that have inspired the repository's design and implementation. This file can provide context on the influences and motivations behind the repository's and/or project development.

## Contributing

Contributions to OpenHanse are welcome and encouraged. If you're interested in contributing, please follow these guidelines:

- Familiarize yourself with the project structure and the specific repository you want to contribute to by reading the relevant `README.md`, `PARSEME.md`, `CONTEXT.md`, and `INSPIRATIONS.md` files.
- If you want to contribute code, please follow the following guidelines:
  - provide proactive and constructive feedback on the task like pointing out potential issues or proposing alternatives
  - always prefer common and simpile solutions over complex ones
  - avoid solutions which ties the project to specific platforms, technologies, or companies
  - do not introduce new dependencies without approval

### Technical Stack

The technical stack for the OpenHanse project is mainly based on platform-agnostic technologies to ensure broad compatibility and ease of contribution. The primary languages and technologies used across the repositories include:

- **Rust** for the business logic, core components, and shared libraries
- **HTML/CSS/JavaScript** for user interfaces

Platform-specific code should be kept to a minimum and just used if needed to deliver the Rust or web-based platform-agnostic code. For example:

- **Swift** for Apple platforms, to launch a WKWebView and launch the Rust code
- **Kotlin** for Android, to launch a WebView and launch the Rust code
- **C#** for the Windows gateway app, to launch a web-based user interface and launch the Rust code