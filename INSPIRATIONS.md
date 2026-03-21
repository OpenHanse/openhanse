# OpenHanse - Inspirations

This document lists inspirations for OpenHanse, including projects, tools, and concepts that have influenced its development.

## Airdroid

Airdroid is a popular application that allows users to manage their Android and iOS devices from a web browser. It provides features such as file transfer, screen mirroring, and remote control. Airdroid's user-friendly interface and comprehensive functionality have inspired OpenHanse's approach to device management and user experience.

### Key Points For OpenHanse

- **Remote Access UX**: Airdroid shows how a browser-based interface can make it easier for users to access and manage their own devices remotely.
- **Platform Independence**: Airdroid's web-based interface allows users to manage their devices regardless of the operating system.

## Rustdesk

The official RustDesk source (https://github.com/rustdesk/rustdesk) and the `rustdesk-server-demo` repository (https://github.com/rustdesk/rustdesk-server-demo) suggest a clear split between a rendezvous/control role and a relay role.

- Peers regularly register themselves with a rendezvous service so they can be found while online.
- Public-key registration and confirmation are handled separately from ordinary presence registration.
- The rendezvous path coordinates connection setup rather than carrying the full session traffic in the normal case.
- Direct peer-to-peer communication is preferred whenever possible to reduce latency and server load.
- Relay is used as a fallback when NAT behavior, proxies, forced-relay settings, or failed direct attempts make a direct path unsuitable.
- Keep-alive and liveness tracking are important so stale registrations disappear and peers can reconnect cleanly.
- The RustDesk demo server is intentionally minimal: it keeps peer state in memory, allows a very small relay flow, and leaves out advanced NAT traversal, persistence, encryption hardening, and production operations.

### Key Points For OpenHanse

- **Proof of Concept**: RustDesk's architecture serves as a proof of concept for a decentralized communication system. While focusing on remote desktop access, the underlying principles of peer discovery, connection management, and fallback mechanisms can be adapted for any general communication use case.
- **Rendezvous/Control Plane**: RustDesk's use of a lightweight control plane for discovery and setup can inform OpenHanse's approach to managing peer connections and session coordination directly between peers.
- **Direct Transport Preference**: RustDesk's preference for direct peer-to-peer communication to reduce latency and server load.
- **Relay Fallback**: RustDesk's use of relay as a fallback helps cover cases where direct communication is not possible.

OpenHanse does not need to copy RustDesk feature-for-feature. The main inspiration is the overall communication shape: lightweight coordination, direct transport when possible, and relay only when needed.

## Alipay

Alipay is one of the best-known "super apps" in China, offering a wide range of services beyond payments in a single app despite restrictions imposed by common gatekeepers. It has a large user base and a strong ecosystem, making it an interesting case study for understanding how super apps operate and succeed.

### Key Points For OpenHanse

- **Mini-Programs**: Alipay's mini-programs allow third-party developers to create and offer services within the app, creating a diverse ecosystem of services that users can access without leaving the app.
- **Integration of Services**: Alipay integrates various services such as payments, financial management, lifestyle services, social features, and government services, providing a comprehensive user experience.

> Note: OpenHanse is not aiming to become another super app or gatekeeper, but understanding the strategies and features of successful super apps like Alipay can provide valuable insights for our own development and growth strategies.

For OpenHanse, the relevant inspiration is not service centralization, but the idea that a host application can provide access to a broader ecosystem of locally useful services with a low-friction user experience.

## Mastodon

Mastodon is a decentralized social media platform that operates on a federated model, allowing users to create their own instances and connect with others across the network. It has gained popularity as an alternative to traditional centralized social media platforms, offering users more control over their data and online presence.

### Key Points For OpenHanse

- **Decentralization**: Mastodon's federated model allows users to create and manage their own instances, giving them more control over their data and online presence and avoiding single points of failure.
- **User Control**: Mastodon emphasizes user control and privacy, allowing users to set their own rules and policies for their instances. This focus on user empowerment can inform OpenHanse's approach to user privacy and data management.
