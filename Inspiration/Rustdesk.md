# RustDesk Research Note

The official RustDesk source (https://github.com/rustdesk/rustdesk) and the `rustdesk-server-demo` repository (https://github.com/rustdesk/rustdesk-server-demo) suggest a clear split between a rendezvous/control role and a relay role.

- Peers regularly register themselves with a rendezvous service so they can be found while online.
- Public-key registration and confirmation are handled separately from ordinary presence registration.
- The rendezvous path coordinates connection setup rather than carrying the full session traffic in the normal case.
- Direct peer-to-peer communication is preferred whenever possible to reduce latency and server load.
- Relay is used as a fallback when NAT behavior, proxies, forced-relay settings, or failed direct attempts make a direct path unsuitable.
- Keep-alive and liveness tracking are important so stale registrations disappear and peers can reconnect cleanly.
- The RustDesk demo server is intentionally minimal: it keeps peer state in memory, allows a very small relay flow, and leaves out advanced NAT traversal, persistence, encryption hardening, and production operations.

## Takeaways And Learnings For OpenHanse

For OpenHanse, the most important takeaway is not to copy RustDesk feature-for-feature, but to copy the shape of the solution:

- a lightweight control plane for discovery and setup
- direct transport as the preferred data path
- relay as a controlled fallback

It's important to note that Rustdesk is focusing on transmitting a graphical desktop session, which has different requirements and constraints than the more general/high-level communication use cases OpenHanse is targeting. For example, RustDesk's relay implementation is optimized for streaming desktop video and input events, while OpenHanse's will mainly be used to transfer text, files or any other general data. This means that while the overall architecture and approach can be similar, the specific implementation details and optimizations may differ significantly between the two projects.
