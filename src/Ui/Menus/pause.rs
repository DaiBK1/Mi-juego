//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui_widgets::Activate};

use crate::ui::{menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        GlobalZIndex(1),
        DespawnOnExit(Menu::Pause),
    ));

    commands
        .spawn((
            widget::ui_root("Pause Menu"),
            GlobalZIndex(2),
            DespawnOnExit(Menu::Pause),
        ))
        .with_children(|p| {
            p.spawn(widget::header("Game paused"));
            p.spawn(widget::button("Continue")).observe(close_menu);
            p.spawn(widget::button("Quit to title")).observe(quit_to_title);
        });
}

fn close_menu(_: On<Activate>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: On<Activate>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}