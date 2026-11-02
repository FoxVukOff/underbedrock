import shutil, subprocess, sys, zipfile
from pathlib import Path
ROOT = Path(__file__).resolve().parent
out = ROOT / 'release'; out.mkdir(exist_ok=True)

# контент-мод: zip, horizon-mod.toml первым входом (так читает Horizon)
src = ROOT / 'content-mod'
names = ['horizon-mod.toml'] + sorted(
    p.relative_to(src).as_posix() for p in src.rglob('*')
    if p.is_file() and p.name != 'horizon-mod.toml')
with zipfile.ZipFile(out / 'underbedrock.hmod', 'w', zipfile.ZIP_DEFLATED) as z:
    for n in names:
        z.write(src / n, n)

# нативный мод: его же упаковщиком из репозитория
subprocess.run([sys.executable, 'tools/package.py'], cwd=ROOT / 'native-mod', check=True)
shutil.copyfile(sorted((ROOT / 'native-mod' / 'target').glob('underbedrock_core-*.hnmod'))[-1],
                out / 'underbedrock_core.hnmod')

# датапак: папкой, её и кидать в datapacks
shutil.copytree(ROOT / 'datapack' / 'underbedrock_world',
                out / 'underbedrock_world', dirs_exist_ok=True)
print('готово в release/:', sorted(p.name for p in out.iterdir()))