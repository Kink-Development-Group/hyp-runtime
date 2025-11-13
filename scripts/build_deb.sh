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
TAR_OUT="$PROJECT_ROOT/release/${NAME}-${VERSION}-linux-x64.tar.gz"
DEB_OUT="$PROJECT_ROOT/release/${NAME}_${VERSION}_${ARCH}.deb"
BINARY_NAME=hypnoscript-cli
INSTALL_NAME=hypnoscript

# Check if fpm is available (optional)
HAS_FPM=false
if command -v fpm >/dev/null 2>&1; then
  HAS_FPM=true
  echo "✓ fpm found - will create .deb package"
else
  echo "⚠ fpm not found - will create tar.gz archive only"
  echo "  (Install fpm via 'gem install fpm' to enable .deb packaging)"
fi

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
  # Try to find cargo in common Windows locations
  if [ -f "$HOME/.cargo/bin/cargo" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
  elif [ -f "$USERPROFILE/.cargo/bin/cargo.exe" ]; then
    export PATH="$USERPROFILE/.cargo/bin:$PATH"
  else
    echo 'Error: cargo is not installed. Please install Rust toolchain first.' >&2
    echo 'Visit https://rustup.rs/ to install Rust' >&2
    exit 1
  fi
fi

echo "=== HypnoScript Linux Release Builder ==="
echo ""

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

# 5. TAR.GZ-Archiv erstellen (immer)
echo "📦 Creating TAR.GZ archive..."
cd "$PROJECT_ROOT/release"
tar -czf "$(basename "$TAR_OUT")" -C linux-x64 .
cd "$PROJECT_ROOT"

# 6. .deb-Paket bauen (nur wenn fpm verfügbar)
if [ "$HAS_FPM" = true ]; then
  echo "📦 Creating .deb package..."
  fpm -s dir \
      -t deb \
      -n "$NAME" \
      -v "$VERSION" \
      --architecture "$ARCH" \
      --description "HypnoScript - Esoterische Programmiersprache mit Hypnose-Metaphern" \
      --url "https://github.com/Kink-Development-Group/hyp-runtime" \
      --license "MIT" \
      --maintainer "HypnoScript Team" \
      --prefix /usr/local/bin \
      --deb-compression xz \
      "$RELEASE_DIR/$INSTALL_NAME=$INSTALL_NAME"

  # Paket verschieben
  mv "${NAME}_${VERSION}_${ARCH}.deb" "$DEB_OUT"

  # Checksum erstellen
  echo "🔐 Generating SHA256 checksum for .deb..."
  sha256sum "$DEB_OUT" > "${DEB_OUT}.sha256"
fi

# 7. Checksum für TAR.GZ erstellen
echo "🔐 Generating SHA256 checksum for tar.gz..."
sha256sum "$TAR_OUT" > "${TAR_OUT}.sha256"

# 8. Informationen ausgeben
echo ""
echo "✅ Build complete!"
echo "📦 TAR.GZ Archive: $TAR_OUT"
echo "🔐 TAR.GZ Checksum: ${TAR_OUT}.sha256"

if [ "$HAS_FPM" = true ]; then
  echo "📦 DEB Package: $DEB_OUT"
  echo "🔐 DEB Checksum: ${DEB_OUT}.sha256"
  echo ""
  echo "DEB Package size: $(du -h "$DEB_OUT" | cut -f1)"
fi

echo ""
echo "TAR.GZ size: $(du -h "$TAR_OUT" | cut -f1)"
echo ""
echo "To install from TAR.GZ:"
echo "  tar -xzf $TAR_OUT"
echo "  sudo mv hypnoscript /usr/local/bin/"

if [ "$HAS_FPM" = true ]; then
  echo ""
  echo "To install from DEB:"
  echo "  sudo dpkg -i $DEB_OUT"
fi

echo ""
echo "To verify:"
echo "  hypnoscript --version"
