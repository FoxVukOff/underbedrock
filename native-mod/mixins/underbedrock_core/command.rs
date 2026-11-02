// /etherium: the pickaxe, how deep the world goes, the dig time and the drop.
fn underbedrock_core_performed(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::underbedrock_core as m;
    use horizon_client::text::Text;
    let mut words = command.split_whitespace();
    if words.next() != Some("etherium") || !game.native_on(m::ID) {
        return;
    }
    *ran = true;
    let line = match words.next() {
        None => m::HELP.to_owned(),
        Some("give") => {
            if game.native_give_items(m::PICKAXE, 1) > 0 {
                "An etherium pickaxe: in the inventory, or at the player's feet.".to_owned()
            } else {
                "The pickaxe didn't fit: is a world open?".to_owned()
            }
        }
        Some("depth") => {
            let (min_y, height) = game.native_world_height();
            m::depth_line(min_y, height)
        }
        Some("dig") => {
            let next = m::next_dig_ticks(m::dig_ticks(|| game.native_setting(m::ID, "dig_ticks")));
            m::set_dig_ticks(next);
            game.native_set_setting(m::ID, "dig_ticks", &next.to_string());
            m::dig_label(next)
        }
        Some("drop") => {
            let next = !m::drops(|| game.native_setting(m::ID, "drop_bedrock"));
            m::set_drops(next);
            game.native_set_setting(m::ID, "drop_bedrock", if next { "true" } else { "false" });
            m::drop_label(next)
        }
        Some(other) => format!("There is no /etherium {other}. {}", m::HELP),
    };
    game.native_feedback(Text::literal(line));
}
