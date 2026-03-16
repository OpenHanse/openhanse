# OpenHanse - Context

This document provides context and background information about the OpenHanse project, including its motivation, goals, technologies, and terminology. It is intended to help agentic readers understand the vision and purpose of OpenHanse.

## Inspiration

Inspired by the Hanseatic League, OpenHanse is a modern digital infrastructure that fosters collaboration and trust among local businesses, communities, and individuals. Just as the Hanseatic League facilitated trade and mutual protection among its members, OpenHanse aims to create a decentralized network that empowers participants to connect, share, and collaborate without relying on centralized platforms or gatekeepers. The name reflects the project's commitment to openness, cooperation, and independence in the digital age.

Several existing technologies and platforms have also influenced OpenHanse, including:

- **RustDesk** which is an open-source remote desktop software that allows users to connect and control their devices remotely. It uses a peer-to-peer architecture to enable direct communication between devices, which can be beneficial for performance and privacy.
- **AliPay** which is a popular mobile payment platform in China that also offers a web-based solution for applications on their platform. It allows developers to create "Mini Programs" that run within the AliPay app, providing a way to reach users without going through traditional app stores.

## Principles

The OpenHanse project is guided by several core principles that shape its design and development:

- **Decentralization**: Supporting self-hosting and peer-to-peer communication to reduce reliance on centralized platforms and services.
- **Open source**: Building in the open with transparent code, shared standards, and the ability for others to inspect, adapt, and contribute.
- **Trust-based exchange**: Supporting collaboration between known, trusted people, devices, and communities rather than assuming a fully anonymous network.
- **Interoperability**: Ensuring that applications and services within the OpenHanse ecosystem can communicate and work together seamlessly, regardless of the underlying technologies or platforms used.
- **Edge-first**: Encouraging applications to run locally on users' devices while still benefiting from network access when available.
- **Sustainability**: Creating a sustainable ecosystem that supports local businesses and communities by enabling them to connect and collaborate.

## Terminology

This list contains terms that are related to OpenHanse and may be used in the context of the project.

- **Gateway** can be an application or service that provides access to the OpenHanse network.
- **Bazaar** is a term that can refer to a marketplace or a place where goods and services are exchanged originally from the Middle East.
- **Fediverse** (Federated universe) is a collection of interconnected servers that communicate with each other using a common protocol.
- **Peer** is a participant in the OpenHanse network. A peer can be an individual, a business, a community, or any entity that connects to the network to share resources, services, or information.

## Candidate Technologies
- **ActivityPub** is a federation protocol for social or publishing-oriented parts of the ecosystem.
- **WASM** (WebAssembly) is a portable runtime for application logic across browsers and other supported environments.
- **Rendezvous** is a peer discovery pattern for finding services, relays and other participants.
- **Relay Server** is a public peer that forwards traffic when direct communication is not possible.
- **DCUtR** is a hole-punching approach for establishing direct peer-to-peer connections through NAT.
- **AutoNAT** is a method for determining whether a peer is reachable directly or behind NAT.
- **HTTP/3 and QUIC** are based on UDP and can provide better performance and reliability for peer-to-peer communication, especially in scenarios with high latency or packet loss.

## Components

### Network Layer

The network layer is about helping peers find each other and communicate without requiring a single central platform.

- peer and service discovery
- relay-based communication when direct peer-to-peer access is not possible
- optional synchronization between devices owned by the same person, family, or business

### Distribution Layer

The distribution layer is about moving applications, updates, and content across trusted devices and marketplaces.

- application and update distribution across trusted marketplaces or direct peer exchange
- content distribution across trusted nodes
- synchronization of software and data between devices and peers

### Application Layer

The application layer is where software runs locally while still benefiting from network access when available.

Examples:

- a family calendar running locally but readable by trusted household members
- a file storage app hosted on a personal computer and reachable from phone or tablet
- a restaurant system running on a local device with a website, menu, and reservation data nearby

## Domains

**OpenHanse.org** is the official domain for the organisation, where all official information, documentation, and resources related to OpenHanse will be hosted in the future while **OpenHanse.com** will redirect to OpenHanse.org to avoid fraud and confusion. The project may also use subdomains for specific services or applications in the future.