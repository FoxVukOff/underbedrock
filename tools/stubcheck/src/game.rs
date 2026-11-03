//! A stub of Horizon's `crate::game`: the shapes the mod's helpers touch, with
//! the real signatures (copied from `crates/horizon-client/src/game/` at the
//! pinned revision) and small behaviour, so the mod's own sources compile here
//! and can be run against a fake world. The game itself doesn't build in a
//! small sandbox; this is the closest check to it.
//!
//! The mod's sources are included where Horizon's build puts them: its runtime
//! as `crate::game::underbedrock_core`, and each helper as a nested function of
//! the hook it is bound to, inside `crate::game::native_hooks` — so the private
//! state the tick helper reads (`Game::mining`, `Mining`'s fields) is reached
//! from a descendant module, exactly as it is in the game.

use std::cell::RefCell;
use std::collections::HashMap;

/// The engine's own dig state (`game.rs`'s private `Mining`): the block the
/// attack key is held on, how far a breakable block's dig is, and the ticks the
/// player has held it — which `tick_mining` counts on for an unbreakable one.
struct Mining {
    pos: [i32; 3],
    progress: f32,
    ticks: u32,
}

/// `horizon_core::geometry::Vec3`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// An entity Horizon keeps without simulating (`game/commands.rs`'
/// `OtherEntity`): a display, a marker, an interaction. The mod's creatures
/// are the displays among them.
pub struct OtherEntity {
    pub id: u32,
    pub kind: String,
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

/// A server mod's bulk block edit (`game/native_world_edit.rs`).
pub mod native_world_edit {
    /// Which of a column's heights a run's `y` counts from.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NativeHeightmap {
        WorldSurface,
        MotionBlocking,
        MotionBlockingNoLeaves,
        OceanFloor,
    }
    /// A run: `height` blocks up a column from `y`, set to palette entry
    /// `block`, only where filter `only` matches when one is given.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativeBlockRun {
        pub x: i32,
        pub z: i32,
        pub y: i32,
        pub height: u32,
        pub surface: Option<NativeHeightmap>,
        pub block: u32,
        pub only: Option<u32>,
    }
    /// A bulk edit: block states and predicates as commands write them, and the
    /// runs naming them by index.
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct NativeBlockEdit {
        pub palette: Vec<String>,
        pub filters: Vec<String>,
        pub runs: Vec<NativeBlockRun>,
    }
}

/// Entities Horizon keeps without simulating, for server mods
/// (`game/native_entities.rs`, API 59): the shape `native_set_display` takes.
pub mod native_entities {
    /// A display's transformation and interpolation.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativeDisplay {
        pub translation: [f32; 3],
        pub left_rotation: [f32; 4],
        pub scale: [f32; 3],
        pub right_rotation: [f32; 4],
        pub interpolation: i32,
        pub teleport: i32,
    }
}

/// Mobs for native server mods (`game/native_mobs.rs`, API 37): the shapes
/// `native_spawn_mob` takes and `native_mobs` gives, with the real fields.
pub mod native_mobs {
    /// A mob as a native mod reads it.
    #[derive(Clone, Debug, PartialEq)]
    pub struct NativeMob {
        pub id: u32,
        pub dimension: String,
        pub kind: String,
        pub pos: [f64; 3],
        pub yaw: f32,
        pub health: f32,
        pub max_health: f32,
    }

    /// What a mob starts with; the default is a plain mob.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct NativeSpawn {
        pub name: Option<String>,
        pub health: Option<f32>,
        pub no_ai: bool,
        pub invisible: bool,
        pub silent: bool,
        pub invulnerable: bool,
        pub no_gravity: bool,
    }
}

/// A mob the stub world keeps, and what it was spawned with.
pub struct StubMob {
    pub id: u32,
    pub kind: String,
    pub pos: [f64; 3],
    pub removed: bool,
    pub spawn: native_mobs::NativeSpawn,
}

/// The hook points' types and the hooks the mod binds to
/// (`game/native_hooks.rs`).
pub mod native_hooks {
    /// A mod's row in Options > Mods (`horizon_client::mod_api::NativeMod`).
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NativeMod {
        pub id: String,
        pub title: String,
        pub description: String,
        pub keys: Vec<[String; 3]>,
    }

    /// A command a native mod adds, as one usage (`NativeCommand`).
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub struct NativeCommand {
        pub usage: String,
        pub level: u8,
    }

    impl NativeCommand {
        pub fn new(usage: &str) -> Self {
            Self {
                usage: usage.to_owned(),
                level: 0,
            }
        }
        pub fn op(usage: &str, level: u8) -> Self {
            Self {
                usage: usage.to_owned(),
                level: level.min(4),
            }
        }
    }

    /// The player whose action or command a hook hears (`NativeServerPlayer`).
    #[derive(Clone, Debug, PartialEq)]
    pub struct NativeServerPlayer {
        pub uuid: u128,
        pub name: String,
        pub permission: u8,
        pub dimension: String,
        pub position: [f64; 3],
        pub game_mode: i32,
        pub health: f32,
        pub yaw: f32,
        pub pitch: f32,
    }

    // The mod's helper files, one function each. Horizon's build nests a helper
    // in the body of the hook it is bound to; here they sit in the same module,
    // which resolves the same absolute paths and sees the same private state.
    include!("../../../native-mod/mixins/underbedrock_core/list.rs");
    include!("../../../native-mod/mixins/underbedrock_core/settings.rs");
    include!("../../../native-mod/mixins/underbedrock_core/settings_click.rs");
    include!("../../../native-mod/mixins/underbedrock_core/tick.rs");
    include!("../../../native-mod/mixins/underbedrock_core/commands.rs");
    include!("../../../native-mod/mixins/underbedrock_core/command.rs");
    include!("../../../native-mod/mixins/underbedrock_core/level_unload.rs");
    include!("../../../native-mod/mixins/underbedrock_core/loot_drops.rs");
    include!("../../../native-mod/mixins/underbedrock_core/net_from_server.rs");

    // Each hook calls its helper with the bindings the toml names.
    impl super::Game<'_> {
        pub fn native_mods(&mut self, mods: &mut Vec<NativeMod>) {
            underbedrock_core_listed(&mut *mods);
        }

        pub fn native_settings(&mut self, mod_id: &str, buttons: &mut Vec<String>) {
            underbedrock_core_page(&mut *self, mod_id, &mut *buttons);
        }

        pub fn native_settings_click(&mut self, mod_id: &str, button: usize) {
            underbedrock_core_click(&mut *self, mod_id, button);
        }

        pub fn native_player_tick(&mut self) {
            underbedrock_core_tick(&mut *self);
        }

        pub fn native_register_commands(&mut self, commands: &mut Vec<NativeCommand>) {
            underbedrock_core_commands(&mut *commands);
        }

        pub fn native_perform_command(
            &mut self,
            player: &NativeServerPlayer,
            command: &str,
            ran: &mut bool,
        ) {
            let _ = player;
            underbedrock_core_performed(&mut *self, command, &mut *ran);
        }

        pub fn native_level_unload(&mut self, dimension: &str) {
            underbedrock_core_level_unload(&mut *self, dimension);
        }

        pub fn native_loot_drops(
            &mut self,
            context: &crate::mod_api::NativeLootDrops<'_>,
            drops: &mut Vec<crate::items::ItemStack>,
        ) {
            underbedrock_core_loot_drops(&mut *self, context, &mut *drops);
        }

        pub fn native_net_from_server(&mut self, channel: &str, data: &[u8]) {
            underbedrock_core_net_from_server(&mut *self, channel, data);
        }
    }
}

/// The game (`game.rs`'s `Game`), stubbed: a world of named blocks, one acting
/// player, and the record of what the hooks did.
pub struct Game<'a> {
    /// The dig in place, as `tick_mining` keeps it.
    mining: Option<Mining>,
    /// The world's blocks, by position; a position without one is air.
    pub blocks: HashMap<[i32; 3], String>,
    /// The acting player's main hand, and the player themself.
    pub held: Option<(String, u32, bool)>,
    pub acting: Option<native_hooks::NativeServerPlayer>,
    /// The level's lowest y and height, and whether it is the one being ticked.
    pub shape: (i32, i32),
    pub level_open: bool,
    /// Whether the registry has air at state id 0, as Horizon's does.
    pub air_at_zero: bool,
    /// The mods the player disabled, and the mod settings' files (`id/key`).
    pub disabled: Vec<String>,
    pub settings: RefCell<HashMap<String, String>>,
    /// What the hooks did, for the harness to assert on.
    pub given: Vec<(String, u32)>,
    pub bars: Vec<(u128, String)>,
    pub told: Vec<(u128, String)>,
    pub feedback: Vec<String>,
    pub edits: Vec<native_world_edit::NativeBlockEdit>,
    /// The entities Horizon keeps without simulating (the mod's creature
    /// displays are among them), the id it hands out next, and the record of
    /// what the mod did to them, for the harness to assert on.
    pub others: Vec<OtherEntity>,
    pub next_entity: u32,
    pub summoned: Vec<(String, [f64; 3], String)>,
    pub moved: Vec<(u32, [f64; 3])>,
    pub posed: Vec<(u32, [f32; 3])>,
    pub removed_entities: Vec<u32>,
    /// The mobs the stub world simulates, the id it hands out next, and the
    /// record of what the mod did with them, for the harness to assert on.
    pub mobs: Vec<StubMob>,
    pub next_mob: u32,
    pub spawned_mobs: Vec<(String, [f64; 3], native_mobs::NativeSpawn)>,
    pub removed_mobs: Vec<u32>,

    /// What the net bridge sent, and the sounds and sparks the client half
    /// played, for the harness to assert on.
    pub net_sent: Vec<(u128, String, Vec<u8>)>,
    pub sounds: Vec<(String, [f64; 3], f32, f32, i32)>,
    pub particles: Vec<([f64; 3], bool)>,

    pub marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Game<'a> {
    /// A world of one player, standing in the Overworld with nothing in hand.
    pub fn new() -> Self {
        Self {
            mining: None,
            blocks: HashMap::new(),
            held: None,
            acting: Some(native_hooks::NativeServerPlayer {
                uuid: 7,
                name: "Steve".into(),
                permission: 4,
                dimension: "minecraft:overworld".into(),
                position: [0.5, -60.0, 0.5],
                game_mode: 0,
                health: 20.0,
                yaw: 0.0,
                pitch: 90.0,
            }),
            shape: (-64, 384),
            level_open: true,
            air_at_zero: true,
            disabled: Vec::new(),
            settings: RefCell::new(HashMap::new()),
            given: Vec::new(),
            bars: Vec::new(),
            told: Vec::new(),
            feedback: Vec::new(),
            edits: Vec::new(),
            others: Vec::new(),
            next_entity: 1,
            summoned: Vec::new(),
            moved: Vec::new(),
            posed: Vec::new(),
            removed_entities: Vec::new(),
            mobs: Vec::new(),
            next_mob: 500,
            spawned_mobs: Vec::new(),
            removed_mobs: Vec::new(),
            net_sent: Vec::new(),
            sounds: Vec::new(),
            particles: Vec::new(),
            marker: std::marker::PhantomData,
        }
    }

    /// The dig the engine keeps while the attack key is held on `pos`.
    pub fn start_dig(&mut self, pos: [i32; 3], ticks: u32) {
        self.mining = Some(Mining {
            pos,
            progress: 0.0,
            ticks,
        });
    }

    /// The player let go, looked away or the dig finished.
    pub fn stop_dig(&mut self) {
        self.mining = None;
    }

    /// A setting's file, as `native_set_setting` writes it.
    pub fn setting_file(&self, mod_id: &str, key: &str) -> Option<String> {
        self.settings
            .borrow()
            .get(&format!("{mod_id}/{key}"))
            .cloned()
    }

    /// The last bulk edit's palette, filters and run count.
    pub fn last_edit(&self) -> Option<(Vec<String>, Vec<String>, usize)> {
        self.edits
            .last()
            .map(|e| (e.palette.clone(), e.filters.clone(), e.runs.len()))
    }

    /// The block at `pos`, as the world keeps it.
    pub fn block_at(&self, pos: [i32; 3]) -> String {
        self.blocks
            .get(&pos)
            .cloned()
            .unwrap_or_else(|| "minecraft:air".into())
    }
}

/// The helpers the mod's hooks call, with Horizon's signatures and stub bodies
/// (`game/native_hooks.rs`, `game/native_server.rs`, `game/native_world_edit.rs`).
impl Game<'_> {
    /// Whether the player left the mod on.
    pub fn native_on(&self, mod_id: &str) -> bool {
        !self.disabled.contains(&mod_id.to_owned())
    }

    /// The acting player, as the hook points name one.
    pub fn native_acting_player(&self) -> Option<native_hooks::NativeServerPlayer> {
        self.acting.clone()
    }

    /// The main hand's stack: its item, count, and whether it is plain.
    pub fn native_held(&self) -> Option<(String, u32, bool)> {
        self.held.clone()
    }

    /// The lowest block y and the height of the world in place.
    pub fn native_world_height(&self) -> (i32, i32) {
        self.shape
    }

    /// The block at `pos`, by name.
    pub fn native_block(&self, pos: [i32; 3]) -> String {
        self.block_at(pos)
    }

    /// The level in place is `dimension` and simulates.
    fn entity_level(&self, dimension: &str) -> bool {
        self.level_open
            && self
                .acting
                .as_ref()
                .is_some_and(|p| p.dimension == dimension)
    }

    /// Spawns an entity of `kind` at `pos` from its compound, as `/summon`
    /// does: its id. The stub keeps the compound's payload (the item or block
    /// a display shows) for the harness.
    pub fn native_summon(
        &mut self,
        dimension: &str,
        kind: &str,
        pos: [f64; 3],
        nbt: horizon_world::nbt::Compound,
    ) -> Option<u32> {
        if !self.entity_level(dimension) || !pos.iter().all(|v| v.is_finite()) {
            return None;
        }
        use horizon_world::nbt::Tag;
        let shown = |key: &str, inner: &str| {
            nbt.get(key).and_then(|t| match t {
                Tag::Compound(c) => c.get(inner),
                _ => None,
            })
        };
        let payload = match shown("item", "id").or_else(|| shown("block_state", "Name")) {
            Some(Tag::String(s)) => s.clone(),
            _ => String::new(),
        };
        let id = self.next_entity;
        self.next_entity += 1;
        self.others.push(OtherEntity {
            id,
            kind: kind.to_owned(),
            position: Vec3 {
                x: pos[0],
                y: pos[1],
                z: pos[2],
            },
            yaw: 0.0,
            pitch: 0.0,
        });
        self.summoned.push((kind.to_owned(), pos, payload));
        Some(id)
    }

    /// Moves an entity Horizon keeps without simulating to `pos`.
    pub fn native_move_entity(
        &mut self,
        dimension: &str,
        id: u32,
        pos: [f64; 3],
        yaw: f32,
        pitch: f32,
    ) -> bool {
        if !self.entity_level(dimension) {
            return false;
        }
        let Some(o) = self.others.iter_mut().find(|o| o.id == id) else {
            return false;
        };
        o.position = Vec3 {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        };
        o.yaw = yaw;
        o.pitch = pitch;
        self.moved.push((id, pos));
        true
    }

    /// Sets a display's transformation and interpolation.
    pub fn native_set_display(
        &mut self,
        dimension: &str,
        id: u32,
        display: &native_entities::NativeDisplay,
    ) -> bool {
        if !self.entity_level(dimension) {
            return false;
        }
        let is_display = self.others.iter().any(|o| {
            o.id == id
                && matches!(
                    o.kind.as_str(),
                    "minecraft:block_display" | "minecraft:item_display" | "minecraft:text_display"
                )
        });
        if !is_display {
            return false;
        }
        self.posed.push((id, display.scale));
        true
    }

    /// Removes an entity Horizon keeps without simulating, as `/kill` does.
    pub fn native_remove_entity(&mut self, dimension: &str, id: u32) -> bool {
        if !self.entity_level(dimension) {
            return false;
        }
        let before = self.others.len();
        self.others.retain(|o| o.id != id);
        let gone = self.others.len() < before;
        if gone {
            self.removed_entities.push(id);
        }
        gone
    }

    /// The living mobs in the level in place, `dimension` (API 37).
    pub fn native_mobs(&self, dimension: &str) -> Vec<native_mobs::NativeMob> {
        if !self.entity_level(dimension) {
            return Vec::new();
        }
        self.mobs
            .iter()
            .filter(|m| !m.removed)
            .map(|m| native_mobs::NativeMob {
                id: m.id,
                dimension: dimension.to_owned(),
                kind: m.kind.clone(),
                pos: m.pos,
                yaw: 0.0,
                health: m.spawn.health.unwrap_or(20.0),
                max_health: 60.0,
            })
            .collect()
    }

    /// Spawns a mob of `kind` at `pos` with `options` (API 37): its id. The
    /// stub spawns anywhere its level simulates (the real one also wants a
    /// loaded chunk, and refuses a monster on peaceful).
    pub fn native_spawn_mob(
        &mut self,
        dimension: &str,
        kind: &str,
        pos: [f64; 3],
        options: &native_mobs::NativeSpawn,
    ) -> Option<u32> {
        if !self.entity_level(dimension) || !pos.iter().all(|v| v.is_finite()) {
            return None;
        }
        let id = self.next_mob;
        self.next_mob += 1;
        self.mobs.push(StubMob {
            id,
            kind: kind.to_owned(),
            pos,
            removed: false,
            spawn: options.clone(),
        });
        self.spawned_mobs
            .push((kind.to_owned(), pos, options.clone()));
        Some(id)
    }

    /// Takes mob `id` out of the world with no death and no loot (API 37).
    pub fn native_remove_mob(&mut self, dimension: &str, id: u32) -> bool {
        if !self.entity_level(dimension) {
            return false;
        }
        let Some(m) = self.mobs.iter_mut().find(|m| m.id == id && !m.removed) else {
            return false;
        };
        m.removed = true;
        self.removed_mobs.push(id);
        true
    }

    /// The block a state is a state of.
    pub fn native_state_block(&self, state: horizon_world::registry::BlockStateId) -> Option<&str> {
        match (self.air_at_zero, state.0) {
            (true, 0) => Some("minecraft:air"),
            _ => None,
        }
    }

    /// A mod's setting `key` (one file per key).
    pub fn native_setting(&self, mod_id: &str, key: &str) -> Option<String> {
        self.settings
            .borrow()
            .get(&format!("{mod_id}/{key}"))
            .cloned()
    }

    /// Keeps a mod's setting `key`.
    pub fn native_set_setting(&self, mod_id: &str, key: &str, value: &str) {
        self.settings
            .borrow_mut()
            .insert(format!("{mod_id}/{key}"), value.to_owned());
    }

    /// Sets a block in the level being ticked, through `Level.setBlock`.
    pub fn native_set_block(
        &mut self,
        dimension: &str,
        pos: [i32; 3],
        state: horizon_world::registry::BlockStateId,
        flags: i32,
    ) -> bool {
        let Some(acting) = &self.acting else {
            return false;
        };
        if !self.level_open || dimension != acting.dimension || flags < 0 {
            return false;
        }
        let name = match (self.air_at_zero, state.0) {
            (true, 0) => "minecraft:air".to_owned(),
            _ => format!("state#{}", state.0),
        };
        let before = self.block_at(pos);
        self.blocks.insert(pos, name.clone());
        before != name
    }

    /// The runs of a bulk edit, straight into the chunks of the level being ticked.
    pub fn native_edit_blocks(
        &mut self,
        dimension: &str,
        edit: &native_world_edit::NativeBlockEdit,
    ) -> Result<u64, String> {
        let Some(acting) = &self.acting else {
            return Err("no level is being ticked".into());
        };
        if !self.level_open || dimension != acting.dimension {
            return Err(format!("{dimension} is not the level being ticked"));
        }
        if edit
            .runs
            .iter()
            .any(|r| r.block as usize >= edit.palette.len())
            || edit
                .runs
                .iter()
                .any(|r| r.only.is_some_and(|f| f as usize >= edit.filters.len()))
        {
            return Err("a run names a palette entry or filter past the end".into());
        }
        self.edits.push(edit.clone());
        let mut changed = 0u64;
        for run in &edit.runs {
            for y in run.y..run.y + run.height as i32 {
                let pos = [run.x, y, run.z];
                let before = self.block_at(pos);
                let matches = match run.only {
                    Some(filter) => edit.filters[filter as usize] == before,
                    None => true,
                };
                if matches && before != edit.palette[run.block as usize] {
                    self.blocks
                        .insert(pos, edit.palette[run.block as usize].clone());
                    changed += 1;
                }
            }
        }
        Ok(changed)
    }

    /// The acting player's items: `item` in their inventory, or at their feet.
    pub fn native_give_items(&mut self, item: &str, count: u32) -> u32 {
        if self.acting.is_none() || count == 0 {
            return 0;
        }
        self.given.push((item.to_owned(), count));
        count
    }

    /// A line over that player's hotbar.
    pub fn native_action_bar(&mut self, uuid: u128, line: horizon_client::text::Text) -> bool {
        if !self.acting.as_ref().is_some_and(|p| p.uuid == uuid) {
            return false;
        }
        self.bars.push((uuid, line.plain()));
        true
    }

    /// A system line in that player's chat.
    pub fn native_tell(&mut self, uuid: u128, line: horizon_client::text::Text) -> bool {
        if !self.acting.as_ref().is_some_and(|p| p.uuid == uuid) {
            return false;
        }
        self.told.push((uuid, line.plain()));
        true
    }

    /// A line of the command's answer, inside `native_perform_command`.
    pub fn native_feedback(&mut self, line: horizon_client::text::Text) {
        self.feedback.push(line.plain());
    }

    /// A message to one player's client, from any server hook (API 40).
    pub fn native_net_send_to_player(
        &mut self,
        player: u128,
        channel: &str,
        data: &[u8],
    ) -> Result<(), String> {
        if !self.level_open {
            return Err("no level is open".to_owned());
        }
        self.net_sent
            .push((player, channel.to_owned(), data.to_vec()));
        Ok(())
    }

    /// A sound played once, right there, in the window's game (API 83).
    pub fn native_play_local_sound(
        &mut self,
        sound: &str,
        at: [f64; 3],
        volume: f32,
        pitch: f32,
        delay: i32,
    ) -> bool {
        self.sounds
            .push((sound.to_owned(), at, volume, pitch, delay));
        true
    }

    /// One flame spark, in the window's game; `soul` is the blue one.
    pub fn native_flame_particle(&mut self, at: [f64; 3], soul: bool) {
        self.particles.push((at, soul));
    }
}

/// The mod's own logic and state, added to the game as a module of `crate::game`.
#[path = "../../../native-mod/mixins/underbedrock_core/runtime.rs"]
pub mod underbedrock_core;
