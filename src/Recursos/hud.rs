use bevy::prelude::*;

#[derive(Resource)]
pub struct HudTextures {
    pub red_bar: Handle<Image>,
    pub hp_bar: Handle<Image>,
    pub yellow_bar: Handle<Image>,
    pub blue_bar: Handle<Image>,
}

pub fn preparar_hud(mut commands: Commands, assets: Res<AssetServer>) {
    let red_bar = assets.load("hud/red_bar.png");
    let hp_bar = assets.load("hud/Hp_bar.png");
    let yellow_bar = assets.load("hud/yellow_bar.png");
    let blue_bar = assets.load("hud/Blue_bar.png");

    commands.insert_resource(HudTextures {
        red_bar,
        hp_bar,
        yellow_bar,
        blue_bar,
    });
}
