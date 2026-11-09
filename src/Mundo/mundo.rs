use bevy::prelude::*;

use crate::juego::forja::{bloquear, golpear, disparar_arco, Inventario, Material};
use crate::ui::menus::Menu;
use crate::ui::Mensaje;
use crate::ui::screens::gameplay::Jugador;
use crate::ui::screens::gameplay::VidaJugador;
use crate::ui::screens::gameplay::Progreso;
use crate::ui::screens::Screen;

/// Returns the index of the closest point within `alcance`, if any.
pub fn indice_mas_cercano(origen: Vec2, puntos: &[Vec2], alcance: f32) -> Option<usize> {
    puntos
        .iter()
        .enumerate()
        .map(|(i, p)| (i, origen.distance(*p)))
        .filter(|(_, d)| *d <= alcance)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(i, _)| i)
}

const ALCANCE_RECOLECTAR: f32 = 90.0;
const ALCANCE_ATAQUE: f32 = 110.0;
const ALCANCE_ENEMIGO: f32 = 80.0;
const CANTIDAD_RECOLECTADA: u32 = 2;
const VIDA_ENEMIGO: u32 = 100;
const DANO_ENEMIGO: u32 = 12;
const COOLDOWN_ENEMIGO: f32 = 1.5;
pub(crate) const EXP_ENEMIGO: u32 = 40;
pub(crate) const EXP_POR_NIVEL: u32 = 100;
const VIDA_POR_NIVEL: u32 = 10;
// TODO: provisional respawn times
const RESPAWN_RECURSO: f32 = 10.0;
const RESPAWN_ENEMIGO: f32 = 30.0;

pub struct MundoPlugin;

impl Plugin for MundoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Spawns>()
            .add_systems(OnEnter(Screen::Gameplay), preparar_mundo)
            .add_systems(
                Update,
                (
                    recolectar,
                    aplicar_golpe,
                    aplicar_disparo,
                    mover_flechas,
                    atacar_enemigo,
                    gestionar_respawn_individual,
                ).run_if(in_state(Screen::Gameplay).and_then(in_state(Menu::None))),
            );
    }
}

#[derive(Component)]
struct Recolectable {
    material: Material,
    respawn: Timer,
}

#[derive(Component)]
struct Enemigo {
    vida: u32,
    vida_max: u32,
    ataque: Timer,
    respawn: Timer,
    posicion_original: Vec2,
}

/// Flecha lanzada por el arco.
#[derive(Component)]
struct Flecha {
    direccion: Vec2,
    velocidad: f32,
    dano: u32,
    vida: Timer,
}

#[derive(Resource)]
struct Spawns {
    recolectables: Vec<(Material, Vec2, Vec2, Color, String)>,
    enemigos: Vec<Vec2>,
}

impl Default for Spawns {
    fn default() -> Self {
        Self {
            recolectables: vec![
                (Material::Madera, Vec2::new(-200.0, 160.0), Vec2::new(40.0, 60.0), Color::srgb(0.15, 0.45, 0.20), "Arbol (Madera)".to_string()),
                (Material::Madera, Vec2::new(-250.0, 200.0), Vec2::new(40.0, 60.0), Color::srgb(0.15, 0.45, 0.20), "Arbol (Madera)".to_string()),
                (Material::Madera, Vec2::new(-150.0, 120.0), Vec2::new(40.0, 60.0), Color::srgb(0.15, 0.45, 0.20), "Arbol (Madera)".to_string()),
                (Material::Hierro, Vec2::new(60.0, 120.0), Vec2::new(44.0, 34.0), Color::srgb(0.45, 0.48, 0.55), "Mineral (Hierro)".to_string()),
                (Material::Hierro, Vec2::new(100.0, 80.0), Vec2::new(44.0, 34.0), Color::srgb(0.45, 0.48, 0.55), "Mineral (Hierro)".to_string()),
                (Material::Hierro, Vec2::new(20.0, 160.0), Vec2::new(44.0, 34.0), Color::srgb(0.45, 0.48, 0.55), "Mineral (Hierro)".to_string()),
                (Material::Cuero, Vec2::new(-120.0, 40.0), Vec2::new(48.0, 30.0), Color::srgb(0.55, 0.36, 0.20), "Animal (Cuero)".to_string()),
                (Material::Cuero, Vec2::new(-80.0, 80.0), Vec2::new(48.0, 30.0), Color::srgb(0.55, 0.36, 0.20), "Animal (Cuero)".to_string()),
                (Material::Cuero, Vec2::new(-160.0, 0.0), Vec2::new(48.0, 30.0), Color::srgb(0.55, 0.36, 0.20), "Animal (Cuero)".to_string()),
            ],
            enemigos: vec![Vec2::new(170.0, 40.0)],
        }
    }
}



fn preparar_mundo(mut commands: Commands, spawns: ResMut<Spawns>) {
    for (material, posicion, tamano, color, etiqueta) in &spawns.recolectables {
        spawn_recolectable(&mut commands, *material, *posicion, *tamano, *color, etiqueta);
    }

    for posicion in &spawns.enemigos {
        spawn_enemigo(&mut commands, *posicion);
    }
}

fn spawn_recolectable(
    commands: &mut Commands,
    material: Material,
    posicion: Vec2,
    tamano: Vec2,
    color: Color,
    _etiqueta: &str,
) {
    commands.spawn((
        Sprite::from_color(color, tamano),
        Transform::from_xyz(posicion.x, posicion.y, 0.5),
        Recolectable {
            material,
            respawn: Timer::from_seconds(RESPAWN_RECURSO, TimerMode::Once),
        },
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn spawn_enemigo(commands: &mut Commands, posicion: Vec2) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.70, 0.15, 0.15), Vec2::new(48.0, 64.0)),
        Transform::from_xyz(posicion.x, posicion.y, 1.0),
        Enemigo {
            vida: VIDA_ENEMIGO,
            vida_max: VIDA_ENEMIGO,
            ataque: Timer::from_seconds(COOLDOWN_ENEMIGO, TimerMode::Repeating),
            respawn: Timer::from_seconds(RESPAWN_ENEMIGO, TimerMode::Once),
            posicion_original: posicion,
        },
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn recolectar(
    teclas: Res<ButtonInput<KeyCode>>,
    jugador: Query<&Transform, With<Jugador>>,
    mut recursos: Query<(Entity, &Transform, &mut Recolectable, &mut Visibility)>,
    mut inventario: ResMut<Inventario>,
    mut mensaje: ResMut<Mensaje>,
) {
    if !teclas.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Some(pos_jugador) = jugador.iter().next().map(|t| t.translation.truncate()) else {
        return;
    };

    let mut posiciones = Vec::new();
    let mut entidades = Vec::new();
    let mut materiales = Vec::new();
    for (entidad, transform, recurso, visibility) in &mut recursos {
        // Only visible (not collected) resources
        if matches!(*visibility, Visibility::Hidden) {
            continue;
        }
        posiciones.push(transform.translation.truncate());
        entidades.push(entidad);
        materiales.push(recurso.material);
    }

    if let Some(indice) = indice_mas_cercano(pos_jugador, &posiciones, ALCANCE_RECOLECTAR) {
        let entidad = entidades[indice];
        let material = materiales[indice];
        *inventario.materiales.entry(material).or_insert(0) += CANTIDAD_RECOLECTADA;
        mensaje.0 = format!("Recolectado: +{} {}", CANTIDAD_RECOLECTADA, material);

        // Hide the resource and schedule its respawn
        if let Ok((_, _, mut recurso, mut visibility)) = recursos.get_mut(entidad) {
            recurso.respawn.reset();
            *visibility = Visibility::Hidden;
        }
    } else {
        mensaje.0 = "No hay nada cerca para recolectar".to_string();
    }
}

/// Suma experiencia. Devuelve el nuevo nivel si el jugador sube.
fn ganar_exp(progreso: &mut Progreso, vida: &mut VidaJugador, cantidad: u32) -> Option<u32> {
    progreso.exp += cantidad;
    let mut subio = false;

    while progreso.exp >= progreso.nivel * EXP_POR_NIVEL {
        progreso.exp -= progreso.nivel * EXP_POR_NIVEL;
        progreso.nivel += 1;
        vida.maximo += VIDA_POR_NIVEL;
        subio = true;
    }

    if subio {
        vida.actual = vida.maximo;
        Some(progreso.nivel)
    } else {
        None
    }
}

/// Check if a node can respawn given the elapsed time.
fn puede_reaparecer(transcurrido: f32, tiempo_respawn: f32) -> bool {
    transcurrido >= tiempo_respawn
}

/// Respawn resources and enemies individually.
fn gestionar_respawn_individual(
    tiempo: Res<Time>,
    mut recursos: Query<(&mut Recolectable, &mut Visibility), Without<Enemigo>>,
    mut enemigos: Query<(&mut Enemigo, &mut Transform, &mut Visibility), (With<Enemigo>, Without<Recolectable>)>,
) {
    // Respawn resources
    for (mut recurso, mut visibility) in &mut recursos {
        if matches!(*visibility, Visibility::Hidden) {
            recurso.respawn.tick(tiempo.delta());
            if puede_reaparecer(recurso.respawn.elapsed_secs(), RESPAWN_RECURSO) {
                *visibility = Visibility::Inherited;
                recurso.respawn.reset();
            }
        }
    }

    // Respawn enemies (only those that died and are hidden)
    for (mut enemigo, mut transform, mut visibility) in &mut enemigos {
        if matches!(*visibility, Visibility::Hidden) {
            enemigo.respawn.tick(tiempo.delta());
            if puede_reaparecer(enemigo.respawn.elapsed_secs(), RESPAWN_ENEMIGO) {
                // Respawn at the original position with full health
                transform.translation.x = enemigo.posicion_original.x;
                transform.translation.y = enemigo.posicion_original.y;
                enemigo.vida = enemigo.vida_max;
                enemigo.ataque.reset();
                *visibility = Visibility::Inherited;
                enemigo.respawn.reset();
            }
        }
    }
}

/// Apply melee damage when the player hits the attack frame.
fn aplicar_golpe(
    mut jugadores: Query<(&mut Jugador, &Transform)>,
    mut enemigos: Query<(&Transform, &mut Enemigo, &mut Visibility)>,
    mut inventario: ResMut<Inventario>,
    mut progreso: ResMut<Progreso>,
    mut vida: ResMut<VidaJugador>,
    mut mensaje: ResMut<Mensaje>,
) {
    let Some((mut jugador, transform)) = jugadores.iter_mut().next() else {
        return;
    };

    if !jugador.golpe {
        return;
    }
    jugador.golpe = false;

    // With a bow equipped, there is no melee damage or durability loss per hit.
    let con_arco = inventario.arma_equipada().map(|i| i.tipo) == Some(crate::juego::forja::Objeto::Arco);
    if con_arco {
        return;
    }

    let pos_jugador = transform.translation.truncate();

    let mut en_rango = false;
    for (transform, mut enemigo, mut visibility) in &mut enemigos {
        // Skip hidden (dead) enemies
        if matches!(*visibility, Visibility::Hidden) {
            continue;
        }
        if transform.translation.truncate().distance(pos_jugador) > ALCANCE_ATAQUE {
            continue;
        }
        en_rango = true;

        let (dano, rota) = golpear(&mut inventario);
        enemigo.vida = enemigo.vida.saturating_sub(dano);

        let mut texto = format!("Golpe: {} de dano", dano);
        if rota {
            texto.push_str(". El arma se ha roto!");
        }
        if enemigo.vida == 0 {
            // Schedule enemy respawn
            *visibility = Visibility::Hidden;
            enemigo.respawn.reset();
            *inventario
                .materiales
                .entry(Material::PiedraMagica)
                .or_insert(0) += 1;

            texto.push_str(&format!(
                ". Enemigo derrotado! +{} exp y 1 Piedra magica",
                EXP_ENEMIGO
            ));
            if let Some(nivel) = ganar_exp(&mut progreso, &mut vida, EXP_ENEMIGO) {
                texto.push_str(&format!(". Subes a nivel {}!", nivel));
            }
        }
        mensaje.0 = texto;
    }

    if !en_rango {
        mensaje.0 = "No hay ningun enemigo cerca".to_string();
    }
}

/// Apply bow shot when the player releases the arrow frame.
fn aplicar_disparo(
    mut jugadores: Query<(&mut Jugador, &Transform)>,
    mut inventario: ResMut<Inventario>,
    mut mensaje: ResMut<Mensaje>,
    mut commands: Commands,
) {
    let Some((mut jugador, transform)) = jugadores.iter_mut().next() else {
        return;
    };

    if !jugador.disparo {
        return;
    }
    jugador.disparo = false;

    // Without arrows, the shot does not fire.
    if inventario.cantidad(Material::Flecha) == 0 {
        mensaje.0 = "No te quedan flechas".to_string();
        return;
    }

    match disparar_arco(&mut inventario) {
        Ok((dano, rota)) => {
            let pos = transform.translation.truncate();
            let direccion = if jugador.mira_izquierda { Vec2::new(-1.0, 0.0) } else { Vec2::new(1.0, 0.0) };
            commands.spawn((
                Flecha {
                    direccion,
                    velocidad: 400.0,
                    dano,
                    vida: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Sprite::from_color(Color::srgb(0.9, 0.8, 0.2), Vec2::new(20.0, 4.0)),
                Transform::from_xyz(pos.x, pos.y, 1.5),
                DespawnOnExit(Screen::Gameplay),
            ));

            let mut texto = format!("Flecha lanzada: {} de dano", dano);
            if rota {
                texto.push_str(". El arco se ha roto!");
            }
            mensaje.0 = texto;
        }
        Err(error) => {
            mensaje.0 = error;
        }
    }
}

fn atacar_enemigo(
    tiempo: Res<Time>,
    jugador: Query<&Transform, With<Jugador>>,
    mut enemigos: Query<(&Transform, &mut Enemigo, &Visibility)>,
    mut vida: ResMut<VidaJugador>,
    mut inventario: ResMut<Inventario>,
    mut mensaje: ResMut<Mensaje>,
) {
    let Some(pos_jugador) = jugador.iter().next().map(|t| t.translation.truncate()) else {
        return;
    };

    for (transform, mut enemigo, visibility) in &mut enemigos {
        // Skip hidden (dead) enemies
        if matches!(*visibility, Visibility::Hidden) {
            continue;
        }
        let cerca = transform.translation.truncate().distance(pos_jugador) <= ALCANCE_ENEMIGO;
        if !cerca {
            enemigo.ataque.reset();
            continue;
        }

        enemigo.ataque.tick(tiempo.delta());
        if !enemigo.ataque.just_finished() {
            continue;
        }

        let (dano, escudo_roto) = bloquear(&mut inventario, DANO_ENEMIGO);
        let bloqueado = dano < DANO_ENEMIGO;
        vida.actual = vida.actual.saturating_sub(dano);

        let mut texto = format!("El enemigo te golpea: -{}", dano);
        if bloqueado {
            texto.push_str(" (escudo)");
        }
        if escudo_roto {
            texto.push_str(". El escudo se ha roto!");
        }
        if vida.actual == 0 {
            vida.actual = vida.maximo;
            texto = "El enemigo te derrota. Reapareces con vida completa".to_string();
        }
        mensaje.0 = texto;
    }
}

/// Move arrows and apply damage when they hit enemies.
fn mover_flechas(
    tiempo: Res<Time>,
    mut commands: Commands,
    mut flechas: Query<(Entity, &mut Flecha, &mut Transform), Without<Enemigo>>,
    mut enemigos: Query<(&Transform, &mut Enemigo, &mut Visibility)>,
    mut progreso: ResMut<Progreso>,
    mut vida: ResMut<VidaJugador>,
    mut mensaje: ResMut<Mensaje>,
) {
    for (entidad, mut flecha, mut transform) in &mut flechas {
        flecha.vida.tick(tiempo.delta());
        if flecha.vida.is_finished() {
            commands.entity(entidad).despawn();
            continue;
        }

        let paso = flecha.direccion * flecha.velocidad * tiempo.delta_secs();
        transform.translation.x += paso.x;
        transform.translation.y += paso.y;

        let pos_flecha = transform.translation.truncate();
        for (transform_enemigo, mut enemigo, mut visibility) in &mut enemigos {
            // Skip hidden (dead) enemies
            if matches!(*visibility, Visibility::Hidden) {
                continue;
            }
            let distancia = transform_enemigo.translation.truncate().distance(pos_flecha);
            if distancia <= 30.0 {
                enemigo.vida = enemigo.vida.saturating_sub(flecha.dano);
                commands.entity(entidad).despawn();

                let mut texto = format!("Flecha impacta: {} de dano", flecha.dano);
                if enemigo.vida == 0 {
                    // Schedule enemy respawn
                    *visibility = Visibility::Hidden;
                    enemigo.respawn.reset();
                    texto.push_str(&format!(". Enemigo derrotado! +{} exp", EXP_ENEMIGO));
                    if let Some(nivel) = ganar_exp(&mut progreso, &mut vida, EXP_ENEMIGO) {
                        texto.push_str(&format!(". Subes a nivel {}!", nivel));
                    }
                }
                mensaje.0 = texto;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puede_reaparecer() {
        // Not enough time elapsed
        assert!(!puede_reaparecer(5.0, 10.0));
        // Exactly the respawn time
        assert!(puede_reaparecer(10.0, 10.0));
        // More than enough time elapsed
        assert!(puede_reaparecer(12.5, 10.0));
    }

    #[test]
    fn test_indice_mas_cercano_elige_mas_cercano() {
        let origen = Vec2::new(0.0, 0.0);
        let puntos = vec![
            Vec2::new(50.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(30.0, 0.0),
        ];
        let alcance = 100.0;
        let resultado = indice_mas_cercano(origen, &puntos, alcance);
        assert_eq!(resultado, Some(1)); // Índice 1 es el más cercano (10.0)
    }

    #[test]
    fn test_indice_mas_cercano_none_si_ninguno_en_rango() {
        let origen = Vec2::new(0.0, 0.0);
        let puntos = vec![
            Vec2::new(200.0, 0.0),
            Vec2::new(300.0, 0.0),
        ];
        let alcance = 100.0;
        let resultado = indice_mas_cercano(origen, &puntos, alcance);
        assert_eq!(resultado, None);
    }

    #[test]
    fn test_indice_mas_cercano_un_solo_en_rango() {
        let origen = Vec2::new(0.0, 0.0);
        let puntos = vec![
            Vec2::new(50.0, 0.0),
            Vec2::new(200.0, 0.0),
            Vec2::new(300.0, 0.0),
        ];
        let alcance = 100.0;
        let resultado = indice_mas_cercano(origen, &puntos, alcance);
        assert_eq!(resultado, Some(0)); // Solo el primero está en rango
    }
}
