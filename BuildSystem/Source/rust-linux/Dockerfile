# syntax=docker/dockerfile:1

# Based on the official Rust image pattern, but pinned to an exact Rust and Debian release
# so the build environment stays reproducible over time.
FROM docker.io/rust:1.94.0-slim-bookworm

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_TERM_COLOR=always \
    PATH=/usr/local/cargo/bin:/usr/local/rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin:/usr/local/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:${PATH}

WORKDIR /workspace

# Keep the image lean and predictable. Add only the small set of packages that are commonly
# useful in containerized Rust build environments.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustc --version \
    && cargo --version

CMD ["bash"]
