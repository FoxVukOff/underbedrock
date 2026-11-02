// A click on the mod's settings page: the next dig time, or the drop flipped.
fn underbedrock_core_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::underbedrock_core as m;
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            let next = m::next_dig_ticks(m::dig_ticks(|| game.native_setting(m::ID, "dig_ticks")));
            m::set_dig_ticks(next);
            game.native_set_setting(m::ID, "dig_ticks", &next.to_string());
        }
        1 => {
            let next = !m::drops(|| game.native_setting(m::ID, "drop_bedrock"));
            m::set_drops(next);
            game.native_set_setting(m::ID, "drop_bedrock", if next { "true" } else { "false" });
        }
        _ => {}
    }
}
