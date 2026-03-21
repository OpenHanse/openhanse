# OpenHanse / Roadmap / Future Idea - SSH And Peer Service Addons

This note captures one possible future direction for OpenHanse after the core gateway and network model have matured further.

## Idea

OpenHanse gateways should remain focused on higher-level peer capabilities such as chat, presence, connection negotiation, relay fallback, and custom application integrations.

Alongside that shared gateway model, OpenHanse could later support optional peer service addons on a subset of platforms. One example would be SSH access on systems such as Linux and macOS.

## Abstract

The key idea is to avoid treating SSH as a built-in gateway feature. Instead, SSH would be exposed as an optional trusted peer service that can be advertised by a gateway runtime and reached through the same OpenHanse network decisions:

- `relay`
- `direct_tcp`
- `direct_udp` in a later hole-punching stage

In that model, OpenHanse would transport a bidirectional stream to a trusted peer service, while SSH itself would remain unchanged on top of that transport.

## Why This Shape Fits Better

- It keeps the gateway concept focused on higher-level OpenHanse features.
- It allows service access to exist only on platforms where it makes sense.
- It avoids turning OpenHanse into a general-purpose VPN or proxy system.
- It leaves room for other future peer services beyond SSH.

## Scope Reminder

If this idea is explored later, it should stay narrowly scoped:

- trusted peers only
- explicitly shared services only
- optional per platform
- not a generic arbitrary proxy for unrestricted traffic
