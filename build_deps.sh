#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")"

echo "=== Building goldberg (gbe_fork) ==="
cd deps/gbe_fork
git submodule update --init --recursive
bash build_linux.sh
cd ../..

echo "=== Building umu-launcher ==="
cd deps/umu-launcher
./configure.sh --user-install
make
cd ../..

echo "=== All deps built ==="
echo "goldberg x64: deps/gbe_fork/build/x64/steamclient.so"
echo "goldberg x32: deps/gbe_fork/build/x32/steamclient.so"
echo "umu-launcher: deps/umu-launcher/builddir/umu-run"
