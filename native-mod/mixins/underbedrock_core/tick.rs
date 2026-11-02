// The dig itself, and the Abyss's own inhabitants. The engine keeps a
// player's dig in its own state — the block the attack key is held on and the
// ticks it has been held for — swapped in for that player's tick, and cleared
// the moment they let go or look away. On bedrock or voidshell, with the
// void-alloy pickaxe in hand, the mod's dig time decides when the block goes;
// the action bar shows how far the dig is. The abyss bedrock at the world's
// bottom is left alone: nothing digs it.
//
// While a player is down in the Abyss, the mod also keeps it inhabited: every
// so many ticks it looks for a dark pocket near the player and spawns one of
// the Abyss's kinds there, up to a cap of them per player.
fn underbedrock_core_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::underbedrock_core as m;
    use horizon_client::text::Text;
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
        underbedrock_core_spawn(game, &player);
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
    let dimension = player.dimension;
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
    if m::dig_drops(target, || game.native_setting(m::ID, "drop_bedrock")) {
        game.native_give_items(m::dig_drop(target), 1);
    }
}

// One look for a place to spawn, on the mod's rhythm: only down in the Abyss,
// only up to the cap, only into air over a solid floor.
fn underbedrock_core_spawn(
    game: &mut crate::game::Game<'_>,
    player: &crate::game::native_hooks::NativeServerPlayer,
) {
    use crate::game::underbedrock_core as m;
    let at = player.position;
    if at[1] >= m::SPAWN_BELOW_Y || !m::spawn_due(player.uuid) {
        return;
    }
    let dimension = player.dimension.clone();
    let near = game
        .native_mobs(&dimension)
        .into_iter()
        .filter(|mob| m::counts_towards_cap(&mob.kind, mob.pos, at))
        .count();
    if near >= m::MOB_CAP {
        return;
    }
    let seed = player.uuid as u64 ^ (at[0] as i64 as u64).wrapping_mul(0x2545_F491);
    for attempt in 0..m::SPAWN_ATTEMPTS {
        let [dx, dy, dz] = m::candidate_offset(seed, attempt);
        let spot = [at[0] + dx as f64, (at[1] + dy as f64).floor(), at[2] + dz as f64];
        let feet = [spot[0] as i32, spot[1] as i32, spot[2] as i32];
        // A pocket: air where the mob would stand, air over its head, and a
        // floor under it that is not the world's own bottom air.
        let open = game.native_block(feet) == m::AIR && game.native_block([feet[0], feet[1] + 1, feet[2]]) == m::AIR;
        let floored = game.native_block([feet[0], feet[1] - 1, feet[2]]) != m::AIR;
        if !(open && floored) {
            continue;
        }
        let kind = m::pick_kind(seed.wrapping_add(attempt as u64));
        let spawn = crate::game::native_mobs::NativeSpawn::default();
        if game
            .native_spawn_mob(&dimension, kind, [spot[0], spot[1], spot[2]], &spawn)
            .is_some()
        {
            return;
        }
    }
}
