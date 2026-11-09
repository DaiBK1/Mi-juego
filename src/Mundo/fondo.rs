/// Map background: grassy ground with tiles, and a sky with a gradient and clouds.

use bevy::prelude::*;

use crate::ui::screens::Screen;

const LADO_BALDOSA: f32 = 200.0;
const XMIN: f32 = -2400.0;
const XMAX: f32 = 2400.0;
const YMIN_SUELO: f32 = -1800.0;
/// Horizon line: below it is the ground, above it the sky.
pub const HORIZONTE: f32 = 800.0;
const YMAX_CIELO: f32 = 2000.0;
const BANDAS_CIELO: i32 = 6;
const NUBES: [(f32, f32, f32); 5] = [
    (-1500.0, 1250.0, 1.0),
    (-300.0, 1500.0, 1.3),
    (700.0, 1150.0, 0.9),
    (1600.0, 1450.0, 1.1),
    (-1900.0, 1000.0, 0.8),
];

pub struct FondoPlugin;

impl Plugin for FondoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), preparar_fondo);
    }
}

fn preparar_fondo(mut commands: Commands) {
    spawn_suelo(&mut commands);
    spawn_cielo(&mut commands);
}

/// Deterministic noise between -0.5 and 0.5 to vary the tone of each tile.
fn ruido(i: i32, j: i32) -> f32 {
    let h = (i.wrapping_mul(73_856_093) ^ j.wrapping_mul(19_349_663)) as u32;
    let h = h.wrapping_mul(2_654_435_761) >> 24;
    h as f32 / 255.0 - 0.5
}

fn spawn_suelo(commands: &mut Commands) {
    let columnas = ((XMAX - XMIN) / LADO_BALDOSA) as i32;
    let filas = ((HORIZONTE - YMIN_SUELO) / LADO_BALDOSA) as i32;

    for j in 0..filas {
        for i in 0..columnas {
            let tono = if (i + j) % 2 == 0 { 0.0 } else { 0.03 };
            let v = ruido(i, j) * 0.08;
            let color = Color::srgb(
                0.20 + tono + v * 0.5,
                0.44 + tono + v,
                0.18 + tono * 0.5 + v * 0.4,
            );

            let x = XMIN + i as f32 * LADO_BALDOSA + LADO_BALDOSA / 2.0;
            let y = YMIN_SUELO + j as f32 * LADO_BALDOSA + LADO_BALDOSA / 2.0;

            commands.spawn((
                Sprite::from_color(color, Vec2::splat(LADO_BALDOSA)),
                Transform::from_xyz(x, y, -10.0),
                DespawnOnExit(Screen::Gameplay),
            ));
        }
    }
}

fn mezcla(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn spawn_cielo(commands: &mut Commands) {
    let alto = (YMAX_CIELO - HORIZONTE) / BANDAS_CIELO as f32;
    // Color near the horizon and color high in the sky
    let cerca = (0.70, 0.86, 0.96);
    let arriba = (0.30, 0.55, 0.86);

    for k in 0..BANDAS_CIELO {
        let t = k as f32 / (BANDAS_CIELO - 1) as f32;
        let color = Color::srgb(
            mezcla(cerca.0, arriba.0, t),
            mezcla(cerca.1, arriba.1, t),
            mezcla(cerca.2, arriba.2, t),
        );
        let y = HORIZONTE + k as f32 * alto + alto / 2.0;

        commands.spawn((
            Sprite::from_color(color, Vec2::new(XMAX - XMIN, alto)),
            Transform::from_xyz(0.0, y, -10.5),
            DespawnOnExit(Screen::Gameplay),
        ));
    }

    for (x, y, escala) in NUBES {
        spawn_nube(commands, x, y, escala);
    }
}

fn spawn_nube(commands: &mut Commands, x: f32, y: f32, escala: f32) {
    let blanco = Color::srgba(1.0, 1.0, 1.0, 0.9);
    let partes = [
        (0.0, 0.0, 260.0, 60.0),
        (-70.0, 35.0, 140.0, 50.0),
        (80.0, 28.0, 120.0, 44.0),
    ];

    for (dx, dy, ancho, alto) in partes {
        commands.spawn((
            Sprite::from_color(blanco, Vec2::new(ancho * escala, alto * escala)),
            Transform::from_xyz(x + dx * escala, y + dy * escala, -9.5),
            DespawnOnExit(Screen::Gameplay),
        ));
    }
}