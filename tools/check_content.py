#!/usr/bin/env python3
"""Check the content mod's declarations the way Horizon's mod loader reads them.

The loader refuses the WHOLE mod on the first bad declaration (the game log
then says "content mods not loaded: ..."). Two rules bit us:

* a block's `template` must be a block the game knows
  (mod_content.rs: "template {} is not a block");
* a block's `properties.sounds` must ALSO name a block — the mod copies that
  block's sounds (mod_content.rs: "sounds {s} is not a block"). Vanilla sound
  type names ("minecraft:metal") and removed ids ("minecraft:grass", which is
  "minecraft:short_grass" now) are not blocks and kill the mod.

It also checks the two ids the native half summons its creatures with
(runtime.rs `WISP_ITEM` / `LANTERN_BLOCK`): the item and the block must be
the content mod's own, or the displays show nothing.

    python3 tools/check_content.py

The block id list (tools/vanilla_blocks.txt) is the vanilla block registry
(PrismarineJS minecraft-data, 1.21.4 — the modern ids, `short_grass` and all;
Horizon's base is later and only adds to it).
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / 'content-mod'
RUNTIME = ROOT / 'native-mod' / 'mixins' / 'underbedrock_core' / 'runtime.rs'


def main() -> int:
    fails: list[str] = []
    vanilla = {line.strip() for line in (ROOT / 'tools' / 'vanilla_blocks.txt').read_text().splitlines() if line.strip()}
    if len(vanilla) < 500:
        fails.append('tools/vanilla_blocks.txt looks empty or truncated')

    blocks_dir = MOD / 'data' / 'underbedrock' / 'horizon' / 'block'
    items_dir = MOD / 'data' / 'underbedrock' / 'horizon' / 'item'
    mod_blocks = {f'underbedrock:{p.stem}' for p in blocks_dir.glob('*.json')} if blocks_dir.is_dir() else set()
    mod_items = {f'underbedrock:{p.stem}' for p in items_dir.glob('*.json')} if items_dir.is_dir() else set()
    if not mod_blocks:
        fails.append('no block declarations found under data/underbedrock/horizon/block/')

    for path in sorted(blocks_dir.glob('*.json')):
        decl = json.loads(path.read_text())
        what = f'block {path.stem}'
        template = decl.get('template')
        if template is None:
            fails.append(f'{what}: no template')
        elif template not in vanilla:
            fails.append(f'{what}: template {template} is not a vanilla block')
        sounds = (decl.get('properties') or {}).get('sounds')
        if sounds is not None and sounds not in vanilla:
            fails.append(f'{what}: sounds {sounds} is not a block (Horizon copies that '
                         f'block\'s sounds; a sound TYPE name or a removed id kills the whole mod)')

    # The native half's creatures show the mod's own item and block.
    runtime = RUNTIME.read_text()
    for const, pool, what in (('WISP_ITEM', mod_items, 'item'),
                              ('LANTERN_BLOCK', mod_blocks, 'block')):
        found = re.search(rf'pub const {const}: &str = "([^"]+)"', runtime)
        if not found:
            fails.append(f'runtime.rs: no {const} constant (the creatures need it)')
        elif found.group(1) not in pool:
            fails.append(f'runtime.rs: {const} {found.group(1)} is not one of the content '
                         f'mod\'s own {what}s; its display would show nothing')

    if fails:
        for line in fails:
            print(f'  {line}')
        print(f'FAILED: {len(fails)} content declaration problem(s)')
        return 1
    print(f'ok: {len(mod_blocks)} blocks, {len(mod_items)} items, every template/sounds '
          f'id is a real block, the creatures\' ids are the mod\'s own')
    return 0


if __name__ == '__main__':
    sys.exit(main())
