//! Helper functions for creating common widgets.

use bevy::{prelude::*, text::FontSize, ui_widgets::Button};

use super::palette::*;

/// Root element that occupies the entire window and centers its content in a column.
pub fn ui_root(nombre: &'static str) -> impl Bundle {
    (
        Name::new(nombre),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
    )
}

/// Large title.
pub fn header(texto: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text::new(texto),
        TextFont {
            font_size: FontSize::Px(40.0),
            ..default()
        },
        TextColor(HEADER_TEXT),
    )
}

/// Simple text.
pub fn label(texto: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text::new(texto),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(LABEL_TEXT),
    )
}

/// Large button. The action is added using `.observe(function)` on the result of `spawn`.
pub fn button(texto: impl Into<String>) -> impl Bundle {
    (
        Name::new("Button"),
        Button,
        Node {
            width: Val::Px(380.0),
            height: Val::Px(80.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BUTTON_BACKGROUND),
        children![(
            Text::new(texto),
            TextFont {
                font_size: FontSize::Px(40.0),
                ..default()
            },
            TextColor(BUTTON_TEXT),
        )],
    )
}