#!/bin/bash
set -euo pipefail

cargo build --release

rm -rf build
mkdir -p build/ build/res build/bin
mkdir -p build/res/goldberg/linux32 build/res/goldberg/linux64

cp target/release/partydeck build/
cp LICENSE build/ && cp COPYING.md build/thirdparty.txt
cp res/GamingModeLauncher.sh build/
cp res/splitscreen_kwin.js res/splitscreen_kwin_vertical.js build/res

# goldberg from source build
cp deps/gbe_fork/build/x64/steamclient.so build/res/goldberg/linux64/steamclient.so
cp deps/gbe_fork/build/x32/steamclient.so build/res/goldberg/linux32/steamclient.so

# umu-launcher from source build
cp deps/umu-launcher/builddir/umu-run build/bin/

# gamescope from source build (optional - falls back to system gamescope)
if [[ -f deps/gamescope/build/src/gamescope ]]; then
  cp deps/gamescope/build/src/gamescope build/bin/gamescope-kbm
elif command -v gamescope >/dev/null 2>&1; then
  cp "$(command -v gamescope)" build/bin/gamescope-kbm
else
  echo "WARNING: gamescope not found, splitscreen may not work"
fi
