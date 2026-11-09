use bevy::prelude::*;

pub mod main;
pub mod pause;
pub mod quest_panel;
pub mod weapon_panel;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Pause,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
    app.add_plugins((main::plugin, pause::plugin));
}