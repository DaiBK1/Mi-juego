mod juego;
mod mundo;
mod recursos;
mod ui;

use bevy::prelude::*;

use juego::clases::ClaseActual;
use juego::forja::{cargar_datos, recetas, Inventario, Recetas};
use juego::personajes::preparar_personajes;
use mundo::fondo::FondoPlugin;
use mundo::mundo::MundoPlugin;
use recursos::asset_tracking;
use recursos::hud::preparar_hud;
use recursos::iconos::preparar_iconos;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            asset_tracking::plugin,
            UiPlugin,
        ))
	        .add_plugins((MundoPlugin, FondoPlugin))
        .insert_resource(ClearColor(Color::srgb(0.07, 0.07, 0.10)))
        .insert_resource(Inventario::inicial())
        .insert_resource(Recetas(recetas()))
        .insert_resource(cargar_datos())
        .insert_resource(ClaseActual::default())
        .add_systems(
            Startup,
            (preparar_personajes, preparar_iconos, preparar_hud, preparar_camara).chain(),
        )
        .run();
}

fn preparar_camara(mut commands: Commands) {
    commands.spawn(Camera2d);
}