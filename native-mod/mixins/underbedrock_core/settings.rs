// The mod's settings page: how long bedrock takes to dig, and whether it drops.
fn underbedrock_core_page(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::underbedrock_core as m;
    if mod_id != m::ID {
        return;
    }
    buttons.push(m::dig_label(m::dig_ticks(|| game.native_setting(m::ID, "dig_ticks"))));
    buttons.push(m::drop_label(m::drops(|| game.native_setting(m::ID, "drop_bedrock"))));
}
