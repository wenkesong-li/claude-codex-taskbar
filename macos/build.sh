#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/macos dist-macos
swiftc -swift-version 5 macos/Usage.swift macos/tests/main.swift -o target/macos/usage-tests
target/macos/usage-tests
sdk="$(xcrun --sdk macosx --show-sdk-path)"
for arch in arm64 x86_64; do
  swiftc -swift-version 5 -O -sdk "$sdk" -target "$arch-apple-macosx13.0" \
    macos/Usage.swift macos/main.swift -framework AppKit -framework Security \
    -o "target/macos/cct-$arch"
done
bundle="target/macos/CCT.app"
mkdir -p "$bundle/Contents/MacOS"
lipo -create target/macos/cct-arm64 target/macos/cct-x86_64 -output "$bundle/Contents/MacOS/cct"
lipo -verify_arch arm64 x86_64 "$bundle/Contents/MacOS/cct"
cp macos/Info.plist "$bundle/Contents/Info.plist"
plutil -lint "$bundle/Contents/Info.plist"
codesign --force --sign - "$bundle"
codesign --verify --deep --strict "$bundle"
# Smoke-test startup on the runner; this does not validate a real user's login or menu layout.
"$bundle/Contents/MacOS/cct" &
pid=$!
sleep 3
kill -0 "$pid"
kill "$pid"
ditto -c -k --sequesterRsrc --keepParent "$bundle" dist-macos/CCT-v0.4.7-macos-universal.zip
mkdir -p target/macos/dmg
cp -R "$bundle" target/macos/dmg/
ln -s /Applications target/macos/dmg/Applications
hdiutil create -volname CCT -srcfolder target/macos/dmg -ov -format UDZO dist-macos/CCT-v0.4.7-macos-universal.dmg
cd dist-macos
shasum -a 256 ./*.zip ./*.dmg > SHA256SUMS-macos.txt
