// The dig itself. The engine keeps a player's dig in its own state — the block
// the attack key is held on and the ticks it has been held for — swapped in for
// that player's tick, and cleared the moment they let go or look away. On
// bedrock, with the etherium pickaxe in hand, the mod's dig time decides when
// the block goes; the action bar shows how far the dig is.
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
    // The dig in place, when it is one the mod's pickaxe digs: bedrock, held.
    let dig = game.mining.as_ref().map(|dig| (dig.pos, dig.ticks));
    let digging = match dig {
        Some((pos, ticks))
            if game.native_block(pos) == m::BEDROCK
                && game.native_held().is_some_and(|(item, _, _)| item == m::PICKAXE) =>
        {
            Some((pos, ticks))
        }
        _ => None,
    };
    let Some((pos, ticks)) = digging else {
        if m::bar_shown(uuid, false) {
            game.native_action_bar(uuid, Text::literal(""));
        }
        return;
    };
    let dig_ticks = m::dig_ticks(|| game.native_setting(m::ID, "dig_ticks"));
    let progress = m::progress(ticks, dig_ticks);
    m::bar_shown(uuid, true);
    game.native_action_bar(uuid, Text::literal(m::bar(progress)));
    if progress < 1.0 {
        return;
    }
    // The dig is through: the bedrock goes, and drops when the mod says so.
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
            filters: vec![m::BEDROCK.to_owned()],
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
            Text::literal("Under Bedrock: the bedrock wouldn't go (its level isn't the one being ticked)."),
        );
        return;
    }
    if m::drops(|| game.native_setting(m::ID, "drop_bedrock")) {
        game.native_give_items(m::BEDROCK, 1);
    }
}
