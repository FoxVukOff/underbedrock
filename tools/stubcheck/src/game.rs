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
            Self { usage: usage.to_owned(), level: 0 }
        }
        pub fn op(usage: &str, level: u8) -> Self {
            Self { usage: usage.to_owned(), level: level.min(4) }
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
            marker: std::marker::PhantomData,
        }
    }

    /// The dig the engine keeps while the attack key is held on `pos`.
    pub fn start_dig(&mut self, pos: [i32; 3], ticks: u32) {
        self.mining = Some(Mining { pos, progress: 0.0, ticks });
    }

    /// The player let go, looked away or the dig finished.
    pub fn stop_dig(&mut self) {
        self.mining = None;
    }

    /// A setting's file, as `native_set_setting` writes it.
    pub fn setting_file(&self, mod_id: &str, key: &str) -> Option<String> {
        self.settings.borrow().get(&format!("{mod_id}/{key}")).cloned()
    }

    /// The last bulk edit's palette, filters and run count.
    pub fn last_edit(&self) -> Option<(Vec<String>, Vec<String>, usize)> {
        self.edits.last().map(|e| (e.palette.clone(), e.filters.clone(), e.runs.len()))
    }

    /// The block at `pos`, as the world keeps it.
    pub fn block_at(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).cloned().unwrap_or_else(|| "minecraft:air".into())
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

    /// The block a state is a state of.
    pub fn native_state_block(&self, state: horizon_world::registry::BlockStateId) -> Option<&str> {
        match (self.air_at_zero, state.0) {
            (true, 0) => Some("minecraft:air"),
            _ => None,
        }
    }

    /// A mod's setting `key` (one file per key).
    pub fn native_setting(&self, mod_id: &str, key: &str) -> Option<String> {
        self.settings.borrow().get(&format!("{mod_id}/{key}")).cloned()
    }

    /// Keeps a mod's setting `key`.
    pub fn native_set_setting(&self, mod_id: &str, key: &str, value: &str) {
        self.settings.borrow_mut().insert(format!("{mod_id}/{key}"), value.to_owned());
    }

    /// Sets a block in the level being ticked, through `Level.setBlock`.
    pub fn native_set_block(
        &mut self,
        dimension: &str,
        pos: [i32; 3],
        state: horizon_world::registry::BlockStateId,
        flags: i32,
    ) -> bool {
        let Some(acting) = &self.acting else { return false };
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
        if edit.runs.iter().any(|r| r.block as usize >= edit.palette.len())
            || edit.runs.iter().any(|r| r.only.is_some_and(|f| f as usize >= edit.filters.len()))
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
                    self.blocks.insert(pos, edit.palette[run.block as usize].clone());
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
}

/// The mod's own logic and state, added to the game as a module of `crate::game`.
#[path = "../../../native-mod/mixins/underbedrock_core/runtime.rs"]
pub mod underbedrock_core;
