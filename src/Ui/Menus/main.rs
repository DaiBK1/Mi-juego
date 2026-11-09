//! The main menu (seen on the title screen).

use bevy::{prelude::*, ui_widgets::Activate};

use crate::{
    recursos::asset_tracking::ResourceHandles,
    ui::{menus::Menu, screens::Screen, theme::widget},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            widget::ui_root("Main Menu"),
            GlobalZIndex(2),
            DespawnOnExit(Menu::Main),
        ))
        .with_children(|p| {
            p.spawn(widget::button("Play")).observe(enter_loading_or_gameplay_screen);
            p.spawn(widget::button("Exit")).observe(exit_app);
        });
}

fn enter_loading_or_gameplay_screen(
    _: On<Activate>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn exit_app(_: On<Activate>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}