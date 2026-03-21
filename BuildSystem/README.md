# OpenHanse / Build System / Container

This build system packages reusable container images for OpenHanse modules. It currently targets the Apple `container` CLI on macOS and exports OCI-compatible tarballs into `Artefact`.

The initial container image here is:

- `rust-linux`: a stable Rust-on-Linux base image for containerized builds

## Design Notes

- Use pinned base image versions for reproducibility.
- Keep images small by starting from slim base images and installing only minimal extras.
- Build scripts should resolve paths via `SCRIPT_DIR` so they work from any current working directory.
- Image tags and output files include the architecture to avoid collisions and make artifacts easier to understand.

The Rust base image is intentionally based on the official Rust slim image family rather than installing Rust manually inside Debian. That keeps the Dockerfile smaller, more stable, and closer to the upstream Rust container maintenance model.

## Prerequisites

```sh
brew install container
container system start
```

For `amd64` container builds on Apple Silicon, Rosetta support may also be needed:

```sh
softwareupdate --install-rosetta --agree-to-license
container system property set build.rosetta true
```

## Build An Image

```sh
./BuildContainerImage.sh rust-linux 1.94.0-bookworm amd64
./BuildContainerImage.sh rust-linux 1.94.0-bookworm arm64
```

This will:

- build the image from `Source/rust-linux`
- tag it as `rust-linux:<version>-<arch>`
- export it into `Artefact` as `rust-linux_<version>_<arch>.oci.tar`

## Artefact

Built OCI archives are written to:

- `Artefact/*.oci.tar`

These tarballs can later be loaded into a compatible container runtime or published as OCI artifacts.

## Notes

- This build system currently uses Apple Container directly. Docker-compatible Dockerfiles are supported, but a Docker CLI fallback is not implemented here yet.
- The current image is a base image, not a final project build artifact by itself.
