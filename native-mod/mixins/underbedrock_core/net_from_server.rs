// The dig's feedback, heard on the client (API 40's bridge): the server's
// tick sent what broke, and the window's half plays the sound and the blue
// sparks there - the only place they can play at all. One function, as a
// mixin helper file must hold exactly one.
fn underbedrock_core_net_from_server(
    game: &mut crate::game::Game<'_>,
    channel: &str,
    data: &[u8],
) {
    use crate::game::underbedrock_core as m;
    if !game.native_on(m::ID) || channel != m::DUG_CHANNEL {
        return;
    }
    let Some((pos, _dig)) = m::decode_dig(data) else {
        return;
    };
    let at = [
        pos[0] as f64 + 0.5,
        pos[1] as f64 + 0.5,
        pos[2] as f64 + 0.5,
    ];
    game.native_play_local_sound(m::DIG_SOUND, at, m::DIG_VOLUME, m::DIG_PITCH, 0);
    for off in m::DIG_BURST {
        game.native_flame_particle([at[0] + off[0], at[1] + off[1], at[2] + off[2]], true);
    }
}
