//! A loading screen that waits for the game's assets.

use bevy::prelude::*;

use crate::{
    recursos::asset_tracking::ResourceHandles,
    ui::{screens::Screen, theme::widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(
        Update,
        enter_gameplay_screen.run_if(in_state(Screen::Loading)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        DespawnOnExit(Screen::Loading),
        children![widget::label("Cargando...")],
    ));
}

fn enter_gameplay_screen(
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    }
}