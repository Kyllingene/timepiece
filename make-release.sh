#!/bin/bash
#
# Compiles release for both Windows and Linux.
# Then, archives both in their respective formats.
#
# Output archives are in the target/ directory.
#
# NOTE: Requires Windows cross-compiler.
# NOTE: Only designed to run on Linux.

set -e

LINUX_TARGET="x86_64-unknown-linux-gnu"
WINDOWS_TARGET="x86_64-pc-windows-gnu"

echo "=== Building Linux..."
cargo build --release --target $LINUX_TARGET

echo "=== Building Windows..."
cargo build --release --target $WINDOWS_TARGET

echo "=== Done building!"
echo "=== Archiving..."

rm -f target/$LINUX_TARGET-timepiece.tar*

set -e
tar -cf target/$LINUX_TARGET-timepiece.tar target/$LINUX_TARGET/release/tp
xz -z target/$LINUX_TARGET-timepiece.tar
set +e

rm target/$WINDOWS_TARGET-timepiece.zip
zip target/$WINDOWS_TARGET-timepiece.zip target/$WINDOWS_TARGET/release/tp.exe

echo "=== Done!"

