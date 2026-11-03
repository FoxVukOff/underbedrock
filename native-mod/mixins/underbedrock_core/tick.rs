// The dig itself, and the Abyss's own life - one function, as a mixin
// helper file must hold exactly one. The engine keeps a player's dig in its
// own state - the block the attack key is held on and the ticks it has been
// held for - swapped in for that player's tick, and cleared the moment they
// let go or look away. On bedrock or voidshell, with the void-alloy pickaxe
// in hand, the mod's dig time decides when the block goes; the action bar
// shows how far the dig is. The abyss bedrock at the world's bottom is left
// alone: nothing digs it.
//
// While a player is down in the Abyss, the mod also keeps it inhabited -
// with its OWN creatures, never a vanilla mob: display entities showing the
// mod's shards and crystal lamp, which the engine keeps without simulating
// and the mod drives from here. Every creature wears its kind as a yaw code,
// which the engine saves with the display's NBT and hands back on load: a
// world reloaded - or the game restarted - adopts its creatures back as the
// kinds they were, and a display that wears no code of the mod's is not the
// mod's to drive and is let go. Every tick the creatures follow the engine's
// list, take a step of their paths, and the ones their player walked away
// from are let go; on the mod's rhythm a look goes out for a dark pocket to
// summon the next one into, the layer at the player's depth picking the kind
// (embers in the lava zone, frostmotes in the frozen wastes). The calm
// deep keeps one kind alone - the gloomling - and a player who HOLDS a void
// alloy near one feeds it tick by tick until it trusts the hand and follows
// them anywhere, up through the gate included (the alloy is held, not
// eaten: the engine has no take-from-inventory API).
//
// Every layer but the calm deep also keeps a hunter of its own: an
// invisible, silent vex the engine's melee drives, wearing an item display
// of the layer's shard as its visible body. The layers' pests are
// annoyances; the warden of the absolute void is the same machine turned
// up. Above its layer a hunt is dismissed at once, and below the frozen
// floor nothing lives that the hunts and the ledger don't know. And the ban
// is real: every tick, any vanilla mob under the gate that isn't a tracked
// hunter is taken out of the world with no death and no loot. Where the
// layers meet, the crossing itself sparkles: a veil of end rods at the
// boundary's own height, the moment a player passes it.
fn underbedrock_core_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::underbedrock_core as m;
    use horizon_client::particles::{ParticleOptions, ParticleType};
    use horizon_client::text::Text;
    use horizon_world::nbt::Tag;
    if !game.native_on(m::ID) {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    let uuid = player.uuid;
    // A world the mod hasn't looked at yet: say how deep it goes, since only
    // the tall one has any room below its bedrock floor.
    let shape = game.native_world_height();
    if !m::shape_seen(shape) && !m::tall(shape.0, shape.1) {
        game.native_tell(uuid, Text::literal(m::depth_line(shape.0, shape.1)));
    }
    let at = player.position;
    let dimension = player.dimension.clone();
    // The transition veil: a boundary just crossed sparkles at its own
    // height around the player. The band watch runs in every dimension (a
    // trip through the Nether leaves no stale band behind), but the veil
    // plays only in the Overworld, where the Abyss's boundaries are.
    let crossed = m::veil_crossing(uuid, at[1]);
    if let (Some(bound), true) = (crossed, dimension == "minecraft:overworld") {
        let veil = ParticleOptions::simple(ParticleType::EndRod);
        game.native_send_particles(
            &veil,
            None,
            [
                at[0],
                m::VEIL_BOUNDS[bound] as f64 + m::VEIL_LIFT,
                at[2],
            ],
            m::VEIL_SPREAD,
            0.0,
            m::VEIL_COUNT,
        );
    }
    // The displays the engine keeps: the mod's creatures, as the engine has
    // them (owned copies, so the engine is free for the calls below), each
    // with the yaw it wears. A hunt's body is the hunt's own and stays out
    // of the list; a fed gloomling is on it at ANY depth - it follows its
    // player up through the gate - while the ambient kinds are the mod's to
    // drive only while their player is down in the Abyss.
    let deep = at[1] < m::SPAWN_BELOW_Y;
    let bodies = m::pest_bodies();
    let live: Vec<(u32, String, f32, [f64; 3])> = game
        .others
        .iter()
        .filter(|other| !bodies.contains(&other.id))
        .map(|other| {
            (
                other.id,
                other.kind.clone(),
                other.yaw,
                [other.position.x, other.position.y, other.position.z],
            )
        })
        .filter(|(_, kind, yaw, pos)| {
            m::is_tamed_code(kind, *yaw) || (deep && m::is_ours(kind, *pos))
        })
        .collect();
    let driving = deep
        || live
            .iter()
            .any(|(_, kind, yaw, _)| m::is_tamed_code(kind, *yaw));
    if driving {
        // A held alloy feeds: every gloomling within reach of the hand
        // trusts it a tick more, and at FEED_TICKS it is the holder's - the
        // kind flips before the step below, so the same tick already moves
        // and poses it as the fed one.
        if deep {
            if let Some((item, _, _)) = game.native_held() {
                if item == m::ALLOY {
                    for (id, _, _, pos) in &live {
                        let away = [pos[0] - at[0], pos[1] - at[1], pos[2] - at[2]];
                        if m::creature_kind(*id) == Some(m::Creature::Gloomling)
                            && (away[0] * away[0] + away[1] * away[1] + away[2] * away[2]).sqrt()
                                <= m::GLOOMLING_REACH
                        {
                            m::feed_creature(*id, uuid);
                        }
                    }
                }
            }
        }
        // One step of every creature: the list follows the engine, the paths
        // move (each with its kind's code yaw kept on), the poses go out on
        // the mod's rhythm, the far ones - and the displays that wear no code
        // of the mod's - are let go. A fed gloomling is never let go: it
        // eases to its player's side instead.
        let seen: Vec<(u32, &str, f32, [f64; 3])> = live
            .iter()
            .map(|(id, kind, yaw, pos)| (*id, kind.as_str(), *yaw, *pos))
            .collect();
        let step = m::step_creatures(at, uuid, &seen);
        for id in step.cull.into_iter().chain(step.purge) {
            game.native_remove_entity(&dimension, id);
        }
        for (id, kind, angle) in step.poses {
            let scale = kind.scale();
            let pose = crate::game::native_entities::NativeDisplay {
                translation: [0.0; 3],
                left_rotation: m::spin_y(angle),
                scale: [scale, scale, scale],
                right_rotation: [0.0, 0.0, 0.0, 1.0],
                interpolation: 10,
                teleport: 3,
            };
            game.native_set_display(&dimension, id, &pose);
        }
        for (id, pos, yaw) in step.moves {
            game.native_move_entity(&dimension, id, pos, yaw, 0.0);
        }
    }
    // The Abyss's law: no vanilla mob lives under the gate. Any that wanders
    // in is taken out of the world quietly (no death, no loot); the only mob
    // the ban spares is a hunter the mod tracks.
    let mobs = game.native_mobs(&dimension);
    for mob in &mobs {
        if mob.pos[1] < m::BAN_BELOW_Y && !m::is_tracked_mob(mob.id) {
            game.native_remove_mob(&dimension, mob.id);
        }
    }
    // The layers' hunts: every layer but the calm deep keeps one pest per
    // player down in it, dismissed the moment they climb out. The engine's
    // list of mobs is the truth - a hunter that fell (or a world that
    // reloaded without the pair) shows as gone, and the hunt reacts.
    let mut pests = m::get_pests(uuid);
    for (layer, cfg) in m::PESTS.iter().enumerate() {
        let in_zone = at[1] < cfg.top && at[1] > cfg.bot;
        let mob_at = pests[layer]
            .mob
            .and_then(|id| mobs.iter().find(|mob| mob.id == id).map(|mob| mob.pos));
        let (mut pest, order) = m::pest_tick(cfg, pests[layer], in_zone, mob_at, at);
        if let Some(id) = order.dismiss_mob {
            game.native_remove_mob(&dimension, id);
        }
        if let Some(id) = order.dismiss_body {
            game.native_remove_entity(&dimension, id);
        }
        // The hunter takes shape on its ring when the order says so.
        let mut body_at = mob_at.map(|pos| [pos[0], pos[1] + m::HUNT_LIFT, pos[2]]);
        if order.summon && pest.mob.is_none() {
            let seed = (uuid as u64 ^ (at[2] as i64 as u64).wrapping_mul(0x9E37_79B9))
                .wrapping_add((layer as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let spot = m::pest_spot(cfg, seed, at);
            let spawn = crate::game::native_mobs::NativeSpawn {
                health: Some(cfg.health),
                invisible: true,
                silent: true,
                no_gravity: true,
                ..Default::default()
            };
            if let Some(mob) = game.native_spawn_mob(&dimension, m::HUNT_KIND, spot, &spawn) {
                pest.mob = Some(mob);
                pest.quiet = 0;
                body_at = Some([spot[0], spot[1] + m::HUNT_LIFT, spot[2]]);
            }
        }
        // Its body: the layer's shard display, summoned once per hunt (and
        // again for a mob whose body summon failed) and posed at the
        // hunter's scale.
        if pest.mob.is_some() && pest.body.is_none() {
            if let Some(spot) = body_at {
                let mut nbt = horizon_world::nbt::Compound::new();
                nbt.insert(
                    "item".into(),
                    Tag::compound([
                        ("id", Tag::String(cfg.body.into())),
                        ("count", Tag::Int(1)),
                    ]),
                );
                nbt.insert("billboard".into(), Tag::String("center".into()));
                if let Some(body) = game.native_summon(&dimension, m::ITEM_DISPLAY, spot, nbt) {
                    let scale = cfg.scale;
                    let pose = crate::game::native_entities::NativeDisplay {
                        translation: [0.0; 3],
                        left_rotation: [0.0, 0.0, 0.0, 1.0],
                        scale: [scale, scale, scale],
                        right_rotation: [0.0, 0.0, 0.0, 1.0],
                        interpolation: 0,
                        teleport: 0,
                    };
                    game.native_set_display(&dimension, body, &pose);
                    pest.body = Some(body);
                }
            }
        }
        // The body rides the mob, faced at the player.
        if let (Some((pos, yaw)), Some(body)) = (order.follow, pest.body) {
            game.native_move_entity(&dimension, body, pos, yaw, 0.0);
        }
        pests[layer] = pest;
    }
    m::set_pests(uuid, pests);
    // The absolute void keeps nothing the mod doesn't know: below the
    // frozen floor, a display that is neither a hunt's body nor a creature
    // on the ledger (a fed gloomling follows its player down) is a stray a
    // restart orphaned, and goes.
    if deep {
        let driven = m::driven_ids();
        // A fresh look at the hunts: a body this very tick summoned is the
        // hunt's already, and no stray.
        let hunting = m::pest_bodies();
        let strays: Vec<u32> = game
            .others
            .iter()
            .filter(|other| {
                other.position.y < m::FROZEN_BOT
                    && !hunting.contains(&other.id)
                    && !driven.contains(&other.id)
                    && matches!(other.kind.as_str(), m::ITEM_DISPLAY | m::BLOCK_DISPLAY)
            })
            .map(|other| other.id)
            .collect();
        for id in strays {
            game.native_remove_entity(&dimension, id);
        }
    }
    // The dig in place, when it is one the mod's pickaxe digs: bedrock or
    // voidshell, held.
    let dig = game.mining.as_ref().map(|dig| (dig.pos, dig.ticks));
    let digging = match dig {
        Some((pos, ticks)) => match (m::dig_target(&game.native_block(pos)), game.native_held()) {
            (Some(target), Some((item, _, _))) if item == m::PICKAXE => {
                Some((pos, ticks, target))
            }
            _ => None,
        },
        None => None,
    };
    let Some((pos, ticks, target)) = digging else {
        if m::bar_shown(uuid, false) {
            game.native_action_bar(uuid, Text::literal(""));
        }
        // Not digging: down in the Abyss, on the mod's rhythm, one look for
        // a pocket to summon a creature into. The plan reads the world
        // through the closure; the summon is the engine's own /summon.
        if deep && m::spawn_due(uuid) {
            let near = live
                .iter()
                .filter(|(_, _, _, pos)| m::counts_towards_cap(*pos, at))
                .count();
            let seed = uuid as u64 ^ (at[0] as i64 as u64).wrapping_mul(0x2545_F491);
            let plan = m::spawn_plan(seed, at, near, |cell| game.native_block(cell) == m::AIR);
            if let Some((creature, spot)) = plan {
                // The display's compound, as /summon takes it: what it shows
                // — the mod's own item or block — faced every way.
                let mut nbt = horizon_world::nbt::Compound::new();
                match creature {
                    m::Creature::Lantern => {
                        nbt.insert(
                            "block_state".into(),
                            Tag::compound([("Name", Tag::String(m::LANTERN_BLOCK.into()))]),
                        );
                    }
                    m::Creature::Wisp
                    | m::Creature::Watcher
                    | m::Creature::Ember
                    | m::Creature::Frostmote
                    | m::Creature::Gloomling
                    | m::Creature::Gloom => {
                        nbt.insert(
                            "item".into(),
                            Tag::compound([
                                ("id", Tag::String(creature.shows().into())),
                                ("count", Tag::Int(1)),
                            ]),
                        );
                    }
                }
                nbt.insert("billboard".into(), Tag::String("center".into()));
                if let Some(id) = game.native_summon(&dimension, creature.display(), spot, nbt) {
                    m::add_creature(id, creature, spot, uuid);
                    // The first pose at once, so it shows at its own scale.
                    let scale = creature.scale();
                    let pose = crate::game::native_entities::NativeDisplay {
                        translation: [0.0; 3],
                        left_rotation: [0.0, 0.0, 0.0, 1.0],
                        scale: [scale, scale, scale],
                        right_rotation: [0.0, 0.0, 0.0, 1.0],
                        interpolation: 0,
                        teleport: 0,
                    };
                    game.native_set_display(&dimension, id, &pose);
                    // Its code yaw at once, so the very first save already
                    // keeps what the creature is.
                    game.native_move_entity(&dimension, id, spot, creature.yaw_code(), 0.0);
                }
            }
        }
        return;
    };
    let dig_ticks = m::dig_ticks(|| game.native_setting(m::ID, "dig_ticks"));
    let progress = m::progress(ticks, dig_ticks);
    m::bar_shown(uuid, true);
    game.native_action_bar(uuid, Text::literal(m::bar(m::dig_name(target), progress)));
    if progress < 1.0 {
        return;
    }
    // The dig is through: the block goes, and drops what the mod says.
    m::bar_shown(uuid, false);
    game.native_action_bar(uuid, Text::literal(""));
    // Air by its state id, when the registry has it first (the level's own
    // write: neighbours, light and the players all hear it), else a bulk edit,
    // which names its blocks.
    let air = horizon_world::registry::BlockStateId(0);
    let removed = if game.native_state_block(air) == Some(m::AIR) {
        game.native_set_block(&dimension, pos, air, 3)
    } else {
        let edit = crate::game::native_world_edit::NativeBlockEdit {
            palette: vec![m::AIR.to_owned()],
            filters: vec![game.native_block(pos).to_owned()],
            runs: vec![crate::game::native_world_edit::NativeBlockRun {
                x: pos[0],
                z: pos[2],
                y: pos[1],
                height: 1,
                surface: None,
                block: 0,
                only: Some(0),
            }],
        };
        game.native_edit_blocks(&dimension, &edit).is_ok_and(|changed| changed > 0)
    };
    if !removed {
        game.native_tell(
            uuid,
            Text::literal("Under Bedrock: the block wouldn't go (its level isn't the one being ticked)."),
        );
        return;
    }
    // The break's feedback goes to the digging player's client: the window's
    // half plays the sound and the blue sparks there (API 40's net bridge).
    if let Err(why) =
        game.native_net_send_to_player(uuid, m::DUG_CHANNEL, &m::encode_dig(pos, target))
    {
        game.native_tell(uuid, Text::literal(m::feedback_failed(&why)));
    }
    if m::dig_drops(target, || game.native_setting(m::ID, "drop_bedrock")) {
        game.native_give_items(m::dig_drop(target), 1);
    }
}
