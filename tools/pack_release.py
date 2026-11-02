#!/usr/bin/env python3
"""Собрать всё, что раздаёт Under Bedrock, в папку release/ — одним Питоном, без Раста.

    python tools/pack_release.py

Делает четыре файла:
    release/underbedrock.hmod          контент-мод (zip: horizon-mod.toml первым, затем assets/ и data/)
    release/underbedrock_core.hnmod    нативный мод (тем же packager'ом, что и в native-mod/tools)
    release/underbedrock_world.zip     датапак мира (папка datapack/underbedrock_world как есть)
    release/underbedrock-install.zip   все три плюс INSTALL.txt одной скачиваемой пачкой

Нужен только Python 3.11+ (tomllib). Раст не обязателен: он нужен лишь проверкам
(cargo test в native-mod, stubcheck), но не самим файлам мода. Исходники блоков,
рецептов и мира уже лежат в content-mod/ и datapack/ — генерировать их заново
(make_content.py, make_textures.py) нужно только правки ради.
"""
import hashlib
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RELEASE = ROOT / 'release'

INSTALL_TXT = """Under Bedrock (Horizon 26.3) — установка. Полная инструкция в README.md.

1. underbedrock.hmod        -> в папку horizon-mods/        (включить: Настройки -> Моды -> Under Bedrock)
2. underbedrock_core.hnmod  -> в папку horizon-native-mods/ (включить там же; лаунчер один раз соберёт игру с модом, нужен интернет)
3. underbedrock_world.zip   -> распаковать в saves/<НОВЫЙ мир>/datapacks/  (папки-датапаки включаются сами)

Мир создавайте ПОСЛЕ датапака: у старых чанков под бедроком будет пустота.
Проверка в игре: /etherium depth  ->  "y -576 .. 319".
Как играть: кирка из зачарованного сплава (3 слитка + 2 стержня пламени), держать ЛКМ на бедроке ~3 c.
"""


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
            archive.write(src / name, name)
    return out


def pack_hnmod():
    """The native mod, by the bundle's own packager (source-only .hnmod)."""
    subprocess.run([sys.executable, 'tools/package.py'], cwd=ROOT / 'native-mod', check=True)
    built = sorted((ROOT / 'native-mod' / 'target').glob('underbedrock_core-*.hnmod'))[-1]
    out = RELEASE / 'underbedrock_core.hnmod'
    shutil.copyfile(built, out)
    return out


def pack_datapack():
    """The world datapack, as a zip a world's datapacks folder takes."""
    src = ROOT / 'datapack' / 'underbedrock_world'
    out = RELEASE / 'underbedrock_world.zip'
    with zipfile.ZipFile(out, 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(src.rglob('*')):
            if path.is_file():
                archive.write(path, path.relative_to(ROOT / 'datapack').as_posix())
    return out


def pack_install(parts):
    out = RELEASE / 'underbedrock-install.zip'
    with zipfile.ZipFile(out, 'w', zipfile.ZIP_STORED) as archive:
        archive.writestr('INSTALL.txt', INSTALL_TXT)
        for part in parts:
            archive.write(part, part.name)
    return out


def main():
    RELEASE.mkdir(exist_ok=True)
    parts = [pack_hmod(), pack_hnmod(), pack_datapack()]
    parts.append(pack_install(parts))
    for part in parts:
        digest = hashlib.sha256(part.read_bytes()).hexdigest()
        print(f'{part.relative_to(ROOT)}  {part.stat().st_size} bytes  sha256 {digest[:16]}')


if __name__ == '__main__':
    main()
