# Under Bedrock

**A world below the bedrock floor, for Horizon (Minecraft 26.3).**
The bedrock at the bottom of the Overworld is no longer the end of the world:
an enchanted-alloy pickaxe digs through it, and half a kilometre of new world
waits underneath — new stone, new ore, new crystals, caves of their own.

Under Bedrock ships as three parts that only work together: a content mod
(`.hmod`), a native mod (`.hnmod`) and a world data pack. **Prebuilt files for
all three are on the [Releases](../../releases/latest) page** — the repository
carries sources only.

---

## What it adds

**A taller Overworld.** The data pack replaces the Overworld's dimension type
and noise settings: the world runs from y −576 to y 319, 896 blocks of height.
The bedrock floor stays where vanilla puts it, at y −64…−60 — except now there
is a world under it.

**The Underbedrock.** Below the floor lies voidstone: a dark rock with its own
caves, etherium and ether-crystal veins, and diamond and iron ore inside it.
Blocks: Voidstone, Voidstone Bricks, Etherium Ore, Block of Etherium,
Ether Crystal.

**Etherium.** A new ore in two places: sparse veins in ordinary deepslate and
stone (y −60…−16), so the pickaxe is craftable long before anyone touches
bedrock — and rich veins down below (etherium y −540…−96, crystals
y −520…−90). The ore drops Raw Etherium; smelting or blasting turns it into
Enchanted Alloy Ingots.

**The Enchanted Alloy Pickaxe.** 2800 durability, mining speed 14, enchantment
glint, three ingots and two blaze rods to craft. It is the only tool that takes
bedrock: hold the attack key on a bedrock block and a progress bar fills over
the hotbar — three seconds by default — then the block goes and, if enabled,
drops itself as an item.

**The rest.** Hand-drawn 16×16 textures, English and Russian languages,
recipes (smelting, blasting, pickaxe, alloy block both ways, voidstone bricks),
loot tables and an “Underbedrock” creative tab.

---

## Installation

Requires **Horizon 26.3**. Mods are off by default in Horizon: a fresh install
has no mod folders and plays exactly like vanilla.

Download the three files from the [latest release](../../releases/latest):

| File | Goes into | Notes |
| --- | --- | --- |
| `underbedrock.hmod` | `horizon-mods/` | create the folder if it does not exist |
| `underbedrock_core.hnmod` | `horizon-native-mods/` | the launcher compiles a modded game once, needs internet |
| `underbedrock_world.zip` | `saves/<world>/datapacks/` | the zip goes in **as it is** — `pack.mcmeta` sits at the archive's root |

Then enable both mods in **Options → Mods** (or write `mod.underbedrock:true`
and the native mod's switch into `horizon-mods.txt` beside the folders).
World data packs in a world's `datapacks/` folder are enabled by the game
itself.

The native mod is source-only code that runs with the player's rights: Horizon
asks for consent before compiling it, and the sources travel inside the
`.hnmod` archive (`mixins/underbedrock_core/*.rs`) to be read beforehand. It
uses nothing but Horizon's official native hook points (`hooks: 126`), so it
builds on any Horizon whose hook points are that version or newer.

**The data pack needs a new world.** A world's height is fixed when it loads,
so the pack must be in place before chunks generate: create the world after
dropping the zip in, or accept that chunks generated earlier keep the old floor
with void below it.

In game, `/etherium depth` answers
`This world: y -576 .. 319 (896 blocks). Under Bedrock's world is below the bedrock floor.`
A world without the pack is reported once, in chat, as `NOT tall`.

---

## Playing

1. Mine down to deepslate (y −60…−16) and find **Etherium Ore** — teal veins
   with violet sparks. Any pickaxe that can mine deepslate takes it.
2. Smelt the raw etherium, craft the pickaxe: three ingots across the top,
   two blaze rods below.
3. Reach the bedrock floor (y −64) and **hold** the left mouse button on a
   bedrock block with the pickaxe in hand. The action bar counts the dig:
   `Bedrock [##########----------] 50%`. Letting go resets it.
4. The bedrock disappears, drops into the inventory (when the drop is on), and
   the Underbedrock opens below: voidstone, ores, crystal clusters. Bring
   torches — nothing down there knows the sun.

### Commands

| Command | Effect |
| --- | --- |
| `/etherium` | lists the commands |
| `/etherium give` | gives an Enchanted Alloy Pickaxe |
| `/etherium depth` | the world's height, and whether the data pack is in place |
| `/etherium dig` | cycles the bedrock dig time (1 / 2 / 3 / 5 / 10 s) |
| `/etherium drop` | toggles whether broken bedrock drops as an item |

### Mod settings

**Options → Mods → Under Bedrock** has two buttons: the bedrock dig time
(1 / 2 / 3 / 5 / 10 seconds) and the bedrock drop (ON / OFF). They are kept in
`horizon-mods/.horizon-mod-data/underbedrock_core/`.

---

## How it works

**Why three parts.** Horizon reads dimension types, noise settings, density
functions and material rules only from data packs, so the tall world lives in
`underbedrock_world`: `dimension_type/overworld.json` (min_y −576, height 896),
the Overworld's `noise_settings`, five density functions and two material rules
that shell the new depth in voidstone with caves. Everything a content mod may
carry — blocks, items, recipes, loot, languages, the creative tab and the ore
features (configured and placed features plus biome modifiers, both for the
deepslate veins and the under-bedrock ones) — lives in the `.hmod`. And bedrock
is unbreakable by design (hardness −1), which is engine territory: the native
mod handles the dig.

**The dig.** The native mod binds six hook points: the mod list row, its
settings page and click, the player tick, and `/etherium`'s registration and
performance. Each player tick it reads the engine's own dig state — the block
the attack key is held on and the ticks it has been held for, which the engine
counts even on unbreakable blocks — and, when that block is bedrock and the
held item is the alloy pickaxe, reports the progress on the action bar. At the
configured dig time the bedrock is removed through `native_set_block` with the
air state id (verified at runtime), with a named-block `native_edit_blocks`
bulk edit as a fallback, and the drop is given through `native_give_items`.
No internals are mixin-patched; the one private field the mod reads
(`Game::mining`, the dig state) is reached as a same-crate descendant module,
and a future rename of it fails the mod's build loudly rather than misbehaving
quietly.

---

## Building from source

The distributable files need **Python 3.11+ only** — no Rust:

```sh
python tools/pack_release.py
```

which writes `release/`: the `.hmod`, the `.hnmod`, the data pack zip
(`pack.mcmeta` at its root) and an `underbedrock-install.zip` bundle with an
`INSTALL.txt`. Regenerating textures or worldgen data (`make_textures.py`,
`make_content.py`, both Python as well) is only needed when editing them.

Rust enters only the checks, all of them runnable without a Horizon install:

```sh
cd native-mod && cargo test                     # the dig logic, 9 tests
cd ../tools/stubcheck && cargo run -- on && cargo run -- off
                                                # the mod's own sources compiled against
                                                # Horizon's exact signatures, 34 scenarios twice
cd ../.. && python tools/nativecheck.py release/underbedrock_core.hnmod
                                                # the bundle read by the registry's own rules
```

The full pipeline — textures, content, Horizon's own content parser and packer
(`tools/modbuild`, which pulls `horizon-mods` from the pinned revision), the
native bundle, all checks — is one command in Git Bash or any shell:

```sh
sh tools/build_all.sh
```

### Repository layout

```
content-mod/                the content mod: blocks, items, recipes, loot, languages, textures
datapack/underbedrock_world the tall world: dimension type, noise settings, density
                            functions, material rules, cave noise
native-mod/                 the native mod: mixins/underbedrock_core/ holds runtime.rs
                            (logic and tests) and one helper file per hook point
tools/make_textures.py      draws every texture (Pillow)
tools/make_content.py       generates content-mod/ and datapack/ from one place
tools/pack_release.py       packs release/ with nothing but Python
tools/modbuild/             checks and packs the content mod with Horizon's own parser
tools/nativecheck.py        reads a .hnmod by the registry's own rules
tools/stubcheck/            compiles the native sources against Horizon's signatures
                            and runs them on a stub world
tools/build_all.sh          everything above, in order
```

Built and checked against Horizon `ea55997` (26.3, native hook points v126,
`NATIVE_HOOKS_API` 153) and horizon-mixins `ddd9700`; every file format was
verified against that revision's sources.

---

## Known limitations

- The bedrock dig is a survival/adventure mechanic; in creative mode bedrock
  breaks instantly, as vanilla does.
- While the dig runs, the player hears the ordinary mining sound the engine
  sends; there is no separate break sound or particle burst — Horizon exposes
  no hook point for them.
- Breaking bedrock costs the pickaxe no durability (no hook point damages a
  held stack); ordinary mining wears it as usual.
- Worlds created before the data pack was installed keep their old height in
  already-generated chunks.
- The native mod reads one private engine field (the player's dig state); see
  “How it works” for why, and what happens if it ever moves.
