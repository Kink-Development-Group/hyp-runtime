#!/bin/bash
set -e

# build_deb.sh
# Erstellt Linux-Binary und .deb-Paket für HypnoScript (Rust-Implementation)

NAME=hypnoscript
VERSION=1.0.0
ARCH=amd64

# Projektverzeichnis ermitteln
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

RELEASE_DIR="$PROJECT_ROOT/release/linux-x64"
DEB_OUT="$PROJECT_ROOT/release/${NAME}_${VERSION}_${ARCH}.deb"
BINARY_NAME=hypnoscript-cli
INSTALL_NAME=hypnoscript

# Check for fpm
if ! command -v fpm >/dev/null 2>&1; then
  echo 'Error: fpm is not installed. Please install fpm (e.g. via `gem install fpm`) before running this script.' >&2
  exit 1
fi

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
  echo 'Error: cargo is not installed. Please install Rust toolchain first.' >&2
  exit 1
fi

# 1. Verzeichnisse vorbereiten
echo "📦 Preparing release directory..."
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# 2. Build
echo "🔨 Building HypnoScript CLI (Release)..."
cd "$PROJECT_ROOT"
cargo build --release --package hypnoscript-cli

# 3. Binary kopieren
echo "📋 Copying binary..."
cp "target/release/$BINARY_NAME" "$RELEASE_DIR/$INSTALL_NAME"
chmod +x "$RELEASE_DIR/$INSTALL_NAME"

# 4. Zusätzliche Dateien
echo "📄 Adding additional files..."
if [ -f "$PROJECT_ROOT/README.md" ]; then
  cp "$PROJECT_ROOT/README.md" "$RELEASE_DIR/"
fi

if [ -f "$PROJECT_ROOT/LICENSE" ]; then
  cp "$PROJECT_ROOT/LICENSE" "$RELEASE_DIR/"
fi

echo "$VERSION" > "$RELEASE_DIR/VERSION.txt"

# 5. .deb-Paket bauen (fpm erforderlich)
echo "📦 Creating .deb package..."
fpm -s dir \
    -t deb \
    -n "$NAME" \
    -v "$VERSION" \
    --architecture "$ARCH" \
    --description "HypnoScript - Esoterische Programmiersprache mit Hypnose-Metaphern" \
    --url "https://github.com/yourusername/hypnoscript" \
    --license "MIT" \
    --maintainer "HypnoScript Team" \
    --prefix /usr/local/bin \
    --deb-compression xz \
    "$RELEASE_DIR/$INSTALL_NAME=$INSTALL_NAME"

# 6. Paket verschieben
mv "${NAME}_${VERSION}_${ARCH}.deb" "$DEB_OUT"

# 7. Checksum erstellen
echo "🔐 Generating SHA256 checksum..."
sha256sum "$DEB_OUT" > "${DEB_OUT}.sha256"

# 8. Informationen ausgeben
echo ""
echo "✅ Build complete!"
echo "📦 Package: $DEB_OUT"
echo "🔐 Checksum: ${DEB_OUT}.sha256"
echo ""
echo "Package size: $(du -h "$DEB_OUT" | cut -f1)"
echo ""
echo "To install:"
echo "  sudo dpkg -i $DEB_OUT"
echo ""
echo "To verify:"
echo "  hypnoscript --version"
