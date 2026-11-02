#!/bin/sh
# Builds and checks everything Under Bedrock ships, in order. Run from the
# mod's root folder; a Rust 1.95 toolchain and python3 with Pillow are needed.
#
#   sh tools/build_all.sh
#
# What it makes:
#   content-mod/ and datapack/   the generated sources and data (textures, blocks, recipes, worldgen)
#   dist/underbedrock.hmod       the content mod, packed by Horizon's own rules
#   dist/underbedrock_core.hnmod the native mod, source-only, as the launcher wants it
#   dist/underbedrock_world.zip  the world datapack, ready to drop into a world's datapacks
set -e
cd "$(dirname "$0")/.."

echo "== textures and content =="
python3 tools/make_textures.py
python3 tools/make_content.py

echo "== the content mod, checked and packed by Horizon's own parser =="
cargo run --manifest-path tools/modbuild/Cargo.toml --quiet -- content-mod dist/underbedrock.hmod

echo "== the native mod: tests, style, lint =="
cd native-mod
cargo test --quiet
cargo fmt -- --check
cargo clippy --quiet --all-targets -- -D warnings
python3 tools/package.py
cd ..
cp native-mod/target/underbedrock_core-*.hnmod dist/underbedrock_core.hnmod

echo "== the native mod's bundle, read the way Horizon's registry reads it =="
python3 tools/nativecheck.py dist/underbedrock_core.hnmod

echo "== the native mod's sources, compiled and run against a stub of the game =="
cd tools/stubcheck
cargo build --quiet
cargo run --quiet -- on
cargo run --quiet -- off
cd ../..

echo "== the world datapack, as a zip a world's datapacks folder takes =="
python3 - <<'PY'
import zipfile
from pathlib import Path
root = Path('datapack/underbedrock_world')
out = Path('dist/underbedrock_world.zip')
out.parent.mkdir(exist_ok=True)
with zipfile.ZipFile(out, 'w', compression=zipfile.ZIP_DEFLATED) as archive:
    for path in sorted(root.rglob('*')):
        if path.is_file():
            archive.write(path, path.relative_to(Path('datapack')).as_posix())
print(f"wrote {out} ({out.stat().st_size} bytes)")
PY

echo "== all built =="
ls -l dist/
