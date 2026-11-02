#!/usr/bin/env python3
"""Pixel-art textures of the Underbedrock mod (16x16 PNG, no external assets).

Everything is drawn from a fixed seed, so the same script always gives the same
bytes. Run: python3 tools/make_textures.py
"""
import random
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1] / "content-mod" / "assets" / "underbedrock" / "textures"
S = 16

# The mod's palette: a cold void stone and the teal "enchanted alloy" in it.
VOID_DARK = (25, 25, 35, 255)
VOID_MID = (36, 36, 50, 255)
VOID_LIGHT = (52, 52, 70, 255)
VOID_EDGE = (17, 17, 25, 255)

ETHER_DEEP = (12, 74, 78, 255)
ETHER_MID = (26, 150, 152, 255)
ETHER_BRIGHT = (94, 235, 224, 255)
ETHER_WHITE = (198, 255, 250, 255)
ETHER_VIOLET = (86, 62, 148, 255)
ETHER_VIOLET_LIGHT = (146, 116, 214, 255)

STEEL_DARK = (52, 56, 66, 255)
STEEL_MID = (96, 100, 112, 255)
STEEL_LIGHT = (146, 150, 162, 255)
WOOD_DARK = (74, 52, 30, 255)
WOOD_MID = (110, 78, 44, 255)


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
    print(f"wrote {path.relative_to(ROOT.parents[3])}")


# --- blocks -----------------------------------------------------------------

def voidstone(seed=11):
    """The stone of the world under the bedrock: colder and darker than deepslate."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_MID] * 6 + [VOID_DARK] * 3 + [VOID_LIGHT] * 2)
    # A few darker cracks and lighter grains, as deepslate's stripes.
    for _ in range(7):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(2, 5)):
            put(px, x, y, VOID_EDGE if rng.random() < 0.6 else VOID_LIGHT)
            x += rng.choice((-1, 0, 1))
            y += rng.choice((0, 1))
    # Faint violet veins: this stone is not quite stone.
    for _ in range(3):
        x, y = rng.randrange(S), rng.randrange(S)
        for _ in range(rng.randint(2, 4)):
            put(px, x, y, ETHER_VIOLET)
            x += 1
            y += rng.choice((-1, 0, 1))
    save(px, "block/voidstone.png")


def voidstone_bricks(seed=29):
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_MID] * 5 + [VOID_DARK] * 3 + [VOID_LIGHT] * 2)
    # Two courses of bricks, offset, with dark mortar.
    for row, offset in ((0, 0), (1, 4)):
        y0 = row * 8
        for y in (y0, y0 + 7):
            for x in range(S):
                put(px, x, y, VOID_EDGE)
        for x in ((offset + 7) % S, (offset + 15) % S):
            for y in range(y0, y0 + 8):
                put(px, x, y, VOID_EDGE)
    for _ in range(20):
        put(px, rng.randrange(S), rng.randrange(S), rng.choice([VOID_LIGHT, VOID_DARK]))
    save(px, "block/voidstone_bricks.png")


def etherium_ore(seed=41):
    """Etherium in the void stone: teal crystals with a bright core."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_MID] * 6 + [VOID_DARK] * 3 + [VOID_LIGHT] * 2)
    for cx, cy in ((4, 4), (11, 6), (6, 11), (12, 12)):
        blob(px, rng, cx, cy, 2, [ETHER_DEEP, ETHER_MID, ETHER_MID])
        put(px, cx, cy, ETHER_BRIGHT)
        put(px, cx + rng.choice((-1, 1)), cy + rng.choice((-1, 0)), ETHER_WHITE)
    save(px, "block/etherium_ore.png")


def etherium_block(seed=53):
    """The alloy's block: dark metal plates with a teal, faintly lit seam."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [ETHER_DEEP] * 5 + [(16, 92, 96, 255)] * 3 + [ETHER_MID])
    for i in range(S):
        put(px, i, 0, ETHER_VIOLET)
        put(px, 0, i, ETHER_VIOLET)
        put(px, i, S - 1, (8, 48, 52, 255))
        put(px, S - 1, i, (8, 48, 52, 255))
    # A bevelled centre and two bright corners, as netherite block's sheen.
    for x in range(3, 13):
        for y in range(3, 13):
            if (x + y) % 7 == 0:
                put(px, x, y, ETHER_BRIGHT)
    for x, y in ((4, 4), (11, 5), (5, 11), (11, 11)):
        put(px, x, y, ETHER_WHITE)
    for _ in range(10):
        put(px, rng.randrange(2, 14), rng.randrange(2, 14), ETHER_VIOLET_LIGHT)
    save(px, "block/etherium_block.png")


def ether_crystal(seed=67):
    """A cluster of glowing crystals in the rock: the world's only light."""
    rng = random.Random(seed)
    px = canvas()
    noise_fill(px, rng, [VOID_DARK] * 6 + [VOID_MID] * 3 + [VOID_EDGE])
    spikes = [((3, 13), (4, 5)), ((8, 14), (8, 3)), ((12, 12), (12, 6)), ((6, 9), (2, 7))]
    for (bx, by), (tx, ty) in spikes:
        # A tapered shard from its base to its tip.
        steps = max(abs(bx - tx), abs(by - ty))
        for i in range(steps + 1):
            t = i / max(steps, 1)
            x = round(bx + (tx - bx) * t)
            y = round(by + (ty - by) * t)
            width = 2 if t < 0.6 else 1
            for dx in range(width):
                put(px, x + dx, y, ETHER_MID if t < 0.75 else ETHER_BRIGHT)
            if t < 0.4:
                put(px, x - 1, y, ETHER_DEEP)
        put(px, tx, ty, ETHER_WHITE)
    for _ in range(8):
        x, y = rng.randrange(S), rng.randrange(S)
        if px[y][x] in (VOID_DARK, VOID_MID, VOID_EDGE) and rng.random() < 0.5:
            put(px, x, y, ETHER_DEEP)
    save(px, "block/ether_crystal.png")


# --- items ------------------------------------------------------------------

def raw_etherium(seed=83):
    """Raw ore chunks, as raw iron's: dull teal nuggets with bright facets."""
    rng = random.Random(seed)
    px = canvas()
    for cx, cy, r in ((5, 5, 3), (11, 7, 3), (7, 11, 3)):
        for _ in range(r * r * 3):
            x = cx + rng.randint(-r, r)
            y = cy + rng.randint(-r, r)
            d = (x - cx) ** 2 + (y - cy) ** 2
            if d <= r * r:
                shade = ETHER_WHITE if d <= 1 else ETHER_BRIGHT if rng.random() < 0.3 else ETHER_MID
                if d >= r * r - 1:
                    shade = ETHER_DEEP
                put(px, x, y, shade)
    save(px, "item/raw_etherium.png")


def etherium_ingot(seed=97):
    """An ingot of the enchanted alloy: violet metal, a lit teal top face."""
    px = canvas()
    rows = [
        "..TTTTTT....",
        ".TttttttT...",
        "TtBBWWBBtT..",
        "TtBWWWWBtT..",
        "TtBBBBBBtT..",
        ".TMMMMMMT...",
        "..TTTTTT....",
    ]
    colors = {
        "T": ETHER_VIOLET,
        "t": ETHER_VIOLET_LIGHT,
        "B": ETHER_MID,
        "W": ETHER_BRIGHT,
        "M": ETHER_DEEP,
    }
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch != ".":
                put(px, x + 2, y + 5, colors[ch])
    put(px, 6, 7, ETHER_WHITE)
    put(px, 9, 8, ETHER_WHITE)
    put(px, 4, 6, ETHER_WHITE)
    save(px, "item/etherium_ingot.png")


def etherium_pickaxe(seed=113):
    """The pickaxe: a dark haft, an alloy head with a lit edge and a violet gleam."""
    rng = random.Random(seed)
    px = canvas()
    head_edge = ETHER_BRIGHT
    head_mid = ETHER_MID
    head_dark = ETHER_DEEP
    gleam = ETHER_WHITE

    # The head: an arc across the top, its two tips pointing down.
    arc = [
        (2, 5), (3, 4), (4, 3), (5, 2), (6, 2), (7, 2), (8, 2), (9, 3), (10, 4), (11, 5),
    ]
    for x, y in arc:
        put(px, x, y, head_edge)
        put(px, x, y + 1, head_mid)
        put(px, x, y + 2, head_dark)
    for x, y in ((2, 6), (11, 6)):
        put(px, x, y + 1, head_dark)
    # The socket the haft goes into.
    for x in (6, 7):
        for y in (4, 5, 6):
            put(px, x, y, head_mid if y < 6 else head_dark)
    put(px, 6, 3, gleam)
    put(px, 9, 3, gleam)

    # The haft: a two-pixel diagonal down to the bottom right.
    for i in range(8):
        x, y = 7 + i, 6 + i
        put(px, x, y, WOOD_MID)
        put(px, x + 1, y, WOOD_DARK)
        put(px, x, y + 1, WOOD_DARK)
    put(px, 14, 13, WOOD_DARK)
    put(px, 7, 6, STEEL_MID)
    # A violet wrap where the head meets the haft, and a spark on the edge.
    put(px, 8, 8, ETHER_VIOLET_LIGHT)
    put(px, 9, 9, ETHER_VIOLET)
    for _ in range(4):
        x = rng.randrange(3, 11)
        y = rng.randrange(2, 6)
        if px[y][x] is None:
            put(px, x, y, ETHER_VIOLET_LIGHT)
    save(px, "item/etherium_pickaxe.png")


def pack_icon():
    """The data pack's 64x64 icon: a bedrock floor with the world under it."""
    size = 64
    img = Image.new("RGBA", (size, size), (8, 8, 12, 255))
    rng = random.Random(7)
    for y in range(size):
        for x in range(size):
            if y < 14:
                img.putpixel((x, y), (30, 30, 42, 255))
            elif y < 24:
                c = rng.choice([(52, 52, 58, 255), (74, 74, 82, 255), (40, 40, 46, 255)])
                img.putpixel((x, y), c)
            else:
                c = rng.choice([VOID_DARK, VOID_MID, VOID_MID, VOID_LIGHT])
                img.putpixel((x, y), c)
    for _ in range(70):
        x, y = rng.randrange(size), rng.randrange(26, size)
        img.putpixel((x, y), rng.choice([ETHER_MID, ETHER_BRIGHT, ETHER_DEEP]))
    for cx, cy in ((18, 40), (44, 50)):
        for dx in range(-3, 4):
            for dy in range(-3, 4):
                if dx * dx + dy * dy <= 9:
                    img.putpixel((cx + dx, cy + dy), ETHER_BRIGHT)
    out = Path(__file__).resolve().parents[1] / "datapack" / "underbedrock_world" / "pack.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    img.save(out)
    print(f"wrote {out}")


if __name__ == "__main__":
    voidstone()
    voidstone_bricks()
    etherium_ore()
    etherium_block()
    ether_crystal()
    raw_etherium()
    etherium_ingot()
    etherium_pickaxe()
    pack_icon()
