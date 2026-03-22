# OpenHanse Network

## Shared Rust Architecture

```mermaid
graph TD
    Core["<b>openhanse-core</b><br/>Shared runtime, models and business logic"]
    Cli["<b>openhanse-cli</b><br/>Command line interface"]
    Gui["<b>openhanse-gui</b><br/>Graphical interface"]
    Apple["<b>openhanse-apple</b><br/>iOS, iPadOS & macOS App"]

    Core --> Cli
    Core --> Gui
    Gui --> Apple
```

## Basic Communication Flow

The current MVP is built around a direct-first communication model: peers register with the OpenHanse hub, keep their presence alive, and ask the shared runtime whether a message should go directly to another peer or fall back to a relay session.

```mermaid
sequenceDiagram
    autonumber
    participant GatewayA as Peer A
    participant Server as OpenHanse Hub
    participant GatewayB as Peer B

    GatewayA->>Server: Register peer and direct endpoint
    GatewayB->>Server: Register peer and direct endpoint

    loop Presence heartbeat
        GatewayA->>Server: Refresh presence lease
        GatewayB->>Server: Refresh presence lease
    end

    GatewayA->>Server: Request connection to Gateway B

    alt Direct path available
        Server-->>GatewayA: Return Peer B direct address
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
