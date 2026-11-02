# Under Bedrock

**A world below the bedrock floor, for Horizon (Minecraft 26.3).**
The bedrock at the bottom of the Overworld is no longer the end of the world:
a void-alloy pickaxe digs through it and through the voidshell gate under it,
and half a kilometre of dark world waits underneath — voidstone caverns lit by
blue crystals, ores, gravel, glowing sprouts, and creatures of their own.

Under Bedrock ships as three parts that only work together: a content mod
(`.hmod`), a native mod (`.hnmod`) and a world data pack. **Prebuilt files for
all three are on the [Releases](../../releases/latest) page** — the repository
carries sources only.

---

## What it adds

**A taller Overworld.** The data pack replaces the Overworld's dimension type
and noise settings: the world runs from y −576 to y 319, 896 blocks of height.
The surface, the oceans and the caves above y −54 are vanilla's own.

**The way down, layer by layer.** Deepslate, then a thin jagged bedrock
transition (y −64…−59), then **Voidshell** (y −72…−65): a black gate block the
engine itself never breaks — only the void-alloy pickaxe digs it, exactly the
way it digs bedrock. Under the gate lies **the Abyss**: voidstone with carved
caves (air, never lava), from y −73 down to y −564. The world's bottom
(y −565…−576) is **Abyss Bedrock**, a second bedrock kind that nothing breaks,
pickaxe included: nobody falls out of the world.

**The Abyss's blocks.** Voidstone, Voidstone Bricks, Void Gravel, Void Crystal
Ore, Block of Void Crystal, Crystal Lamp (full light), Crystal Sprout (a
glowing cave plant), Voidshell, Abyss Bedrock — all with hand-drawn 16×16
textures in a cold blue-on-black palette.

**Void crystal.** The ore sits in two places: sparse veins in ordinary
deepslate and stone (y −60…−16), so the pickaxe is craftable long before
anyone touches bedrock — and rich veins down below, with crystal lamps and
coal, iron and diamond ore in the voidstone besides. Smelting or blasting the
ore gives crystals; a crystal and an iron ingot make a **Void Alloy Ingot**.

**The Void Alloy Pickaxe.** 2800 durability, mining speed 14, enchantment
glint, three alloy ingots and two sticks to craft. It is the only tool that
takes bedrock and voidshell: hold the attack key on the block and a progress
bar fills over the hotbar — three seconds by default — then the block goes
and, if enabled, drops itself. Only the player who holds the pickaxe digs;
everyone else, and every other tool, gets an unbreakable block.

**Inhabitants.** The Abyss is populated by the mod's **own creatures** — no
vanilla mob spawns down there. While a player is below y −80, the native mod
summons drifting crystal **wisps** (its void-crystal item, glowing blue),
hanging **lanterns** (its crystal-lamp block) and the odd **watcher** — a
bigger crystal that closes in on a player who comes near, keeps its distance,
and watches them. Up to ten per player, saved with the world.

**The rest.** English and Russian languages, recipes (smelting, blasting,
alloy, pickaxe, crystal block both ways, bricks, lamp, sprout), loot tables
and an “Underbedrock” creative tab.

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
dropping the zip in, or accept that chunks generated earlier keep the old
floor with void below it.

In game, `/abyss depth` answers
`This world: y -576 .. 319 (896 blocks). Under Bedrock's Abyss is below the bedrock floor.`
A world without the pack is reported once, in chat, as `NOT tall`.

---

## Playing

1. Mine down to deepslate (y −60…−16) and find **Void Crystal Ore** — blue
   crystals in dark stone, faintly lit. Any pickaxe that can mine deepslate
   takes it.
2. Smelt the ore into crystals, alloy them with iron, craft the pickaxe:
   three ingots across the top, two sticks below.
3. Reach the bedrock floor (y −64) and **hold** the left mouse button on a
   bedrock block with the pickaxe in hand. The action bar counts the dig:
   `Bedrock [##########----------] 50%`. Letting go resets it.
4. The bedrock disappears — and under it the voidshell gate waits for the same
   treatment (`Voidshell [#####---------------] 25%`). Behind the gate the
   Abyss opens: dark voidstone caverns, blue crystal veins, glowing lamps.
   The creatures down there are the engine's own kinds; bring a sword.
5. At the very bottom, Abyss Bedrock stops the world. It does not break, for
   anyone, with anything — that is the point of it.

### Commands

| Command | Effect |
| --- | --- |
| `/abyss` | lists the commands |
| `/abyss give` | gives a Void Alloy Pickaxe |
| `/abyss depth` | the world's height, and whether the data pack is in place |
| `/abyss dig` | cycles the bedrock/voidshell dig time (1 / 2 / 3 / 5 / 10 s) |
| `/abyss drop` | toggles whether broken bedrock drops as an item |

### Mod settings

**Options → Mods → Under Bedrock** has two buttons: the dig time
(1 / 2 / 3 / 5 / 10 seconds) and the bedrock drop (ON / OFF). They are kept in
`horizon-mods/.horizon-mod-data/underbedrock_core/`.

---

## How it works

**Why three parts.** Horizon reads dimension types, noise settings, density
functions and material rules only from data packs, so the tall world lives in
`underbedrock_world`: `dimension_type/overworld.json` (min_y −576, height 896),
the Overworld's `noise_settings` with an `aquifers` configuration, three
density functions and one material rule that builds every depth band.
Everything a content mod may carry — blocks, items, recipes, loot, languages,
the creative tab and the ore features (configured and placed features plus
biome modifiers, both for the deepslate veins and the Abyss's) — lives in the
`.hmod`. And bedrock is unbreakable by design (hardness −1), which is engine
territory: the native mod handles the dig.

**No lava, and dry caves.** Horizon's global fluid picker fills every cave
below y −54 with lava unless the noise settings carry an `aquifers` object;
with one, its lava function decides — ours is a flat zero, and the density
under y −54 is plain solid rock, so no fluid cell exists down there at all.
The Abyss's caves are carved by the material rule (a cheese-and-tunnels noise
threshold, plus wide caverns in the middle depths) out of that solid, after
the fill. The pack also replaces the three Overworld carvers with ones that
never start: a carver tunnel below y −54 would be filled with the engine's
deep lava sea. The surface keeps vanilla's density caves, oceans and rivers
through the aquifer's floodedness noise.

**The dig.** The native mod binds six hook points: the mod list row, its
settings page and click, the player tick, and `/abyss`'s registration and
performance. Each player tick it reads the engine's own dig state — the block
the attack key is held on and the ticks it has been held for, which the engine
counts even on unbreakable blocks — and, when that block is bedrock or
voidshell and the held item is the void-alloy pickaxe, reports the progress on
the action bar. At the configured dig time the block is removed through
`native_set_block` with the air state id (verified at runtime), with a
named-block `native_edit_blocks` bulk edit as a fallback, and the drop is
given through `native_give_items`. Abyss bedrock is simply never a dig target.
No internals are mixin-patched; the one private field the mod reads
(`Game::mining`, the dig state) is reached as a same-crate descendant module,
and a future rename of it fails the mod's build loudly rather than misbehaving
quietly.

**The creatures.** No vanilla mob is spawned: each creature is a *display
entity* — an entity Horizon keeps without simulating — that the mod drives
itself (the engine's own documented recipe for a mod's moving models,
`native-mods.md` "Entities", API 59). The same player tick keeps the rhythm:
every three seconds per player, below y −80, it counts the mod's displays
within 40 blocks and, under the cap of ten, tries four deterministic spots in
a ring around the player; a spot with air at the feet and head and a solid
floor gets an `item_display` showing the mod's void crystal (a wisp, or the
bigger watcher) or a `block_display` showing its crystal lamp (a lantern),
through `native_summon`. Every deep tick the mod moves each creature along
its path (`native_move_entity`) and resends its pose — scale and spin — every
eight ticks (`native_set_display`; the clients ease between the poses); a
creature whose player walks 96 blocks away is let go (`native_remove_entity`),
and a display a reloaded world saved with its chunks is adopted and driven
again.

---

## Building from source

The distributable files need **Python 3.11+ only** — no Rust:

```sh
python tools/pack_release.py
```

which writes `release/`: the `.hmod`, the `.hnmod` and the data pack zip
(`pack.mcmeta` at its root) — the three files GitHub Releases carries.
`python tools/pack_release.py --install` adds a downloadable bundle of the
three with an `INSTALL.txt`; it is a transport archive, not repo content. Regenerating textures or worldgen data (`make_textures.py`,
`make_content.py`, both plain Python as well — no Pillow, no Rust) is only needed
when editing them.

Rust enters only the checks, all of them runnable without a Horizon install:

```sh
cd native-mod && cargo test                     # the dig and creature logic, 20 tests
cd ../tools/stubcheck && cargo run -- on && cargo run -- off
                                                # the mod's own sources compiled against
                                                # Horizon's exact signatures, 41 scenarios twice
cd ../.. && python tools/nativecheck.py release/underbedrock_core.hnmod
                                                # the bundle read by the registry's own rules
python tools/check_worldgen.py                  # worldgen JSONs vs Horizon's decoder,
                                                # plus the no-lava / no-infinite-band invariants
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
datapack/underbedrock_world the tall world: dimension type, noise settings and aquifers,
                            density functions, material rule, carver overrides
native-mod/                 the native mod: mixins/underbedrock_core/ holds runtime.rs
                            (logic and tests) and one helper file per hook point
tools/make_textures.py      draws every texture (pure Python, writes its own PNGs)
tools/make_content.py       generates content-mod/ and datapack/ from one place
tools/pack_release.py       packs release/ with nothing but Python
tools/modbuild/             checks and packs the content mod with Horizon's own parser
tools/nativecheck.py        reads a .hnmod by the registry's own rules
tools/stubcheck/            compiles the native sources against Horizon's signatures
                            and runs them on a stub world
tools/check_worldgen.py     the worldgen JSONs vs Horizon's decoder and the mod's invariants
tools/build_all.sh          everything above, in order
```

Built and checked against Horizon `ea55997` (26.3, native hook points v126,
`NATIVE_HOOKS_API` 153) and horizon-mixins `ddd9700`; every file format was
verified against that revision's sources.

---

## Known limitations

- The bedrock and voidshell dig is a survival/adventure mechanic; in creative
  mode bedrock breaks instantly, as vanilla does.
- While the dig runs, the player hears the ordinary mining sound the engine
  sends; there is no separate break sound or particle burst — Horizon exposes
  no hook point for them.
- Breaking bedrock or voidshell costs the pickaxe no durability (no hook point
  damages a held stack); ordinary mining wears it as usual.
- The Abyss's creatures are display entities the native mod summons and
  drives: Horizon has no custom living entity types for mods (a content mod's
  entity types are thrown-item projectiles only), so the creatures are
  ambience — they drift, hang and watch, but they neither attack nor drop
  loot. They spawn only near a player in the Abyss, not as part of world
  generation.
- Worlds created before the data pack was installed keep their old height in
  already-generated chunks.
- The native mod reads one private engine field (the player's dig state); see
  “How it works” for why, and what happens if it ever moves.
