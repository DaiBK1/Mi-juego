pub mod gameplay;
mod loading;
mod title;

use bevy::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Title,
    Loading,
    Gameplay,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((gameplay::plugin, loading::plugin, title::plugin));
}