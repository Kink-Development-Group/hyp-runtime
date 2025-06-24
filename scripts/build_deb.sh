#!/bin/bash
set -e

# Check for fpm
if ! command -v fpm >/dev/null 2>&1; then
  echo 'Error: fpm is not installed. Please install fpm (e.g. via `gem install fpm`) before running this script.' >&2
  exit 1
fi

# build_deb.sh
# Erstellt self-contained Linux-Binary und .deb-Paket

NAME=hypnoscript
VERSION=1.0.0
ARCH=amd64
PUBLISH_DIR=../publish/linux
DEB_OUT=../publish/${NAME}_${VERSION}_${ARCH}.deb

# 1. Build
echo 'Baue self-contained Linux-Binary...'
dotnet publish ../HypnoScript.CLI -c Release -r linux-x64 --self-contained true -p:PublishSingleFile=true -o $PUBLISH_DIR

# 2. Paket bauen (fpm erforderlich)
echo 'Erzeuge .deb-Paket...'
fpm -s dir -t deb -n $NAME -v $VERSION --prefix /usr/local/bin $PUBLISH_DIR/HypnoScript.CLI=$NAME

# 3. Paket verschieben
mv ${NAME}_${VERSION}_${ARCH}.deb $DEB_OUT

echo "Fertig! .deb-Paket liegt in $DEB_OUT"
