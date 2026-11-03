#!/usr/bin/env python3
"""Pixel-art textures of the Underbedrock mod (16x16 PNG, no external assets).

Everything is drawn from a fixed seed, so the same script always gives the same
bytes, and the PNGs are written by hand (no Pillow needed).
Run: python3 tools/make_textures.py

The palette is the Abyss: near-black blue stone, and void crystals in it that
glow cold blue.
"""
import random
import shutil
import struct
import zlib
from pathlib import Path


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


def _chunk(tag, data):
    body = tag + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)


def write_png(path, rows):
    """A truecolour RGBA PNG, written by hand: no Pillow anywhere."""
    width, height = len(rows[0]), len(rows)
    raw = b"".join(
        b"\x00" + b"".join(bytes(px) if px is not None else b"\x00\x00\x00\x00" for px in row)
        for row in rows
    )
    png = b"\x89PNG\r\n\x1a\n"
    png += _chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
    png += _chunk(b"IDAT", zlib.compress(raw, 9))
    png += _chunk(b"IEND", b"")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(png)


def save(px, name):
    write_png(ROOT / name, px)
    print(f"wrote {name}")


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
    """A void sprout: two teal stems with broad leaves and a crystal tip,
    readable at a glance against the dark stone it grows on."""
    px = canvas()
    stem = (36, 84, 108, 255)
    leaf = (52, 116, 140, 255)
    # The main stem, two pixels wide so it survives the block's cross model.
    for y in range(8, 16):
        put(px, 7, y, stem)
        put(px, 8, y, stem)
    # A side shoot.
    for y in range(11, 16):
        put(px, 11, y, stem)
    # Broad leaves, one per stem.
    for x, y in ((5, 10), (4, 9), (5, 9), (6, 10), (10, 12), (12, 11), (12, 12)):
        put(px, x, y, leaf)
    put(px, 4, 8, BLUE_DEEP)
    put(px, 13, 10, BLUE_DEEP)
    # The crystal fan on the main tip: a bright spike with two shoulders.
    for x, y, c in ((7, 7, BLUE_BRIGHT), (8, 7, BLUE_BRIGHT),
                    (7, 6, BLUE_WHITE), (8, 6, BLUE_BRIGHT),
                    (7, 5, BLUE_WHITE), (8, 4, BLUE_WHITE),
                    (6, 7, BLUE_MID), (9, 7, BLUE_MID),
                    (6, 6, BLUE_DEEP), (9, 6, BLUE_DEEP),
                    (11, 10, BLUE_MID), (11, 9, BLUE_BRIGHT), (11, 8, BLUE_WHITE)):
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
    """The ingot: a bevelled bar with a bright poured top and a blue glow line
    through its dark body - an ingot's shape, not a flat plate."""
    px = canvas()
    body = [(60, 66, 92, 255), (44, 48, 68, 255), (32, 34, 50, 255)]
    for x in range(4, 12):
        put(px, x, 5, BLUE_MID)          # the bevel's top edge
    for x in range(3, 13):
        put(px, x, 6, BLUE_WHITE)        # the poured top face
        put(px, x, 7, BLUE_BRIGHT)
        put(px, x, 8, body[0])
        put(px, x, 9, body[1])
        put(px, x, 10, body[2])
    for x in range(4, 12):
        put(px, x, 11, (22, 24, 36, 255))  # the bevel's bottom edge
    # The glow line the void leaves in the pour.
    for x in range(4, 12):
        put(px, x, 8, BLUE_MID if x % 2 else BLUE_BRIGHT)
    put(px, 5, 6, BLUE_WHITE)
    put(px, 10, 7, BLUE_WHITE)
    put(px, 3, 8, BLUE_DEEP)
    put(px, 12, 9, BLUE_DEEP)
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


def emberstone(seed=101):
    """Lava zone rock: dark iron-brown stone with glowing ember cracks."""
    rng = random.Random(seed)
    px = canvas()
    base = [(38, 20, 16, 255), (52, 26, 18, 255), (30, 16, 14, 255), (60, 30, 20, 255)]
    noise_fill(px, rng, base * 3)
    for _ in range(6):
        blob(px, rng, rng.randrange(3, 13), rng.randrange(3, 13), 2,
             [(232, 108, 24, 255), (255, 158, 42, 255), (196, 66, 18, 255)])
    save(px, "block/emberstone.png")


def scorched_stone(seed=103):
    """Inactive hot rock: cooled ash-grey stone with dark scorch patches."""
    rng = random.Random(seed)
    px = canvas()
    base = [(44, 40, 38, 255), (56, 50, 46, 255), (36, 33, 31, 255), (66, 58, 52, 255)]
    noise_fill(px, rng, base * 3)
    for _ in range(5):
        blob(px, rng, rng.randrange(3, 13), rng.randrange(3, 13), 3,
             [(28, 25, 24, 255), (24, 21, 20, 255)])
    save(px, "block/scorched_stone.png")


def froststone(seed=107):
    """Frozen wastes rock: pale blue-grey ice stone with frost veins."""
    rng = random.Random(seed)
    px = canvas()
    base = [(122, 156, 178, 255), (146, 182, 202, 255), (98, 128, 150, 255), (168, 200, 216, 255)]
    noise_fill(px, rng, base * 3)
    for _ in range(6):
        blob(px, rng, rng.randrange(3, 13), rng.randrange(3, 13), 2,
             [(214, 236, 246, 255), (232, 246, 252, 255)])
    save(px, "block/froststone.png")


def frost_gravel(seed=109):
    """Frozen gravel: pale blue-grey pebbles with bright frost specks."""
    rng = random.Random(seed)
    px = canvas()
    base = [(110, 138, 158, 255), (88, 112, 132, 255), (140, 168, 186, 255), (70, 90, 108, 255)]
    noise_fill(px, rng, base * 4)
    for _ in range(10):
        put(px, rng.randrange(S), rng.randrange(S), (206, 228, 240, 255))
    save(px, "block/frost_gravel.png")


def _shard(colors, seed):
    """A crystal shard sprite: a rough diamond with a bright rim."""
    rng = random.Random(seed)
    px = canvas()
    for y in range(S):
        for x in range(S):
            d = abs(x - 8) + abs(y - 9)
            if d <= 6 - abs(y - 9) // 3 and rng.random() < 0.92:
                put(px, x, y, colors[(x + y) % len(colors)])
    return px


def ember_shard(seed=113):
    save(_shard([(255, 158, 42, 255), (232, 108, 24, 255), (255, 200, 90, 255), (196, 66, 18, 255)], seed),
         "item/ember_shard.png")


def frost_shard(seed=127):
    """A frost shard: one pale icicle, white at the core and icy at the edges,
    instead of a rough blob."""
    px = canvas()
    ice = [(232, 246, 252, 255), (214, 236, 246, 255), (146, 182, 202, 255), (98, 128, 150, 255)]
    put(px, 8, 3, ice[0])
    for y in range(4, 7):
        put(px, 8, y, ice[0])
        put(px, 7, y, ice[1])
    for y in range(7, 11):
        put(px, 8, y, ice[0])
        put(px, 7, y, ice[1])
        put(px, 9, y, ice[2])
        put(px, 6, y, ice[3])
    for y in range(11, 14):
        put(px, 8, y, ice[1])
        put(px, 7, y, ice[2])
        put(px, 9, y, ice[3])
    put(px, 8, 14, ice[2])
    put(px, 9, 5, ice[0])
    put(px, 6, 9, ice[3])
    save(px, "item/frost_shard.png")


def cinder_shard(seed=137):
    """A cinder shard: the ember's cooled cousin - a dull grey-black chunk
    with one faint orange crack still glowing through it."""
    px = canvas()
    rng = random.Random(seed)
    ash = [(58, 54, 52, 255), (44, 41, 40, 255), (74, 68, 64, 255), (30, 28, 27, 255)]
    glow = [(214, 108, 34, 255), (255, 158, 64, 255)]
    for y in range(4, 13):
        w = 5 - abs(y - 8) // 2
        for x in range(8 - w // 2, 8 + w // 2 + 1):
            put(px, x, y, ash[rng.randrange(len(ash))])
    for y in (6, 7, 8, 9):
        put(px, 8 - (y % 2), y, glow[0 if y % 2 else 1])
    put(px, 9, 5, ash[2])
    put(px, 6, 10, ash[3])
    save(px, "item/cinder_shard.png")


def gloom_pearl(seed=139):
    """A gloom pearl: a round deep-blue pearl with one soft pale light in
    its belly - the gloomling's own body, and what the deep keeps."""
    px = canvas()
    shell = [(38, 52, 92, 255), (26, 36, 66, 255), (52, 72, 122, 255), (18, 24, 46, 255)]
    core = [(148, 196, 232, 255), (108, 156, 204, 255)]
    for y in range(4, 13):
        w = 7 - abs(y - 8)
        for x in range(8 - w // 2, 8 + w // 2 + 1):
            d = abs(x - 8) + abs(y - 8)
            put(px, x, y, shell[min(d // 2, 3)] if d > 2 else core[d % 2])
    put(px, 7, 6, core[0])
    put(px, 6, 5, (210, 234, 250, 255))
    save(px, "item/gloom_pearl.png")


def stalker_shard(seed=131):
    """The stalker's shard: a jagged near-black spike with one red eye slit
    down its face - the hunter's own colour, for its visible body."""
    px = canvas()
    dark = [(24, 10, 14, 255), (48, 12, 18, 255), (12, 6, 8, 255), (70, 16, 22, 255)]
    put(px, 8, 2, dark[3])
    for y in range(3, 6):
        put(px, 8, y, dark[1])
        put(px, 7, y, dark[2])
    for y in range(6, 11):
        put(px, 7, y, dark[0])
        put(px, 8, y, dark[1])
        put(px, 9, y, dark[2])
        put(px, 6, y, dark[2])
    for y in range(11, 14):
        put(px, 7, y, dark[2])
        put(px, 8, y, dark[0])
        put(px, 9, y, dark[2])
        put(px, 10, y, dark[2])
    put(px, 8, 14, dark[2])
    # The eye slit, and a jag on each side.
    for y in range(6, 11):
        put(px, 8, y, (226, 40, 46, 255))
    put(px, 8, 8, (255, 120, 110, 255))
    put(px, 5, 8, dark[3])
    put(px, 11, 10, dark[3])
    save(px, "item/stalker_shard.png")


TOOL_HEADS = {
    "sword": [(8, 2), (9, 3), (10, 4), (11, 5), (11, 6), (10, 5), (9, 4), (8, 3),
              (7, 2), (7, 3), (8, 4), (9, 5), (10, 6), (11, 7), (12, 6),
              (6, 7), (5, 8), (7, 6), (6, 6)],
    "axe": [(8, 2), (9, 2), (10, 3), (11, 4), (10, 5), (9, 6), (8, 3), (9, 4), (10, 4),
            (9, 5), (8, 4), (7, 5), (8, 6)],
    "hoe": [(9, 2), (10, 2), (11, 2), (11, 3), (10, 3), (9, 4), (8, 5)],
    "shovel": [(9, 2), (10, 2), (9, 3), (10, 3), (9, 4), (10, 4), (8, 5), (9, 5)],
}

TOOL_METAL = [(88, 74, 128, 255), (112, 96, 158, 255), (70, 58, 104, 255), (134, 118, 182, 255)]


def void_tool(kind, seed):
    """A void-alloy tool head on a dark wood handle, silhouette per tool kind."""
    rng = random.Random(seed)
    px = canvas()
    for (x, y) in TOOL_HEADS[kind]:
        put(px, x, y, TOOL_METAL[(x + y + rng.randrange(0, 1)) % len(TOOL_METAL)])
    for i in range(6):
        put(px, 7 - i, 8 + i, (94, 66, 40, 255) if i % 2 else (72, 50, 30, 255))
    put(px, 8, 7, TOOL_METAL[3])
    save(px, f"item/void_alloy_{kind}.png")


ARMOR_SHAPES = {
    "helmet": [(x, y) for y in range(3, 9) for x in range(4, 12)
               if not (y > 5 and 6 <= x <= 9)],
    "chestplate": [(x, y) for y in range(3, 12) for x in range(4, 12)
                   if not (4 <= x <= 5 and y > 8) and not (10 <= x <= 11 and y > 8)
                   and not (7 <= x <= 8 and y > 5)],
    "leggings": [(x, y) for y in range(3, 12) for x in range(5, 11)
                 if not (7 <= x <= 8 and y > 6)],
    "boots": [(x, y) for y in range(6, 12) for x in range(5, 11)
              if not (7 <= x <= 8 and y < 10)],
}


def void_armor(piece, seed):
    """Void-alloy armor sprite, shape per piece."""
    rng = random.Random(seed)
    px = canvas()
    for (x, y) in ARMOR_SHAPES[piece]:
        put(px, x, y, TOOL_METAL[(x * 3 + y) % len(TOOL_METAL)])
    save(px, f"item/void_alloy_{piece}.png")


def pack_icon():
    """The pack's icon: a blue crystal in the dark stone, 64x64."""
    rng = random.Random(101)
    big = 64
    rows = [[rng.choice([VOID_DARK, VOID_MID, VOID_DARK, VOID_EDGE]) for _ in range(big)]
            for _ in range(big)]
    # A diamond of crystal light in the middle.
    cx = cy = big // 2
    for y in range(big):
        for x in range(big):
            d = abs(x - cx) + abs(y - cy)
            if d < 14:
                rows[y][x] = (BLUE_WHITE if d < 4 else BLUE_BRIGHT if d < 8
                              else BLUE_MID if d < 11 else BLUE_DEEP)
    path = ROOT.parent.parent.parent.parent / "datapack" / "underbedrock_world" / "pack.png"
    write_png(path, rows)
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
    emberstone()
    scorched_stone()
    froststone()
    frost_gravel()
    ember_shard()
    frost_shard()
    cinder_shard()
    gloom_pearl()
    stalker_shard()
    void_tool("sword", 141)
    void_tool("axe", 142)
    void_tool("hoe", 143)
    void_tool("shovel", 144)
    void_armor("helmet", 151)
    void_armor("chestplate", 152)
    void_armor("leggings", 153)
    void_armor("boots", 154)
    pack_icon()
    print("wrote the mod's textures")
