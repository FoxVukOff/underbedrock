// Under Bedrock's native half: the void-alloy pickaxe digs the bedrock floor
// and the voidshell gate under it open, so the player gets into the Abyss the
// mod's datapack makes below; and the Abyss gets its own creatures, summoned
// and driven by the mod itself where a player is deep enough. This file is both the mod's Cargo library
// (its tests) and the module the bundle adds to the game
// (`crate::game::underbedrock_core`). It holds the mod's logic and state and
// no Horizon in it: the helpers beside it do the Game work and call in here.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "underbedrock_core";
pub const NAME: &str = "Under Bedrock";
pub const VERSION: &str = "0.0.1";
pub const DESCRIPTION: &str = "The void-alloy pickaxe digs bedrock and the voidshell gate under it: hold it on the block and the block goes, into the dark world the mod's datapack makes below, where its own creatures live. /abyss lists the commands.";

/// The pickaxe that digs bedrock, and what it digs: the bedrock floor and the
/// voidshell gate under it. The abyss bedrock at the world's bottom is dug by
/// nothing, the pickaxe included.
pub const PICKAXE: &str = "underbedrock:void_alloy_pickaxe";
/// The ingot a held hand feeds the calm deep's gloomling with.
pub const ALLOY: &str = "underbedrock:void_alloy";
pub const BEDROCK: &str = "minecraft:bedrock";
pub const VOIDSHELL: &str = "underbedrock:voidshell";
pub const ABYSS_BEDROCK: &str = "underbedrock:abyss_bedrock";
pub const AIR: &str = "minecraft:air";

/// The dig's feedback, on the nightly's net bridge (API 40): sounds and
/// particles are the window's to play, so the server's dig tells the digging
/// player's client, and the client half plays a slowed deepslate break (the
/// floor is bedrock) and a burst of soul flames - the void's own blue.
pub const DUG_CHANNEL: &str = "underbedrock_core:dug";
/// The veil's channel: the server's tick names the boundary a player
/// crossed and where they crossed it; the window plays the sparkles.
pub const VEIL_CHANNEL: &str = "underbedrock_core:veil";
pub const DIG_SOUND: &str = "minecraft:block.deepslate.break";
pub const DIG_VOLUME: f32 = 1.5;
pub const DIG_PITCH: f32 = 0.5;
/// Where the sparks sit around the broken block's centre.
pub const DIG_BURST: [[f64; 3]; 9] = [
    [0.0, 0.0, 0.0],
    [0.4, 0.2, 0.0],
    [-0.4, 0.2, 0.0],
    [0.0, 0.2, 0.4],
    [0.0, 0.2, -0.4],
    [0.3, -0.3, 0.3],
    [-0.3, -0.3, 0.3],
    [0.3, -0.3, -0.3],
    [-0.3, -0.3, -0.3],
];

/// How deep a world must go to have the room below its bedrock floor: the
/// mod's own datapack makes `min_y` -1008 with 1328 blocks of height; the
/// first pack made -576 with 896, and a world that carries it stays tall
/// enough - the thresholds accept both.
pub const TALL_MIN_Y: i32 = -576;
pub const TALL_HEIGHT: i32 = 896;

/// The dig times the settings page and `/abyss dig` cycle through, in game
/// ticks (20 a second).
pub const DIG_TIMES: [u32; 5] = [20, 40, 60, 100, 200];
/// The dig time until the player picks one: three seconds.
pub const DEFAULT_DIG_TICKS: u32 = 60;

/// The action bar's progress bar, in characters.
const BAR_WIDTH: usize = 20;

/// `/abyss`'s answer.
pub const HELP: &str = "Under Bedrock: /abyss give (the pickaxe), /abyss depth (how deep this world goes), /abyss dig (the break time), /abyss drop (whether bedrock drops).";

// --- the Abyss's own creatures ---------------------------------------------
// The Abyss is inhabited by the mod's OWN creatures: no vanilla mob is spawned
// down there. Each creature is a display entity, an entity Horizon keeps
// without simulating (docs/native-mods.md, "Entities"): the mod summons it
// once into a dark pocket and drives it from the tick helper itself — an
// `item_display` showing the mod's void crystal item (a wisp, and the bigger
// watcher), or a `block_display` showing its crystal lamp (a lantern that
// hangs in the dark). The engine saves them with their chunks and syncs them
// to the clients; the paths, the posing and the life cycle are the mod's.

/// What a wisp's or watcher's display shows: the mod's own void crystal item.
pub const WISP_ITEM: &str = "underbedrock:void_crystal";
/// What a lantern's display shows: the mod's own crystal lamp block.
pub const LANTERN_BLOCK: &str = "underbedrock:crystal_lamp";
/// What an ember's display shows: the mod's ember shard of the lava zone.
pub const EMBER_ITEM: &str = "underbedrock:ember_shard";
/// What a frostmote's display shows: the mod's frost shard of the cold rim.
pub const FROSTMOTE_ITEM: &str = "underbedrock:frost_shard";
/// The display entity kinds the creatures ride.
pub const ITEM_DISPLAY: &str = "minecraft:item_display";
pub const BLOCK_DISPLAY: &str = "minecraft:block_display";

/// A player below this y is in the Abyss, and gets its company.
pub const SPAWN_BELOW_Y: f64 = -80.0;
/// No vanilla mob lives under the gate: the ban takes out any that wanders in.
pub const BAN_BELOW_Y: f64 = -55.0;
/// The most of the mod's creatures within reach of one player at a time.
pub const CREATURE_CAP: usize = 10;
/// Creatures are looked for this many blocks around the player.
pub const CREATURE_RANGE: f64 = 40.0;
/// A creature whose player gets this far from its anchor is let go.
pub const CREATURE_CULL: f64 = 96.0;
/// How often a player's tick looks for a place to summon one, in ticks.
pub const SPAWN_EVERY: u32 = 60;
/// The summon attempts of one look.
pub const SPAWN_ATTEMPTS: u32 = 4;
/// A creature's path advances this much of a turn per tick, and its pose is
/// sent again every this many ticks (each client eases between the poses).
pub const PHASE_STEP: f32 = 0.05;
pub const POSE_EVERY: u32 = 8;
/// A watcher closes on a player that comes within this many blocks, and
/// keeps this many.
pub const WATCHER_NEAR: f64 = 16.0;
pub const WATCHER_KEEP: f64 = 4.0;

/// The gloomling's pearl, and the layer bands of the Abyss of 2026-11-03:
/// the lava zone under the gate down to the void the warden hunts in.
pub const GLOOM_ITEM: &str = "underbedrock:gloom_pearl";
/// How far off its player a fed gloomling hangs once it has caught up.
pub const GLOOM_KEEP: f32 = 2.5;
/// The absolute-void warden's shard, and the body its hunt wears.
pub const STALKER_ITEM: &str = "underbedrock:stalker_shard";
pub const LAVA_BOT: f64 = -220.0;
pub const INACTIVE_BOT: f64 = -360.0;
pub const DEEPS_BOT: f64 = -500.0;
pub const COOL_BOT: f64 = -640.0;
pub const FROZEN_BOT: f64 = -780.0;
pub const VOID_BOT: f64 = -960.0;
/// The ticks of a held void alloy near a gloomling that make it trust the
/// hand, and the y of every layer boundary a transition veil plays at.
pub const FEED_TICKS: u32 = 40;
pub const VEIL_BOUNDS: [i32; 6] = [-79, -221, -361, -501, -641, -781];
/// How close to the held alloy a gloomling has to be to eat from it.
pub const GLOOMLING_REACH: f64 = 5.0;
/// The crossing has to be this far past a boundary before its veil plays:
/// a player hovering on the line gets no burst every tick.
pub const VEIL_HYSTERESIS: f64 = 2.0;
/// The veil's sparkles, played on the crossing player's client: a ring of
/// blue soul flames, this many of them, this wide, a lift over the
/// boundary's own y, around the x and z the server's message names.
pub const VEIL_LIFT: f64 = 1.0;
pub const VEIL_RING: f64 = 2.5;
pub const VEIL_SPARKS: usize = 12;

/// How close a display's yaw has to sit to a kind's code to read as that
/// kind. The round-trip is exact - the move keeps the code as it is, the
/// engine saves it as the same f32 bits (`Rotation` in the display's NBT) and
/// hands those bits back on load - so the window is tight, and a foreign
/// display has to wear a code almost to the degree to be taken for one of
/// the mod's.
pub const YAW_TOL: f32 = 2.0;

/// The kinds of creature the Abyss gets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Creature {
    /// A small crystal light that circles and bobs where it was summoned.
    Wisp,
    /// The mod's crystal lamp, hanging in the dark with a slow sway.
    Lantern,
    /// A bigger crystal that closes on a player who comes near, keeps its
    /// distance, and watches them.
    Watcher,
    /// An ember of the lava zone: a fast flicker of the ember shard.
    Ember,
    /// A frostmote of the cold rim: the frost shard, drifting slow.
    Frostmote,
    /// A gloomling of the world in the deep: the calm layer's one creature,
    /// a pearl that drifts shy and can be fed.
    Gloomling,
    /// A gloomling fed from the hand: it follows its player anywhere.
    Gloom,
}

impl Creature {
    /// The display entity kind that carries it.
    pub fn display(self) -> &'static str {
        match self {
            Self::Lantern => BLOCK_DISPLAY,
            Self::Wisp
            | Self::Watcher
            | Self::Ember
            | Self::Frostmote
            | Self::Gloomling
            | Self::Gloom => ITEM_DISPLAY,
        }
    }
    /// The item or block its display shows: the mod's own.
    pub fn shows(self) -> &'static str {
        match self {
            Self::Lantern => LANTERN_BLOCK,
            Self::Ember => EMBER_ITEM,
            Self::Frostmote => FROSTMOTE_ITEM,
            Self::Wisp | Self::Watcher => WISP_ITEM,
            Self::Gloomling | Self::Gloom => GLOOM_ITEM,
        }
    }
    /// The scale its display is posed at.
    pub fn scale(self) -> f32 {
        match self {
            Self::Wisp => 0.9,
            Self::Lantern => 0.55,
            Self::Watcher => 1.5,
            Self::Ember => 0.8,
            Self::Frostmote => 0.85,
            Self::Gloomling => 0.7,
            Self::Gloom => 0.8,
        }
    }
    /// Whether a display kind is one the mod's creatures ride: the item
    /// display (a wisp by default) and the block display (the lantern).
    /// Which creature a display is says its yaw code, not its kind.
    pub fn of(display_kind: &str) -> Option<Self> {
        match display_kind {
            BLOCK_DISPLAY => Some(Self::Lantern),
            ITEM_DISPLAY => Some(Self::Wisp),
            _ => None,
        }
    }
    /// The yaw this kind wears. The engine saves a display's yaw with its
    /// NBT (`Rotation`) and hands it back when the chunk loads, and the
    /// visible spin of a creature is its transformation, not its yaw - so
    /// the yaw is free, and the mod writes its kinds into it: a memory of
    /// what every creature was, kept by the engine itself, good across a
    /// world reload and a full game restart alike. The codes sit 72 degrees
    /// apart on odd values, away from the cardinal points and from the zero
    /// a `/summon` without a rotation writes: a foreign display is much
    /// likelier to wear those than a code.
    pub fn yaw_code(self) -> f32 {
        match self {
            Self::Wisp => 23.0,
            Self::Ember => 95.0,
            Self::Watcher => 167.0,
            Self::Lantern => 239.0,
            Self::Frostmote => 311.0,
            Self::Gloomling => 183.0,
            Self::Gloom => 215.0,
        }
    }
}

/// The creature a display of `display_kind` turned to `yaw` is: the kind
/// whose code the yaw reads as, on a display kind that carries it. A yaw
/// that reads as no code is no creature of the mod's - a display the mod
/// never summoned (or one an older version of it left behind) is not the
/// mod's to drive.
pub fn kind_from_yaw(display_kind: &str, yaw: f32) -> Option<Creature> {
    use Creature::{Ember, Frostmote, Gloom, Gloomling, Lantern, Watcher, Wisp};
    Creature::of(display_kind)?;
    let yaw = yaw.rem_euclid(360.0);
    let near = |code: f32| {
        let d = (yaw - code).abs().rem_euclid(360.0);
        d.min(360.0 - d) <= YAW_TOL
    };
    [Wisp, Ember, Watcher, Lantern, Frostmote, Gloomling, Gloom]
        .into_iter()
        .find(|kind| kind.display() == display_kind && near(kind.yaw_code()))
}

/// What the pickaxe digs: the bedrock floor, or the voidshell gate under it.
/// The abyss bedrock, and everything else, is left to the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dig {
    Bedrock,
    Voidshell,
}

/// The block the dig in place is on, when it is one the pickaxe digs.
pub fn dig_target(block: &str) -> Option<Dig> {
    match block {
        BEDROCK => Some(Dig::Bedrock),
        VOIDSHELL => Some(Dig::Voidshell),
        _ => None,
    }
}

/// The action bar's name for what is being dug.
pub fn dig_name(dig: Dig) -> &'static str {
    match dig {
        Dig::Bedrock => "Bedrock",
        Dig::Voidshell => "Voidshell",
    }
}

/// Whether what was dug drops: bedrock as its setting says, voidshell always
/// (it is a block of the mod, and the dig is the only way to get it).
pub fn dig_drops(dig: Dig, get: impl FnOnce() -> Option<String>) -> bool {
    match dig {
        Dig::Bedrock => drops(get),
        Dig::Voidshell => true,
    }
}

/// The item a dug block drops.
pub fn dig_drop(dig: Dig) -> &'static str {
    match dig {
        Dig::Bedrock => BEDROCK,
        Dig::Voidshell => VOIDSHELL,
    }
}

/// The stored dig time: one of the mod's own, else the default.
pub fn read_dig_ticks(stored: Option<String>) -> u32 {
    stored
        .and_then(|stored| stored.trim().parse::<u32>().ok())
        .filter(|ticks| DIG_TIMES.contains(ticks))
        .unwrap_or(DEFAULT_DIG_TICKS)
}

/// The dig time after `ticks` in the cycle; an unknown one starts it.
pub fn next_dig_ticks(ticks: u32) -> u32 {
    match DIG_TIMES.iter().position(|found| *found == ticks) {
        Some(at) => DIG_TIMES[(at + 1) % DIG_TIMES.len()],
        None => DIG_TIMES[0],
    }
}

/// The stored drop flag: bedrock drops unless its file says `false`.
pub fn read_drop(stored: Option<String>) -> bool {
    stored.is_none_or(|stored| stored.trim() != "false")
}

/// The settings page's two buttons.
pub fn dig_label(ticks: u32) -> String {
    format!(
        "Break time: {:.1} s (bedrock, voidshell)",
        ticks as f32 / 20.0
    )
}

pub fn drop_label(drops: bool) -> String {
    format!("Bedrock drops: {}", if drops { "ON" } else { "OFF" })
}

/// How far a dig the engine counted `ticks` ticks of is through `dig_ticks`
/// (0.0 to 1.0; a dig time of 0 breaks at once).
pub fn progress(ticks: u32, dig_ticks: u32) -> f32 {
    if dig_ticks == 0 {
        return 1.0;
    }
    (ticks as f32 / dig_ticks as f32).clamp(0.0, 1.0)
}

/// The action bar's line: `Bedrock [#####---------------] 25%`.
pub fn bar(name: &str, progress: f32) -> String {
    let progress = progress.clamp(0.0, 1.0);
    let filled = (progress * BAR_WIDTH as f32).round() as usize;
    format!(
        "{name} [{}{}] {}%",
        "#".repeat(filled),
        "-".repeat(BAR_WIDTH - filled),
        (progress * 100.0).round() as u32
    )
}

/// Whether the world in place is the tall one the datapack makes: a world
/// that stops at the vanilla floor has no room below its bedrock.
pub fn tall(min_y: i32, height: i32) -> bool {
    min_y <= TALL_MIN_Y && height >= TALL_HEIGHT
}

/// The line that says how deep the world in place goes, and whether the
/// datapack that makes the room below the bedrock is there.
pub fn depth_line(min_y: i32, height: i32) -> String {
    let shape = format!(
        "This world: y {min_y} .. {} ({height} blocks).",
        min_y + height - 1
    );
    if tall(min_y, height) {
        format!("{shape} Under Bedrock's Abyss is below the bedrock floor.")
    } else {
        format!(
            "{shape} NOT tall: install the datapack underbedrock_world (the world's datapacks, or the resources' data/minecraft/datapacks) and make a new world."
        )
    }
}

thread_local! {
    /// The dig time and the drop flag: read once from their files, then kept,
    /// so a player's tick doesn't read the disk.
    static DIG: Cell<Option<u32>> = const { Cell::new(None) };
    static DROPS: Cell<Option<bool>> = const { Cell::new(None) };
    /// The world shape already looked at: each world is reported on once.
    static SHAPE: Cell<Option<(i32, i32)>> = const { Cell::new(None) };
    /// The players a progress line is on the action bar for.
    static BARS: RefCell<HashSet<u128>> = RefCell::new(HashSet::new());
}

/// The dig time, read once through `get` (the `dig_ticks` file).
pub fn dig_ticks(get: impl FnOnce() -> Option<String>) -> u32 {
    DIG.with(|cell| {
        let found = cell.get().unwrap_or_else(|| read_dig_ticks(get()));
        cell.set(Some(found));
        found
    })
}

/// Keeps a new dig time (the settings page or `/abyss dig` chose it).
pub fn set_dig_ticks(ticks: u32) {
    DIG.with(|cell| cell.set(Some(ticks)));
}

/// Whether bedrock drops, read once through `get` (the `drop_bedrock` file).
pub fn drops(get: impl FnOnce() -> Option<String>) -> bool {
    DROPS.with(|cell| {
        let found = cell.get().unwrap_or_else(|| read_drop(get()));
        cell.set(Some(found));
        found
    })
}

/// Keeps a new drop flag.
pub fn set_drops(drops: bool) {
    DROPS.with(|cell| cell.set(Some(drops)));
}

/// Whether the world's shape `shape` was looked at already; a shape is looked
/// at once, so a world the mod hasn't seen is the one it reports on.
pub fn shape_seen(shape: (i32, i32)) -> bool {
    SHAPE.with(|cell| {
        let seen = cell.get() == Some(shape);
        cell.set(Some(shape));
        seen
    })
}

/// Keeps that a progress line is on `uuid`'s action bar (`shown`), and says
/// whether one was there: the caller clears it once, with an empty line.
pub fn bar_shown(uuid: u128, shown: bool) -> bool {
    BARS.with(|bars| {
        let mut bars = bars.borrow_mut();
        let was = bars.contains(&uuid);
        if shown {
            bars.insert(uuid);
        } else {
            bars.remove(&uuid);
        }
        was
    })
}

thread_local! {
    /// Each player's tick count, for the spawn look's rhythm.
    static TICKS: RefCell<HashMap<u128, u32>> = RefCell::new(HashMap::new());
}

/// Counts one of `uuid`'s ticks and says whether this one looks for a place
/// to spawn: every [`SPAWN_EVERY`] ticks. A player the mod has counted 4096
/// times without a look starts over, so a stale entry never lives long.
pub fn spawn_due(uuid: u128) -> bool {
    TICKS.with(|ticks| {
        let mut ticks = ticks.borrow_mut();
        let tick = ticks.get(&uuid).copied().unwrap_or(0).wrapping_add(1);
        ticks.insert(uuid, if tick > 4096 { 0 } else { tick });
        tick % SPAWN_EVERY == 0
    })
}

/// One mixing step, so a seed and an attempt give a spread of offsets.
fn mix(seed: u64) -> u64 {
    let mut h = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h = (h ^ (h >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    h ^ (h >> 31)
}

/// Where the `attempt`'s spawn is tried, relative to the player: a ring 8 to 24
/// blocks out, a little below or above the player's feet.
pub fn candidate_offset(seed: u64, attempt: u32) -> [i32; 3] {
    let h = mix(seed.wrapping_add(attempt as u64 * 0x1F12_3BB5));
    let ring = |v: u64, span: i32, bias: i32| -> i32 { (v % span as u64) as i32 + bias };
    let mut dx = ring(h, 33, -16) * 3 / 2;
    let mut dz = ring(h >> 16, 33, -16) * 3 / 2;
    let dy = ring(h >> 32, 13, -8);
    // Keep the ring: nothing spawns in the player's own pocket.
    if dx.abs() < 8 && dz.abs() < 8 {
        dx += 8;
        dz -= 8;
    }
    [dx, dy, dz]
}

/// The creature the `seed`'s summon is, for the layer at `y`: every layer of
/// the Abyss keeps its own company - embers and lamps in the lava zone, wisps
/// and lamps in the inactive one, the gloomling alone in the calm deep, the
/// classic company in the cool lands, frostmotes in the frozen wastes - and
/// the absolute void below [`FROZEN_BOT`] keeps none at all.
pub fn pick_creature(seed: u64, y: f64) -> Option<Creature> {
    if y < FROZEN_BOT {
        return None;
    }
    let roll = mix(seed ^ 0x0AB5_5EED) % 10;
    Some(if y < COOL_BOT {
        Creature::Frostmote
    } else if y < DEEPS_BOT {
        match roll {
            0..=4 => Creature::Wisp,
            5..=7 => Creature::Watcher,
            _ => Creature::Lantern,
        }
    } else if y < INACTIVE_BOT {
        Creature::Gloomling
    } else if y < LAVA_BOT {
        match roll {
            0..=5 => Creature::Wisp,
            _ => Creature::Lantern,
        }
    } else {
        match roll {
            0..=5 => Creature::Ember,
            6..=7 => Creature::Lantern,
            _ => Creature::Wisp,
        }
    })
}

pub fn is_ours(display_kind: &str, pos: [f64; 3]) -> bool {
    Creature::of(display_kind).is_some() && pos[1] < SPAWN_BELOW_Y && pos[1] > FROZEN_BOT
}

/// Whether a display wears the fed gloomling's yaw code: one that does is
/// the mod's to drive at ANY depth - a tamed creature follows its player up
/// through the gate and into the daylight, where the ambient bands end, and
/// a restart that emptied the ledger adopts it back by its code.
pub fn is_tamed_code(display_kind: &str, yaw: f32) -> bool {
    display_kind == ITEM_DISPLAY && (yaw - Creature::Gloom.yaw_code()).abs() <= YAW_TOL
}

/// Whether a creature is near enough to the player to count against the cap.
pub fn counts_towards_cap(pos: [f64; 3], player: [f64; 3]) -> bool {
    (pos[0] - player[0]).abs() <= CREATURE_RANGE && (pos[2] - player[2]).abs() <= CREATURE_RANGE
}

/// One look for a place to summon: `near` creatures of the mod's are within
/// reach already, and `air` says whether a cell is open (the caller reads its
/// world). The first attempt with air at the feet and the head, and a floor
/// under the feet, wins: its creature and spot. None when the cap is full or
/// no attempt finds a pocket.
pub fn spawn_plan(
    seed: u64,
    at: [f64; 3],
    near: usize,
    air: impl Fn([i32; 3]) -> bool,
) -> Option<(Creature, [f64; 3])> {
    if near >= CREATURE_CAP {
        return None;
    }
    for attempt in 0..SPAWN_ATTEMPTS {
        let [dx, dy, dz] = candidate_offset(seed, attempt);
        let spot = [
            at[0] + dx as f64,
            (at[1] + dy as f64).floor(),
            at[2] + dz as f64,
        ];
        let feet = [spot[0] as i32, spot[1] as i32, spot[2] as i32];
        // A pocket: air where the mob would stand, air over its head, and a
        // floor under it that is not the world's own bottom air.
        if !(air(feet)
            && air([feet[0], feet[1] + 1, feet[2]])
            && !air([feet[0], feet[1] - 1, feet[2]]))
        {
            continue;
        }
        let creature = pick_creature(seed.wrapping_add(attempt as u64), at[1])?;
        return Some((creature, spot));
    }
    None
}

/// Where a creature is at `phase` of its path around `anchor`: a wisp circles
/// and bobs, a lantern hangs with a slow sway, a watcher circles tighter and
/// faster. All three stay in the air their pocket gave them.
pub fn drift(kind: Creature, anchor: [f64; 3], phase: f32) -> [f64; 3] {
    let (radius, spin, bob, lift) = match kind {
        Creature::Wisp => (2.5f32, 0.7, 0.6, 1.0),
        Creature::Lantern => (0.35, 0.25, 0.25, 1.8),
        Creature::Watcher => (1.4, 1.1, 0.4, 1.2),
        Creature::Ember => (2.2, 1.3, 0.9, 1.0),
        Creature::Frostmote => (1.8, 0.35, 0.5, 1.4),
        // The deep's calm: a gloomling drifts little and slow, and a fed one
        // barely leaves its player's shoulder.
        Creature::Gloomling => (1.2, 0.2, 0.3, 0.8),
        Creature::Gloom => (0.5, 0.3, 0.2, 1.2),
    };
    let turn = phase * spin;
    [
        anchor[0] + (radius * turn.sin()) as f64,
        anchor[1] + lift + (bob * (turn * 1.7).sin()) as f64,
        anchor[2] + (radius * turn.cos()) as f64,
    ]
}

/// Where a watcher's anchor goes this tick: a player within [`WATCHER_NEAR`]
/// blocks is closed on, a twentieth of the remaining way each tick, until the
/// watcher hangs [`WATCHER_KEEP`] and a half blocks off them; a player
/// further away (or right on top of it) leaves it where it is.
pub fn watcher_anchor(anchor: [f64; 3], player: [f64; 3]) -> [f64; 3] {
    let eye = [player[0], player[1] + 1.0, player[2]];
    let away = [anchor[0] - eye[0], anchor[1] - eye[1], anchor[2] - eye[2]];
    let dist = (away[0] * away[0] + away[1] * away[1] + away[2] * away[2]).sqrt();
    if !(1.0e-6..=WATCHER_NEAR).contains(&dist) {
        return anchor;
    }
    let keep = (WATCHER_KEEP + 0.5) as f32;
    let want = [
        eye[0] + away[0] / dist * keep as f64,
        eye[1] + away[1] / dist * keep as f64,
        eye[2] + away[2] / dist * keep as f64,
    ];
    [
        anchor[0] + (want[0] - anchor[0]) * 0.05,
        anchor[1] + (want[1] - anchor[1]) * 0.05,
        anchor[2] + (want[2] - anchor[2]) * 0.05,
    ]
}

/// Where a fed gloomling's anchor goes this tick: it eases in to
/// [`GLOOM_KEEP`] blocks of its player's eye, a tenth of the remaining way
/// each tick, however far they are - a tamed creature follows through the
/// whole Abyss, and up out of it.
pub fn follow_anchor(anchor: [f64; 3], player: [f64; 3]) -> [f64; 3] {
    let eye = [player[0], player[1] + 1.0, player[2]];
    let away = [anchor[0] - eye[0], anchor[1] - eye[1], anchor[2] - eye[2]];
    let dist = (away[0] * away[0] + away[1] * away[1] + away[2] * away[2]).sqrt();
    if dist <= GLOOM_KEEP as f64 || dist < 1.0e-6 {
        return anchor;
    }
    let keep = GLOOM_KEEP as f64;
    let want = [
        eye[0] + away[0] / dist * keep,
        eye[1] + away[1] / dist * keep,
        eye[2] + away[2] / dist * keep,
    ];
    [
        anchor[0] + (want[0] - anchor[0]) * 0.1,
        anchor[1] + (want[1] - anchor[1]) * 0.1,
        anchor[2] + (want[2] - anchor[2]) * 0.1,
    ]
}

/// The yaw that looks from `from` at `to`, in the engine's degrees: 0 south,
/// -90 east.
pub fn face_yaw(from: [f64; 3], to: [f64; 3]) -> f32 {
    (-(to[0] - from[0]) as f32)
        .atan2((to[2] - from[2]) as f32)
        .to_degrees()
}

/// A display's rotation of `angle` radians about the y axis, as the engine's
/// quaternion wants it (`[x, y, z, w]`).
pub fn spin_y(angle: f32) -> [f32; 4] {
    [0.0, (angle * 0.5).sin(), 0.0, (angle * 0.5).cos()]
}

/// One creature the mod drives: its display's engine id, what it is, where it
/// circles, how far round its path it is, whose company it keeps (it is let
/// go when that player walks away), and the ticks until its pose is sent
/// again.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CreatureState {
    pub id: u32,
    pub kind: Creature,
    pub anchor: [f64; 3],
    pub phase: f32,
    pub owner: u128,
    pub pose_in: u32,
    /// The ticks a gloomling has been fed from a held alloy so far.
    pub feed: u32,
}

/// What one step of the creatures asks the engine to do.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreatureStep {
    /// `native_remove_entity`: let go, their player walked away.
    pub cull: Vec<u32>,
    /// `native_remove_entity`: a display under the gate that wears no yaw
    /// code of the mod's - not a creature of the mod's, and the Abyss keeps
    /// nothing under its gate that isn't on the mod's ledger.
    pub purge: Vec<u32>,
    /// `native_move_entity`: the id, where to, facing which yaw.
    pub moves: Vec<(u32, [f64; 3], f32)>,
    /// `native_set_display`: the id, the kind (for its scale) and the spin.
    pub poses: Vec<(u32, Creature, f32)>,
}

thread_local! {
    /// The creatures the mod drives, in the world in place.
    static CREATURES: RefCell<Vec<CreatureState>> = const { RefCell::new(Vec::new()) };
}

/// The kind the mod drives `id` as, if it drives it at all.
pub fn creature_kind(id: u32) -> Option<Creature> {
    CREATURES.with(|creatures| {
        creatures
            .borrow()
            .iter()
            .find(|creature| creature.id == id)
            .map(|creature| creature.kind)
    })
}

/// Feeds the gloomling `id` from `owner`'s held alloy by one tick: at
/// [`FEED_TICKS`] it trusts the hand and becomes theirs - the kind flips to
/// the fed one (and so does the yaw code the next move writes), the owner is
/// who fed it. Says whether this tick was the one.
pub fn feed_creature(id: u32, owner: u128) -> bool {
    CREATURES.with(|creatures| {
        let mut creatures = creatures.borrow_mut();
        let Some(creature) = creatures.iter_mut().find(|c| c.id == id) else {
            return false;
        };
        if creature.kind != Creature::Gloomling {
            return false;
        }
        creature.feed += 1;
        if creature.feed < FEED_TICKS {
            return false;
        }
        creature.kind = Creature::Gloom;
        creature.owner = owner;
        creature.feed = 0;
        true
    })
}

/// The ids of the fed gloomlings: the tick keeps them in its live list at
/// any depth, even up in the daylight behind their player.
pub fn tamed_ids() -> Vec<u32> {
    CREATURES.with(|creatures| {
        creatures
            .borrow()
            .iter()
            .filter(|c| c.kind == Creature::Gloom)
            .map(|c| c.id)
            .collect()
    })
}

/// The ids of every creature the mod drives this tick: nothing the tick
/// itself placed is a stray to clean up.
pub fn driven_ids() -> Vec<u32> {
    CREATURES.with(|creatures| creatures.borrow().iter().map(|c| c.id).collect())
}

/// Empties the creature list: a world closed, or the harness runs again.
pub fn clear_creatures() {
    CREATURES.with(|creatures| creatures.borrow_mut().clear());
}

/// How many creatures the mod drives.
pub fn creatures_len() -> usize {
    CREATURES.with(|creatures| creatures.borrow().len())
}

/// Adds a creature the caller just summoned at `anchor`, for `owner`.
pub fn add_creature(id: u32, kind: Creature, anchor: [f64; 3], owner: u128) {
    CREATURES.with(|creatures| {
        let mut creatures = creatures.borrow_mut();
        creatures.retain(|creature| creature.id != id);
        creatures.push(CreatureState {
            id,
            kind,
            anchor,
            phase: 0.0,
            owner,
            pose_in: 0,
            feed: 0,
        });
    });
}

/// One tick of the creatures, given the displays the engine has in the Abyss
/// (`live`: the id, the display kind, the yaw and the position of each) and
/// the player whose tick this is. The engine's list is the truth: a state
/// whose display is gone drops off (a world reloaded, a `/kill`). A display
/// the list doesn't know is read by its yaw: one wearing a kind's code is a
/// creature of the mod's, saved with an earlier session - the engine keeps
/// the code in the display's NBT and hands it back on load, so a reopened
/// world brings its kinds back through a reload and a full game restart
/// alike - and one wearing no code was never the mod's, and is purged: the
/// Abyss keeps nothing under its gate that isn't on the mod's ledger. The
/// rest take a step of their paths
/// — watchers easing toward the player — and one whose own player is beyond
/// [`CREATURE_CULL`] of its anchor is let go. Every move keeps a creature's
/// code on it, so the memory the engine saves is always current. Each
/// player's tick steps every creature once, so a LAN game's creatures run a
/// little faster the more players are down there; one player's is exact.
pub fn step_creatures(
    player: [f64; 3],
    owner: u128,
    live: &[(u32, &str, f32, [f64; 3])],
) -> CreatureStep {
    let mut step = CreatureStep::default();
    CREATURES.with(|cell| {
        let mut creatures = cell.borrow_mut();
        creatures.retain(|creature| live.iter().any(|(id, _, _, _)| *id == creature.id));
        for (id, display_kind, yaw, pos) in live {
            if creatures.iter().any(|creature| creature.id == *id) {
                continue;
            }
            match kind_from_yaw(display_kind, *yaw) {
                Some(kind) => creatures.push(CreatureState {
                    id: *id,
                    kind,
                    anchor: *pos,
                    phase: *id as f32 * 0.7,
                    owner,
                    pose_in: 0,
                    feed: 0,
                }),
                None => step.purge.push(*id),
            }
        }
        creatures.retain_mut(|creature| {
            if creature.kind == Creature::Watcher {
                creature.anchor = watcher_anchor(creature.anchor, player);
            }
            if creature.kind == Creature::Gloom {
                // A fed gloomling is let go by nothing: it keeps its player,
                // easing in to a pearl's length of them while they are the
                // one whose tick this is, and holding where it waits when
                // they are not down here.
                if creature.owner == owner {
                    creature.anchor = follow_anchor(creature.anchor, player);
                }
            } else if creature.owner == owner {
                let away = [
                    player[0] - creature.anchor[0],
                    player[2] - creature.anchor[2],
                ];
                if (away[0] * away[0] + away[1] * away[1]).sqrt() > CREATURE_CULL {
                    step.cull.push(creature.id);
                    return false;
                }
            }
            creature.phase += PHASE_STEP;
            let pos = drift(creature.kind, creature.anchor, creature.phase);
            step.moves
                .push((creature.id, pos, creature.kind.yaw_code()));
            if creature.pose_in == 0 {
                step.poses
                    .push((creature.id, creature.kind, creature.phase));
                creature.pose_in = POSE_EVERY;
            }
            creature.pose_in -= 1;
            true
        });
    });
    step
}

// --- the pests of the layers, and the warden of the void --------------------
// Every layer of the Abyss but the calm deep keeps one hunter of its own: a
// vex (API 37) - the engine's flying melee that walks through stone - spawned
// invisible, silent and weightless, wearing an item display of the layer's
// shard as its visible body, moved onto the mob every tick and faced at the
// player. The engine's melee does the damage; the mod only shows, tracks and
// cleans up. The layer pests are annoyances - a few swings end them, they
// take shape far off and come back slowly. The warden of the absolute void
// is the same machine turned up: it dislikes the player's presence, takes
// shape close, comes back quick and takes some ending. Above a layer its
// pair is dismissed at once; when a hunter falls, its layer is quiet for a
// while, then sends another.

/// The mob every hunter rides.
pub const HUNT_KIND: &str = "minecraft:vex";
/// How far over its mob a body hangs.
pub const HUNT_LIFT: f64 = 0.45;

/// One layer's hunter: the band it hunts in, the body it wears and drops,
/// and how annoying it is - the health it takes, the ring it takes shape on,
/// the quiet after it falls, the pause before a summon that found no room
/// (peaceful, an unloaded chunk) is tried again, the body's scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PestCfg {
    pub name: &'static str,
    pub top: f64,
    pub bot: f64,
    pub body: &'static str,
    pub health: f32,
    pub ring: f64,
    pub quiet: u32,
    pub retry: u32,
    pub scale: f32,
}

/// The five hunters, shallow to deep. The calm deep keeps none - its one
/// creature is the gloomling, which only eats.
pub const PESTS: [PestCfg; 5] = [
    PestCfg {
        name: "emberling",
        top: -73.0,
        bot: LAVA_BOT,
        body: "underbedrock:ember_shard",
        health: 16.0,
        ring: 18.0,
        quiet: 420,
        retry: 40,
        scale: 0.9,
    },
    PestCfg {
        name: "cinder",
        top: -221.0,
        bot: INACTIVE_BOT,
        body: "underbedrock:cinder_shard",
        health: 16.0,
        ring: 18.0,
        quiet: 420,
        retry: 40,
        scale: 0.9,
    },
    PestCfg {
        name: "shade",
        top: -501.0,
        bot: COOL_BOT,
        body: "underbedrock:void_crystal",
        health: 18.0,
        ring: 16.0,
        quiet: 360,
        retry: 40,
        scale: 1.0,
    },
    PestCfg {
        name: "rime",
        top: -641.0,
        bot: FROZEN_BOT,
        body: "underbedrock:frost_shard",
        health: 20.0,
        ring: 16.0,
        quiet: 320,
        retry: 40,
        scale: 1.0,
    },
    // The warden: closer, quicker back, twice the health - the one who
    // dislikes the player's presence in the absolute void.
    PestCfg {
        name: "warden",
        top: -781.0,
        bot: VOID_BOT,
        body: STALKER_ITEM,
        health: 32.0,
        ring: 10.0,
        quiet: 120,
        retry: 20,
        scale: 1.5,
    },
];

/// One player's hunt in one layer: the mob's and its body's engine ids, and
/// the ticks of quiet left before the layer sends another.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pest {
    pub mob: Option<u32>,
    pub body: Option<u32>,
    pub quiet: u32,
}

/// What one tick of a hunt asks the engine to do.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PestOrder {
    /// `native_remove_mob`: the hunt is over.
    pub dismiss_mob: Option<u32>,
    /// `native_remove_entity`: a body to let go.
    pub dismiss_body: Option<u32>,
    /// `native_spawn_mob`: send the hunter in.
    pub summon: bool,
    /// `native_summon`: the mob hunts but shows no body (a summon that
    /// failed) - make one at the mob.
    pub summon_body: bool,
    /// `native_move_entity`: where the body goes (over the mob) and the yaw
    /// it faces (the player).
    pub follow: Option<([f64; 3], f32)>,
}

/// Where a hunter takes shape: on its ring around the player, at their
/// depth (it flies, and walks through stone).
pub fn pest_spot(cfg: &PestCfg, seed: u64, at: [f64; 3]) -> [f64; 3] {
    let turn = (mix(seed) % 36_000) as f64 * std::f64::consts::TAU / 36_000.0;
    [
        at[0] + cfg.ring * turn.cos(),
        at[1],
        at[2] + cfg.ring * turn.sin(),
    ]
}

/// One tick of a player's hunt in `cfg`'s layer: `in_zone` is whether they
/// are in it, `mob_at` where the engine has their vex this tick (None once
/// it is gone - killed, or a ban of a world the mod lost track of). The
/// order is what the engine must do; the state that comes back is what the
/// caller keeps, with the ids a successful summon hands out written over it.
pub fn pest_tick(
    cfg: &PestCfg,
    pest: Pest,
    in_zone: bool,
    mob_at: Option<[f64; 3]>,
    player: [f64; 3],
) -> (Pest, PestOrder) {
    let mut order = PestOrder::default();
    if !in_zone {
        // Out of the layer: the hunt is over, at once and quietly.
        order.dismiss_mob = pest.mob;
        order.dismiss_body = pest.body;
        return (Pest::default(), order);
    }
    let mut pest = Pest {
        quiet: pest.quiet.saturating_sub(1),
        ..pest
    };
    match (pest.mob, mob_at) {
        (Some(_), Some(pos)) => {
            let body_at = [pos[0], pos[1] + HUNT_LIFT, pos[2]];
            if pest.body.is_some() {
                let eye = [player[0], player[1] + 1.0, player[2]];
                order.follow = Some((body_at, face_yaw(body_at, eye)));
            } else {
                order.summon_body = true;
            }
        }
        (Some(_), None) => {
            // It fell (or went): the body follows, and the layer is quiet.
            order.dismiss_body = pest.body;
            pest = Pest {
                mob: None,
                body: None,
                quiet: cfg.quiet,
            };
        }
        (None, _) => {
            if pest.quiet == 0 {
                order.summon = true;
                pest.quiet = cfg.retry;
            }
        }
    }
    (pest, order)
}

thread_local! {
    /// Each player's five hunts, in the world in place.
    static HUNTS: RefCell<HashMap<u128, [Pest; 5]>> = RefCell::new(HashMap::new());
}

/// The hunts the mod keeps for `uuid` (all default when it keeps none).
pub fn get_pests(uuid: u128) -> [Pest; 5] {
    HUNTS.with(|hunts| hunts.borrow().get(&uuid).copied().unwrap_or_default())
}

/// Keeps `uuid`'s hunts; all-default clears the entry, so a finished hunt
/// leaves nothing behind.
pub fn set_pests(uuid: u128, pests: [Pest; 5]) {
    HUNTS.with(|hunts| {
        let mut hunts = hunts.borrow_mut();
        if pests == [Pest::default(); 5] {
            hunts.remove(&uuid);
        } else {
            hunts.insert(uuid, pests);
        }
    });
}

/// Whether mob `id` is a hunter the mod tracks (any player's, any layer's):
/// the ban under the gate spares exactly these.
pub fn is_tracked_mob(id: u32) -> bool {
    HUNTS.with(|hunts| {
        hunts
            .borrow()
            .values()
            .any(|pests| pests.iter().any(|pest| pest.mob == Some(id)))
    })
}

/// The body displays the hunts wear this tick: the tick keeps them out of
/// the creature list and out of the stray-cleanup below the frozen floor.
pub fn pest_bodies() -> Vec<u32> {
    HUNTS.with(|hunts| {
        hunts
            .borrow()
            .values()
            .flat_map(|pests| pests.iter().filter_map(|pest| pest.body))
            .collect()
    })
}

/// What a tracked hunter's death drops: its layer's shard, and nothing for
/// a mob the mod does not track (depth alone is not proof of a hunt).
pub fn pest_drop(id: u32) -> Option<&'static str> {
    HUNTS.with(|hunts| {
        hunts.borrow().values().find_map(|pests| {
            pests
                .iter()
                .position(|pest| pest.mob == Some(id))
                .map(|slot| PESTS[slot].body)
        })
    })
}

/// Empties the hunts: a world closed, or the harness runs again.
pub fn clear_pests() {
    HUNTS.with(|hunts| hunts.borrow_mut().clear());
}

/// The dig's feedback, packed for the net bridge: the block's position and
/// which dig it was, in 13 bytes.
pub fn encode_dig(pos: [i32; 3], dig: Dig) -> [u8; 13] {
    let mut data = [0u8; 13];
    for (i, value) in pos.iter().enumerate() {
        data[i * 4..i * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    data[12] = match dig {
        Dig::Bedrock => 0,
        Dig::Voidshell => 1,
    };
    data
}

/// The feedback unpacked; bytes that aren't a whole dig pack are nothing.
pub fn decode_dig(data: &[u8]) -> Option<([i32; 3], Dig)> {
    if data.len() != 13 {
        return None;
    }
    let mut pos = [0i32; 3];
    for (i, value) in pos.iter_mut().enumerate() {
        *value = i32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().ok()?);
    }
    let dig = match data[12] {
        0 => Dig::Bedrock,
        1 => Dig::Voidshell,
        _ => return None,
    };
    Some((pos, dig))
}

/// The veil's message: which boundary was crossed (one byte) and the
/// player's x and z in blocks (four bytes each, little endian) - the window
/// knows how to sparkle but not where its player stands.
pub fn encode_veil(bound: usize, at: [f64; 3]) -> [u8; 9] {
    let mut data = [0u8; 9];
    data[0] = bound as u8;
    data[1..5].copy_from_slice(&(at[0].floor() as i32).to_le_bytes());
    data[5..9].copy_from_slice(&(at[2].floor() as i32).to_le_bytes());
    data
}

/// The veil unpacked; anything else (torn bytes, a boundary past the end)
/// is nothing.
pub fn decode_veil(data: &[u8]) -> Option<(usize, [i32; 2])> {
    if data.len() != 9 || data[0] as usize >= VEIL_BOUNDS.len() {
        return None;
    }
    Some((
        data[0] as usize,
        [
            i32::from_le_bytes(data[1..5].try_into().ok()?),
            i32::from_le_bytes(data[5..9].try_into().ok()?),
        ],
    ))
}

/// The ring a crossing plays on the player's client: [`VEIL_SPARKS`] points
/// on a circle of [`VEIL_RING`] blocks around (x, z), [`VEIL_LIFT`] over the
/// boundary's own y - blue soul flames, the void's own colour.
pub fn veil_ring(x: f64, z: f64, bound: usize) -> Vec<[f64; 3]> {
    let y = VEIL_BOUNDS[bound] as f64 + VEIL_LIFT;
    (0..VEIL_SPARKS)
        .map(|i| {
            let turn = i as f64 * std::f64::consts::TAU / VEIL_SPARKS as f64;
            [x + VEIL_RING * turn.cos(), y, z + VEIL_RING * turn.sin()]
        })
        .collect()
}

/// The band of boundaries a y sits in: 0 above the gate, then one per
/// layer down to 6, the absolute void.
pub fn veil_band(y: f64) -> usize {
    VEIL_BOUNDS
        .iter()
        .filter(|&&bound| (bound as f64) > y)
        .count()
}

thread_local! {
    /// Each player's last band, for the crossing watch.
    static VEIL_BANDS: RefCell<HashMap<u128, usize>> = RefCell::new(HashMap::new());
}

/// The boundary a player has just crossed, for the transition veil: the
/// first sight of a player records their band and plays nothing; after that
/// a band change answers with the index (in [`VEIL_BOUNDS`]) of the
/// boundary crossed - but only once the player is [`VEIL_HYSTERESIS`] past
/// it, so hovering on the line plays nothing every tick. The tick puts the
/// sparkles where the boundary itself is, not where the player is.
pub fn veil_crossing(uuid: u128, y: f64) -> Option<usize> {
    VEIL_BANDS.with(|bands| {
        let mut bands = bands.borrow_mut();
        let band = veil_band(y);
        let Some(&last) = bands.get(&uuid) else {
            bands.insert(uuid, band);
            return None;
        };
        if band == last {
            return None;
        }
        let crossed = last.min(band);
        let bound = VEIL_BOUNDS[crossed] as f64;
        let past = if band > last {
            y <= bound - VEIL_HYSTERESIS
        } else {
            y >= bound + VEIL_HYSTERESIS
        };
        if !past {
            return None;
        }
        bands.insert(uuid, band);
        Some(crossed)
    })
}

/// A level closed (API 134's `native_level_unload`): the Overworld's close
/// forgets everything the mod keeps for it - the creatures, the hunts, the
/// players' rhythms and the shape it announced - so a world reopened (or
/// another one) starts clean and no stale engine id is ever moved again.
/// Nothing is lost by it: every creature's kind rides the yaw code the
/// engine saves with its display, so the reopened world's first deep tick
/// adopts the saved displays back as the kinds they were. Another
/// dimension's close is none of the mod's business.
pub fn forget_world(dimension: &str) -> bool {
    if dimension != "minecraft:overworld" {
        return false;
    }
    clear_creatures();
    clear_pests();
    VEIL_BANDS.with(|bands| bands.borrow_mut().clear());
    SHAPE.with(|cell| cell.set(None));
    TICKS.with(|ticks| ticks.borrow_mut().clear());
    BARS.with(|bars| bars.borrow_mut().clear());
    true
}

/// What a death's drops (API 75's `native_loot_drops`) gain: the shard of
/// the layer whose hunt tracked the dying vex, in the Overworld, right now.
/// Depth alone is not proof - a vanilla vex can chase a player down a dug
/// shaft and die in the deep, and that death owes nobody a trophy - so an
/// untracked vex drops nothing however deep it falls.
pub fn pest_drops(dimension: &str, entity: Option<(u32, &str)>) -> Option<&'static str> {
    let (id, kind) = entity?;
    if kind != HUNT_KIND || dimension != "minecraft:overworld" {
        return None;
    }
    pest_drop(id)
}

/// The line a player hears when the dig's feedback cannot reach their client
/// (the net bridge refused it): the break stays silent, and this says why.
pub fn feedback_failed(why: &str) -> String {
    format!(
        "Under Bedrock: the dig feedback could not reach your client ({why});          the break sound and sparks stay off."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dig_time_is_one_of_the_mods() {
        assert_eq!(read_dig_ticks(None), DEFAULT_DIG_TICKS);
        assert_eq!(read_dig_ticks(Some("20\n".into())), 20);
        assert_eq!(read_dig_ticks(Some("7".into())), DEFAULT_DIG_TICKS);
        assert_eq!(read_dig_ticks(Some("x".into())), DEFAULT_DIG_TICKS);
    }

    #[test]
    fn dig_times_cycle() {
        assert_eq!(next_dig_ticks(20), 40);
        assert_eq!(next_dig_ticks(100), 200);
        assert_eq!(next_dig_ticks(200), 20);
        assert_eq!(next_dig_ticks(7), 20);
        assert_eq!(dig_label(60), "Break time: 3.0 s (bedrock, voidshell)");
        assert_eq!(dig_label(20), "Break time: 1.0 s (bedrock, voidshell)");
    }

    #[test]
    fn bedrock_drops_unless_stored_false() {
        assert!(read_drop(None));
        assert!(read_drop(Some("true".into())));
        assert!(!read_drop(Some("false\n".into())));
        assert_eq!(drop_label(true), "Bedrock drops: ON");
        assert_eq!(drop_label(false), "Bedrock drops: OFF");
    }

    #[test]
    fn progress_counts_to_the_dig_time() {
        assert_eq!(progress(0, 60), 0.0);
        assert_eq!(progress(30, 60), 0.5);
        assert_eq!(progress(60, 60), 1.0);
        assert_eq!(progress(900, 60), 1.0);
        assert_eq!(progress(5, 0), 1.0);
    }

    #[test]
    fn the_bar_reads_as_a_bar() {
        assert_eq!(bar("Bedrock", 0.0), "Bedrock [--------------------] 0%");
        assert_eq!(bar("Bedrock", 0.25), "Bedrock [#####---------------] 25%");
        assert_eq!(
            bar("Voidshell", 1.0),
            "Voidshell [####################] 100%"
        );
        assert_eq!(bar("Bedrock", 1.4), bar("Bedrock", 1.0));
        assert_eq!(bar("Bedrock", -0.2), bar("Bedrock", 0.0));
    }

    #[test]
    fn the_pickaxe_digs_bedrock_and_voidshell_only() {
        assert_eq!(dig_target(BEDROCK), Some(Dig::Bedrock));
        assert_eq!(dig_target(VOIDSHELL), Some(Dig::Voidshell));
        // The world's bottom stays: not even the pickaxe digs it.
        assert_eq!(dig_target(ABYSS_BEDROCK), None);
        assert_eq!(dig_target("minecraft:stone"), None);
        assert_eq!(dig_name(Dig::Bedrock), "Bedrock");
        assert_eq!(dig_name(Dig::Voidshell), "Voidshell");
        assert_eq!(dig_drop(Dig::Bedrock), BEDROCK);
        assert_eq!(dig_drop(Dig::Voidshell), VOIDSHELL);
        assert!(dig_drops(Dig::Voidshell, || Some("false".into())));
        assert!(!dig_drops(Dig::Bedrock, || Some("false".into())));
        // The flag is read once, then kept: a later read hears the first.
        assert!(!dig_drops(Dig::Bedrock, || None));
    }

    #[test]
    fn bedrock_drops_by_default_when_dug() {
        assert!(dig_drops(Dig::Bedrock, || None));
    }

    #[test]
    fn spawn_offsets_stay_in_the_ring() {
        for seed in [1u64, 7, 12345, 0xDEAD_BEEF] {
            for attempt in 0..SPAWN_ATTEMPTS {
                let [dx, dy, dz] = candidate_offset(seed, attempt);
                assert!((8..=24).contains(&dx.abs().max(dz.abs())), "{dx},{dz}");
                assert!((-8..=4).contains(&dy), "{dy}");
            }
        }
        assert_ne!(candidate_offset(3, 0), candidate_offset(3, 1));
    }

    #[test]
    fn a_spawn_plan_needs_a_pocket_and_room_under_the_cap() {
        let at = [0.5, -200.0, 0.5];
        // The absolute void is the warden's ground: no ambient plan there.
        assert_eq!(spawn_plan(7, [0.5, -900.0, 0.5], 0, |_| true), None);
        // Everything open: no floor under any feet, so no plan.
        assert_eq!(spawn_plan(7, at, 0, |_| true), None);
        // At the cap: no plan even with a pocket.
        let floor_of = |seed: u64| {
            let [dx, dy, dz] = candidate_offset(seed, 0);
            [
                (at[0] + dx as f64) as i32,
                (at[1] + dy as f64).floor() as i32 - 1,
                (at[2] + dz as f64) as i32,
            ]
        };
        let floor = floor_of(7);
        let pocket = move |cell: [i32; 3]| cell != floor;
        assert_eq!(spawn_plan(7, at, CREATURE_CAP, pocket), None);
        // With room and a floor: the first attempt's creature and spot.
        let (creature, spot) = spawn_plan(7, at, CREATURE_CAP - 1, pocket).expect("a plan");
        assert_eq!(
            creature,
            pick_creature(7, at[1]).expect("a lava-layer pick")
        );
        let [dx, dy, dz] = candidate_offset(7, 0);
        assert_eq!(
            spot,
            [
                at[0] + dx as f64,
                (at[1] + dy as f64).floor(),
                at[2] + dz as f64
            ]
        );
    }

    #[test]
    fn each_layer_picks_its_own_company() {
        let mut seen = [0u32; 6];
        for seed in 0..600u64 {
            // The lava zone: embers, lanterns and the odd wisp.
            match pick_creature(seed, -100.0).expect("the lava zone picks") {
                Creature::Ember => seen[0] += 1,
                Creature::Lantern => seen[1] += 1,
                Creature::Wisp => seen[2] += 1,
                other => panic!("the lava zone picked {other:?}"),
            }
            // The inactive layer: wisps and lanterns, no embers.
            match pick_creature(seed, -300.0).expect("the inactive layer picks") {
                Creature::Wisp => seen[2] += 1,
                Creature::Lantern => seen[1] += 1,
                other => panic!("the inactive layer picked {other:?}"),
            }
            // The calm deep: the gloomling alone.
            assert_eq!(pick_creature(seed, -400.0), Some(Creature::Gloomling));
            seen[5] += 1;
            // The cool lands: the classic company.
            match pick_creature(seed, -550.0).expect("the cool lands pick") {
                Creature::Wisp => seen[2] += 1,
                Creature::Watcher => seen[3] += 1,
                Creature::Lantern => seen[1] += 1,
                other => panic!("the cool lands picked {other:?}"),
            }
            // The frozen wastes: frostmotes only.
            assert_eq!(pick_creature(seed, -700.0), Some(Creature::Frostmote));
            seen[4] += 1;
        }
        assert!(seen.iter().all(|count| *count > 0), "{seen:?}");
        // The absolute void: nothing ambient, only the warden hunts.
        assert_eq!(pick_creature(7, -800.0), None);
        assert_eq!(pick_creature(7, -950.0), None);
        // What each shows and rides.
        assert_eq!(Creature::Ember.shows(), EMBER_ITEM);
        assert_eq!(Creature::Frostmote.shows(), FROSTMOTE_ITEM);
        assert_eq!(Creature::Gloomling.shows(), GLOOM_ITEM);
        assert_eq!(Creature::Gloom.shows(), GLOOM_ITEM);
        assert_eq!(Creature::Ember.display(), ITEM_DISPLAY);
        assert_eq!(Creature::Gloom.display(), ITEM_DISPLAY);
        assert_eq!(Creature::of(BLOCK_DISPLAY), Some(Creature::Lantern));
        assert_eq!(Creature::of(ITEM_DISPLAY), Some(Creature::Wisp));
        assert_eq!(Creature::of("minecraft:text_display"), None);
    }

    #[test]
    fn the_cap_counts_the_mods_creatures_near_the_player() {
        let player = [0.0, -200.0, 0.0];
        assert!(is_ours(ITEM_DISPLAY, [10.0, -210.0, 0.0]));
        assert!(is_ours(BLOCK_DISPLAY, [-30.0, -90.0, 20.0]));
        // Above the floor, or not a display the mod rides: not the mod's.
        assert!(!is_ours(ITEM_DISPLAY, [10.0, -70.0, 0.0]));
        // The frozen wastes keep frostmotes now; the absolute void below
        // them keeps no ambient display at all - the warden's body is the
        // hunt's own, not a creature's.
        assert!(is_ours(ITEM_DISPLAY, [10.0, -700.0, 0.0]));
        assert!(!is_ours(ITEM_DISPLAY, [10.0, -900.0, 0.0]));
        assert!(!is_ours("minecraft:text_display", [10.0, -210.0, 0.0]));
        assert!(!is_ours("minecraft:marker", [10.0, -210.0, 0.0]));
        // Near enough to count against the cap, and too far.
        assert!(counts_towards_cap([10.0, -210.0, 0.0], player));
        assert!(!counts_towards_cap([60.0, -210.0, 0.0], player));
    }

    #[test]
    fn creatures_drift_on_their_paths_around_their_anchors() {
        let anchor = [100.0, -200.0, 100.0];
        for kind in [
            Creature::Wisp,
            Creature::Lantern,
            Creature::Watcher,
            Creature::Ember,
            Creature::Frostmote,
            Creature::Gloomling,
            Creature::Gloom,
        ] {
            let mut phase = 0.0;
            for _ in 0..2000 {
                let pos = drift(kind, anchor, phase);
                let out = (pos[0] - anchor[0]).abs().max((pos[2] - anchor[2]).abs());
                assert!(out <= 2.6, "{kind:?} drifts {out} blocks out");
                let up = pos[1] - anchor[1];
                assert!(up > 0.0 && up < 2.6, "{kind:?} bobs out of its air: {up}");
                phase += PHASE_STEP;
            }
        }
    }

    #[test]
    fn a_watcher_closes_and_keeps_its_distance() {
        let player = [0.0, -200.0, 0.0];
        // From just inside its notice: it eases toward the player and stops
        // off them.
        let start = [10.0, -199.0, 0.0];
        let mut anchor = start;
        for _ in 0..400 {
            anchor = watcher_anchor(anchor, player);
        }
        let eye = [player[0], player[1] + 1.0, player[2]];
        let dist = ((anchor[0] - eye[0]).powi(2)
            + (anchor[1] - eye[1]).powi(2)
            + (anchor[2] - eye[2]).powi(2))
        .sqrt();
        assert!(
            dist < 6.0 && dist > WATCHER_KEEP - 0.5,
            "it ends {dist} out"
        );
        assert_ne!(anchor, start);
        // From far away: it holds where it is.
        let far = [200.0, -199.0, 0.0];
        assert_eq!(watcher_anchor(far, player), far);
        // Right on the player: no jump, no division by nothing.
        let on = [0.0, -199.0, 0.0];
        assert_eq!(watcher_anchor(on, player), on);
    }

    #[test]
    fn the_yaw_and_spin_helpers_point_where_they_say() {
        // 0 is south (+z), -90 east, as the engine's yaw is.
        assert_eq!(face_yaw([0.0; 3], [0.0, 0.0, 5.0]).round(), 0.0);
        assert_eq!(face_yaw([0.0; 3], [5.0, 0.0, 0.0]).round(), -90.0);
        assert_eq!(face_yaw([0.0; 3], [-5.0, 0.0, 0.0]).round(), 90.0);
        // No turn is the identity, and every spin is a unit quaternion.
        assert_eq!(spin_y(0.0), [0.0, 0.0, 0.0, 1.0]);
        for angle in [0.3f32, 1.0, -2.5, 12.0] {
            let [x, y, z, w] = spin_y(angle);
            let norm = (x * x + y * y + z * z + w * w).sqrt();
            assert!((norm - 1.0).abs() < 1.0e-5, "spin_y({angle}) norm {norm}");
        }
    }

    #[test]
    fn the_creature_list_follows_the_engine() {
        clear_creatures();
        let player = [0.0, -200.0, 0.0];
        let owner = 0x0BED_0007;
        let code = Creature::Wisp.yaw_code();
        // A display the list doesn't know but whose yaw wears the mod's code
        // is adopted and driven at once...
        let step = step_creatures(
            player,
            owner,
            &[(11, ITEM_DISPLAY, code, [3.0, -201.0, 3.0])],
        );
        assert_eq!(creatures_len(), 1);
        assert_eq!(creature_kind(11), Some(Creature::Wisp));
        assert_eq!(step.moves.len(), 1);
        assert_eq!(step.moves[0].0, 11);
        assert_eq!(step.moves[0].2, code, "every move keeps the code on");
        assert_eq!(step.poses.len(), 1, "a fresh creature poses at once");
        assert!(step.cull.is_empty() && step.purge.is_empty());
        // ...its pose comes again on the mod's rhythm...
        let mut poses = 0;
        for _ in 0..POSE_EVERY {
            let step = step_creatures(
                player,
                owner,
                &[(11, ITEM_DISPLAY, code, [3.0, -201.0, 3.0])],
            );
            poses += step.poses.len();
            assert_eq!(step.moves.len(), 1);
        }
        assert_eq!(poses, 1);
        // ...a display wearing no code is not the mod's: it goes, undriven...
        let step = step_creatures(
            player,
            owner,
            &[(14, ITEM_DISPLAY, 33.0, [3.0, -201.0, 3.0])],
        );
        assert_eq!(step.purge, [14]);
        assert!(creature_kind(14).is_none());
        assert!(step.moves.iter().all(|(id, _, _)| *id != 14));
        // ...one the engine no longer has drops off the list...
        let step = step_creatures(player, owner, &[]);
        assert_eq!(creatures_len(), 0);
        assert!(
            step.moves.is_empty()
                && step.poses.is_empty()
                && step.cull.is_empty()
                && step.purge.is_empty()
        );
        // ...and one whose own player walks away is let go.
        add_creature(12, Creature::Wisp, [0.0, -200.0, 0.0], owner);
        let far = [500.0, -200.0, 0.0];
        let step = step_creatures(far, owner, &[(12, ITEM_DISPLAY, code, [0.0, -200.0, 0.0])]);
        assert_eq!(step.cull, [12]);
        assert_eq!(creatures_len(), 0);
        assert!(step.moves.is_empty());
        // Another player's creature is left alone by this player's tick.
        add_creature(13, Creature::Wisp, [0.0, -200.0, 0.0], owner);
        let step = step_creatures(
            far,
            owner + 1,
            &[(13, ITEM_DISPLAY, code, [0.0, -200.0, 0.0])],
        );
        assert!(step.cull.is_empty());
        assert_eq!(creatures_len(), 1);
        clear_creatures();
    }

    #[test]
    fn a_spawn_is_looked_for_on_rhythm() {
        let uuid = 0x0BED_0002;
        let dues: Vec<bool> = (0..SPAWN_EVERY * 2 + 1).map(|_| spawn_due(uuid)).collect();
        assert_eq!(dues.iter().filter(|d| **d).count(), 2);
        assert!(dues[SPAWN_EVERY as usize - 1]);
    }

    #[test]
    fn every_layer_sends_its_pest_and_takes_it_back() {
        let warden = &PESTS[4];
        let player = [0.0, -850.0, 0.0];
        let mob = [10.0, -850.0, 10.0];
        // In the void with the quiet over: a summon is ordered, and the
        // retry waits in case it finds no room (peaceful, an unloaded chunk).
        let (pest, order) = pest_tick(warden, Pest::default(), true, None, player);
        assert!(order.summon && pest.quiet == warden.retry);
        assert!(!order.summon_body && order.follow.is_none());
        let mut pest = pest;
        for _ in 0..warden.retry - 1 {
            let (next, order) = pest_tick(warden, pest, true, None, player);
            assert!(!order.summon, "the retry waits its pause");
            pest = next;
        }
        let (_, order) = pest_tick(warden, pest, true, None, player);
        assert!(order.summon, "the retry comes");
        // The caller wrote the ids in: the body follows the mob, faced at
        // the player.
        let hunting = Pest {
            mob: Some(5),
            body: Some(6),
            quiet: 0,
        };
        let (kept, order) = pest_tick(warden, hunting, true, Some(mob), player);
        assert_eq!(kept, hunting);
        let (at, yaw) = order.follow.expect("the body follows");
        assert!((at[1] - (mob[1] + HUNT_LIFT)).abs() < 1.0e-9);
        assert_eq!(yaw, face_yaw(at, [player[0], player[1] + 1.0, player[2]]));
        // A mob with no body (a summon that failed): the body is asked for.
        let (_, order) = pest_tick(
            warden,
            Pest {
                mob: Some(5),
                body: None,
                quiet: 0,
            },
            true,
            Some(mob),
            player,
        );
        assert!(order.summon_body && order.follow.is_none());
        // It fell: the body goes with it, and the layer is quiet.
        let (after, order) = pest_tick(warden, hunting, true, None, player);
        assert_eq!(order.dismiss_body, Some(6));
        assert!(order.dismiss_mob.is_none() && !order.summon);
        assert_eq!(
            after,
            Pest {
                mob: None,
                body: None,
                quiet: warden.quiet
            }
        );
        let (_, order) = pest_tick(warden, after, true, None, player);
        assert!(!order.summon, "the quiet holds");
        // The player climbs out mid-hunt: both go, at once.
        let (after, order) = pest_tick(warden, hunting, false, Some(mob), [0.0, -300.0, 0.0]);
        assert_eq!(order.dismiss_mob, Some(5));
        assert_eq!(order.dismiss_body, Some(6));
        assert_eq!(after, Pest::default());
        // Out of the layer with nothing to take back: no orders at all.
        let (after, order) = pest_tick(warden, Pest::default(), false, None, player);
        assert_eq!(after, Pest::default());
        assert_eq!(order, PestOrder::default());
        // A shallow layer pest: the same machine, gentler numbers than the
        // warden's - annoying, not lethal.
        let emberling = &PESTS[0];
        let (pest, order) = pest_tick(emberling, Pest::default(), true, None, [0.0, -150.0, 0.0]);
        assert!(order.summon && pest.quiet == emberling.retry);
        assert!(emberling.health < warden.health);
        assert!(emberling.ring > warden.ring && emberling.quiet > warden.quiet);
    }

    #[test]
    fn a_hunter_takes_shape_on_its_ring() {
        for (cfg, at) in [
            (&PESTS[0], [0.5, -150.0, 0.5]),
            (&PESTS[4], [0.5, -850.0, 0.5]),
        ] {
            for seed in [1u64, 99, 0xF00D] {
                let spot = pest_spot(cfg, seed, at);
                let away = ((spot[0] - at[0]).powi(2) + (spot[2] - at[2]).powi(2)).sqrt();
                assert!((away - cfg.ring).abs() < 1.0e-9, "{away} off the ring");
                assert_eq!(spot[1], at[1], "at the player's depth");
            }
            assert_ne!(pest_spot(cfg, 1, at), pest_spot(cfg, 2, at));
        }
    }

    #[test]
    fn a_hunter_is_known_by_its_mob_and_drops_its_shard() {
        clear_pests();
        let mut mine = [Pest::default(); 5];
        mine[0] = Pest {
            mob: Some(9),
            body: Some(10),
            quiet: 0,
        };
        mine[4] = Pest {
            mob: Some(19),
            body: None,
            quiet: 3,
        };
        set_pests(1, mine);
        assert!(is_tracked_mob(9) && is_tracked_mob(19));
        assert!(!is_tracked_mob(11));
        assert_eq!(get_pests(1)[4].quiet, 3);
        assert_eq!(pest_bodies(), vec![10]);
        // A tracked hunter's death drops its own layer's shard; an
        // untracked vex, however deep, owes nobody a trophy.
        assert_eq!(
            pest_drops("minecraft:overworld", Some((9, HUNT_KIND))),
            Some(PESTS[0].body)
        );
        assert_eq!(
            pest_drops("minecraft:overworld", Some((19, HUNT_KIND))),
            Some(PESTS[4].body)
        );
        assert_eq!(
            pest_drops("minecraft:overworld", Some((99, HUNT_KIND))),
            None
        );
        assert_eq!(
            pest_drops("minecraft:overworld", Some((9, "minecraft:zombie"))),
            None
        );
        assert_eq!(
            pest_drops("minecraft:the_nether", Some((9, HUNT_KIND))),
            None
        );
        assert_eq!(pest_drops("minecraft:overworld", None), None);
        // The default clears: a finished hunt leaves no entry.
        set_pests(1, [Pest::default(); 5]);
        assert!(!is_tracked_mob(9));
        assert_eq!(get_pests(1), [Pest::default(); 5]);
        clear_pests();
        assert!(!is_tracked_mob(9));
    }

    #[test]
    fn a_fed_gloomling_keeps_its_player() {
        clear_creatures();
        let owner = 0x0BED_0009;
        // A gloomling the deep spawned: shy, unowned, unfed. Feeding is a
        // held thing, tick by tick, and the last tick is the one that tames.
        add_creature(31, Creature::Gloomling, [10.0, -400.0, 10.0], owner);
        for _ in 0..FEED_TICKS - 1 {
            assert!(!feed_creature(31, owner));
        }
        assert!(feed_creature(31, owner), "the last tick tames it");
        assert_eq!(creature_kind(31), Some(Creature::Gloom));
        // Fed, it is let go by nothing: even a tick from far away moves it,
        // never culls it, and the tick keeps it in the live list anywhere.
        let far = [200.0, -400.0, 200.0];
        let step = step_creatures(
            far,
            owner,
            &[(31, ITEM_DISPLAY, 215.0, [10.0, -400.0, 10.0])],
        );
        assert!(step.cull.is_empty());
        assert_eq!(step.moves[0].2, Creature::Gloom.yaw_code());
        assert_eq!(tamed_ids(), vec![31]);
        assert!(driven_ids().contains(&31));
        // A wild gloomling its player walks away from is let go as usual.
        add_creature(32, Creature::Gloomling, [10.0, -400.0, 10.0], owner);
        let step = step_creatures(
            far,
            owner,
            &[(32, ITEM_DISPLAY, 183.0, [10.0, -400.0, 10.0])],
        );
        assert_eq!(step.cull, vec![32]);
        clear_creatures();
    }

    #[test]
    fn a_crossing_plays_its_veil_once() {
        assert_eq!(veil_band(0.0), 0);
        assert_eq!(veil_band(-100.0), 1);
        assert_eq!(veil_band(-250.0), 2);
        assert_eq!(veil_band(-850.0), 6);
        // The first sight of a player records their band and plays nothing.
        assert_eq!(veil_crossing(5, -100.0), None);
        // Down through the lava zone's floor: the boundary at index 1.
        assert_eq!(veil_crossing(5, -230.0), Some(1));
        // Inside the band: nothing. Back on the line: the hysteresis holds.
        assert_eq!(veil_crossing(5, -222.0), None);
        assert_eq!(veil_crossing(5, -220.0), None);
        // Past the line the other way: the same boundary, crossed up.
        assert_eq!(veil_crossing(5, -218.0), Some(1));
        // A fall to the void plays the one boundary the watch last knew.
        assert_eq!(veil_crossing(5, -850.0), Some(1));
        // The watch is per player...
        assert_eq!(veil_crossing(6, -850.0), None);
        // ...and a closed world forgets it.
        assert!(forget_world("minecraft:overworld"));
        assert_eq!(veil_crossing(5, -850.0), None);
    }

    #[test]
    fn a_fed_gloomling_eases_to_its_players_side() {
        let player = [0.0, 60.0, 0.0];
        let anchor = [40.0, -400.0, 0.0];
        // Every tick closes a tenth of the way...
        let mut at = anchor;
        let first = follow_anchor(at, player);
        assert!(first[0] < at[0] && first[1] > at[1]);
        // ...until it hangs GLOOM_KEEP off the player's eye, and holds.
        for _ in 0..400 {
            at = follow_anchor(at, player);
        }
        let eye = [player[0], player[1] + 1.0, player[2]];
        let away =
            ((at[0] - eye[0]).powi(2) + (at[1] - eye[1]).powi(2) + (at[2] - eye[2]).powi(2)).sqrt();
        assert!((away - GLOOM_KEEP as f64).abs() < 1.0e-6, "{away}");
        // The easing is asymptotic: at the keep it stops moving by any
        // amount that matters.
        let held = follow_anchor(at, player);
        assert!((held[0] - at[0]).abs() < 1.0e-9);
        assert!((held[1] - at[1]).abs() < 1.0e-9);
        assert!((held[2] - at[2]).abs() < 1.0e-9);
        // Right on top of its player, it holds exactly where it is.
        let eye = [player[0], player[1] + 1.0, player[2]];
        assert_eq!(follow_anchor(eye, player), eye);
    }

    #[test]
    fn a_veil_message_carries_its_boundary_and_place() {
        let at = [12.7, -230.0, -4.2];
        let data = encode_veil(1, at);
        assert_eq!(decode_veil(&data), Some((1, [12, -5])));
        // Torn bytes, or a boundary past the end: nothing.
        assert_eq!(decode_veil(&data[..8]), None);
        assert_eq!(decode_veil(&[6, 0, 0, 0, 0, 0, 0, 0, 0]), None);
        assert_eq!(decode_veil(&[]), None);
        // The ring the window plays: on the boundary's own height, a circle
        // of the ring's width around the place the message named.
        let ring = veil_ring(12.0, -5.0, 1);
        assert_eq!(ring.len(), VEIL_SPARKS);
        for p in &ring {
            assert_eq!(p[1], VEIL_BOUNDS[1] as f64 + VEIL_LIFT);
            let away = ((p[0] - 12.0).powi(2) + (p[2] + 5.0).powi(2)).sqrt();
            assert!((away - VEIL_RING).abs() < 1.0e-9, "{away}");
        }
    }

    #[test]
    fn a_fed_gloomling_is_known_by_its_code_anywhere() {
        assert!(is_tamed_code(ITEM_DISPLAY, 215.0));
        assert!(is_tamed_code(ITEM_DISPLAY, 216.5));
        assert!(!is_tamed_code(ITEM_DISPLAY, 212.0));
        assert!(!is_tamed_code(ITEM_DISPLAY, 183.0));
        assert!(!is_tamed_code(BLOCK_DISPLAY, 215.0));
    }

    #[test]
    fn a_closed_world_is_forgotten() {
        clear_creatures();
        clear_pests();
        add_creature(21, Creature::Wisp, [0.0, -200.0, 0.0], 7);
        let mut hunts = [Pest::default(); 5];
        hunts[4] = Pest {
            mob: Some(3),
            body: Some(4),
            quiet: 0,
        };
        set_pests(7, hunts);
        assert!(!shape_seen((-576, 896)), "a fresh world's shape is unseen");
        assert!(shape_seen((-576, 896)), "and remembered after the look");
        assert!(!bar_shown(7, true), "the bar is noted once");
        // Another dimension's close is not the mod's business.
        assert!(!forget_world("minecraft:the_nether"));
        assert_eq!(creatures_len(), 1);
        // The Overworld's close forgets all of it.
        assert!(forget_world("minecraft:overworld"));
        assert_eq!(creatures_len(), 0);
        assert_eq!(get_pests(7), [Pest::default(); 5]);
        assert!(
            !shape_seen((-576, 896)),
            "a reopened world is looked at once again"
        );
        assert!(!bar_shown(7, true), "the bars were cleared");
        SHAPE.with(|cell| cell.set(None));
        BARS.with(|bars| bars.borrow_mut().clear());
    }

    #[test]
    fn a_creatures_kind_rides_its_yaw() {
        // Every kind reads back from its own code, with room on both sides,
        // and its code on the wrong display kind reads as nothing.
        for kind in [
            Creature::Wisp,
            Creature::Lantern,
            Creature::Watcher,
            Creature::Ember,
            Creature::Frostmote,
            Creature::Gloomling,
            Creature::Gloom,
        ] {
            let code = kind.yaw_code();
            assert_eq!(kind_from_yaw(kind.display(), code), Some(kind));
            assert_eq!(kind_from_yaw(kind.display(), code + 1.5), Some(kind));
            assert_eq!(kind_from_yaw(kind.display(), code - 1.5), Some(kind));
            let other = if kind.display() == ITEM_DISPLAY {
                BLOCK_DISPLAY
            } else {
                ITEM_DISPLAY
            };
            assert_eq!(kind_from_yaw(other, code), None);
        }
        // Just outside the window: nothing.
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 23.0 + 2.5), None);
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 23.0 - 2.5), None);
        // A yaw that is no code - above all the zero a plain /summon writes
        // and the cardinal points - and a display kind the mod doesn't ride:
        // nothing of the mod's.
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 0.0), None);
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 90.0), None);
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 180.0), None);
        assert_eq!(kind_from_yaw(ITEM_DISPLAY, 33.0), None);
        assert_eq!(kind_from_yaw(BLOCK_DISPLAY, 0.0), None);
        assert_eq!(kind_from_yaw(BLOCK_DISPLAY, 239.0 + 40.0), None);
        assert_eq!(kind_from_yaw("minecraft:text_display", 23.0), None);
        assert_eq!(kind_from_yaw("minecraft:marker", 95.0), None);
    }

    #[test]
    fn a_reopened_world_adopts_its_kinds_back() {
        clear_creatures();
        add_creature(11, Creature::Ember, [3.0, -100.0, 3.0], 7);
        // The world closes: the mod forgets what it drove.
        assert!(forget_world("minecraft:overworld"));
        assert_eq!(creatures_len(), 0);
        // The engine saved the display with the code yaw every move kept,
        // and hands it back when the chunk loads - a world reloaded in the
        // same session or the whole game restarted, alike: the first deep
        // tick adopts it back as the ember it was.
        step_creatures(
            [0.0, -100.0, 0.0],
            7,
            &[(
                11,
                ITEM_DISPLAY,
                Creature::Ember.yaw_code(),
                [3.0, -100.0, 3.0],
            )],
        );
        assert_eq!(creature_kind(11), Some(Creature::Ember));
        // A stranger's display - a /summon, another mod's - wears no code:
        // it is let go, never driven.
        let step = step_creatures(
            [0.0, -100.0, 0.0],
            7,
            &[(12, ITEM_DISPLAY, 41.0, [3.0, -100.0, 3.0])],
        );
        assert_eq!(step.purge, [12]);
        assert_eq!(creature_kind(12), None);
        clear_creatures();
    }

    #[test]
    fn the_dig_feedback_packs_and_unpacks() {
        for (pos, dig) in [
            ([3, -64, -7], Dig::Bedrock),
            ([-11, -70, 15], Dig::Voidshell),
        ] {
            let data = encode_dig(pos, dig);
            assert_eq!(decode_dig(&data), Some((pos, dig)));
        }
        assert_eq!(decode_dig(&[]), None);
        assert_eq!(decode_dig(&[0u8; 12]), None);
        let mut bad = encode_dig([0, 0, 0], Dig::Bedrock);
        bad[12] = 7;
        assert_eq!(decode_dig(&bad), None);
    }

    #[test]
    fn a_tall_world_has_room_below_its_floor() {
        assert!(tall(-576, 896));
        assert!(tall(-1008, 1328), "the pack's own shape");
        assert!(tall(-1024, 1280));
        assert!(!tall(-64, 384));
        assert!(!tall(-576, 640));
        assert!(depth_line(-64, 384).contains("NOT tall"));
        assert!(depth_line(-64, 384).contains("y -64 .. 319"));
        assert!(depth_line(-576, 896).contains("below the bedrock floor"));
    }

    #[test]
    fn settings_are_read_once_then_kept() {
        let reads = Cell::new(0u32);
        assert_eq!(
            dig_ticks(|| {
                reads.set(reads.get() + 1);
                Some("40".into())
            }),
            40
        );
        assert_eq!(
            dig_ticks(|| {
                reads.set(reads.get() + 1);
                Some("100".into())
            }),
            40
        );
        assert_eq!(reads.get(), 1);
        set_dig_ticks(100);
        assert_eq!(dig_ticks(|| None), 100);
        assert_eq!(reads.get(), 1);

        assert!(!drops(|| Some("false".into())));
        assert!(!drops(|| Some("false".into())));
        set_drops(true);
        assert!(drops(|| None));
    }

    #[test]
    fn a_world_shape_is_reported_once() {
        assert!(!shape_seen((-11, 12)));
        assert!(shape_seen((-11, 12)));
        assert!(!shape_seen((-576, 896)));
    }

    #[test]
    fn a_bar_is_cleared_once() {
        let uuid = 0x0BED_0001;
        assert!(!bar_shown(uuid, true));
        assert!(bar_shown(uuid, true));
        assert!(bar_shown(uuid, false));
        assert!(!bar_shown(uuid, false));
    }
}
