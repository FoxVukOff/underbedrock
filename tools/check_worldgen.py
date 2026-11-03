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


AQUIFER_KEYS = ("barrier", "exclusion", "fluid_level_floodedness",
                "fluid_level_spread", "lava", "surface_level")
BUILTIN_DENSITIES = ("minecraft:overworld/", "minecraft:y")
BUILTIN_RULES = ("minecraft:overworld",)


def structural_checks(fails):
    """The invariants the Abyss's worldgen depends on (see make_content.py)."""
    pack = ROOT / 'datapack' / 'underbedrock_world' / 'data'

    settings = json.loads((pack / 'minecraft/worldgen/noise_settings/overworld.json').read_text())
    aquifers = settings.get('aquifers')
    if not isinstance(aquifers, dict):
        fails.append('noise_settings: no "aquifers" object: without it Horizon\'s global '
                     'fluid picker floods every cave below y -54 with lava')
    else:
        missing = [k for k in AQUIFER_KEYS if k not in aquifers]
        if missing:
            fails.append(f'noise_settings aquifers: missing {missing}')
        if aquifers.get('lava') != 0.0:
            fails.append('noise_settings aquifers: lava must be the constant 0.0 '
                         '(no lava pocket anywhere)')
    final = json.loads((pack / 'underbedrock/worldgen/density_function/final_density.json').read_text())
    if final.get('max_exclusive') != -54.0:
        fails.append('final_density: the solid underground must start at y -54 '
                     '(the lava sea line)')
    if not isinstance(final.get('when_in_range'), (int, float)) or final['when_in_range'] <= 0:
        fails.append('final_density: below y -54 the density must be a positive constant '
                     '(solid rock; the caves are carved by the material rule, so no fluid '
                     'is ever placed down there)')

    surface = json.loads((pack / 'underbedrock/worldgen/material_rule/surface.json').read_text())
    text = json.dumps(surface)
    for anchor, what in (('-64', 'the bedrock transition band'),
                         ('-564', 'the abyss bedrock floor'),
                         ('-65', 'the voidshell gate'),
                         ('-72', 'the carved world')):
        if f'"absolute": {anchor}' not in text:
            fails.append(f'surface rule: no window anchored at {anchor} ({what})')
    if '"minecraft:bedrock"' not in text or '"underbedrock:abyss_bedrock"' not in text \
            or '"underbedrock:voidshell"' not in text or '"underbedrock:voidstone"' not in text:
        fails.append('surface rule: a depth block (bedrock/abyss_bedrock/voidshell/voidstone) is missing')
    # A vertical gradient is true at every depth below its band: before any
    # block rule under one, a y_above condition must narrow it to a window, or
    # the block paints the whole world below the band (the old "infinite
    # bedrock" bug).
    def narrowed(node):
        if isinstance(node, dict):
            if node.get('type') == 'minecraft:condition':
                if node.get('if_true', {}).get('type') == 'minecraft:y_above':
                    return True
                return narrowed(node.get('then_run'))
            if node.get('type') == 'minecraft:sequence':
                return all(narrowed(child) for child in node.get('sequence', []))
            if node.get('type') == 'minecraft:block':
                return False
        return True
    def find_gradients(node, where):
        if isinstance(node, dict):
            if node.get('type') == 'minecraft:condition' \
                    and node.get('if_true', {}).get('type') == 'minecraft:vertical_gradient' \
                    and not narrowed(node.get('then_run')):
                fails.append(f'{where}: a vertical_gradient condition reaches a block rule '
                             f'with no y_above window: it would paint every depth below its band')
            for value in node.values():
                find_gradients(value, where)
        elif isinstance(node, list):
            for child in node:
                find_gradients(child, where)
    find_gradients(surface, 'surface rule')

    for name in ('cave', 'cave_extra_underground', 'canyon'):
        carver = json.loads((pack / f'minecraft/worldgen/carver/{name}.json').read_text())
        if carver.get('probability') != 0.0:
            fails.append(f'carver {name}: probability must be 0 (a carver tunnel below '
                         f'y -54 would be filled with the engine\'s deep lava sea)')

    # Horizon's VerticalAnchor decodes from a map only ({"absolute": -55});
    # vanilla also allows a bare integer, Horizon crashes the integrated
    # server on one ("invalid type: integer, expected a map"). Every height
    # provider in the pack and the content mod's features must carry maps.
    ANCHOR_KEYS = {'absolute', 'above_bottom', 'below_top', 'relative_to_sea_level'}
    HEIGHT_TYPES = {'minecraft:constant', 'minecraft:uniform', 'minecraft:biased_to_bottom',
                    'minecraft:very_biased_to_bottom', 'minecraft:trapezoid'}

    def anchor_ok(value):
        return isinstance(value, dict) and len(ANCHOR_KEYS & set(value)) == 1

    def height_provider_ok(provider):
        if not isinstance(provider, dict) or provider.get('type') not in HEIGHT_TYPES:
            return False
        if provider['type'] == 'minecraft:constant':
            return anchor_ok(provider.get('value'))
        return anchor_ok(provider.get('min_inclusive')) and anchor_ok(provider.get('max_inclusive'))

    def check_providers(node, where):
        if isinstance(node, dict):
            if node.get('type') in ('minecraft:cave', 'minecraft:nether_cave', 'minecraft:canyon') \
                    and 'y' in node and not height_provider_ok(node['y']):
                fails.append(f'{where}: the carver\'s y must be a height provider whose '
                             f'anchors are maps ({{"absolute": N}}), never bare integers')
            if node.get('type') == 'minecraft:height_range' \
                    and not height_provider_ok(node.get('height')):
                fails.append(f'{where}: height_range must carry a height provider whose '
                             f'anchors are maps ({{"absolute": N}}), never bare integers')
            for value in node.values():
                check_providers(value, where)
        elif isinstance(node, list):
            for child in node:
                check_providers(child, where)

    for folder in (pack, ROOT / 'content-mod' / 'data'):
        for path in sorted(folder.rglob('*.json')):
            try:
                check_providers(json.loads(path.read_text()), path.name)
            except json.JSONDecodeError:
                fails.append(f'{path.name}: not JSON')

    # Every density function and material rule reference resolves.
    df = pack / 'underbedrock/worldgen/density_function'
    have_df = {f'underbedrock:{p.stem}' for p in df.glob('*.json')}
    have_noise = {f'underbedrock:{p.stem}' for p in (pack / 'underbedrock/worldgen/noise').glob('*.json')}
    have_rule = {f'underbedrock:{p.stem}' for p in (pack / 'underbedrock/worldgen/material_rule').glob('*.json')}

    def refs(node, where):
        if isinstance(node, dict):
            if node.get('type') == 'minecraft:noise' and isinstance(node.get('noise'), str):
                name = node['noise']
                if name.startswith('underbedrock:') and name not in have_noise:
                    fails.append(f'{where}: noise {name} has no file')
            # Material conditions sample the NOISE registry, never density
            # functions: a density name in noise_threshold fails every chunk
            # at the surface stage ("unknown material noise", empty world).
            if node.get('type') == 'minecraft:noise_threshold' and isinstance(node.get('noise'), str):
                name = node['noise']
                if name.startswith('underbedrock:') and name not in have_noise:
                    fails.append(f'{where}: noise_threshold noise {name} is not a noise '
                                 f'(conditions sample worldgen/noise, not density_function)')
            for key, value in node.items():
                if key in ('noise', 'random_name') or key in STATE_KEYS or key in BLOCK_KEYS:
                    continue
                if isinstance(value, str) and value.startswith('underbedrock:'):
                    pool = have_df if key in ('input', 'left', 'right', 'when_in_range',
                                              'when_out_of_range', 'argument') else have_rule
                    if value not in pool and value not in have_rule:
                        fails.append(f'{where}: reference {value} has no file')
                else:
                    refs(value, where)
        elif isinstance(node, list):
            for child in node:
                refs(child, where)
    for path in sorted(df.glob('*.json')):
        refs(json.loads(path.read_text()), path.name)
    refs(surface, 'surface rule')
    # The lava ban, revised by the user 2026-11: the hot expanse layer keeps
    # small lava lakes and glowing veins, so the surface material rule may
    # name lava - and only it. Everywhere else lava stays banned, and the
    # aquifer's lava level stays a flat zero.
    lava_ok = {ROOT / 'datapack' / 'underbedrock_world' / 'data' / 'underbedrock'
               / 'worldgen' / 'material_rule' / 'surface.json'}
    for base in (ROOT / 'content-mod' / 'data', ROOT / 'datapack'):
        for path in sorted(base.rglob('*.json')):
            if path in lava_ok:
                continue
            text = path.read_text()
            if 'minecraft:lava' in text:
                fails.append(f'{path.relative_to(ROOT)}: mentions minecraft:lava '
                             f'(only the hot expanse band of the surface rule may)')


def main():
    known = mod_blocks()
    fails, checked = [], 0
    structural_checks(fails)
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
