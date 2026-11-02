// Under Bedrock's native half: the void-alloy pickaxe digs the bedrock floor
// and the voidshell gate under it open, so the player gets into the Abyss the
// mod's datapack makes below; and the Abyss gets its own inhabitants, spawned
// where a player is deep enough. This file is both the mod's Cargo library
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

// --- the Abyss's own inhabitants -------------------------------------------

/// The kinds the mod spawns under the bedrock floor: a bat for ambient life,
/// and the three cave monsters, so the dark reads as a world and not a hole.
pub const SPAWN_KINDS: [&str; 4] = [
    "minecraft:bat",
    "minecraft:skeleton",
    "minecraft:spider",
    "minecraft:zombie",
];
/// A player below this y is in the Abyss, and gets its company.
pub const SPAWN_BELOW_Y: f64 = -80.0;
/// The most of the mod's mobs within reach of one player at a time.
pub const MOB_CAP: usize = 10;
/// The mobs are looked for this many blocks around the player.
pub const MOB_RANGE: f64 = 40.0;
/// How often a player's tick looks for a place to spawn one, in ticks.
pub const SPAWN_EVERY: u32 = 60;
/// The spawn attempts of one look.
pub const SPAWN_ATTEMPTS: u32 = 4;

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

/// The kind the `seed`'s spawn is: bats a little oftener than each monster.
pub fn pick_kind(seed: u64) -> &'static str {
    match mix(seed ^ 0xAB55_5EED) % 10 {
        0..=3 => SPAWN_KINDS[0],
        4..=6 => SPAWN_KINDS[1],
        7..=8 => SPAWN_KINDS[2],
        _ => SPAWN_KINDS[3],
    }
}

/// Whether a mob of the mod's kinds is near enough to count against the cap:
/// within [`MOB_RANGE`] blocks in x and z, and under the bedrock floor.
pub fn counts_towards_cap(kind: &str, pos: [f64; 3], player: [f64; 3]) -> bool {
    SPAWN_KINDS.contains(&kind)
        && pos[1] < SPAWN_BELOW_Y
        && (pos[0] - player[0]).abs() <= MOB_RANGE
        && (pos[2] - player[2]).abs() <= MOB_RANGE
}

/// One look for a place to spawn: `near` mobs of the mod's are within reach
/// already, and `air` says whether a cell is open (the caller reads its
/// world). The first attempt with air at the feet and the head, and a floor
/// under the feet, wins: its kind and spot. None when the cap is full or no
/// attempt finds a pocket.
pub fn spawn_plan(
    seed: u64,
    at: [f64; 3],
    near: usize,
    air: impl Fn([i32; 3]) -> bool,
) -> Option<(&'static str, [f64; 3])> {
    if near >= MOB_CAP {
        return None;
    }
    for attempt in 0..SPAWN_ATTEMPTS {
        let [dx, dy, dz] = candidate_offset(seed, attempt);
        let spot = [at[0] + dx as f64, (at[1] + dy as f64).floor(), at[2] + dz as f64];
        let feet = [spot[0] as i32, spot[1] as i32, spot[2] as i32];
        // A pocket: air where the mob would stand, air over its head, and a
        // floor under it that is not the world's own bottom air.
        if !(air(feet) && air([feet[0], feet[1] + 1, feet[2]]) && !air([feet[0], feet[1] - 1, feet[2]])) {
            continue;
        }
        return Some((pick_kind(seed.wrapping_add(attempt as u64)), spot));
    }
    None
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
        assert_eq!(spawn_plan(7, at, MOB_CAP, pocket), None);
        // With room and a floor: the first attempt's kind and spot.
        let (kind, spot) = spawn_plan(7, at, MOB_CAP - 1, pocket).expect("a plan");
        assert_eq!(kind, pick_kind(7));
        let [dx, dy, dz] = candidate_offset(7, 0);
        assert_eq!(
            spot,
            [at[0] + dx as f64, (at[1] + dy as f64).floor(), at[2] + dz as f64]
        );
    }

    #[test]
    fn spawned_kinds_are_the_mods() {
        for seed in 0..40u64 {
            assert!(SPAWN_KINDS.contains(&pick_kind(seed)));
        }
    }

    #[test]
    fn the_cap_counts_the_mods_mobs_near_the_player() {
        let player = [0.0, -200.0, 0.0];
        assert!(counts_towards_cap(
            "minecraft:bat",
            [10.0, -210.0, 0.0],
            player
        ));
        assert!(counts_towards_cap(
            "minecraft:zombie",
            [-30.0, -90.0, 20.0],
            player
        ));
        // Too far, too high, or not the mod's kind: not counted.
        assert!(!counts_towards_cap(
            "minecraft:bat",
            [60.0, -210.0, 0.0],
            player
        ));
        assert!(!counts_towards_cap(
            "minecraft:bat",
            [10.0, -70.0, 0.0],
            player
        ));
        assert!(!counts_towards_cap(
            "minecraft:cow",
            [10.0, -210.0, 0.0],
            player
        ));
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
