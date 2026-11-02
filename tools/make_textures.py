#!/usr/bin/env python3
"""Pixel-art textures of the Underbedrock mod (16x16 PNG, no external assets).

Everything is drawn from a fixed seed, so the same script always gives the same
bytes. Run: python3 tools/make_textures.py

The palette is the Abyss: near-black blue stone, and void crystals in it that
glow cold blue.
"""
import random
import shutil
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1] / "content-mod" / "assets" / "underbedrock" / "textures"
S = 16

# The mod's palette: a cold void stone and the blue crystal light in it.
VOID_DARK = (18, 18, 26, 255)
VOID_MID = (28, 28, 40, 255)
VOID_LIGHT = (44, 46, 64, 255)
VOID_EDGE = (12, 12, 18, 255)

BLUE_DEEP = (16, 60, 120, 255)
BLUE_MID = (40, 120, 220, 255)
BLUE_BRIGHT = (110, 190, 255, 255)
BLUE_WHITE = (205, 240, 255, 255)

SHELL_DARK = (10, 10, 16, 255)
SHELL_MID = (18, 18, 28, 255)
SHELL_VEIN = (28, 76, 150, 255)

ABYSS_DARK = (8, 8, 10, 255)
ABYSS_MID = (16, 16, 22, 255)
ABYSS_BLUE = (18, 36, 72, 255)

GRAVEL_A = (30, 30, 42, 255)
GRAVEL_B = (44, 44, 58, 255)
GRAVEL_C = (22, 22, 32, 255)
GRAVEL_D = (56, 58, 76, 255)

WOOD_MID = (70, 58, 88, 255)
WOOD_DARK = (48, 40, 62, 255)


def canvas():
    return [[None for _ in range(S)] for _ in range(S)]


def put(px, x, y, color):
    if 0 <= x < S and 0 <= y < S:
        px[y][x] = color


def noise_fill(px, rng, colors, weights=None):
    """A mottled stone base: every pixel one of `colors`."""
    for y in range(S):
        for x in range(S):
            put(px, x, y, rng.choices(colors, weights)[0])


def blob(px, rng, cx, cy, r, colors, tries=26):
    """A random roundish blob of pixels, as vanilla ores are drawn."""
    for _ in range(tries):
        x = cx + rng.randint(-r, r)
        y = cy + rng.randint(-r, r)
        if (x - cx) ** 2 + (y - cy) ** 2 <= r * r + rng.randint(0, 2):
            put(px, x, y, rng.choice(colors))


def save(px, name):
    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    for y in range(S):
        for x in range(S):
            if px[y][x] is not None:
                img.putpixel((x, y), px[y][x])
    path = ROOT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)
    print(f"wrote {path.name}")


# --- blocks -----------------------------------------------------------------

def voidstone(seed=11):
    """The stone of the world under the bedrock: colder and darker than deepslate."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_MID] * 6 + [VOID_DARK] * 3 + [VOID_LIGHT] * 2)
    for _ in range(7):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(2, 5)):
            put(px, x, y, VOID_EDGE if rng.random() < 0.6 else VOID_LIGHT)
            x += rng.choice((-1, 0, 1))
            y += rng.choice((0, 1))
    # Faint blue grains: this stone is not quite stone.
    for _ in range(4):
        put(px, rng.randrange(S), rng.randrange(S), BLUE_DEEP)
    save(px, "block/voidstone.png")


def voidstone_bricks(seed=29):
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_MID] * 5 + [VOID_LIGHT] * 2 + [VOID_DARK])
    for y in range(S):
        for x in range(S):
            if y in (3, 7, 11, 15):
                put(px, x, y, VOID_EDGE)
            elif (x + (0 if y < 7 else 4 if y < 11 else 8 if y < 15 else 12)) % 8 == 7:
                put(px, x, y, VOID_EDGE)
    for _ in range(10):
        put(px, rng.randrange(S), rng.randrange(S), VOID_LIGHT)
    save(px, "block/voidstone_bricks.png")


def void_gravel(seed=37):
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [GRAVEL_A] * 4 + [GRAVEL_B] * 3 + [GRAVEL_C] * 3 + [GRAVEL_D])
    for _ in range(6):
        blob(px, rng, rng.randrange(2, 14), rng.randrange(2, 14), 1, [GRAVEL_D, GRAVEL_B], tries=6)
    for _ in range(3):
        put(px, rng.randrange(S), rng.randrange(S), BLUE_DEEP)
    save(px, "block/void_gravel.png")


def void_crystal_ore(seed=41):
    px = canvas()
    voidstone_into(px, 41)
    rng = random.Random(seed)
    for cx, cy in ((4, 4), (11, 6), (6, 11), (12, 12)):
        blob(px, rng, cx, cy, 1, [BLUE_MID, BLUE_BRIGHT, BLUE_DEEP], tries=10)
        put(px, cx, cy, BLUE_WHITE)
    save(px, "block/void_crystal_ore.png")


def voidstone_into(px, seed):
    rng = random.Random(seed)
    noise_fill(px, rng, [VOID_MID] * 6 + [VOID_DARK] * 3 + [VOID_LIGHT] * 2)
    for _ in range(6):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(2, 4)):
            put(px, x, y, VOID_EDGE)
            x += rng.choice((-1, 0, 1))
            y += rng.choice((0, 1))


def void_crystal_block(seed=53):
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [BLUE_MID] * 5 + [BLUE_DEEP] * 3 + [BLUE_BRIGHT] * 2)
    # Facets: bright diagonal lines, deep seams.
    for i in range(S):
        put(px, i, (i + 2) % S, BLUE_BRIGHT)
        put(px, i, (i + 9) % S, BLUE_DEEP)
    for _ in range(6):
        put(px, rng.randrange(S), rng.randrange(S), BLUE_WHITE)
    save(px, "block/void_crystal_block.png")


def crystal_lamp(seed=61):
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [BLUE_BRIGHT] * 6 + [BLUE_WHITE] * 3 + [BLUE_MID] * 2)
    # A lattice of deep lines, as glowstone's crust but cold.
    for i in range(0, S, 4):
        for t in range(S):
            put(px, (i + t) % S, t, BLUE_MID)
            put(px, t, (i + t) % S, BLUE_DEEP if t % 2 else BLUE_MID)
    for _ in range(8):
        put(px, rng.randrange(S), rng.randrange(S), BLUE_WHITE)
    save(px, "block/crystal_lamp.png")


def crystal_sprout(seed=71):
    px = canvas()
    stem = (36, 84, 108, 255)
    for y in range(10, 16):
        put(px, 8, y, stem)
    put(px, 7, 12, stem)
    put(px, 9, 13, stem)
    # A small crystal fan on top.
    for x, y, c in ((8, 9, BLUE_MID), (8, 8, BLUE_BRIGHT), (8, 7, BLUE_WHITE),
                    (7, 9, BLUE_MID), (7, 8, BLUE_BRIGHT), (6, 8, BLUE_DEEP),
                    (9, 9, BLUE_MID), (9, 8, BLUE_BRIGHT), (10, 8, BLUE_DEEP),
                    (8, 6, BLUE_BRIGHT), (7, 5, BLUE_DEEP), (9, 5, BLUE_DEEP)):
        put(px, x, y, c)
    save(px, "block/crystal_sprout.png")


def voidshell(seed=73):
    """The gate block: black shell with cold veins, denser than obsidian."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [SHELL_DARK] * 6 + [SHELL_MID] * 3)
    for _ in range(5):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(3, 6)):
            put(px, x, y, SHELL_VEIN)
            x += rng.choice((-1, 0, 1))
            y += rng.choice((-1, 0, 1))
    for _ in range(4):
        put(px, rng.randrange(S), rng.randrange(S), BLUE_DEEP)
    save(px, "block/voidshell.png")


def abyss_bedrock(seed=79):
    """The world's bottom: bedrock gone dark, with dead-blue mottling."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [ABYSS_DARK] * 5 + [ABYSS_MID] * 3 + [ABYSS_BLUE])
    for _ in range(8):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(2, 4)):
            put(px, x, y, ABYSS_MID if rng.random() < 0.7 else ABYSS_BLUE)
            x += rng.choice((-1, 0, 1))
            y += rng.choice((-1, 0, 1))
    save(px, "block/abyss_bedrock.png")


# --- items ------------------------------------------------------------------

def void_crystal(seed=83):
    px = canvas()
    for x, y, c in ((8, 2, BLUE_WHITE), (7, 3, BLUE_BRIGHT), (8, 3, BLUE_BRIGHT),
                    (9, 3, BLUE_MID), (7, 4, BLUE_BRIGHT), (8, 4, BLUE_MID),
                    (9, 4, BLUE_MID), (6, 5, BLUE_MID), (7, 5, BLUE_MID),
                    (8, 5, BLUE_DEEP), (9, 5, BLUE_MID), (10, 5, BLUE_DEEP),
                    (6, 6, BLUE_MID), (7, 6, BLUE_DEEP), (8, 6, BLUE_MID),
                    (9, 6, BLUE_DEEP), (10, 6, BLUE_MID), (7, 7, BLUE_DEEP),
                    (8, 7, BLUE_DEEP), (9, 7, BLUE_DEEP), (8, 8, BLUE_DEEP)):
        put(px, x, y, c)
    save(px, "item/void_crystal.png")


def void_alloy(seed=89):
    px = canvas()
    for y, row in enumerate(range(6, 11)):
        for x in range(3, 13):
            c = BLUE_WHITE if y == 0 else BLUE_BRIGHT if y == 1 else \
                (60, 66, 92, 255) if y == 2 else (44, 48, 68, 255) if y == 3 else (32, 34, 50, 255)
            put(px, x, row, c)
    put(px, 4, 6, BLUE_WHITE)
    put(px, 11, 8, BLUE_DEEP)
    save(px, "item/void_alloy.png")


def void_alloy_pickaxe(seed=97):
    """A proper pickaxe: a full-width head with two prongs, a straight haft.

    The head is a band across the top with a short left prong and a longer
    right one hanging down; the haft runs from under the head's right centre
    to the bottom-left corner, two pixels wide, as vanilla tools do.
    """
    px = canvas()
    M, H, Sd = BLUE_MID, BLUE_WHITE, BLUE_DEEP
    W, Wd = WOOD_MID, WOOD_DARK
    head = {
        1: [(5, M), (6, M), (7, M), (8, M), (9, M), (10, M), (11, Sd)],
        2: [(4, Sd), (5, M), (6, H), (7, M), (8, M), (9, M), (10, M), (11, M), (12, Sd)],
        3: [(4, Sd), (5, M), (6, M), (7, M), (8, M), (9, H), (10, M), (11, M), (12, M), (13, Sd)],
        4: [(4, Sd), (5, Sd), (11, M), (12, M), (13, M)],
        5: [(12, M), (13, M)],
        6: [(12, M), (13, Sd)],
        7: [(12, Sd), (13, Sd)],
    }
    for y, row in head.items():
        for x, c in row:
            put(px, x, y, c)
    haft = [(10, 4), (9, 5), (10, 5), (8, 6), (9, 6), (7, 7), (8, 7), (6, 8), (7, 8),
            (5, 9), (6, 9), (4, 10), (5, 10), (3, 11), (4, 11), (2, 12), (3, 12),
            (2, 13), (3, 13), (2, 14)]
    for i, (x, y) in enumerate(haft):
        put(px, x, y, W if i % 3 else Wd)
    save(px, "item/void_alloy_pickaxe.png")


def pack_icon():
    """The pack's icon: a blue crystal in the dark stone, 64x64."""
    rng = random.Random(101)
    big = 64
    img = Image.new("RGBA", (big, big), (0, 0, 0, 255))
    for y in range(big):
        for x in range(big):
            c = rng.choice([VOID_DARK, VOID_MID, VOID_DARK, VOID_EDGE])
            img.putpixel((x, y), c)
    # A diamond of crystal light in the middle.
    cx = cy = big // 2
    for y in range(big):
        for x in range(big):
            d = abs(x - cx) + abs(y - cy)
            if d < 14:
                c = BLUE_WHITE if d < 4 else BLUE_BRIGHT if d < 8 else BLUE_MID if d < 11 else BLUE_DEEP
                img.putpixel((x, y), c)
    path = ROOT.parent.parent.parent.parent / "datapack" / "underbedrock_world" / "pack.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)
    print(f"wrote {path.name}")


if __name__ == "__main__":
    shutil.rmtree(ROOT, ignore_errors=True)
    voidstone()
    voidstone_bricks()
    void_gravel()
    void_crystal_ore()
    void_crystal_block()
    crystal_lamp()
    crystal_sprout()
    voidshell()
    abyss_bedrock()
    void_crystal()
    void_alloy()
    void_alloy_pickaxe()
    pack_icon()
    print("wrote the mod's textures")
