#!/bin/bash
set -xeu

ARCH=${1}
echo "Installing cross..."
cargo install cross --git https://github.com/cross-rs/cross
echo "Building for arch ${ARCH}..."
cross build --release --target ${ARCH}-unknown-linux-gnu
echo "Copying bin file..."
OUT_FOLDER="alfred-console"
BIN_FOLDER="target/${ARCH}-unknown-linux-gnu/release"
mkdir $OUT_FOLDER
cp $BIN_FOLDER/alfred-console $OUT_FOLDER/
cd $OUT_FOLDER
tar czf ../alfred-console_${ARCH}.tar.gz *