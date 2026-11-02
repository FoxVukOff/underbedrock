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
pub const BEDROCK: &str = "minecraft:bedrock";
pub const VOIDSHELL: &str = "underbedrock:voidshell";
pub const ABYSS_BEDROCK: &str = "underbedrock:abyss_bedrock";
pub const AIR: &str = "minecraft:air";

/// The Overworld the mod's datapack makes: 512 blocks below the vanilla
/// floor (`min_y` -576, 896 blocks of height).
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
/// The display entity kinds the creatures ride.
pub const ITEM_DISPLAY: &str = "minecraft:item_display";
pub const BLOCK_DISPLAY: &str = "minecraft:block_display";

/// A player below this y is in the Abyss, and gets its company.
pub const SPAWN_BELOW_Y: f64 = -80.0;
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
}

impl Creature {
    /// The display entity kind that carries it.
    pub fn display(self) -> &'static str {
        match self {
            Self::Lantern => BLOCK_DISPLAY,
            Self::Wisp | Self::Watcher => ITEM_DISPLAY,
        }
    }
    /// The item or block its display shows: the mod's own.
    pub fn shows(self) -> &'static str {
        match self {
            Self::Lantern => LANTERN_BLOCK,
            Self::Wisp | Self::Watcher => WISP_ITEM,
        }
    }
    /// The scale its display is posed at.
    pub fn scale(self) -> f32 {
        match self {
            Self::Wisp => 0.9,
            Self::Lantern => 0.55,
            Self::Watcher => 1.5,
        }
    }
    /// The creature a display of `display_kind` is: a watcher found again
    /// after a restart comes back a wisp (it drifts the same, only smaller).
    pub fn of(display_kind: &str) -> Option<Self> {
        match display_kind {
            BLOCK_DISPLAY => Some(Self::Lantern),
            ITEM_DISPLAY => Some(Self::Wisp),
            _ => None,
        }
    }
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

/// The creature the `seed`'s summon is: wisps the oftenest, then watchers,
/// and a lantern here and there.
pub fn pick_creature(seed: u64) -> Creature {
    match mix(seed ^ 0xAB55_5EED) % 10 {
        0..=4 => Creature::Wisp,
        5..=7 => Creature::Watcher,
        _ => Creature::Lantern,
    }
}

/// Whether a display entity is one of the mod's creatures: a display kind,
/// under the bedrock floor (nothing up top is the mod's).
pub fn is_ours(display_kind: &str, pos: [f64; 3]) -> bool {
    Creature::of(display_kind).is_some() && pos[1] < SPAWN_BELOW_Y
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
        return Some((pick_creature(seed.wrapping_add(attempt as u64)), spot));
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
}

/// What one step of the creatures asks the engine to do.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreatureStep {
    /// `native_remove_entity`: let go, their player walked away.
    pub cull: Vec<u32>,
    /// `native_move_entity`: the id, where to, facing which yaw.
    pub moves: Vec<(u32, [f64; 3], f32)>,
    /// `native_set_display`: the id, the kind (for its scale) and the spin.
    pub poses: Vec<(u32, Creature, f32)>,
}

thread_local! {
    /// The creatures the mod drives, in the world in place.
    static CREATURES: RefCell<Vec<CreatureState>> = const { RefCell::new(Vec::new()) };
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
        });
    });
}

/// One tick of the creatures, given the displays the engine has in the Abyss
/// (`live`: the id, the display kind and the position of each) and the player
/// whose tick this is. The engine's list is the truth: a state whose display
/// is gone drops off (a world reloaded, a `/kill`), and a display the list
/// doesn't know is adopted (the engine saved it with its chunk in an earlier
/// session; a watcher comes back a wisp). The rest take a step of their path
/// — watchers easing toward the player — and one whose own player is beyond
/// [`CREATURE_CULL`] of its anchor is let go. Each player's tick steps every
/// creature once, so a LAN game's creatures run a little faster the more
/// players are down there; one player's is exact.
pub fn step_creatures(
    player: [f64; 3],
    owner: u128,
    live: &[(u32, &str, [f64; 3])],
) -> CreatureStep {
    let mut step = CreatureStep::default();
    CREATURES.with(|cell| {
        let mut creatures = cell.borrow_mut();
        creatures.retain(|creature| live.iter().any(|(id, _, _)| *id == creature.id));
        for (id, display_kind, pos) in live {
            if creatures.iter().any(|creature| creature.id == *id) {
                continue;
            }
            let Some(kind) = Creature::of(display_kind) else {
                continue;
            };
            creatures.push(CreatureState {
                id: *id,
                kind,
                anchor: *pos,
                phase: *id as f32 * 0.7,
                owner,
                pose_in: 0,
            });
        }
        creatures.retain_mut(|creature| {
            if creature.owner == owner {
                let away = [
                    player[0] - creature.anchor[0],
                    player[2] - creature.anchor[2],
                ];
                if (away[0] * away[0] + away[1] * away[1]).sqrt() > CREATURE_CULL {
                    step.cull.push(creature.id);
                    return false;
                }
            }
            if creature.kind == Creature::Watcher {
                creature.anchor = watcher_anchor(creature.anchor, player);
            }
            creature.phase += PHASE_STEP;
            let pos = drift(creature.kind, creature.anchor, creature.phase);
            let yaw = match creature.kind {
                Creature::Watcher => face_yaw(pos, player),
                _ => (creature.phase * 30.0).rem_euclid(360.0),
            };
            step.moves.push((creature.id, pos, yaw));
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
        assert_eq!(creature, pick_creature(7));
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
    fn picked_creatures_are_the_mods_own() {
        let mut seen = [0u32; 3];
        for seed in 0..400u64 {
            match pick_creature(seed) {
                Creature::Wisp => seen[0] += 1,
                Creature::Watcher => seen[1] += 1,
                Creature::Lantern => seen[2] += 1,
            }
        }
        assert!(seen.iter().all(|count| *count > 0), "{seen:?}");
        // A wisp and a watcher show the mod's item; a lantern its block.
        assert_eq!(Creature::Wisp.shows(), WISP_ITEM);
        assert_eq!(Creature::Watcher.display(), ITEM_DISPLAY);
        assert_eq!(Creature::Lantern.shows(), LANTERN_BLOCK);
        assert_eq!(Creature::Lantern.display(), BLOCK_DISPLAY);
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
        assert!(!is_ours("minecraft:text_display", [10.0, -210.0, 0.0]));
        assert!(!is_ours("minecraft:marker", [10.0, -210.0, 0.0]));
        // Near enough to count against the cap, and too far.
        assert!(counts_towards_cap([10.0, -210.0, 0.0], player));
        assert!(!counts_towards_cap([60.0, -210.0, 0.0], player));
    }

    #[test]
    fn creatures_drift_on_their_paths_around_their_anchors() {
        let anchor = [100.0, -200.0, 100.0];
        for kind in [Creature::Wisp, Creature::Lantern, Creature::Watcher] {
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
        // A display the list doesn't know is adopted and driven at once...
        let step = step_creatures(player, owner, &[(11, ITEM_DISPLAY, [3.0, -201.0, 3.0])]);
        assert_eq!(creatures_len(), 1);
        assert_eq!(step.moves.len(), 1);
        assert_eq!(step.moves[0].0, 11);
        assert_eq!(step.poses.len(), 1, "a fresh creature poses at once");
        assert!(step.cull.is_empty());
        // ...its pose comes again on the mod's rhythm...
        let mut poses = 0;
        for _ in 0..POSE_EVERY {
            let step = step_creatures(player, owner, &[(11, ITEM_DISPLAY, [3.0, -201.0, 3.0])]);
            poses += step.poses.len();
            assert_eq!(step.moves.len(), 1);
        }
        assert_eq!(poses, 1);
        // ...one the engine no longer has drops off the list...
        let step = step_creatures(player, owner, &[]);
        assert_eq!(creatures_len(), 0);
        assert!(step.moves.is_empty() && step.poses.is_empty() && step.cull.is_empty());
        // ...and one whose own player walks away is let go.
        add_creature(12, Creature::Wisp, [0.0, -200.0, 0.0], owner);
        let far = [500.0, -200.0, 0.0];
        let step = step_creatures(far, owner, &[(12, ITEM_DISPLAY, [0.0, -200.0, 0.0])]);
        assert_eq!(step.cull, [12]);
        assert_eq!(creatures_len(), 0);
        assert!(step.moves.is_empty());
        // Another player's creature is left alone by this player's tick.
        add_creature(13, Creature::Wisp, [0.0, -200.0, 0.0], owner);
        let step = step_creatures(far, owner + 1, &[(13, ITEM_DISPLAY, [0.0, -200.0, 0.0])]);
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
    fn a_tall_world_has_room_below_its_floor() {
        assert!(tall(-576, 896));
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
