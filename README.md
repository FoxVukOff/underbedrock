# Under Bedrock

**A world below the bedrock floor, for Horizon (Minecraft 26.3).**
The bedrock at the bottom of the Overworld is no longer the end of the world:
a void-alloy pickaxe digs through it and through the voidshell gate under it,
and a kilometre of dark world waits underneath — seven layers of it, from a
glowing lava zone down to the absolute void where the warden hunts, with
ores, ruins, glowing plants, creatures of their own — and one beast down
there that can be tamed.

Under Bedrock ships as three parts that only work together: a content mod
(`.hmod`), a native mod (`.hnmod`) and a world data pack. The prebuilt files
for all three live in this repository's `release/` folder.

---

## What it adds

**A taller Overworld.** The data pack replaces the Overworld's dimension type
and noise settings: the world runs from y −1008 to y 319, 1328 blocks of height.
The surface, the oceans and the caves above y −54 are vanilla's own.

**The way down, layer by layer.** Deepslate, then a thin jagged bedrock
transition (y −64…−59), then **Voidshell** (y −72…−65): a black gate block the
engine itself never breaks — only the void-alloy pickaxe digs it, exactly the
way it digs bedrock. Under the gate lies **the Abyss**, seven layers of it,
each colder and darker than the one above. The boundaries are not walls: each
one is a blend of the two rocks over a dozen blocks — and crossing one
sparkles, a veil of end-rod light at the boundary's own height:

| Layer | Depth | What it is |
| --- | --- | --- |
| **Lava zone** | y −73…−220 | A wide open space of magma-like **emberstone** islands (faintly lit) over **contained lava basins** — lakes with a stone floor and a rim ring, nothing to fall over — and glowing lava veins in the rock |
| **Inactive lava zone** | y −221…−360 | Cooled **scorched stone**: the lava here solidified into glowing emberstone veins, with ordinary caves and gravel beds |
| **World-in-the-depths** | y −361…−500 | The calmest layer: **voidstone** with wide caverns and the mod's decor — ruin bricks, crystal lamps, glowing sprouts, gravel — the ores (crystal, coal, iron, diamond), and one creature of its own, the tameable gloomling |
| **Cool lands** | y −501…−640 | **Voidstone** caverns and the classic company: wisps, lanterns and the odd watcher |
| **Frozen wastes** | y −641…−780 | Pale **froststone**, sparse silent caves, frost gravel and frostmotes |
| **Absolute void** | y −781…−960 | Near-black stone, no ambient life at all — and the warden |

The world's bottom (y −961…−1008) is **Abyss Bedrock**, a second bedrock kind
that nothing breaks, pickaxe included: nobody falls out of the world.

**The Abyss's blocks.** Voidstone, Voidstone Bricks, Void Gravel, Void Crystal
Ore, Block of Void Crystal, Crystal Lamp (full light), Crystal Sprout (a
glowing cave plant), Emberstone (light 4), Scorched Stone, Froststone, Frost
Gravel, Voidshell, Abyss Bedrock — thirteen blocks, all with hand-drawn 16×16
textures, hot layers in ember tones and cold ones in blue-on-black.

**Void crystal.** The ore sits in two places: sparse pockets in the ordinary
deepslate of the Overworld (y −60…−16), so the pickaxe is craftable long
before anyone touches bedrock — and pockets in the Abyss layers, where coal,
iron and diamond ore keep them company.
Smelting or blasting the ore gives crystals; a crystal and an iron ingot make
a **Void Alloy Ingot**. Smelting a layer's own stone gives its shard: an
**Ember Shard** from emberstone, a **Frost Shard** from froststone; the
inactive layer's **Cinder Shard** comes only from the pest that wears it.

**The Void Alloy Pickaxe.** 2800 durability, mining speed 14, three alloy
ingots and two sticks to craft. It is the only tool that
takes bedrock and voidshell: hold the attack key on the block and a progress
bar fills over the hotbar — three seconds by default — then the block goes
and, if enabled, drops itself. Only the player who holds the pickaxe digs;
everyone else, and every other tool, gets an unbreakable block.

**The full void set.** The pickaxe is not alone: a void-alloy **sword, axe,
hoe and shovel** and a **helmet, chestplate, leggings and boots** — netherite-
tier stats, repaired with void alloy ingots, eight shaped recipes, and no
enchantment glint anywhere: the set reads by its own textures, not by a
purple shine. The tab lists the dig and the set first, then the materials,
then the Abyss layer by layer from the voidshell gate down to the Abyss
bedrock (which joins the tab for builders), and the lamps and sprouts last.
The **Stalker Shard** sits among the materials as a trophy item (it is also
what the warden wears, and what killing one drops — see below), and the
**Gloom Pearl** beside it is what the deep's gloomling shows — a
creative-tab material, since a tamed creature is nobody's loot.

**Inhabitants.** The Abyss is populated by the mod's **own creatures** — no
vanilla mob lives down there, and the native mod *enforces* it: every tick,
any vanilla mob below y −55 that the mod does not track is removed from the
world (no death, no loot, no sound). Each layer keeps its own company:
**embers** (the ember shard, a fast flicker), drifting crystal **wisps** and
hanging **lanterns** in the lava zone; wisps and lanterns among the cooled
stone; the shy **gloomling** (a gloom pearl) alone in the
world-in-the-depths; wisps, lanterns and the odd **watcher** — a bigger
crystal that closes in and watches the player — in the cool lands; and
**frostmotes** (the frost shard, slow and pale) in the frozen wastes. The
absolute void keeps none. Up to ten per player, saved with the world.

**The pests of the layers.** Every layer but the calm deep sends a hunter of
its own after a player in it — an invisible, silent, weightless vex that the
engine's flying melee drives (through stone, like the ghost it is), wearing
an item display of the layer's shard as its visible body, faced at the player
every tick. They are meant to annoy, not to wall: the **emberling** of the
lava zone and the **cinder** of the inactive one (16 HP, taking shape 18
blocks out, a minute of quiet after each), the **shade** of the cool lands
(18 HP, a 16-block ring) and the **rime** of the frozen wastes (20 HP,
quicker back). Killing one drops its shard as the trophy; climb out of a
layer and its hunt ends at once.

**The warden.** The absolute void below y −780 is the one place that hunts in
earnest. The **warden** is the same machine turned up: 32 HP, taking shape 10
blocks out, back six seconds after it falls, and a body half as big again —
the stalker shard display. Bring the void set.

**The gloomling.** The world-in-the-depths keeps one creature, and it can be
won over: **hold a Void Alloy Ingot** in the hand within five blocks of a
gloomling and it eats from the hand, tick by tick — two seconds of holding
and it trusts the player and is theirs. A fed gloomling becomes a **gloom**:
it follows its player everywhere — through every layer, up through the gate,
into the daylight — and is never let go, not when they walk away and not
across a restart (its fed kind rides the same saved yaw code as every
creature's). The alloy is held, not eaten: the ingot stays in the hand.

**The rest.** English and Russian languages, recipes (smelting, blasting,
alloy, pickaxe, crystal block both ways, bricks, lamp, sprout), loot tables
and an “Underbedrock” creative tab.

---

## Installation

Requires **Horizon 26.3**. Mods are off by default in Horizon: a fresh install
has no mod folders and plays exactly like vanilla.

Take the three prebuilt files from this repository's `release/` folder:

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
uses nothing but Horizon's official native hook points (`hooks: 164`), so it
builds on any Horizon whose hook points are that version or newer — in
practice a recent **nightly** build: the state-block hook the dig verifies
through is newer than the 26.3 stable's hook set.

**The data pack needs a new world.** A world's height is fixed when it loads,
so the pack must be in place before chunks generate: create the world after
dropping the zip in, or accept that chunks generated earlier keep the old
floor with void below it.

In game, `/abyss depth` answers
`This world: y -1008 .. 319 (1328 blocks). Under Bedrock's Abyss is below the bedrock floor.`
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
   Abyss opens: the lava zone glows under the gate, and every layer down
   is colder and darker — each crossing sparkles its veil of end-rod light.
   The creatures are the mod's own; every layer but the calm deep sends its
   pest, and below y −780 the warden hunts — bring the void set. In the
   world-in-the-depths, hold an alloy ingot near the gloomling to win it
   over.
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
`underbedrock_world`: `dimension_type/overworld.json` (min_y −1008, height 1328, ambient light 0.05
so the layers read without torches), the Overworld's `noise_settings` with an
`aquifers` configuration, one density function, five noises and one material
rule that builds every depth band.
Everything a content mod may carry — blocks, items, recipes, loot, languages
and the creative tab — lives in the `.hmod`. All ore and decor placement is
the material rule's job: Horizon logs "placed features cannot run and will be
skipped" for feature-based placement, so the Abyss grows its pockets as noise
windows in the rule itself, and nothing in the `.hmod` needs a biome. And bedrock is unbreakable by design (hardness −1), which is engine
territory: the native mod handles the dig.

**Lava only where it belongs.** Horizon's global fluid picker fills every
cave below y −54 with lava unless the noise settings carry an `aquifers`
object; with one, its lava function decides — ours is a flat zero, and the
density under y −54 is plain solid rock, so no fluid cell exists down there
at all. The only lava in the Abyss is placed by the material rule itself, in
the lava zone alone, and only as **contained basins**: a lake floor of stone
laid under the lava's level, a rim ring of solid at the level itself and the
lava in the middle of the window — a lake with a floor and walls and nothing
to fall over, so no drop of it escapes (the uncontained lakes of an earlier
version were one reason dropped items vanished down there). The inactive
layer keeps only the cooled memory of lava: emberstone veins in the scorched
stone. The Abyss's caves are carved by
the material rule (cheese-and-tunnels noise windows per layer — wide open
islands in the heat, big caverns in the cool, sparse cuts in the frost) out
of that solid, after the fill. The pack also replaces the three Overworld carvers with ones that
never start: a carver tunnel below y −54 would be filled with the engine's
deep lava sea. The surface keeps vanilla's density caves, oceans and rivers
through the aquifer's floodedness noise.

**The dig.** The native mod binds nine hook points: the mod list row, its
settings page and click, the player tick, `/abyss`'s registration and
performance, a level's unload, a death's drops, and the messages its own net
channel carries to a client. Each player tick it reads the engine's own dig state — the block
the attack key is held on and the ticks it has been held for, which the engine
counts even on unbreakable blocks — and, when that block is bedrock or
voidshell and the held item is the void-alloy pickaxe, reports the progress on
the action bar. At the configured dig time the block is removed through
`native_set_block` with the air state id (verified at runtime), with a
named-block `native_edit_blocks` bulk edit as a fallback, and the drop is
given through `native_give_items`. The break is also told to the digging
player's client over the mod's own net channel (`native_net_send_to_player`,
API 40), and the client half plays it there — a slowed deepslate break and a
burst of blue soul-flame sparks (`native_play_local_sound`,
`native_flame_particle`): sounds and particles only ever play in the window's
game, so the channel is the bridge. Abyss bedrock is simply never a dig target.
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
floor gets an `item_display` of the layer's shard or crystal (a wisp, an
ember, a frostmote, the bigger watcher) or a `block_display` showing its
crystal lamp (a lantern), through `native_summon`. Every deep tick the mod
moves each creature along its path (`native_move_entity`) and resends its
pose — scale and spin — every eight ticks (`native_set_display`; the clients
ease between the poses); a creature whose player walks 96 blocks away is let
go (`native_remove_entity`) — except a fed gloomling, which is never let go:
it eases to its player's side instead, at any depth, and the tick's live list
admits its code even above the gate. Every creature wears its kind as a yaw
code — 23° a wisp, 95° an ember, 167° a watcher, 183° a gloomling, 215° a
fed one, 239° a lantern, 311° a frostmote —
kept on by every move, and the engine saves the yaw with the display's NBT
(`Rotation`, exact f32 bits) and hands it back when its chunk loads: a world
reloaded adopts its creatures back as the kinds they were, within a game
session and across a full restart alike, with no memory of the mod's own. The
codes sit on odd values away from the cardinals and the zero a plain
`/summon` writes, and the reading window is tight (±2°), so a foreign
display has to wear a code almost to the degree to pass — an accepted
residual risk, for the API lets a mod write a display's NBT but never read
one back, and the yaw is the only identity channel that survives a restart.
A display under the gate that wears no code — a `/summon` of someone's,
another mod's, or one an older Under Bedrock saved before codes existed — is
not the mod's to drive and is let go as well: the Abyss keeps nothing under
its gate that isn't on the mod's ledger. (An update from a code-less version
therefore clears its old creatures once, on the first deep tick; the
replenish brings fresh ones at once, and nothing else in the world is
touched.)

**The ban, the hunts and the veil.** The same tick reads the engine's mob
list (`native_mobs`, API 37): every mob below y −55 whose id the mod does not
track as one of its hunters is removed (`native_remove_mob` — no death, no
loot), so the Abyss's "no vanilla mobs" is a server-side fact, not a
spawning accident. The hunters are vexes the mod spawns per player per layer
(`native_spawn_mob` with the invisible, silent, no-gravity flags and the
layer's health): the engine's own melee AI does the hunting and the damage,
while the mod moves the shard-display body onto the mob every tick and faces
it at the player. A summon that finds no room (peaceful, an unloaded chunk)
is retried after a pause. A mob the engine no longer lists (killed, gone)
takes its body with it, and its layer stays quiet for a while — 420 ticks
for the shallow pests, 120 for the warden; the death's drops then gain the
layer's shard as a trophy (`native_loot_drops`, API 75) — and only a tracked
hunt's vex counts: an untracked one drops nothing however deep it dies, for
depth alone is not proof of a hunt. A player who climbs out of a layer
dismisses its hunt at once. Below the frozen floor (y −780) nothing lives
that the hunts and the ledger do not know — a display a restart orphaned
there is cleaned up. Feeding needs no hook of its own: the same tick reads
the held item, and an alloy held within five blocks of a gloomling counts a
tick of trust. And the tick watches each player's depth band: a layer
boundary crossed sends a burst of end-rod particles at the boundary's own
height around the player (`native_send_particles`, API 103) — the veil of the
transition, hysteresis-damped so hovering on a line replays nothing. A world
that closes is forgotten whole (`native_level_unload`, API 134): the
creatures, the hunts, the crossing watch and the players' rhythms the mod
kept are dropped, so a reopened world starts clean — and loses nothing by
it, for every creature's kind rides the yaw code the engine saved with its
display.

---

## Building from source

The distributable files need **Python 3.11+ only** — no Rust:

```sh
python tools/pack_release.py
```

which writes `release/`: the `.hmod`, the `.hnmod` and the data pack zip
(`pack.mcmeta` at its root) — the three prebuilt files the repository carries.
Regenerating textures or worldgen data (`make_textures.py`,
`make_content.py`, both plain Python as well — no Pillow, no Rust) is only needed
when editing them.

Rust enters only the checks, all of them runnable without a Horizon install:

```sh
cd native-mod && cargo test                     # the dig, creature, hunt and tame logic, 31 tests
cd ../tools/stubcheck && cargo run -- on && cargo run -- off
                                                # the mod's own sources compiled against
                                                # Horizon's exact signatures, 67 scenarios twice
cd ../.. && python tools/nativecheck.py release/underbedrock_core.hnmod
                                                # the bundle read by the registry's own rules
python tools/check_worldgen.py                  # worldgen JSONs vs Horizon's decoder, plus the
                                                # lava-only-in-the-hot-expanse / no-infinite-band rules
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

Built and checked against Horizon `ea55997` (game 26.3) and horizon-mixins
`ddd9700`; every file format was verified against that revision's sources.
The sources pin that revision on purpose: the 2026-11-03 nightly's
horizon-client `lib.rs` carries `#![recursion_limit = "256"]`, which the mixin
compiler reads as an unexpanded attribute macro and refuses to patch under
(`E_UNSUPPORTED_MACRO`), while every hook point this mod binds exists at the
pin with the same signature. What the mod RUNS on is newer: it declares hook
points v164, so the launcher loads it only on a Horizon at v164 or later -
today, a nightly.

---

## Known limitations

- The bedrock and voidshell dig is a survival/adventure mechanic; in creative
  mode bedrock breaks instantly, as vanilla does.
- While the dig runs, the player hears only the ordinary mining sound the
  engine sends. The break itself does answer — its own sound and blue sparks,
  played on the digging player's client through the mod's net channel — but
  the progress toward it stays silent.
- Breaking bedrock or voidshell costs the pickaxe no durability (no hook point
  damages a held stack); ordinary mining wears it as usual.
- The ambient creatures are display entities the native mod summons and
  drives: Horizon has no custom living entity types for mods (a content mod's
  entity types are thrown-item projectiles only), so wisps, lanterns, embers
  and frostmotes are ambience — they drift, hang and watch, but they neither
  attack nor drop loot. The hunters — the layers' pests and the warden — are
  the exception that proves the rule: they fight and hurt, because under the
  shard display each is a real (invisible) vex the engine itself drives. The
  gloomling, tameable as it is, stays a display: it follows and trusts, but
  there is no living entity to leash. They spawn only near a player in the Abyss,
  not as part of world generation.
- Vanilla's own hollows used to leak into the Abyss (mineshafts, amethyst
  geodes), since structure placement knows no depth. The pack now switches
  the mineshaft and ancient-city structure sets off for this world entirely
  (a structure set cannot be depth-gated), and overrides the amethyst
  geode's placement: vanilla anchors its floor six blocks over the world's
  bottom, which in a world this deep would be y −1002, so the override pins
  the band vanilla gives a normal world (−58..30, absolute) and keeps every
  other vanilla modifier in place (the 1-in-24 rarity, the in-square jitter,
  the biome filter). The surface loses mineshafts and ancient cities with
  them; every other structure keeps spawning as usual.
- In chunks the engine has not generated yet, the sky can show through the
  Abyss's rock (a glimpse of bright "unloaded" above the head). That is chunk
  streaming, not light — it closes as the chunks load; a bigger render
  distance shows less of it.
- Worlds created before the data pack was installed keep their old height in
  already-generated chunks. A world re-generated with a new layer stack needs
  a fresh world (or fresh chunks) to show the new layers.
- Taming the gloomling holds the alloy in the hand: the engine has no API to
  take an item from a player's inventory, so the ingot is never consumed.
- The native mod reads one private engine field (the player's dig state); see
  “How it works” for why, and what happens if it ever moves.
