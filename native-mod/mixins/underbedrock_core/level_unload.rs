// A world closed under the mod (API 134): forget what was kept for it, so a
// world reopened - or another one - starts clean. One function, as a mixin
// helper file must hold exactly one.
fn underbedrock_core_level_unload(game: &mut crate::game::Game<'_>, dimension: &str) {
    use crate::game::underbedrock_core as m;
    if !game.native_on(m::ID) {
        return;
    }
    m::forget_world(dimension);
}
