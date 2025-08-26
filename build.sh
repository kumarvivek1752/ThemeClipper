#!/usr/bin/env bash
set -euo pipefail

# Extract package name and version from Cargo.toml
PACKAGE_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
PACKAGE_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

DIST_DIR="./dist"
mkdir -p "$DIST_DIR"

# Define targets
TARGETS=(
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  "x86_64-apple-darwin"
)

for TARGET in "${TARGETS[@]}"; do
  echo "Building for $TARGET..."
  cross build --release --target "$TARGET"

  # Set binary filename
  if [[ "$TARGET" == *"windows"* ]]; then
    BIN_NAME="${PACKAGE_NAME}-${PACKAGE_VERSION}-${TARGET}.exe"
    cp "target/$TARGET/release/${PACKAGE_NAME}.exe" "$DIST_DIR/$BIN_NAME"
  else
    BIN_NAME="${PACKAGE_NAME}-${PACKAGE_VERSION}-${TARGET}"
    cp "target/$TARGET/release/${PACKAGE_NAME}" "$DIST_DIR/$BIN_NAME"
  fi
  echo "Built $BIN_NAME"
done

echo "All builds are in $DIST_DIR"
