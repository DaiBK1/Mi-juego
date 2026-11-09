use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::juego::forja::Objeto;
use crate::juego::personajes::generar_icono;

const ANCHO: usize = 16;

// K contorno, W brillo, B azul, C crema, M mango/madera, G guarda dorada, . transparente
// K outline, W gloss, B blue, C cream, M mango/wood, G gold guard, . transparent
pub const ESPADA_ARTE: &[&str] = &[
    ".......KK.......",
    "......KWWK......",
    "......KWBK......",
    "......KWBK......",
    "......KWBK......",
    "......KWBK......",
    "......KWBK......",
    "......KWBK......",
    "....KKKKKKKK....",
    "...KGGGGGGGGK...",
    "....KKKKKKKK....",
    ".......KMK......",
    ".......KMK......",
    ".......KMK......",
    ".......KKK......",
    "................",
];

pub const ESCUDO_ARTE: &[&str] = &[
    "..KKKKKKKKKKKK..",
    ".KWWWWWWWWWWWWK.",
    ".KWBBBBBBBBBBWK.",
    ".KWBBBCCCCBBBWK.",
    ".KWBBCCCCCCBBWK.",
    ".KWBCCCCCCCCBWK.",
    ".KWBBCCCCCCBBWK.",
    ".KWBBBCCCCBBBWK.",
    ".KWBBBBBBBBBBWK.",
    "..KWWWWWWWWWWK..",
    "..KWBBBBBBBBWK..",
    "...KWBBBBBBWK...",
    "...KWBBCCBBWK...",
    "....KWBCCBWK....",
    ".....KWBBWK.....",
    ".......KK.......",
];

#[derive(Resource)]
pub struct Iconos(HashMap<&'static str, Handle<Image>>);

impl Iconos {
    pub fn get(&self, ruta: &str) -> Option<Handle<Image>> {
        self.0.get(ruta).cloned()
    }
}

pub fn preparar_iconos(mut commands: Commands, mut imagenes: ResMut<Assets<Image>>) {
    let mut mapa = HashMap::new();
    mapa.insert("espada.png", crear_imagen(ESPADA_ARTE, &mut imagenes));
    mapa.insert("escudo.png", crear_imagen(ESCUDO_ARTE, &mut imagenes));
    mapa.insert("hacha.png", generar_icono(Objeto::Hacha, &mut imagenes));
    mapa.insert("arco.png", generar_icono(Objeto::Arco, &mut imagenes));
    mapa.insert("lanza.png", generar_icono(Objeto::Lanza, &mut imagenes));
    commands.insert_resource(Iconos(mapa));
}

fn crear_imagen(filas: &[&str], imagenes: &mut Assets<Image>) -> Handle<Image> {
    let alto = filas.len();
    let mut datos = vec![0u8; ANCHO * alto * 4];

    for (y, fila) in filas.iter().enumerate() {
        for (x, c) in fila.chars().take(ANCHO).enumerate() {
            let i = (y * ANCHO + x) * 4;
            datos[i..i + 4].copy_from_slice(&color_arte(c));
        }
    }

    let imagen = Image::new(
        Extent3d {
            width: ANCHO as u32,
            height: alto as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        datos,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    imagenes.add(imagen)
}

pub fn color_arte(c: char) -> [u8; 4] {
    match c {
        'K' => [26, 42, 58, 255],
        'W' => [232, 251, 255, 255],
        'B' => [95, 184, 232, 255],
        'C' => [240, 230, 190, 255],
        'M' => [122, 74, 42, 255],
        'G' => [224, 176, 64, 255],
        _ => [0, 0, 0, 0],
    }
}