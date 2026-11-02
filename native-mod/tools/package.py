#!/usr/bin/env python3
"""Package the mod as a source-only `.hnmod`, the file Horizon's launcher takes.

The archive holds `horizon-native.json` and `mixins/<id>/` (only `.rs` and `.toml`), with fixed
dates and modes, so the same sources always give the same bytes and the same SHA-256. Horizon's
`horizon_mods::index::native` reads it without running anything; the launcher compiles it on the
player's Horizon (this mod names the hook points' version it needs, so it isn't pinned to one
revision). Based on the native mod template's packager.
"""
import hashlib
import json
from pathlib import Path
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]
GAME = '26.3'


def package():
    compat = json.loads((ROOT / 'compatibility.json').read_text())
    manifests = sorted((ROOT / 'mixins').glob('*/horizon-mixin.toml'))
    if len(manifests) != 1:
        raise SystemExit('a .hnmod holds one bundle: keep one mixins/<id>/horizon-mixin.toml')
    bundle = manifests[0].parent
    mixin = tomllib.loads(manifests[0].read_text())
    mod = mixin['mod']
    if bundle.name != mod['id']:
        raise SystemExit(f"rename mixins/{bundle.name}/ to mixins/{mod['id']}/ (the registry finds it by id)")
    if not mod['id'].replace('_', '').isalnum() or not mod['id'].islower():
        raise SystemExit('the registry takes ids of lowercase letters, digits and _ only')
    if mixin.get('compatibility', {}).get('horizon_revision') != compat['horizon_revision']:
        raise SystemExit('horizon-mixin.toml and compatibility.json pin different Horizon revisions')
    manifest = dict(schema=1, id=mod['id'], version=mod['version'], game=GAME,
                    horizon=compat['horizon_revision'], mixins=compat['horizon_mixins_revision'])
    # A mod that hooks only Horizon's native hook points names the version it
    # needs, and the launcher builds it on any Horizon that has it or a later one.
    hooks = compat.get('hooks')
    if hooks:
        manifest['hooks'] = hooks
    files = {'horizon-native.json': (json.dumps(manifest, indent=2, sort_keys=True) + '\n').encode()}
    for path in sorted(bundle.iterdir()):
        if path.is_file() and path.suffix in ('.rs', '.toml'):
            files[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    out = ROOT / 'target' / f"{mod['id']}-{mod['version']}.hnmod"
    out.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(out, 'w', compression=zipfile.ZIP_DEFLATED) as archive:
        for name, content in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, content)
    print(json.dumps(dict(file=str(out), size=out.stat().st_size,
                          sha256=hashlib.sha256(out.read_bytes()).hexdigest()), indent=2))


if __name__ == '__main__':
    package()
