#![allow(dead_code)]

use bevy::prelude::*;

use crate::ui::theme::panel_style::*;

/// Data shown in the quest panel.
pub struct QuestInfo {
    pub title: String,
    pub name: String,
    pub description: String,
    pub objective: String,
    pub failure: String,
}

/// Spawns the quest panel, normally placed top-right via `layout`.
pub fn spawn_quest_panel(commands: &mut Commands, info: &QuestInfo, layout: Node) -> Entity {
    commands
        .spawn(panel_node(layout, LIGHT_PANEL_BG))
        .with_children(|parent| {
            parent.spawn(text_line(&info.title, 16.0, TEXT_COLOR));
            parent.spawn(text_line(format!("<{}>", info.name), 20.0, TEXT_COLOR));
            parent.spawn(text_line(&info.description, 16.0, TEXT_COLOR));
            parent.spawn(text_line(format!("Objetivo: {}", info.objective), 16.0, TEXT_COLOR));
            parent.spawn(text_line(format!("Fracaso: {}", info.failure), 14.0, GOLD));
        })
        .id()
}