// The dig itself, and the Abyss's own creatures — one function, as a mixin
// helper file must hold exactly one. The engine keeps a player's dig in its
// own state — the block the attack key is held on and the ticks it has been
// held for — swapped in for that player's tick, and cleared the moment they
// let go or look away. On bedrock or voidshell, with the void-alloy pickaxe
// in hand, the mod's dig time decides when the block goes; the action bar
// shows how far the dig is. The abyss bedrock at the world's bottom is left
// alone: nothing digs it.
//
// While a player is down in the Abyss, the mod also keeps it inhabited — with
// its OWN creatures, never a vanilla mob: display entities showing the mod's
// shards and crystal lamp, which the engine keeps without simulating and the
// mod drives from here. Every tick they follow the engine's list (a world
// reloaded adopts what its chunks saved), take a step of their paths, and the
// ones their player walked away from are let go; on the mod's rhythm a look
// goes out for a dark pocket to summon the next one into, the layer at the
// player's depth picking the kind (embers in the hot expanse, frostmotes at
// the cold rim). The frozen wastes keep no ambient company: they are the
// stalker's hunting ground, and a player who descends into them is hunted by
// one — an invisible, silent vex the engine's own melee drives, wearing an
// item display of the stalker shard as its visible body. And the ban is real:
// every tick, any vanilla mob under the gate that isn't a tracked stalker is
// taken out of the world with no death and no loot.
fn underbedrock_core_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::underbedrock_core as m;
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
    // The displays the engine keeps in the Abyss: the mod's creatures, as the
    // engine has them (owned copies, so the engine is free for the calls
    // below). Up top there are none of the mod's, and it does nothing.
    let deep = at[1] < m::SPAWN_BELOW_Y;
    let live: Vec<(u32, String, [f64; 3])> = if deep {
        game.others
            .iter()
            .map(|other| {
                (
                    other.id,
                    other.kind.clone(),
                    [other.position.x, other.position.y, other.position.z],
                )
            })
            .filter(|(_, kind, pos)| m::is_ours(kind, *pos))
            .collect()
    } else {
        Vec::new()
    };
    if deep {
        // One step of every creature: the list follows the engine, the paths
        // move, the poses go out on the mod's rhythm, the far ones are let go.
        let seen: Vec<(u32, &str, [f64; 3])> = live
            .iter()
            .map(|(id, kind, pos)| (*id, kind.as_str(), *pos))
            .collect();
        let step = m::step_creatures(at, uuid, &seen);
        for id in step.cull {
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
    // the ban spares is a stalker the mod tracks.
    let mobs = game.native_mobs(&dimension);
    for mob in &mobs {
        if mob.pos[1] < m::BAN_BELOW_Y && !m::is_stalker_mob(mob.id) {
            game.native_remove_mob(&dimension, mob.id);
        }
    }
    // The frozen wastes hunt: one stalker per player down there, dismissed
    // the moment they climb out. The engine's list of mobs is the truth —
    // a stalker that fell (or a world that reloaded without the pair) shows
    // as gone, and the hunt reacts.
    let in_zone = at[1] < m::STALK_BELOW_Y;
    let stalker_mob = m::get_stalk(uuid).mob;
    let mob_at = stalker_mob.and_then(|id| {
        mobs.iter()
            .find(|mob| mob.id == id)
            .map(|mob| mob.pos)
    });
    let (mut stalk, order) = m::stalk_tick(m::get_stalk(uuid), in_zone, mob_at, at);
    if let Some(id) = order.dismiss_mob {
        game.native_remove_mob(&dimension, id);
    }
    if let Some(id) = order.dismiss_body {
        game.native_remove_entity(&dimension, id);
    }
    // The wastes keep nothing the hunt doesn't know: a body a restart
    // orphaned goes too (no ambient display lives below the stalker's depth).
    if in_zone {
        let strays: Vec<u32> = game
            .others
            .iter()
            .filter(|other| {
                other.position.y < m::STALK_BELOW_Y
                    && Some(other.id) != stalk.body
                    && matches!(other.kind.as_str(), m::ITEM_DISPLAY | m::BLOCK_DISPLAY)
            })
            .map(|other| other.id)
            .collect();
        for id in strays {
            game.native_remove_entity(&dimension, id);
        }
    }
    // The hunter takes shape on its ring when the order says so.
    let mut body_at = mob_at.map(|pos| [pos[0], pos[1] + m::STALKER_LIFT, pos[2]]);
    if order.summon && stalk.mob.is_none() {
        let seed = uuid as u64 ^ (at[2] as i64 as u64).wrapping_mul(0x9E37_79B9);
        let spot = m::stalker_spot(seed, at);
        let spawn = crate::game::native_mobs::NativeSpawn {
            health: Some(m::STALKER_HEALTH),
            invisible: true,
            silent: true,
            no_gravity: true,
            ..Default::default()
        };
        if let Some(mob) = game.native_spawn_mob(&dimension, m::STALKER_KIND, spot, &spawn) {
            stalk.mob = Some(mob);
            stalk.quiet = 0;
            body_at = Some([spot[0], spot[1] + m::STALKER_LIFT, spot[2]]);
        }
    }
    // Its body: the shard display, summoned once per hunt (and again for a
    // mob whose body summon failed) and posed at the stalker's scale.
    if stalk.mob.is_some() && stalk.body.is_none() {
        if let Some(spot) = body_at {
            let mut nbt = horizon_world::nbt::Compound::new();
            nbt.insert(
                "item".into(),
                Tag::compound([
                    ("id", Tag::String(m::STALKER_ITEM.into())),
                    ("count", Tag::Int(1)),
                ]),
            );
            nbt.insert("billboard".into(), Tag::String("center".into()));
            if let Some(body) = game.native_summon(&dimension, m::ITEM_DISPLAY, spot, nbt) {
                let scale = m::STALKER_SCALE;
                let pose = crate::game::native_entities::NativeDisplay {
                    translation: [0.0; 3],
                    left_rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [scale, scale, scale],
                    right_rotation: [0.0, 0.0, 0.0, 1.0],
                    interpolation: 0,
                    teleport: 0,
                };
                game.native_set_display(&dimension, body, &pose);
                stalk.body = Some(body);
            }
        }
    }
    // The body rides the mob, faced at the player.
    if let (Some((pos, yaw)), Some(body)) = (order.follow, stalk.body) {
        game.native_move_entity(&dimension, body, pos, yaw, 0.0);
    }
    m::set_stalk(uuid, stalk);
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
                .filter(|(_, _, pos)| m::counts_towards_cap(*pos, at))
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
                    | m::Creature::Frostmote => {
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
    let _ = game.native_net_send_to_player(uuid, m::DUG_CHANNEL, &m::encode_dig(pos, target));
    if m::dig_drops(target, || game.native_setting(m::ID, "drop_bedrock")) {
        game.native_give_items(m::dig_drop(target), 1);
    }
}
