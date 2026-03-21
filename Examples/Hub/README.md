# OpenHanse / Examples / Hub

This example shows how to run a simple OpenHanse hub on a Linux machine.

## Build

Build the Linux binary in the `openhanse-hub` repository:

```bash
cd openhanse-hub
./Build.sh
```

This produces artifacts such as:

- `openhanse-hub/Build/openhanse-hub-linux-x86_64`
- `openhanse-hub/Build/openhanse-hub-linux-aarch64`

This deploy example currently assumes `openhanse-hub-linux-x86_64`.

## Upload

Use the deploy script in this example to upload only the binary:

```bash
cd openhanse/Examples/Hub
./DeployHub.sh user@example.com
```

The binary is uploaded to `~/.local/lib/openhanse-hub/openhanse-hub`.

## Run With systemd

The `systemd` setup is intentionally done manually by the server owner.

An example user service file is available at [Linux/systemd/user/openhanse-hub.service](/Volumes/Git/GitHub/OpenHanse/openhanse/Examples/Hub/Linux/systemd/user/openhanse-hub.service). Copy or adapt it into:

```bash
~/.config/systemd/user/openhanse-hub.service
```

Then reload and start it:

```bash
systemctl --user daemon-reload
systemctl --user enable openhanse-hub.service
systemctl --user restart openhanse-hub.service
```

If the service should keep running after logout, enable linger for that user:

```bash
loginctl enable-linger <user>
```

This location is intentionally user-local and fits a `systemctl --user` service without requiring `sudo`.

## Required Runtime Settings

The hub currently expects these defaults:

- TCP HTTP API on `0.0.0.0:8080` via `OPENHANSE_BIND`
- UDP discovery on `0.0.0.0:3478` via `OPENHANSE_DISCOVERY_UDP_BIND`

Make sure the host firewall allows TCP `8080` and UDP `3478` if the hub should be reachable from outside the machine.
