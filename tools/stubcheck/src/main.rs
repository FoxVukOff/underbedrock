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

/// `horizon_client::items` (the stacks `native_loot_drops` hands out).
pub mod items {
    /// One stack (`items.rs`'s `ItemStack`); the real one also carries
    /// `damage` and `extra`, which the mod never touches.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ItemStack {
        pub item: std::sync::Arc<str>,
        pub count: u32,
    }

    impl ItemStack {
        pub fn new(item: &str, count: u32) -> Self {
            Self {
                item: item.into(),
                count,
            }
        }
    }
}

/// `horizon_client::mod_api` (the context the `loot_drops` hook hears).
pub mod mod_api {
    /// What dropped the items `native_loot_drops` hears (`mod_api.rs`'s
    /// `NativeLootDrops`).
    #[derive(Clone, Copy, Debug)]
    pub struct NativeLootDrops<'a> {
        pub table: &'a str,
        pub dimension: &'a str,
        pub origin: [f64; 3],
        pub block: Option<([i32; 3], crate::registry::BlockStateId)>,
        pub entity: Option<(u32, &'a str)>,
        pub player: Option<u128>,
        pub tool: Option<&'a crate::items::ItemStack>,
        pub explosion: Option<f32>,
        pub used: bool,
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
    use game::underbedrock_core as m;

    let mode = std::env::args().nth(1).unwrap_or_else(|| "on".into());
    let tall = mode != "off";
    let air_at_zero = tall;
    // The dig time and the drop the run starts with, and what they become
    // after the settings page's first button and `/abyss dig`.
    let (first_dig, second_dig, third_dig) = if tall { (60, 100, 200) } else { (20, 40, 60) };
    let first_drop = tall;
    let mut fails = 0u32;
    println!(
        "stubcheck {mode}: dig {first_dig} ticks, bedrock drops {first_drop}, air at state 0 {air_at_zero}"
    );

    let mut game = Game::new();
    game.shape = if tall { (-1008, 1328) } else { (-64, 384) };
    game.air_at_zero = air_at_zero;
    if !tall {
        game.settings
            .borrow_mut()
            .insert("underbedrock_core/dig_ticks".into(), "20".into());
        game.settings
            .borrow_mut()
            .insert("underbedrock_core/drop_bedrock".into(), "false".into());
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
        usages
            == [
                "abyss",
                "abyss give",
                "abyss depth",
                "abyss dig",
                "abyss drop"
            ]
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
        buttons
            == [
                format!(
                    "Break time: {:.1} s (bedrock, voidshell)",
                    first_dig as f32 / 20.0
                ),
                format!("Bedrock drops: {}", if first_drop { "ON" } else { "OFF" })
            ]
    );

    // A tick with nothing going on: the world's shape is looked at once.
    game.native_player_tick();
    check!(
        fails,
        if tall {
            "a tall world needs no warning"
        } else {
            "a short world is reported once"
        },
        game.told.len() == usize::from(!tall)
            && game
                .told
                .first()
                .is_none_or(|(_, line)| line.contains("NOT tall"))
    );
    game.native_player_tick();
    check!(
        fails,
        "and not reported again",
        game.told.len() == usize::from(!tall)
    );
    check!(
        fails,
        "nothing broke",
        game.block_at(bedrock) == "minecraft:bedrock"
    );

    // Digging without the pickaxe, and a block that isn't bedrock.
    game.held = None;
    game.start_dig(bedrock, first_dig * 4);
    game.native_player_tick();
    check!(
        fails,
        "an empty hand digs no bedrock",
        game.block_at(bedrock) == "minecraft:bedrock"
    );
    game.held = Some(("underbedrock:void_alloy_pickaxe".into(), 1, true));
    game.start_dig(stone, first_dig * 4);
    game.native_player_tick();
    check!(
        fails,
        "stone is the engine's to break",
        game.block_at(stone) == "minecraft:stone"
    );
    check!(
        fails,
        "and no progress line shows for it",
        game.bars.is_empty()
    );

    // Half way through the dig of bedrock.
    game.start_dig(bedrock, first_dig / 2);
    game.native_player_tick();
    check!(
        fails,
        "the action bar shows the dig's progress",
        game.bars
            .last()
            .is_some_and(|(_, line)| line.contains("50%") && line.starts_with("Bedrock ["))
    );
    check!(
        fails,
        "the bedrock is still there",
        game.block_at(bedrock) == "minecraft:bedrock"
    );
    check!(fails, "and nothing dropped yet", game.given.is_empty());

    // A tick short of the dig time, then the dig time itself.
    game.start_dig(bedrock, first_dig - 1);
    game.native_player_tick();
    check!(
        fails,
        "a tick short, it holds",
        game.block_at(bedrock) == "minecraft:bedrock"
    );
    game.start_dig(bedrock, first_dig);
    game.native_player_tick();
    check!(
        fails,
        "at the dig time the bedrock goes",
        game.block_at(bedrock) == "minecraft:air"
    );
    check!(
        fails,
        "the progress line is cleared",
        game.bars.last().is_some_and(|(_, line)| line.is_empty())
    );
    check!(
        fails,
        if first_drop {
            "the bedrock drops"
        } else {
            "the drop is off"
        },
        game.given.len() == usize::from(first_drop)
            && game
                .given
                .first()
                .is_none_or(|(item, count)| item == "minecraft:bedrock" && *count == 1)
    );
    check!(
        fails,
        if air_at_zero {
            "air went in by its state id"
        } else {
            "a bulk edit named the blocks"
        },
        game.edits.is_empty() == air_at_zero
            && game.last_edit().is_none_or(|(palette, filters, runs)| {
                palette == ["minecraft:air".to_owned()]
                    && filters == ["minecraft:bedrock".to_owned()]
                    && runs == 1
            })
    );

    // The break's feedback (the nightly's net bridge, API 40): the server
    // told the digging player's client what broke, and the client half hears
    // it and plays the sound and the blue sparks there.
    check!(
        fails,
        "the break was told to the player's client",
        game.net_sent.len() == 1
            && game.net_sent[0].0 == 7
            && game.net_sent[0].1 == m::DUG_CHANNEL
            && m::decode_dig(&game.net_sent[0].2) == Some((bedrock, m::Dig::Bedrock))
    );
    let (channel, data) = (game.net_sent[0].1.clone(), game.net_sent[0].2.clone());
    game.native_net_from_server(&channel, &data);
    let centre = [
        bedrock[0] as f64 + 0.5,
        bedrock[1] as f64 + 0.5,
        bedrock[2] as f64 + 0.5,
    ];
    check!(
        fails,
        "and the client plays the sound and the sparks",
        game.sounds.len() == 1
            && game.sounds[0].0 == m::DIG_SOUND
            && game.sounds[0].1 == centre
            && game.sounds[0].2 == m::DIG_VOLUME
            && game.sounds[0].3 == m::DIG_PITCH
            && game.sounds[0].4 == 0
            && game.particles.len() == m::DIG_BURST.len()
            && game.particles.iter().all(|(_, soul)| *soul)
    );
    // Another channel's message, and torn bytes on the mod's, change nothing.
    game.native_net_from_server("other_mod:hello", &data);
    game.native_net_from_server(m::DUG_CHANNEL, &[1, 2, 3]);
    check!(
        fails,
        "an unknown channel or torn bytes play nothing",
        game.sounds.len() == 1 && game.particles.len() == m::DIG_BURST.len()
    );

    // Holding on the hole it left does nothing more.
    game.start_dig(bedrock, first_dig * 4);
    game.native_player_tick();
    check!(
        fails,
        "the hole stays a hole",
        game.block_at(bedrock) == "minecraft:air"
    );
    check!(
        fails,
        "and nothing drops twice",
        game.given.len() == usize::from(first_drop)
    );

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
        buttons
            == [
                format!(
                    "Break time: {:.1} s (bedrock, voidshell)",
                    second_dig as f32 / 20.0
                ),
                format!("Bedrock drops: {}", if first_drop { "OFF" } else { "ON" })
            ]
            && game
                .setting_file("underbedrock_core", "dig_ticks")
                .as_deref()
                == Some(second_dig.to_string().as_str())
            && game
                .setting_file("underbedrock_core", "drop_bedrock")
                .as_deref()
                == Some(if first_drop { "false" } else { "true" })
    );
    // The new dig time is the one the dig counts against, at once, and the
    // flipped drop flag is the one the break obeys.
    game.start_dig(bedrock, first_dig);
    game.native_player_tick();
    check!(
        fails,
        "the old dig time no longer breaks it",
        game.block_at(bedrock) == "minecraft:bedrock"
    );
    let given_before = game.given.len();
    game.start_dig(bedrock, second_dig);
    game.native_player_tick();
    check!(
        fails,
        "the new one does",
        game.block_at(bedrock) == "minecraft:air"
    );
    check!(
        fails,
        if first_drop {
            "and the drop it flipped off stays off"
        } else {
            "and the drop it flipped on gives one"
        },
        game.given.len() == given_before + usize::from(!first_drop)
    );

    // /abyss, and a command that isn't the mod's.
    let player = game.acting.clone().expect("a player");
    let feedback = |game: &Game, from: usize| game.feedback[from..].join(" | ");
    let mut ran = false;
    game.native_perform_command(&player, "give", &mut ran);
    check!(
        fails,
        "another mod's command is left alone",
        !ran && game.feedback.is_empty()
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss", &mut ran);
    check!(
        fails,
        "/abyss lists the commands",
        ran && feedback(&game, mark).contains("/abyss give")
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss give", &mut ran);
    check!(
        fails,
        "/abyss give gives the pickaxe",
        ran && game
            .given
            .last()
            .is_some_and(|(item, count)| item == "underbedrock:void_alloy_pickaxe" && *count == 1)
            && feedback(&game, mark).contains("void-alloy pickaxe")
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss depth", &mut ran);
    let depth = feedback(&game, mark);
    check!(
        fails,
        "/abyss depth says how deep the world goes",
        ran && depth.contains(&format!(
            "y {} .. {}",
            game.shape.0,
            game.shape.0 + game.shape.1 - 1
        )) && depth.contains(if tall {
            "below the bedrock floor"
        } else {
            "NOT tall"
        })
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss dig", &mut ran);
    check!(
        fails,
        "/abyss dig cycles the dig time",
        ran && feedback(&game, mark).contains(&format!("{:.1} s", third_dig as f32 / 20.0))
            && game
                .setting_file("underbedrock_core", "dig_ticks")
                .as_deref()
                == Some(third_dig.to_string().as_str())
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss drop", &mut ran);
    check!(
        fails,
        "/abyss drop flips the drop back",
        ran && feedback(&game, mark).contains(if first_drop { "ON" } else { "OFF" })
            && game
                .setting_file("underbedrock_core", "drop_bedrock")
                .as_deref()
                == Some(if first_drop { "true" } else { "false" })
    );
    let mark = game.feedback.len();
    game.native_perform_command(&player, "abyss fly", &mut ran);
    check!(
        fails,
        "an unknown subcommand says so",
        ran && feedback(&game, mark).contains("no /abyss fly")
    );

    // A bridge that refuses the message is not silent: the break still
    // happens, and the player is told why nothing played.
    game.blocks.insert(bedrock, "minecraft:bedrock".into());
    game.net_broken = true;
    let (told, sent) = (game.told.len(), game.net_sent.len());
    game.start_dig(bedrock, third_dig);
    game.native_player_tick();
    game.net_broken = false;
    check!(
        fails,
        "a refused feedback tells the player instead of vanishing",
        game.block_at(bedrock) == "minecraft:air"
            && game.told.len() == told + 1
            && game.net_sent.len() == sent
    );

    // The Abyss keeps its own company, with the mod's OWN creatures: deep
    // enough, with a pocket to hang in, the mod summons one of its displays;
    // at the cap, or up top, it doesn't. No vanilla mob is ever spawned.
    let deep: [f64; 3] = [0.5, -400.0, 0.5];
    game.acting.as_mut().expect("a player").position = deep;
    for _ in 0..60 {
        game.native_player_tick();
    }
    check!(
        fails,
        "no pocket in the rock, no creature",
        game.summoned.is_empty()
    );
    // The descent itself: the gate's crossing went to the player's client
    // over the veil channel - and the window's half, handed the very same
    // message, plays the ring of blue sparks at the boundary's own height.
    let veils = |game: &Game| {
        game.net_sent
            .iter()
            .filter(|(_, channel, _)| channel == m::VEIL_CHANNEL)
            .count()
    };
    check!(
        fails,
        "the crossing of the gate is told to the player's client",
        veils(&game) == 1
            && game.net_sent.last().is_some_and(|(who, channel, data)| {
                *who == 7
                    && channel == m::VEIL_CHANNEL
                    && m::decode_veil(data) == Some((0, [0, 0]))
            })
    );
    let sparks = game.particles.len();
    let (vchannel, vdata) = game
        .net_sent
        .last()
        .map(|(_, channel, data)| (channel.clone(), data.clone()))
        .expect("the veil's message");
    game.native_net_from_server(&vchannel, &vdata);
    check!(
        fails,
        "and the client plays the veil's ring at the boundary itself",
        game.particles.len() == sparks + m::VEIL_SPARKS
            && game.particles[sparks..].iter().all(|(at, soul)| {
                *soul
                    && at[1] == m::VEIL_BOUNDS[0] as f64 + m::VEIL_LIFT
                    && (((at[0] - deep[0].floor()).powi(2)
                        + (at[2] - deep[2].floor()).powi(2))
                        .sqrt()
                        - m::VEIL_RING)
                        .abs()
                        < 1.0e-9
            })
    );
    // Torn bytes on the veil channel play nothing, as on the dig's.
    game.native_net_from_server(m::VEIL_CHANNEL, &[9, 9]);
    check!(
        fails,
        "torn veil bytes play nothing",
        game.particles.len() == sparks + m::VEIL_SPARKS
    );
    let seed = 7u64 ^ (deep[0] as i64 as u64).wrapping_mul(0x2545_F491);
    let [dx, dy, dz] = m::candidate_offset(seed, 0);
    // The same cells the mod's tick looks at: the offset added to the
    // player's fractional position, then truncated.
    let feet = [
        (deep[0] + dx as f64) as i32,
        (deep[1] + dy as f64).floor() as i32,
        (deep[2] + dz as f64) as i32,
    ];
    game.blocks.insert(
        [feet[0], feet[1] - 1, feet[2]],
        "underbedrock:voidstone".into(),
    );
    for _ in 0..60 {
        game.native_player_tick();
    }
    let creature = m::pick_creature(seed, deep[1]).expect("the calm deep keeps its gloomling");
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
            && game
                .posed
                .first()
                .is_none_or(|(_, scale)| scale[0] == creature.scale())
    );
    // At the cap the mod stops summoning.
    while game.others.len() < m::CREATURE_CAP {
        let n = game.others.len() as u32;
        game.others.push(game::OtherEntity {
            id: 100 + n,
            kind: m::ITEM_DISPLAY.into(),
            position: game::Vec3 {
                x: deep[0] + 5.0,
                y: deep[1] - 10.0,
                z: deep[2],
            },
            yaw: m::Creature::Wisp.yaw_code(),
            pitch: 0.0,
        });
    }
    let summoned = game.summoned.len();
    for _ in 0..60 {
        game.native_player_tick();
    }
    check!(
        fails,
        "at the cap the Abyss stays as it is",
        game.summoned.len() == summoned
    );
    // A player who walks away lets the mod's creatures go: every one of them,
    // summoned or adopted, is out of the engine's list on the next deep tick.
    game.acting.as_mut().expect("a player").position = [500.5, deep[1], 0.5];
    game.native_player_tick();
    check!(
        fails,
        "walking away lets every creature go",
        game.others.is_empty() && game.removed_entities.len() == m::CREATURE_CAP
    );
    // What a world saved and a restart lost: the engine has displays the
    // mod's list doesn't know. One wears the lantern's yaw code - the code
    // rides the display's saved NBT - and is adopted back and driven; the
    // other wears no code of the mod's (a /summon, another mod's) and is
    // let go, never driven.
    game.acting.as_mut().expect("a player").position = deep;
    game.others.push(game::OtherEntity {
        id: 999,
        kind: m::BLOCK_DISPLAY.into(),
        position: game::Vec3 {
            x: deep[0] + 3.0,
            y: deep[1] - 1.0,
            z: deep[2],
        },
        yaw: m::Creature::Lantern.yaw_code(),
        pitch: 0.0,
    });
    game.others.push(game::OtherEntity {
        id: 998,
        kind: m::ITEM_DISPLAY.into(),
        position: game::Vec3 {
            x: deep[0] + 4.0,
            y: deep[1] - 1.0,
            z: deep[2],
        },
        yaw: 33.0,
        pitch: 0.0,
    });
    game.moved.clear();
    game.native_player_tick();
    check!(
        fails,
        "a display a world saved is adopted by its yaw code and driven",
        game.moved.iter().any(|(id, _)| *id == 999)
            && m::creature_kind(999) == Some(m::Creature::Lantern)
    );
    check!(
        fails,
        "a display that wears no code of the mod's is let go, not driven",
        game.removed_entities.contains(&998) && !game.moved.iter().any(|(id, _)| *id == 998)
    );
    // The Abyss's law: no vanilla mob lives under the gate, and every layer
    // but the calm deep sends its own hunter after a player who descends
    // into it - an invisible, silent vex wearing an item display of the
    // layer's shard as its body. The absolute void sends the warden: the
    // same machine turned up - twice the health, a closer ring, quicker
    // back.
    let the_void: [f64; 3] = [0.5, -850.0, 0.5];
    game.acting.as_mut().expect("a player").position = the_void;
    // A zombie that wandered in under the gate goes, quietly.
    game.mobs.push(game::StubMob {
        id: 900,
        kind: "minecraft:zombie".into(),
        pos: [3.0, -100.0, 3.0],
        removed: false,
        spawn: Default::default(),
    });
    // A display a restart orphaned in the absolute void goes too: below the
    // frozen floor, nothing lives that the hunts and the ledger don't know.
    game.others.push(game::OtherEntity {
        id: 777,
        kind: m::ITEM_DISPLAY.into(),
        position: game::Vec3 {
            x: the_void[0] + 2.0,
            y: -851.0,
            z: the_void[2],
        },
        yaw: 0.0,
        pitch: 0.0,
    });
    let summoned = game.summoned.len();
    game.native_player_tick();
    check!(
        fails,
        "a vanilla mob under the gate is banned",
        game.removed_mobs.contains(&900)
    );
    check!(
        fails,
        "an orphaned display in the void is cleaned up, not adopted",
        game.removed_entities.contains(&777) && !game.moved.iter().any(|(id, _)| *id == 777)
    );
    check!(
        fails,
        "the fall to the void is told to the client as its own crossing",
        veils(&game) == 2
            && game.net_sent.last().is_some_and(|(_, channel, data)| {
                channel == m::VEIL_CHANNEL && m::decode_veil(data) == Some((3, [0, 0]))
            })
    );
    check!(
        fails,
        "the void sends its warden: invisible, silent, weightless",
        game.spawned_mobs.len() == 1
            && game.spawned_mobs[0].0 == m::HUNT_KIND
            && game.spawned_mobs[0].2.invisible
            && game.spawned_mobs[0].2.silent
            && game.spawned_mobs[0].2.no_gravity
            && game.spawned_mobs[0].2.health == Some(m::PESTS[4].health)
    );
    check!(
        fails,
        "and the body it wears: the stalker shard display",
        game.summoned.len() == summoned + 1
            && game.summoned[summoned].0 == m::ITEM_DISPLAY
            && game.summoned[summoned].2 == m::STALKER_ITEM
    );
    // The ban spares the warden itself, and its body rides the mob, faced
    // at the player, every tick of the hunt.
    let warden = game.spawned_mobs.first().map(|_| 500u32);
    let body = game
        .others
        .iter()
        .find(|other| other.kind == m::ITEM_DISPLAY)
        .map(|other| other.id);
    check!(
        fails,
        "the body is posed at the warden's own scale",
        body.is_some_and(|id| game
            .posed
            .iter()
            .any(|(posed, scale)| *posed == id && scale[0] == m::PESTS[4].scale))
    );
    game.moved.clear();
    game.native_player_tick();
    check!(
        fails,
        "the hunt keeps its warden (the ban spares it) and moves its body",
        warden.is_some_and(|id| !game.removed_mobs.contains(&id))
            && body.is_some_and(|id| game.moved.iter().any(|(moved, _)| *moved == id))
    );
    // The warden's death (API 75's loot hook): its shard drops as the
    // trophy, on the ground, with no player's state touched.
    let mut drops = Vec::new();
    game.native_loot_drops(
        &crate::mod_api::NativeLootDrops {
            table: "minecraft:entities/vex",
            dimension: "minecraft:overworld",
            origin: the_void,
            block: None,
            entity: Some((500, m::HUNT_KIND)),
            player: Some(7),
            tool: None,
            explosion: None,
            used: false,
        },
        &mut drops,
    );
    check!(
        fails,
        "the warden's death drops its shard",
        drops == vec![horizon_client::items::ItemStack::new(m::STALKER_ITEM, 1)]
    );
    // An evoker's vex up on the surface is none of the mod's.
    drops.clear();
    game.native_loot_drops(
        &crate::mod_api::NativeLootDrops {
            table: "minecraft:entities/vex",
            dimension: "minecraft:overworld",
            origin: [0.5, 70.0, 0.5],
            block: None,
            entity: Some((999, m::HUNT_KIND)),
            player: Some(7),
            tool: None,
            explosion: None,
            used: false,
        },
        &mut drops,
    );
    check!(
        fails,
        "a surface vex drops nothing of the mod's",
        drops.is_empty()
    );
    // Nor is an untracked one deep in the void: depth alone proves nothing,
    // a vanilla vex can chase a player down a dug shaft.
    drops.clear();
    game.native_loot_drops(
        &crate::mod_api::NativeLootDrops {
            table: "minecraft:entities/vex",
            dimension: "minecraft:overworld",
            origin: the_void,
            block: None,
            entity: Some((777, m::HUNT_KIND)),
            player: Some(7),
            tool: None,
            explosion: None,
            used: false,
        },
        &mut drops,
    );
    check!(
        fails,
        "an untracked vex in the void drops no trophy",
        drops.is_empty()
    );

    // The player climbs out: the warden and its body go at once, and the
    // layer they drop into starts its own hunt - the cinder's, weaker and
    // slower than the warden's.
    game.acting.as_mut().expect("a player").position = [0.5, -300.0, 0.5];
    game.native_player_tick();
    check!(
        fails,
        "climbing out of the void ends the hunt",
        warden.is_some_and(|id| game.removed_mobs.contains(&id))
            && body.is_some_and(|id| game.removed_entities.contains(&id))
    );
    check!(
        fails,
        "and the layer landed in sends its own, gentler pest",
        game.spawned_mobs.len() == 2
            && game.spawned_mobs[1].2.health == Some(m::PESTS[1].health)
            && game
                .summoned
                .last()
                .is_some_and(|(kind, _, shown)| kind == m::ITEM_DISPLAY
                    && shown == m::PESTS[1].body)
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

    // The calm deep's one tameable: a gloomling eats from a HELD void
    // alloy - every tick within reach of the hand trusts it a little more,
    // and at FEED_TICKS of them it is the holder's, following them
    // anywhere, up through the gate included. The alloy is held, not
    // eaten: the engine has no take-from-inventory API, and the ingot
    // stays in the hand.
    game.acting.as_mut().expect("a player").position = deep;
    game.others.push(game::OtherEntity {
        id: 995,
        kind: m::ITEM_DISPLAY.into(),
        position: game::Vec3 {
            x: deep[0] + 2.0,
            y: deep[1] - 1.0,
            z: deep[2],
        },
        yaw: m::Creature::Gloomling.yaw_code(),
        pitch: 0.0,
    });
    game.held = Some((m::ALLOY.into(), 1, false));
    let given = game.given.len();
    for _ in 0..m::FEED_TICKS {
        game.native_player_tick();
    }
    check!(
        fails,
        "the gloomling is adopted back by its code, and feeds from the hand",
        m::creature_kind(995) == Some(m::Creature::Gloomling)
    );
    game.native_player_tick();
    check!(
        fails,
        "a held alloy tames it, and the alloy stays in the hand",
        m::creature_kind(995) == Some(m::Creature::Gloom) && game.given.len() == given
    );
    check!(
        fails,
        "and its yaw now wears the fed code, for any restart to adopt",
        game
            .others
            .iter()
            .find(|other| other.id == 995)
            .is_some_and(|other| (other.yaw - m::Creature::Gloom.yaw_code()).abs() <= m::YAW_TOL)
    );
    // It follows its player out of the Abyss: driven at any depth, and
    // never let go - not at the surface, not from far away.
    game.held = None;
    game.acting.as_mut().expect("a player").position = [0.5, -60.0, 0.5];
    game.moved.clear();
    game.native_player_tick();
    check!(
        fails,
        "the fed one follows its player up through the gate",
        game.moved.iter().any(|(id, _)| *id == 995)
            && !game.removed_entities.contains(&995)
    );
    game.acting.as_mut().expect("a player").position = [500.5, -60.0, 0.5];
    game.moved.clear();
    game.native_player_tick();
    check!(
        fails,
        "and walking away does not let it go",
        game.moved.iter().any(|(id, _)| *id == 995)
            && !game.removed_entities.contains(&995)
    );

    // A world closed (API 134): the Overworld's close forgets everything the
    // mod kept for it, so a reopened world starts clean; another dimension's
    // close is none of the mod's business.
    m::clear_creatures();
    m::add_creature(21, m::Creature::Ember, [0.0, -200.0, 0.0], 7);
    let mut hunts = [m::Pest::default(); 5];
    hunts[4] = m::Pest {
        mob: Some(3),
        body: Some(4),
        quiet: 0,
    };
    m::set_pests(7, hunts);
    game.native_level_unload("minecraft:the_nether");
    check!(
        fails,
        "another dimension's close touches nothing",
        m::creatures_len() == 1
    );
    game.native_level_unload("minecraft:overworld");
    check!(
        fails,
        "the Overworld's close forgets the mod's state",
        m::creatures_len() == 0 && m::get_pests(7) == [m::Pest::default(); 5]
    );
    // The world reopens - in the same session or after a full game restart,
    // alike: the engine saved the ember's display with the code yaw every
    // move kept, hands it back when the chunk loads, and the first deep tick
    // adopts it back as the ember it was. The mod itself remembers nothing.
    game.acting.as_mut().expect("a player").position = [0.5, -200.0, 0.5];
    game.others.push(game::OtherEntity {
        id: 21,
        kind: m::ITEM_DISPLAY.into(),
        position: game::Vec3 {
            x: 0.0,
            y: -200.0,
            z: 0.0,
        },
        yaw: m::Creature::Ember.yaw_code(),
        pitch: 0.0,
    });
    game.native_player_tick();
    check!(
        fails,
        "a reopened world adopts its creatures back as their own kinds",
        m::creature_kind(21) == Some(m::Creature::Ember)
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
            && game.bars.len() == bars
            && game.told.len() == told
            && game.feedback.len() == feedbacks
    );

    println!("{}", if fails == 0 { "all ok" } else { "FAILURES" });
    std::process::exit(if fails == 0 { 0 } else { 1 });
}
