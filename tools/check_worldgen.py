#!/usr/bin/env python3
"""Check the worldgen JSONs the way Horizon's worldgen decoder reads them.

Horizon's block-state codec (`crates/horizon-world/src/registry/json.rs`, `decode` with
`full=false`) takes a bare block name or `{"id": name, "properties": {...}}`. Vanilla's
`{"Name": ...}` object crashes a world load with "missing block identifier" the moment a
data pack changes the world generation — so this script fails the build on any state that
isn't in Horizon's format, and on any mod-namespaced name the content mod doesn't define.

    python3 tools/check_worldgen.py
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STATE_KEYS = ('state', 'result_state', 'default_block', 'default_fluid')
BLOCK_KEYS = ('block',)  # rule tests and conditions name a block, not a state


def mod_blocks():
    folder = ROOT / 'content-mod' / 'data' / 'underbedrock' / 'horizon' / 'block'
    return {f'underbedrock:{p.stem}' for p in folder.glob('*.json')}


def walk(folder):
    for path in sorted(folder.rglob('*.json')):
        yield path, json.loads(path.read_text())


def states(value, path, key_path, out):
    """Every (key, value) under a state or block key, with where it was found."""
    if isinstance(value, dict):
        for key, child in value.items():
            if key in STATE_KEYS:
                out.append((f'{path}:{key_path}{key}', 'state', child))
            elif key in BLOCK_KEYS:
                out.append((f'{path}:{key_path}{key}', 'block', child))
            else:
                states(child, path, key_path + key + '.', out)
    elif isinstance(value, list):
        for i, child in enumerate(value):
            states(child, path, f'{key_path}[{i}].', out)


def main():
    known = mod_blocks()
    fails, checked = [], 0
    for base in (ROOT / 'content-mod' / 'data', ROOT / 'datapack'):
        for path, doc in walk(base):
            found = []
            states(doc, path.relative_to(ROOT).as_posix(), '', found)
            for where, kind, value in found:
                checked += 1
                if kind == 'block':
                    names = [value] if isinstance(value, str) else value
                    for name in names if isinstance(names, list) else [names]:
                        if not isinstance(name, str) or ':' not in name:
                            fails.append(f'{where}: block name {name!r} is not an identifier')
                        elif name.startswith('underbedrock:') and name not in known:
                            fails.append(f'{where}: {name} is not a block of the content mod')
                    continue
                # A state: a bare name, or an object with `id` (and properties).
                if isinstance(value, str):
                    name = value
                elif isinstance(value, dict) and isinstance(value.get('id'), str):
                    name = value['id']
                    if 'Name' in value:
                        fails.append(f'{where}: Horizon reads `id`, not vanilla `Name`')
                else:
                    fails.append(
                        f'{where}: {json.dumps(value)[:60]} is not a Horizon block state '
                        f'(a bare name or {{"id": ...}})'
                    )
                    continue
                if ':' not in name:
                    fails.append(f'{where}: {name!r} is not a namespaced identifier')
                elif name.startswith('underbedrock:') and name not in known:
                    fails.append(f'{where}: {name} is not a block of the content mod')
    for line in fails:
        print(f'FAIL {line}')
    print(f'{"ok" if not fails else "FAILED"}: {checked} state/block references in worldgen, '
          f'{len(known)} mod blocks known')
    return 1 if fails else 0


if __name__ == '__main__':
    sys.exit(main())
