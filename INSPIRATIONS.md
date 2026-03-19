# OpenHanse - Inspirations

This document lists inspirations for OpenHanse, including projects, tools, and concepts that have influenced its development.

## Airdroid

Airdroid is a popular application that allows users to manage their Android and iOS devices from a web browser. It provides features such as file transfer, screen mirroring, and remote control. Airdroid's user-friendly interface and comprehensive functionality have inspired OpenHanse's approach to device management and user experience.

## Rustdesk

The official RustDesk source (https://github.com/rustdesk/rustdesk) and the `rustdesk-server-demo` repository (https://github.com/rustdesk/rustdesk-server-demo) suggest a clear split between a rendezvous/control role and a relay role.

- Peers regularly register themselves with a rendezvous service so they can be found while online.
- Public-key registration and confirmation are handled separately from ordinary presence registration.
- The rendezvous path coordinates connection setup rather than carrying the full session traffic in the normal case.
- Direct peer-to-peer communication is preferred whenever possible to reduce latency and server load.
- Relay is used as a fallback when NAT behavior, proxies, forced-relay settings, or failed direct attempts make a direct path unsuitable.
- Keep-alive and liveness tracking are important so stale registrations disappear and peers can reconnect cleanly.
- The RustDesk demo server is intentionally minimal: it keeps peer state in memory, allows a very small relay flow, and leaves out advanced NAT traversal, persistence, encryption hardening, and production operations.

### Takeaways And Learnings For OpenHanse

For OpenHanse, the most important takeaway is not to copy RustDesk feature-for-feature, but to copy the shape of the solution:

- a lightweight control plane for discovery and setup
- direct transport as the preferred data path
- relay as a controlled fallback

It's important to note that Rustdesk is focusing on transmitting a graphical desktop session, which has different requirements and constraints than the more general/high-level communication use cases OpenHanse is targeting. For example, RustDesk's relay implementation is optimized for streaming desktop video and input events, while OpenHanse's will mainly be used to transfer text, files or any other general data. This means that while the overall architecture and approach can be similar, the specific implementation details and optimizations may differ significantly between the two projects.

## Alipay

Alipay is one of the famous "super apps" in China, offering a wide range of services beyond just payments in one app, despite of restrictions implemented by the common gatekeepers. It has a large user base and a strong ecosystem, making it an interesting case study for understanding how super apps operate and succeed.

### Key Features of Alipay

1. **Payments**: Alipay started as a payment platform and continues to be a core feature, allowing users to make payments, transfer money, and manage their finances.
2. **Financial Services**: Alipay offers various financial services, including wealth management, insurance, and loans, making it a one-stop shop for users' financial needs.
3. **Lifestyle Services**: Users can access a variety of lifestyle services such as food delivery, ride-hailing, and ticket booking directly through the app. These services are integrated as so called "mini-programs" within the app, allowing third-party developers to create and offer their services independently of the iOS or Android app store.
4. **Social Features**: Alipay incorporates social features, allowing users to connect with friends, share red envelopes (digital gifts), and participate in group activities, enhancing user engagement.
5. **E-commerce**: Alipay is integrated with various e-commerce platforms, enabling users to shop online and make payments seamlessly.
6. **Government Services**: Alipay provides access to various government services, such as paying utility bills, taxes, and accessing public transportation services.

### Conclusion

Alipay's success can be attributed to its ability to offer a wide range of services within a single app, creating a seamless and convenient user experience. Its integration of financial, lifestyle, social, and government services has made it an essential part of many users' daily lives, contributing to its status as a leading super app in China and beyond. This application can also be seen as an example to legally open another way to offer services on iOS without being restricted by the common gatekeepers.

> Note: OpenHanse is not aiming to become another super app or gatekeeper, but understanding the strategies and features of successful super apps like Alipay can provide valuable insights for our own development and growth strategies.