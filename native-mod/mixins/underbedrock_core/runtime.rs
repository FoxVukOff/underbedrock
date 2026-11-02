// Under Bedrock's native half: the etherium pickaxe digs the bedrock floor
// open, so the player gets into the world the mod's datapack makes below it.
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game (`crate::game::underbedrock_core`). It holds the
// mod's logic and state and no Horizon in it: the helpers beside it do the
// Game work and call in here.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "underbedrock_core";
pub const NAME: &str = "Under Bedrock";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "The etherium pickaxe digs bedrock: hold it on the bedrock floor and the block goes, into the world the mod's datapack makes below. /etherium lists the commands.";

/// The pickaxe that digs bedrock, the block it digs and the block it leaves.
pub const PICKAXE: &str = "underbedrock:etherium_pickaxe";
pub const BEDROCK: &str = "minecraft:bedrock";
pub const AIR: &str = "minecraft:air";

/// The Overworld the mod's datapack makes: 512 blocks below the vanilla
/// floor (`min_y` -576, 896 blocks of height).
pub const TALL_MIN_Y: i32 = -576;
pub const TALL_HEIGHT: i32 = 896;

/// The dig times the settings page and `/etherium dig` cycle through, in game
/// ticks (20 a second).
pub const DIG_TIMES: [u32; 5] = [20, 40, 60, 100, 200];
/// The dig time until the player picks one: three seconds.
pub const DEFAULT_DIG_TICKS: u32 = 60;

/// The action bar's progress bar, in characters.
const BAR_WIDTH: usize = 20;

/// `/etherium`'s answer.
pub const HELP: &str = "Under Bedrock: /etherium give (the pickaxe), /etherium depth (how deep this world goes), /etherium dig (the break time), /etherium drop (whether bedrock drops).";

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
    format!("Bedrock break time: {:.1} s", ticks as f32 / 20.0)
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
pub fn bar(progress: f32) -> String {
    let progress = progress.clamp(0.0, 1.0);
    let filled = (progress * BAR_WIDTH as f32).round() as usize;
    format!(
        "Bedrock [{}{}] {}%",
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
        format!("{shape} Under Bedrock's world is below the bedrock floor.")
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

/// Keeps a new dig time (the settings page or `/etherium dig` chose it).
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
        assert_eq!(dig_label(60), "Bedrock break time: 3.0 s");
        assert_eq!(dig_label(20), "Bedrock break time: 1.0 s");
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
        assert_eq!(bar(0.0), "Bedrock [--------------------] 0%");
        assert_eq!(bar(0.25), "Bedrock [#####---------------] 25%");
        assert_eq!(bar(1.0), "Bedrock [####################] 100%");
        assert_eq!(bar(1.4), bar(1.0));
        assert_eq!(bar(-0.2), bar(0.0));
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
