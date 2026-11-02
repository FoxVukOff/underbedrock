#!/usr/bin/env python3
"""Check a native mod's `.hnmod` the way Horizon's registry does.

A port of `horizon_mods::index::native::Bundle::read` (crates/horizon-mods/src/index/native.rs):
the same path rules, the same manifest checks, the same agreement between `horizon-native.json`
and the bundle's `horizon-mixin.toml`. The Rust original sits behind the crate's `index` feature,
which pulls naga; this runs anywhere.

    python3 tools/nativecheck.py dist/underbedrock_core.hnmod
"""
import json
import re
import sys
import tomllib
import zipfile

MANIFEST = 'horizon-native.json'
WASM = 'mod.wasm'
MAX_FILE = 64 * 1024 * 1024
ID = re.compile(r'^[a-z0-9_]{1,64}$')
REVISION = re.compile(r'^[0-9a-f]{40}$')


def fail(why):
    print(f'error: {why}')
    return 1


def content(name):
    """A `.hmod`'s content entries, which the native build leaves alone."""
    return name == 'horizon-mod.toml' or name.startswith(('assets/', 'data/'))


def in_data(rest):
    parts = rest.split('/')
    return len(parts) >= 2 and (parts[0] == 'data' or (len(parts) >= 3 and parts[1] == 'data'))


def text(rest):
    parts = rest.split('/')
    name = parts[-1]
    return len(parts) <= 2 and (
        name.startswith(('README', 'LICENSE', 'LICENCE', 'COPYING', 'NOTICE')) or name == '.gitignore'
    )


def allowed(rest):
    return rest.endswith(('.rs', '.toml')) or in_data(rest) or text(rest)


def check(path):
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        files, content_id, total = {}, None, 0
        for info in archive.infolist():
            name = info.filename
            if info.is_dir():
                continue
            mode = info.external_attr >> 16
            if (mode & 0o170000 not in (0, 0o100000)
                    or '\\' in name or name.startswith('/') or ':' in name
                    or any(p in ('', '.', '..') for p in name.split('/'))):
                return fail(f'invalid native bundle path: {name}')
            if name == WASM:
                return fail('mod.wasm: WebAssembly mods do not run any more')
            data = archive.read(name)
            if content(name):
                if name == 'horizon-mod.toml':
                    content_id = tomllib.loads(data.decode())['id']
                continue
            if len(files) >= 512:
                return fail('native bundle has too many entries')
            total += len(data)
            if total > MAX_FILE:
                return fail('native bundle expands beyond 64 MiB')
            if name in files:
                return fail(f'duplicate native bundle path: {name}')
            files[name] = data

    if MANIFEST not in files:
        return fail(f'missing {MANIFEST}')
    try:
        manifest = json.loads(files[MANIFEST])
    except ValueError as e:
        return fail(f'{MANIFEST}: {e}')
    known = {'schema', 'id', 'version', 'game', 'horizon', 'mixins', 'hooks'}
    unknown = set(manifest) - known
    if unknown:
        return fail(f'{MANIFEST}: unknown fields {sorted(unknown)} (the manifest denies them)')
    if (manifest.get('schema') != 1
            or not ID.match(str(manifest.get('id', '')))
            or not manifest.get('version')
            or not manifest.get('game')
            or not REVISION.match(str(manifest.get('horizon', '')))
            or not REVISION.match(str(manifest.get('mixins', '')))
            or manifest.get('hooks') == 0):
        return fail('invalid native manifest: schema 1, id, version, game and exact revisions required')
    if content_id is not None and content_id != manifest['id']:
        return fail(f'its horizon-mod.toml is mod {content_id}, its {MANIFEST} {manifest["id"]}: one file is one mod')

    prefix = f'mixins/{manifest["id"]}/'
    for name in files:
        if name == MANIFEST:
            continue
        if not name.startswith(prefix) or not allowed(name[len(prefix):]):
            return fail(f'unexpected native source: {name}')
    if f'{prefix}horizon-mixin.toml' not in files:
        return fail('missing horizon-mixin.toml')
    mixin = tomllib.loads(files[f'{prefix}horizon-mixin.toml'].decode())
    mod = mixin.get('mod', {})
    if (mod.get('id') != manifest['id']
            or mod.get('version') != manifest['version']
            or mixin.get('compatibility', {}).get('horizon_revision') != manifest['horizon']):
        return fail('native manifest and mixin identity/revision differ')

    hooks = manifest.get('hooks')
    print(f'ok: {MANIFEST} id {manifest["id"]} version {manifest["version"]} game {manifest["game"]}')
    print(f'    pins horizon {manifest["horizon"][:12]} mixins {manifest["mixins"][:12]}'
          + (f', hook points v{hooks}' if hooks else ', pinned to its revision'))
    print(f'    api {manifest["id"]}: native-1/{manifest["horizon"]}/{manifest["mixins"]}')
    modules = mixin.get('module', [])
    injections = mixin.get('injection', [])
    print(f'    {len(modules)} module, {len(injections)} injections:')
    for module in modules:
        print(f'      module {module["id"]}: {module["source"]} -> {module["file"]} as crate::game::{module["name"]}')
    for injection in injections:
        target = injection.get('target', f'hook {injection.get("hook")}')
        print(f'      {injection["kind"]} {target.split("::")[-1]} <- {injection["helper"]}'
              f' ({len(injection.get("bindings", []))} bindings)')
    for name, data in sorted(files.items()):
        print(f'    {name} ({len(data)} bytes)')
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 2:
        raise SystemExit('usage: nativecheck.py <bundle.hnmod>')
    raise SystemExit(check(sys.argv[1]))
