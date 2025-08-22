#!/bin/bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
SRC_ICON=${1:-"$ROOT_DIR/icon.png"}
DEST_DIR="$ROOT_DIR/app/src-tauri/icons"
ICONSET_DIR="$DEST_DIR/icon.iconset"

if [ ! -f "$SRC_ICON" ]; then
  echo "Source icon not found: $SRC_ICON" >&2
  exit 1
fi

mkdir -p "$DEST_DIR" "$ICONSET_DIR"
# Normalize to PNG RGBA via sips; sips preserves alpha if present
cp -f "$SRC_ICON" "$DEST_DIR/icon-src.png"
sips -s format png "$DEST_DIR/icon-src.png" --out "$DEST_DIR/icon-src.png" >/dev/null

# Generate required sizes for ICNS
for sz in 16 32 64 128 256 512 1024; do
  sips -z $sz $sz "$DEST_DIR/icon-src.png" --out "$ICONSET_DIR/icon_${sz}x${sz}.png" >/dev/null
  # also 2x variants where applicable
  if [ $sz -le 512 ]; then
    db=$((sz*2))
    sips -z $db $db "$DEST_DIR/icon-src.png" --out "$ICONSET_DIR/icon_${sz}x${sz}@2x.png" >/dev/null || true
  fi
done

# Build ICNS
iconutil -c icns "$ICONSET_DIR" -o "$DEST_DIR/icon.icns"

# Provide a mid-size PNG for Tauri macro expectations
cp -f "$ICONSET_DIR/icon_128x128.png" "$DEST_DIR/icon.png"

# Clean up
rm -rf "$ICONSET_DIR" "$DEST_DIR/icon-src.png"

echo "Icon updated at:"
echo "  $DEST_DIR/icon.icns"
echo "  $DEST_DIR/icon.png"
