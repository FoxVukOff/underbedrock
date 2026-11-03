#!/usr/bin/env python3
"""Собрать всё, что раздаёт Under Bedrock, в папку release/ — одним Питоном, без Раста.

    python tools/pack_release.py

Делает три файла — их и коммитят в репозиторий (бинарников сверх этого в нём
быть не должно):
    release/underbedrock.hmod          контент-мод (zip: horizon-mod.toml первым, затем assets/ и data/)
    release/underbedrock_core.hnmod    нативный мод (тем же packager'ом, что и в native-mod/tools)
    release/underbedrock_world.zip     датапак мира, pack.mcmeta в корне архива: кладётся в datapacks/ как есть

Нужен только Python 3.11+ (tomllib). Раст не обязателен: он нужен лишь проверкам
(cargo test в native-mod, stubcheck), но не самим файлам мода. Исходники блоков,
рецептов и мира уже лежат в content-mod/ и datapack/ — генерировать их заново
(make_content.py, make_textures.py) нужно только правки ради.
"""
import hashlib
import json
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RELEASE = ROOT / 'release'

# A fixed stamp for every archive entry: the same sources pack to the same
# bytes on every run, so a rebuild that changed nothing leaves the release
# hashes alone.
STAMP = (2026, 1, 1, 0, 0, 0)


def put(archive, src, name):
    """One file into the archive, with the fixed stamp and plain permissions."""
    info = zipfile.ZipInfo(name, date_time=STAMP)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.external_attr = 0o644 << 16
    archive.writestr(info, src.read_bytes())

def pack_hmod():
    """The content mod: a zip with its manifest first, then every file, sorted."""
    src = ROOT / 'content-mod'
    manifest = 'horizon-mod.toml'
    names = [manifest] + sorted(
        p.relative_to(src).as_posix()
        for p in src.rglob('*')
        if p.is_file() and p.name != manifest
    )
    out = RELEASE / 'underbedrock.hmod'
    with zipfile.ZipFile(out, 'w', zipfile.ZIP_DEFLATED) as archive:
        for name in names:
            put(archive, src / name, name)
    return out


def pack_hnmod():
    """The native mod, by the bundle's own packager (source-only .hnmod).
    The packager prints the artifact it just wrote as JSON; taking the path
    from its own mouth is exact - a glob-and-sort over target/ could pick up
    a stale build of another version."""
    done = subprocess.run(
        [sys.executable, 'tools/package.py'],
        cwd=ROOT / 'native-mod',
        check=True,
        capture_output=True,
        text=True,
    )
    built = Path(json.loads(done.stdout)['file'])
    if not built.is_file():
        raise SystemExit(f'package.py named no file on disk: {built}')
    out = RELEASE / 'underbedrock_core.hnmod'
    shutil.copyfile(built, out)
    return out


def pack_datapack():
    """The world datapack as a zip a world's datapacks folder takes as it is:
    `pack.mcmeta` at the archive's root, the way Horizon's pack discovery
    (`Zip::contains("pack.mcmeta")`) wants it — no repacking, no unpacking."""
    src = ROOT / 'datapack' / 'underbedrock_world'
    out = RELEASE / 'underbedrock_world.zip'
    with zipfile.ZipFile(out, 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(src.rglob('*')):
            if path.is_file():
                put(archive, path, path.relative_to(src).as_posix())
    return out


def main():
    RELEASE.mkdir(exist_ok=True)
    parts = [pack_hmod(), pack_hnmod(), pack_datapack()]
    for part in parts:
        digest = hashlib.sha256(part.read_bytes()).hexdigest()
        print(f'{part.relative_to(ROOT)}  {part.stat().st_size} bytes  sha256 {digest[:16]}')


if __name__ == '__main__':
    main()
