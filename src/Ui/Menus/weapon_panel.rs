#![allow(dead_code)]

use bevy::prelude::*;

use crate::ui::theme::panel_style::*;

/// Data shown in the weapon detail panel.
pub struct WeaponInfo {
    pub name: String,
    pub rarity: Rarity,
    /// Pairs of (label, value), e.g. ("Ataque", "356").
    pub stats: Vec<(String, String)>,
    /// Optional skill granted by the weapon.
    pub skill: Option<String>,
}

/// Spawns the weapon detail panel. `layout` sets position and width.
pub fn spawn_weapon_panel(commands: &mut Commands, info: &WeaponInfo, layout: Node) -> Entity {
    commands
        .spawn(panel_node(layout, PANEL_BG))
        .with_children(|parent| {
            // Item name in its rarity color
            parent.spawn(text_line(&info.name, 24.0, info.rarity.color()));

            // Rarity line
            parent.spawn(text_line(
                format!("Rango: {}", info.rarity.name()),
                16.0,
                info.rarity.color(),
            ));

            // Stat lines, centered
            for (label, value) in &info.stats {
                parent.spawn(text_line(format!("{label}: {value}"), 18.0, TEXT_COLOR));
            }

            // Granted skill
            if let Some(skill) = &info.skill {
                parent.spawn(text_line(
                    format!("*Genera la habilidad <{skill}>"),
                    16.0,
                    GOLD,
                ));
            }
        })
        .id()
}