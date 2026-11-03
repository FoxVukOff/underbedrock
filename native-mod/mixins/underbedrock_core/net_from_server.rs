// The dig's feedback and the transition veils, heard on the client (API
// 40's bridge): the server's tick sent what broke, or which layer boundary
// the player crossed and where, and the window's half plays it there - the
// slowed deepslate break and the blue sparks of a dig, the ring of blue
// sparks at a boundary - the only place they can play at all. One function,
// as a mixin helper file must hold exactly one.
fn underbedrock_core_net_from_server(
    game: &mut crate::game::Game<'_>,
    channel: &str,
    data: &[u8],
) {
    use crate::game::underbedrock_core as m;
    if !game.native_on(m::ID) {
        return;
    }
    if channel == m::VEIL_CHANNEL {
        // A layer boundary the server saw this player cross: its ring of
        // blue sparks, at the boundary's own height around the x and z the
        // message names (the window keeps no position of its own to read).
        if let Some((bound, [x, z])) = m::decode_veil(data) {
            for at in m::veil_ring(x as f64, z as f64, bound) {
                game.native_flame_particle(at, true);
            }
        }
        return;
    }
    if channel != m::DUG_CHANNEL {
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
