#!/bin/bash

### BuildContainerImage.sh
# Usage examples:
#   ./BuildContainerImage.sh rust-linux 1.94.0 amd64
#   ./BuildContainerImage.sh rust-linux 1.94.0 arm64
###

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/Source"
OUTPUT_DIR="$SCRIPT_DIR/Artefact"

usage() {
    echo "Usage: $0 <image-name> <image-version> <arch>"
    echo
    echo "Supported arches: amd64, arm64"
}

if [ "$#" -ne 3 ]; then
    usage
    exit 1
fi

if ! command -v container >/dev/null 2>&1; then
    echo "Missing Apple Container CLI: install it with 'brew install container'" >&2
    exit 1
fi

IMAGE_NAME="$1"
IMAGE_VERSION="$2"
ARCH="$3"
IMAGE_SOURCE_DIR="$SOURCE_DIR/${IMAGE_NAME}"

case "$ARCH" in
    amd64|arm64) ;;
    *)
        echo "Unsupported arch: $ARCH" >&2
        usage
        exit 1
        ;;
esac

if [ ! -d "$IMAGE_SOURCE_DIR" ]; then
    echo "Missing image source directory: $IMAGE_SOURCE_DIR" >&2
    exit 1
fi

mkdir -p "$OUTPUT_DIR"

IMAGE_TAG="${IMAGE_NAME}:${IMAGE_VERSION}-${ARCH}"
OUTPUT_FILE="$OUTPUT_DIR/${IMAGE_NAME}_${IMAGE_VERSION}_${ARCH}.oci.tar"

echo "Building $IMAGE_TAG from $IMAGE_SOURCE_DIR..."
(
    cd "$IMAGE_SOURCE_DIR"
    container build --arch "$ARCH" --tag "$IMAGE_TAG" --file "Dockerfile" .
)

echo "Exporting $IMAGE_TAG to $OUTPUT_FILE..."
rm -f "$OUTPUT_FILE"
container image save "$IMAGE_TAG" --output "$OUTPUT_FILE"

echo "OCI image archive written to $OUTPUT_FILE"
