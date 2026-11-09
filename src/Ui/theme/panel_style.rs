use bevy::prelude::*;

// Colors taken from the reference menus
pub const PANEL_BG: Color = Color::srgba(0.20, 0.29, 0.37, 0.85);
pub const LIGHT_PANEL_BG: Color = Color::srgba(0.35, 0.51, 0.67, 0.75);
pub const BORDER_COLOR: Color = Color::srgb(0.56, 0.78, 0.94);
pub const TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
pub const GOLD: Color = Color::srgb(0.95, 0.76, 0.31);

#[derive(Clone, Copy, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    /// Display name shown in the UI.
    pub fn name(self) -> &'static str {
        match self {
            Rarity::Common => "Común",
            Rarity::Uncommon => "Poco común",
            Rarity::Rare => "Raro",
            Rarity::Epic => "Épico",
            Rarity::Legendary => "Legendario",
        }
    }

    /// Color used for the rarity label and the item name.
    pub fn color(self) -> Color {
        match self {
            Rarity::Common => Color::srgb(0.75, 0.75, 0.75),
            Rarity::Uncommon => Color::srgb(0.40, 0.85, 0.40),
            Rarity::Rare => Color::srgb(0.30, 0.60, 1.00),
            Rarity::Epic => Color::srgb(0.70, 0.40, 0.95),
            Rarity::Legendary => GOLD,
        }
    }
}

/// Bordered, translucent panel. `layout` only contributes position and size;
/// padding, flex direction and border are set here.
pub fn panel_node(layout: Node, color: Color) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..layout
        },
        BackgroundColor(color),
        BorderColor::all(BORDER_COLOR),
    )
}

/// One centered line of text.
pub fn text_line(text: impl Into<String>, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: bevy::text::FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    )
}

/// Thin horizontal accent bar used on the top and bottom of scroll-style panels.
pub fn accent_bar() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(BORDER_COLOR),
    )
}

/// Bottom-center subtitle with a black drop shadow.
pub fn spawn_subtitle(commands: &mut Commands, text: &str) -> Entity {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: bevy::text::FontSize::Px(28.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
                TextShadow {
                    offset: Vec2::splat(2.0),
                    color: Color::BLACK,
                },
            ));
        })
        .id()
}