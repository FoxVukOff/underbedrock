//! Compiles the Under Bedrock native mod's own sources against a stub of
//! Horizon's game API and runs them: `stubcheck on` exercises the world the
//! datapack makes with the registry's air at state 0, `stubcheck off` a short
//! world (so the mod says the datapack is missing) with the bulk-edit fallback.
//! Both run the same scenarios, so each path is covered twice.
#![allow(dead_code)]

extern crate self as horizon_client;
extern crate self as horizon_world;

/// `horizon_client::text` (`crates/horizon-client-core/src/text.rs`).
pub mod text {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Text(String);

    impl Text {
        pub fn literal(s: impl Into<String>) -> Self {
            Self(s.into())
        }
        pub fn with_color(self, _color: u32) -> Self {
            self
        }
        pub fn plain(&self) -> String {
            self.0.clone()
        }
    }

    pub const GOLD: u32 = 0xFFAA00;
}

/// `horizon_world::registry` (the state ids `native_set_block` takes).
pub mod registry {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BlockStateId(pub u32);
}

/// `horizon_world::nbt` (the compounds `native_summon` takes): the shapes the
/// mod's sources touch, with the real names.
pub mod nbt {
    pub type JavaString = String;
    pub type Compound = std::collections::BTreeMap<JavaString, Tag>;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Tag {
        Int(i32),
        String(JavaString),
        Compound(Compound),
    }

    impl Tag {
        pub fn compound(entries: impl IntoIterator<Item = (impl Into<JavaString>, Tag)>) -> Self {
            Self::Compound(entries.into_iter().map(|(k, v)| (k.into(), v)).collect())
        }
        pub fn get(&self, key: &str) -> Option<&Tag> {
            match self {
                Self::Compound(map) => map.get(key),
                _ => None,
            }
        }
    }
}

pub mod game;

use game::Game;

macro_rules! check {
    ($fails:expr, $what:expr, $ok:expr) => {
        if $ok {
            println!("  ok    {}", $what);
        } else {
            println!("  FAIL  {}", $what);
            $fails += 1;
        }
    };
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "on".into());
    let tall = mode != "off";
    let air_at_zero = tall;
    // The dig time and the drop the run starts with, and what they become
    // after the settings page's first button and `/abyss dig`.
    let (first_dig, second_dig, third_dig) = if tall { (60, 100, 200) } else { (20, 40, 60) };
    let first_drop = tall;
    let mut fails = 0u32;
    println!("stubcheck {mode}: dig {first_dig} ticks, bedrock drops {first_drop}, air at state 0 {air_at_zero}");

    let mut game = Game::new();
    game.shape = if tall { (-576, 896) } else { (-64, 384) };
    game.air_at_zero = air_at_zero;
    if !tall {
        game.settings.borrow_mut().insert("underbedrock_core/dig_ticks".into(), "20".into());
        game.settings.borrow_mut().insert("underbedrock_core/drop_bedrock".into(), "false".into());
    }
    let bedrock: [i32; 3] = [3, -64, -7];
    let stone: [i32; 3] = [4, -64, -7];
    game.blocks.insert(bedrock, "minecraft:bedrock".into());
    game.blocks.insert(stone, "minecraft:stone".into());

    // The mod's row and its commands.
    let mut mods = Vec::new();
    game.native_mods(&mut mods);
    check!(fails, "one row in Options > Mods", mods.len() == 1);
    check!(
        fails,
        "the row is the mod's",
        mods.first().is_some_and(|m| m.id == "underbedrock_core"
            && m.title == "Under Bedrock 0.0.1"
            && m.description.contains("void-alloy pickaxe")
            && m.keys.is_empty())
    );
    let mut commands = Vec::new();
    game.native_register_commands(&mut commands);
    let usages: Vec<&str> = commands.iter().map(|c| c.usage.as_str()).collect();
    check!(
        fails,
        "five usages of /abyss, every player's",
        usages == ["abyss", "abyss give", "abyss depth", "abyss dig", "abyss drop"]
            && commands.iter().all(|c| c.level == 0)
    );

    // The settings page reads the mod's files.
    let mut buttons = Vec::new();
    game.native_settings("another_mod", &mut buttons);
    check!(fails, "another mod's page stays empty", buttons.is_empty());
    game.native_settings("underbedrock_core", &mut buttons);
    check!(
        fails,
        "the page shows the dig time and the drop",
        buttons == [format!("Break time: {:.1} s (bedrock, voidshell)", first_dig as f32 / 20.0),
                    format!("Bedrock drops: {}", if first_drop { "ON" } else { "OFF" })]
    );

    // A tick with nothing going on: the world's shape is looked at once.
    game.native_player_tick();
    check!(
        fails,
        if tall { "a tall world needs no warning" } else { "a short world is reported once" },
        game.told.len() == usize::from(!tall)
            && game.told.first().is_none_or(|(_, line)| line.contains("NOT tall"))
    );
    game.native_player_tick();
    check!(fails, "and not reported again", game.told.len() == usize::from(!tall));
    check!(fails, "nothing broke", game.block_at(bedrock) == "minecraft:bedrock");

    // Digging without the pickaxe, and a block that isn't bedrock.
    game.held = None;
    game.start_dig(bedrock, first_dig * 4);
    game.native_player_tick();
    check!(fails, "an empty hand digs no bedrock", game.block_at(bedrock) == "minecraft:bedrock");
    game.held = Some(("underbedrock:void_alloy_pickaxe".into(), 1, true));
    game.start_dig(stone, first_dig * 4);
    game.native_player_tick();
    check!(fails, "stone is the engine's to break", game.block_at(stone) == "minecraft:stone");
    check!(fails, "and no progress line shows for it", game.bars.is_empty());

    // Half way through the dig of bedrock.
    game.start_dig(bedrock, first_dig / 2);
    game.native_player_tick();
    check!(
        fails,
        "the action bar shows the dig's progress",
        game.bars.last().is_some_and(|(_, line)| line.contains("50%") && line.starts_with("Bedrock ["))
    );
    check!(fails, "the bedrock is still there", game.block_at(bedrock) == "minecraft:bedrock");
    check!(fails, "and nothing dropped yet", game.given.is_empty());

    // A tick short of the dig time, then the dig time itself.
    game.start_dig(bedrock, first_dig - 1);
    game.native_player_tick();
    check!(fails, "a tick short, it holds", game.block_at(bedrock) == "minecraft:bedrock");
    game.start_dig(bedrock, first_dig);
    game.native_player_tick();
    check!(fails, "at the dig time the bedrock goes", game.block_at(bedrock) == "minecraft:air");
    check!(
        fails,
        "the progress line is cleared",
        game.bars.last().is_some_and(|(_, line)| line.is_empty())
    );
    check!(
        fails,
        if first_drop { "the bedrock drops" } else { "the drop is off" },
        game.given.len() == usize::from(first_drop)
            && game.given.first().is_none_or(|(item, count)| item == "minecraft:bedrock" && *count == 1)
    );
    check!(
        fails,
        if air_at_zero { "air went in by its state id" } else { "a bulk edit named the blocks" },
        game.edits.is_empty() == air_at_zero
            && game.last_edit().is_none_or(|(palette, filters, runs)| {
                palette == ["minecraft:air".to_owned()]
                    && filters == ["minecraft:bedrock".to_owned()]
                    && runs == 1
            })
    );

    // Holding on the hole it left does nothing more.
    game.start_dig(bedrock, first_dig * 4);
    game.native_player_tick();
    check!(fails, "the hole stays a hole", game.block_at(bedrock) == "minecraft:air");
    check!(fails, "and nothing drops twice", game.given.len() == usize::from(first_drop));

    // Letting go clears the line once.
    game.blocks.insert(bedrock, "minecraft:bedrock".into());
    game.start_dig(bedrock, first_dig / 2);
    game.native_player_tick();
    game.stop_dig();
    let bars = game.bars.len();
    game.native_player_tick();
    game.native_player_tick();
    check!(
        fails,
        "letting go clears the line once",
        game.bars.len() == bars + 1 && game.bars.last().is_some_and(|(_, line)| line.is_empty())
    );

    // The settings page's buttons: the next dig time, and the drop flipped.
    let mut buttons = Vec::new();
    game.native_settings_click("another_mod", 0);
    game.native_settings_click("underbedrock_core", 0);
    game.native_settings_click("underbedrock_core", 1);
    game.native_settings_click("underbedrock_core", 9);
    game.native_settings("underbedrock_core", &mut buttons);
    check!(
        fails,
        "a click cycles the dig time and flips the drop",
        buttons == [format!("Break time: {:.1} s (bedrock, voidshell)", second_dig as f32 / 20.0),
                    format!("Bedrock drops: {}", if first_drop { "OFF" } else { "ON" })]
            && game.setting_file("underbedrock_core", "dig_ticks").as_deref()
                == Some(second_dig.to_string().as_str())
            && game.setting_file("underbedrock_core", "drop_bedrock").as_deref()
                == Some(if first_drop { "false" } else { "true" })
    );
    // The new dig time is the one the dig counts against, at once, and the
    // flipped drop flag is the one the break obeys.
    game.start_dig(bedrock, first_dig);
    game.native_player_tick();
    check!(fails, "the old dig time no longer breaks it", game.block_at(bedrock) == "minecraft:bedrock");
    let given_before = game.given.len();
    game.start_dig(bedrock, second_dig);
    game.native_player_tick();
    check!(fails, "the new one does", game.block_at(bedrock) == "minecraft:air");
    check!(
        fails,
        if first_drop { "and the drop it flipped off stays off" } else { "and the drop it flipped on gives one" },
        game.given.len() == given_before + usize::from(!first_drop)
    );

    // /abyss, and a command that isn't the mod's.
    let player = game.acting.clone().expect("a player");
    let feedback = |game: &Game, from: usize| game.feedback[from..].join(" | ");
    let mut ran = false;
    game.native_perform_command(&player, "give", &mut ran);
    check!(fails, "another mod's command is left alone", !ran && game.feedback.is_empty());
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss", &mut ran);
    check!(fails, "/abyss lists the commands", ran && feedback(&game, mark).contains("/abyss give"));
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss give", &mut ran);
    check!(
        fails,
        "/abyss give gives the pickaxe",
        ran && game.given.last().is_some_and(|(item, count)| item == "underbedrock:void_alloy_pickaxe" && *count == 1)
            && feedback(&game, mark).contains("void-alloy pickaxe")
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss depth", &mut ran);
    let depth = feedback(&game, mark);
    check!(
        fails,
        "/abyss depth says how deep the world goes",
        ran && depth.contains(&format!("y {} .. {}", game.shape.0, game.shape.0 + game.shape.1 - 1))
            && depth.contains(if tall { "below the bedrock floor" } else { "NOT tall" })
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss dig", &mut ran);
    check!(
        fails,
        "/abyss dig cycles the dig time",
        ran && feedback(&game, mark).contains(&format!("{:.1} s", third_dig as f32 / 20.0))
            && game.setting_file("underbedrock_core", "dig_ticks").as_deref()
                == Some(third_dig.to_string().as_str())
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss drop", &mut ran);
    check!(
        fails,
        "/abyss drop flips the drop back",
        ran && feedback(&game, mark).contains(if first_drop { "ON" } else { "OFF" })
            && game.setting_file("underbedrock_core", "drop_bedrock").as_deref()
                == Some(if first_drop { "true" } else { "false" })
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss fly", &mut ran);
    check!(fails, "an unknown subcommand says so", ran && feedback(&game, mark).contains("no /abyss fly"));

    // The Abyss keeps its own company, with the mod's OWN creatures: deep
    // enough, with a pocket to hang in, the mod summons one of its displays;
    // at the cap, or up top, it doesn't. No vanilla mob is ever spawned.
    use game::underbedrock_core as m;
    let deep: [f64; 3] = [0.5, -200.0, 0.5];
    game.acting.as_mut().expect("a player").position = deep;
    for _ in 0..60 {
        game.native_player_tick();
    }
    check!(fails, "no pocket in the rock, no creature", game.summoned.is_empty());
    let seed = 7u64 ^ (deep[0] as i64 as u64).wrapping_mul(0x2545_F491);
    let [dx, dy, dz] = m::candidate_offset(seed, 0);
    // The same cells the mod's tick looks at: the offset added to the
    // player's fractional position, then truncated.
    let feet = [
        (deep[0] + dx as f64) as i32,
        (deep[1] + dy as f64).floor() as i32,
        (deep[2] + dz as f64) as i32,
    ];
    game.blocks.insert([feet[0], feet[1] - 1, feet[2]], "underbedrock:voidstone".into());
    for _ in 0..60 {
        game.native_player_tick();
    }
    let creature = m::pick_creature(seed);
    check!(
        fails,
        "a pocket in the Abyss gets one of the mod's own creatures",
        game.summoned.len() == 1
            && game.summoned[0].0 == creature.display()
            && game.summoned[0].2 == creature.shows()
            && game.others.len() == 1
    );
    // The creature is the mod's to drive: every deep tick moves it, and its
    // pose (with its own scale) goes out on the mod's rhythm.
    let (moves, poses) = (game.moved.len(), game.posed.len());
    game.native_player_tick();
    check!(
        fails,
        "the mod moves its creature, and poses it at its own scale",
        game.moved.len() == moves + 1
            && game.posed.len() >= poses
            && game.posed.first().is_none_or(|(_, scale)| scale[0] == creature.scale())
    );
    // At the cap the mod stops summoning.
    while game.others.len() < m::CREATURE_CAP {
        let n = game.others.len() as u32;
        game.others.push(game::OtherEntity {
            id: 100 + n,
            kind: m::ITEM_DISPLAY.into(),
            position: game::Vec3 { x: deep[0] + 5.0, y: -210.0, z: deep[2] },
            yaw: 0.0,
            pitch: 0.0,
        });
    }
    let summoned = game.summoned.len();
    for _ in 0..60 {
        game.native_player_tick();
    }
    check!(fails, "at the cap the Abyss stays as it is", game.summoned.len() == summoned);
    // A player who walks away lets the mod's creatures go: every one of them,
    // summoned or adopted, is out of the engine's list on the next deep tick.
    game.acting.as_mut().expect("a player").position = [500.5, -200.0, 0.5];
    game.native_player_tick();
    check!(
        fails,
        "walking away lets every creature go",
        game.others.is_empty() && game.removed_entities.len() == m::CREATURE_CAP
    );
    // What a world saved and a restart lost: the engine has a display the
    // mod's list doesn't know; the next deep tick adopts it and drives it.
    game.acting.as_mut().expect("a player").position = deep;
    game.others.push(game::OtherEntity {
        id: 999,
        kind: m::BLOCK_DISPLAY.into(),
        position: game::Vec3 { x: deep[0] + 3.0, y: -201.0, z: deep[2] },
        yaw: 0.0,
        pitch: 0.0,
    });
    game.moved.clear();
    game.native_player_tick();
    check!(
        fails,
        "a display a world saved is adopted and driven",
        game.moved.iter().any(|(id, _)| *id == 999)
    );
    // Back at the surface: no summons, and the engine's own spawning is the
    // only one.
    game.acting.as_mut().expect("a player").position = [0.5, -60.0, 0.5];
    let summoned = game.summoned.len();
    let moves = game.moved.len();
    for _ in 0..60 {
        game.native_player_tick();
    }
    check!(
        fails,
        "the surface is the engine's to populate",
        game.summoned.len() == summoned && game.moved.len() == moves
    );

    // A disabled mod does nothing at all.
    game.disabled.push("underbedrock_core".into());
    game.blocks.insert(bedrock, "minecraft:bedrock".into());
    game.start_dig(bedrock, third_dig * 2);
    let (bars, told, feedbacks) = (game.bars.len(), game.told.len(), game.feedback.len());
    game.native_player_tick();
    let mut ran = false;
    game.native_perform_command(&player, "abyss give", &mut ran);
    check!(
        fails,
        "a disabled mod digs nothing and runs no command",
        !ran && game.block_at(bedrock) == "minecraft:bedrock"
            && game.bars.len() == bars && game.told.len() == told && game.feedback.len() == feedbacks
    );

    println!("{}", if fails == 0 { "all ok" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
