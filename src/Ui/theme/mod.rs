//! Reusable UI widgets & theming.

// Unused utilities may trigger this lint undesirably.
#![allow(dead_code)]

pub mod palette;
pub mod widget;
pub mod panel_style;

use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {}