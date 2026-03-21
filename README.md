# OpenHanse - README.md

This document is written for human readers. Agentic readers should start with [PARSEME.md](./PARSEME.md) for a structured overview of the project structure, source of truth, working rules, and technical stack for contributing to OpenHanse.

## What Is OpenHanse?

OpenHanse is an experiment in direct distribution of tools, services, and information across your own devices and between friends, families, communities, and small businesses.

> **Current Status**: OpenHanse is still at an early stage and currently focused on defining the vision, exploring the problem space, and building a first prototype.

If you are reading this project as an agent, start with these files:

- [PARSEME.md](./PARSEME.md): A structured overview of the project layout, source-of-truth files, and technical stack.
- [CONTEXT.md](./CONTEXT.md): A broader explanation of the problem space, design goals, and vision for OpenHanse.
- [INSPIRATIONS.md](./INSPIRATIONS.md): Related projects, technologies, and ideas that have influenced the OpenHanse direction.

You can continue reading this README for the concise human-facing overview.

## What Is The Problem?

Thanks to the rise of AI, more people than ever can create software. But distribution, data exchange, and access are still bottlenecks that often require significant effort and expertise.

Today, there are two main approaches to software distribution: websites, which are easy to share but often limited by connectivity, hosting, and platform constraints, and native apps, which are powerful but often depend on tightly controlled platforms.

OpenHanse explores a middle ground: easier distribution and access with fewer technical and economic barriers.

## What We're Going To Build

The long-term vision is an open stack for discovering, distributing, and running local-first applications.

The current implementation path is intentionally smaller:

- `openhanse-hub`: the Rust central hub for rendezvous and relay
- `openhanse-cli`: the command-line gateway flavor used to prove the client/runtime split
- `openhanse-apple`: the first native host app embedding the shared web gateway

Phase 1 still focuses on proving the communication model end to end before broad platform rollout, but the gateway side is now split into reusable Rust layers so both CLI and native host apps can share the same runtime behavior.

## Shared Rust Architecture

```mermaid
graph TD
    Protocol["openhanse-protocol<br/>Shared wire models"]
    GatewayCore["openhanse-gateway-core<br/>Shared OpenHanse client runtime"]
    GatewayCli["openhanse-gateway-cli<br/>Command-line gateway"]
    GatewayWeb["openhanse-gateway-web<br/>REST API + web UI + C ABI"]
    Hub["openhanse-hub<br/>Central rendezvous and relay app"]
    Apple["openhanse-apple<br/>Swift host app with WKWebView"]

    Protocol --> Hub
    Protocol --> GatewayCore
    GatewayCore --> GatewayCli
    GatewayCore --> GatewayWeb
    GatewayWeb --> Apple
```

## Basic Communication

The Phase 1 MVP is built around a direct-first communication model: gateways register with the OpenHanse server, keep their presence alive, and ask the server whether a message should go directly to another gateway or fall back to a relay session.

```mermaid
sequenceDiagram
    autonumber
    participant GatewayA as Gateway A
    participant Server as OpenHanse Server
    participant GatewayB as Gateway B

    GatewayA->>Server: Register peer and direct endpoint
    GatewayB->>Server: Register peer and direct endpoint

    loop Presence heartbeat
        GatewayA->>Server: Refresh presence lease
        GatewayB->>Server: Refresh presence lease
    end

    GatewayA->>Server: Request connection to Gateway B

    alt Direct path available
        Server-->>GatewayA: Return Gateway B direct address
        GatewayA->>GatewayB: Send message directly
        GatewayB-->>GatewayA: Respond directly
    else Direct path unavailable
        Server-->>GatewayA: Return relay session
        GatewayA->>Server: Send message via relay
        Server->>GatewayB: Forward relayed message
        GatewayB-->>Server: Relay response or acknowledgement
        Server-->>GatewayA: Forward response
    end
```

## Get In Touch

If you're interested in learning more, contributing, or just want to chat about the project, feel free to reach out via GitHub.
