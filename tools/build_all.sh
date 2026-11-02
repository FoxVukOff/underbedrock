#!/bin/sh
# Builds and checks everything Under Bedrock ships, in order. Run from the
# mod's root folder; a Rust 1.95 toolchain and python3 (no packages) are needed.
#
#   sh tools/build_all.sh
#
# What it makes:
#   content-mod/ and datapack/   the generated sources and data (textures, blocks, recipes, worldgen)
#   release/underbedrock.hmod       the content mod, packed by Horizon's own rules
#   release/underbedrock_core.hnmod the native mod, source-only, as the launcher wants it
#   release/underbedrock_world.zip  the world datapack, ready to drop into a world's datapacks
set -e
cd "$(dirname "$0")/.."

echo "== textures and content =="
python3 tools/make_textures.py
python3 tools/make_content.py

echo "== worldgen states, in the format Horizon's decoder reads =="
python3 tools/check_worldgen.py

echo "== the content declarations, the way Horizon's mod loader reads them =="
python3 tools/check_content.py

echo "== the content mod, checked and packed by Horizon's own parser =="
cargo run --manifest-path tools/modbuild/Cargo.toml --quiet -- content-mod release/underbedrock.hmod

echo "== the native mod: tests, style, lint =="
cd native-mod
cargo test --quiet
cargo fmt -- --check
cargo clippy --quiet --all-targets -- -D warnings
python3 tools/package.py
cd ..
cp native-mod/target/underbedrock_core-*.hnmod release/underbedrock_core.hnmod

echo "== the native mod's bundle, read the way Horizon's registry reads it =="
python3 tools/nativecheck.py release/underbedrock_core.hnmod

echo "== the native mod's sources, compiled and run against a stub of the game =="
cd tools/stubcheck
cargo build --quiet
cargo run --quiet -- on
cargo run --quiet -- off
cd ../..

echo "== the distributable set: hmod, hnmod, datapack zip (pack.mcmeta at its root), install bundle =="
python3 tools/pack_release.py

echo "== all built =="
ls -l release/
