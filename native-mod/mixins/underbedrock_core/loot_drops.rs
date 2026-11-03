// What a death drops (API 75): the Abyss's stalker leaves its shard as a
// trophy - the drops fall on the ground, so no player's state is touched.
// One function, as a mixin helper file must hold exactly one.
fn underbedrock_core_loot_drops(
    game: &mut crate::game::Game<'_>,
    context: &horizon_client::mod_api::NativeLootDrops<'_>,
    drops: &mut Vec<horizon_client::items::ItemStack>,
) {
    use crate::game::underbedrock_core as m;
    if !game.native_on(m::ID) {
        return;
    }
    if m::stalker_drops(context.dimension, context.entity) {
        drops.push(horizon_client::items::ItemStack::new(m::STALKER_ITEM, 1));
    }
}
