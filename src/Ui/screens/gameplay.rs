//! Pantalla de juego: un solo personaje controlado por el jugador y su camara.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    juego::{
        forja::{desequipar_incompatibles, nombre_clase, Inventario, Material, Objeto},
        personaje_atlas::{capas, rango, Accion, Clase},
        personajes::Personaje,
    },
    recursos::hud::HudTextures,
    ui::{menus::Menu, screens::Screen, Mensaje},
};
use crate::mundo::mundo::EXP_POR_NIVEL;

/// Velocidad al caminar y al correr, en pixeles por segundo.
const VELOCIDAD: f32 = 300.0;
const VELOCIDAD_CORRER: f32 = 450.0;
const ESCALA: f32 = 2.0;
/// Limites del terreno: el personaje no puede salir de ellos.
const LIMITE_X: f32 = 1200.0;
const LIMITE_Y_MIN: f32 = -1700.0;
const LIMITE_Y_MAX: f32 = 760.0;
/// Velocidad con la que la camara sigue al personaje (mayor = mas pegada).
const VELOCIDAD_CAMARA: f32 = 6.0;
/// Frame del ataque (contando desde el inicio) en el que sale la flecha.
const FRAME_SUELTA_FLECHA: usize = 5;
/// Frame del ataque cuerpo a cuerpo en el que se aplica el daño.
const FRAME_GOLPE: usize = 3;
/// Drenaje de estamina por segundo al correr.
const DRENAJE_ESTAMINA: f32 = 30.0;
/// Regeneración de estamina por segundo cuando no se corre.
const REGENERACION_ESTAMINA: f32 = 15.0;
/// Tiempo de recarga del arco en segundos.
const RECARGA_ARCO: f32 = 1.0;
/// HUD dimensions.
const S: f32 = 3.0;
const MARCO_W: f32 = 116.0 * S;
const MARCO_H: f32 = 64.0 * S;
const ORBE_W: f32 = 56.0 * S;
const ORBE_H: f32 = 54.0 * S;
const ESTAMINA_W: f32 = 80.0 * S;
const ESTAMINA_H: f32 = 8.0 * S;
const MANA_W: f32 = 60.0 * S;
const MANA_H: f32 = 6.0 * S;
const XP_H: f32 = 6.0 * S;

/// Drain stamina over time. Clamped between 0 and maximo.
fn drenar_estamina(actual: f32, dt: f32, maximo: u32) -> f32 {
    (actual - DRENAJE_ESTAMINA * dt).max(0.0).min(maximo as f32)
}

/// Regenerate stamina over time. Clamped between 0 and maximo.
fn regenerar_estamina(actual: f32, dt: f32, maximo: u32) -> f32 {
    (actual + REGENERACION_ESTAMINA * dt).max(0.0).min(maximo as f32)
}

/// Calculate XP progress ratio for the XP bar.
fn ratio_exp(exp: u32, nivel: u32) -> f32 {
    if nivel == 0 {
        return 0.0;
    }
    let requerido = nivel * EXP_POR_NIVEL;
    (exp as f32 / requerido as f32).clamp(0.0, 1.0)
}

/// Vida del jugador que muestra la barra roja del HUD.
#[derive(Resource, Clone)]
pub struct VidaJugador {
    pub actual: u32,
    pub maximo: u32,
}

impl Default for VidaJugador {
    fn default() -> Self {
        Self {
            actual: 100,
            maximo: 100,
        }
    }
}

/// Progreso del jugador (nivel y experiencia) que muestra la barra azul del HUD.
#[derive(Resource, Clone)]
pub struct Progreso {
    pub nivel: u32,
    pub exp: u32,
}

impl Default for Progreso {
    fn default() -> Self {
        Self { nivel: 1, exp: 0 }
    }
}

/// Tiempo de preparacion del arco: `lista()` es true cuando se puede disparar.
#[derive(Resource)]
pub struct Recarga {
    pub restante: f32,
}

impl Recarga {
    pub fn lista(&self) -> bool {
        self.restante <= 0.0
    }

    pub fn restante(&self) -> f32 {
        self.restante
    }

    pub fn tick(&mut self, dt: f32) {
        self.restante = (self.restante - dt).max(0.0);
    }
}

impl Default for Recarga {
    fn default() -> Self {
        Self { restante: 0.0 }
    }
}

/// Estamina del jugador que muestra la barra amarilla del HUD.
#[derive(Resource)]
pub struct Estamina {
    pub actual: f32,
    pub maximo: u32,
}

impl Default for Estamina {
    fn default() -> Self {
        Self {
            actual: 100.0,
            maximo: 100,
        }
    }
}

/// Maná del jugador que muestra la barra azul del HUD.
#[derive(Resource)]
pub struct Mana {
    pub actual: u32,
    pub maximo: u32,
}

impl Default for Mana {
    fn default() -> Self {
        Self {
            actual: 100,
            maximo: 100,
        }
    }
}

#[derive(Component)]
pub struct Jugador {
    pub clase: Clase,
    /// Personaje equivalente que leen `mundo.rs` y las reglas de equipo.
    pub personaje: Personaje,
    /// Se activa en el frame de suelta del arco; `mundo.rs` crea la flecha.
    pub disparo: bool,
    /// Se activa en el frame de golpe cuerpo a cuerpo; `mundo.rs` aplica el daño.
    pub golpe: bool,
    accion: Accion,
    moviendose: bool,
    corriendo: bool,
    pub mira_izquierda: bool,
    /// Player level. Default is 1, which maintains current behavior.
    #[allow(dead_code)]
    pub nivel: u32,
    /// Player current health. Default is 100, which maintains current behavior.
    /// TODO: unify with VidaJugador in mundo.rs
    #[allow(dead_code)]
    pub vida: u32,
    /// Player attack power. Default is 0 (weapon damage is used separately), which maintains current behavior.
    #[allow(dead_code)]
    pub ataque: u32,
    /// Player defense. Default is 0, which maintains current behavior.
    #[allow(dead_code)]
    pub defensa: u32,
}

#[derive(Component)]
struct Animacion {
    frame: usize,
    timer: Timer,
}

/// Capa de la espada de la chica: solo se ve cuando la lleva equipada.
#[derive(Component)]
struct CapaEspada;

/// Health bar fill (red orb)
#[derive(Component)]
struct HealthBarFill;

/// Stamina bar fill (yellow bar)
#[derive(Component)]
struct StaminaBarFill;

/// Mana bar fill (blue bar)
#[derive(Component)]
struct ManaBarFill;

/// XP bar fill (blue bar)
#[derive(Component)]
struct XpBarFill;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<VidaJugador>()
        .init_resource::<Progreso>()
        .init_resource::<Recarga>()
        .init_resource::<Estamina>()
        .init_resource::<Mana>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_inicial)
        .add_systems(OnEnter(Screen::Gameplay), spawn_hud)
        .add_systems(
            Update,
            (
                (actualizar_jugador, cambiar_personaje, animar_jugador, actualizar_espada, actualizar_recarga)
                    .chain()
                    .run_if(in_state(Screen::Gameplay).and_then(in_state(Menu::None))),
                seguir_camara.run_if(in_state(Screen::Gameplay)),
                actualizar_hud.run_if(in_state(Screen::Gameplay)),
                abrir_pausa.run_if(
                    in_state(Screen::Gameplay)
                        .and_then(in_state(Menu::None))
                        .and_then(input_just_pressed(KeyCode::Escape)),
                ),
            ),
        );
}

/// La chica usa las reglas de equipo del guerrero por ahora.
fn personaje_de(clase: Clase) -> Personaje {
    match clase {
        Clase::Chica => Personaje::Guerrero,
        Clase::Arquero => Personaje::Arquero,
    }
}

fn spawn_inicial(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_jugador(
        &mut commands,
        &assets,
        &mut layouts,
        Clase::Chica,
        Vec3::new(0.0, -20.0, 2.0),
    );
}

/// Spawn HUD elements: red orb on left, bars on right
fn spawn_hud(mut commands: Commands, hud_textures: Res<HudTextures>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                width: Val::Px(MARCO_W + 150.0),
                height: Val::Px(MARCO_H),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(30.0),
                ..default()
            },
            GlobalZIndex(10),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|p| {
            // Orbe rojo de vida con marco (izquierda)
            p.spawn((
                Node {
                    position_type: PositionType::Relative,
                    width: Val::Px(MARCO_W),
                    height: Val::Px(MARCO_H),
                    ..default()
                },
            ))
            .with_children(|orbe| {
                // Marco
                orbe.spawn((
                    ImageNode::new(hud_textures.hp_bar.clone()),
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(MARCO_W),
                        height: Val::Px(MARCO_H),
                        ..default()
                    },
                ));
                // Orbe rojo container (altura la cambia actualizar_hud)
                orbe.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px((MARCO_W - ORBE_W) / 2.0),
                        bottom: Val::Px((MARCO_H - ORBE_H) / 2.0),
                        width: Val::Px(ORBE_W),
                        height: Val::Px(ORBE_H),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    HealthBarFill,
                ))
                .with_children(|contenedor| {
                    // Red orb image, anchored to bottom
                    contenedor.spawn((
                        ImageNode::new(hud_textures.red_bar.clone()),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(ORBE_W),
                            height: Val::Px(ORBE_H),
                            bottom: Val::Px(0.0),
                            ..default()
                        },
                    ));
                });
            });

            // Barras a la derecha (estamina, maná y XP) en columnas
            p.spawn((
                Node {
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    align_items: AlignItems::Start,
                    ..default()
                },
            ))
            .with_children(|barras| {
                // Barra amarilla de estamina (ancho lo cambia actualizar_hud)
                barras.spawn((
                    ImageNode::new(hud_textures.yellow_bar.clone()),
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Px(ESTAMINA_W),
                        height: Val::Px(ESTAMINA_H),
                        ..default()
                    },
                    StaminaBarFill,
                ));
                // Barra azul de maná (ancho lo cambia actualizar_hud)
                barras.spawn((
                    ImageNode::new(hud_textures.blue_bar.clone()),
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Px(MANA_W),
                        height: Val::Px(MANA_H),
                        ..default()
                    },
                    ManaBarFill,
                ));
                // Barra azul de XP (ancho lo cambia actualizar_hud)
                barras.spawn((
                    ImageNode::new(hud_textures.blue_bar.clone()),
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Px(MANA_W),
                        height: Val::Px(XP_H),
                        ..default()
                    },
                    XpBarFill,
                ));
            });
        });
}
/// La chica dibuja mirando a la izquierda; el arquero, a la derecha.
fn flip_de(clase: Clase, mira_izquierda: bool) -> bool {
    match clase {
        Clase::Chica => !mira_izquierda,
        Clase::Arquero => mira_izquierda,
    }
}

/// Crea el personaje: una entidad raiz con una capa por hijo, en el orden de `capas`.
fn spawn_jugador(
    commands: &mut Commands,
    assets: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    clase: Clase,
    posicion: Vec3,
) {
    let (celda, columnas, filas, rutas) = capas(clase);
    let layout = layouts.add(TextureAtlasLayout::from_grid(celda, columnas, filas, None, None));
    let (inicio, _) = rango(clase, Accion::Quieto);

    commands
        .spawn((
            Jugador {
                clase,
                personaje: personaje_de(clase),
                disparo: false,
                golpe: false,
                accion: Accion::Quieto,
                moviendose: false,
                corriendo: false,
                mira_izquierda: false,
                nivel: 1,
                vida: 100,
                ataque: 0,
                defensa: 0,
            },
            Animacion {
                frame: inicio,
                timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            },
            Transform::from_translation(posicion).with_scale(Vec3::splat(ESCALA)),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|hijos| {
            for (i, ruta) in rutas.iter().enumerate() {
                let mut capa = hijos.spawn((
                    Sprite::from_atlas_image(
                        assets.load(*ruta),
                        TextureAtlas {
                            layout: layout.clone(),
                            index: inicio,
                        },
                    ),
                    Transform::from_xyz(0.0, 0.0, i as f32 * 0.1),
                ));
                if ruta.contains("Sword") {
                    capa.insert(CapaEspada);
                }
            }
        });
}

fn cambiar_accion(jugador: &mut Jugador, anim: &mut Animacion, nueva: Accion) {
    let (inicio, _) = rango(jugador.clase, nueva);
    jugador.accion = nueva;
    anim.frame = inicio;

    let segundos = match nueva {
        Accion::Atacar => 0.08,
        Accion::Correr => 0.09,
        _ => 0.12,
    };
    anim.timer = Timer::from_seconds(segundos, TimerMode::Repeating);
}

/// Accion que corresponde al movimiento actual cuando no se esta atacando.
fn accion_de_movimiento(jugador: &Jugador) -> Accion {
    if !jugador.moviendose {
        Accion::Quieto
    } else if jugador.corriendo {
        Accion::Correr
    } else {
        Accion::Caminar
    }
}

/// Movimiento (WASD o flechas, Shift para correr), ataque (F) y limites del mapa.
fn actualizar_jugador(
    teclas: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    inventario: Res<Inventario>,
    recarga: Res<Recarga>,
    mut estamina: ResMut<Estamina>,
    mut mensaje: ResMut<Mensaje>,
    mut query: Query<(&mut Transform, &mut Jugador, &mut Animacion)>,
) {
    let mut direccion = Vec2::ZERO;
    if teclas.pressed(KeyCode::KeyA) || teclas.pressed(KeyCode::ArrowLeft) {
        direccion.x -= 1.0;
    }
    if teclas.pressed(KeyCode::KeyD) || teclas.pressed(KeyCode::ArrowRight) {
        direccion.x += 1.0;
    }
    if teclas.pressed(KeyCode::KeyW) || teclas.pressed(KeyCode::ArrowUp) {
        direccion.y += 1.0;
    }
    if teclas.pressed(KeyCode::KeyS) || teclas.pressed(KeyCode::ArrowDown) {
        direccion.y -= 1.0;
    }

    let quieto = direccion == Vec2::ZERO;
    let correr = teclas.pressed(KeyCode::ShiftLeft) || teclas.pressed(KeyCode::ShiftRight);
    // Consumir estamina al correr, regenerar cuando no se corre
    if !quieto && correr && estamina.actual > 0.0 {
        estamina.actual = drenar_estamina(estamina.actual, time.delta_secs(), estamina.maximo);
    } else if !correr && estamina.actual < estamina.maximo as f32 {
        estamina.actual = regenerar_estamina(estamina.actual, time.delta_secs(), estamina.maximo);
    }
    let velocidad = if correr && estamina.actual > 0.0 { VELOCIDAD_CORRER } else { VELOCIDAD };
    let f_pulsada = teclas.just_pressed(KeyCode::KeyF);
    let con_arco = inventario.arma_equipada().map(|i| i.tipo) == Some(Objeto::Arco);

    for (mut transform, mut jugador, mut anim) in &mut query {
        jugador.moviendose = !quieto;
        jugador.corriendo = correr && !quieto && estamina.actual > 0.0;
        if direccion.x != 0.0 {
            jugador.mira_izquierda = direccion.x < 0.0;
        }

        if f_pulsada && jugador.accion != Accion::Atacar {
            // Sin arco equipado, F es un golpe con el puño o la espada: no pide flechas.
            let puede = if !con_arco {
                true
            } else if inventario.cantidad(Material::Flecha) == 0 {
                mensaje.0 = "No te quedan flechas".to_string();
                false
            } else if !recarga.lista() {
                mensaje.0 = format!("Preparando el arco: {:.1} s", recarga.restante());
                false
            } else {
                true
            };
            if puede {
                cambiar_accion(&mut jugador, &mut anim, Accion::Atacar);
            }
        }

        if jugador.accion != Accion::Atacar {
            let nueva = accion_de_movimiento(&jugador);
            if nueva != jugador.accion {
                cambiar_accion(&mut jugador, &mut anim, nueva);
            }

            if !quieto {
                let paso = direccion.normalize() * velocidad * time.delta_secs();
                transform.translation.x =
                    (transform.translation.x + paso.x).clamp(-LIMITE_X, LIMITE_X);
                transform.translation.y =
                    (transform.translation.y + paso.y).clamp(LIMITE_Y_MIN, LIMITE_Y_MAX);
            }
        }
    }
}

/// Avanza los frames y pone el frame actual en cada capa.
fn animar_jugador(
    time: Res<Time>,
    inventario: Res<Inventario>,
    mut recarga: ResMut<Recarga>,
    mut query: Query<(&mut Jugador, &mut Animacion, &Children)>,
    mut sprites: Query<&mut Sprite>,
) {
    for (mut jugador, mut anim, hijos) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let (inicio, fin) = rango(jugador.clase, jugador.accion);
            anim.frame += 1;

            // In the release frame, mundo.rs creates the arrow.
            if jugador.accion == Accion::Atacar
                && jugador.clase == Clase::Arquero
                && anim.frame == inicio + FRAME_SUELTA_FLECHA
            {
                jugador.disparo = true;
                // TODO: Add recarga system
                recarga.restante = RECARGA_ARCO;
            }

            // In the melee hit frame, mundo.rs applies melee damage.
            if jugador.accion == Accion::Atacar
                && anim.frame == inicio + FRAME_GOLPE
            {
                // With a bow equipped, there is no melee damage.
                let con_arco = inventario.arma_equipada().map(|i| i.tipo) == Some(Objeto::Arco);
                if !con_arco {
                    jugador.golpe = true;
                }
            }

            if anim.frame > fin {
                if jugador.accion == Accion::Atacar {
                    let nueva = accion_de_movimiento(&jugador);
                    cambiar_accion(&mut jugador, &mut anim, nueva);
                } else {
                    anim.frame = inicio;
                }
            }
        }

        for hijo in hijos.iter() {
            if let Ok(mut sprite) = sprites.get_mut(hijo) {
                sprite.flip_x = flip_de(jugador.clase, jugador.mira_izquierda);
                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    atlas.index = anim.frame;
                }
            }
        }
    }
}

/// La espada de la chica solo se ve cuando la lleva equipada.
fn actualizar_espada(
    inventario: Res<Inventario>,
    jugadores: Query<&Children, With<Jugador>>,
    mut capas_espada: Query<&mut Visibility, With<CapaEspada>>,
) {
    let con_espada = inventario.arma_equipada().map(|i| i.tipo) == Some(Objeto::Espada);

    for hijos in &jugadores {
        for hijo in hijos.iter() {
            if let Ok(mut visibilidad) = capas_espada.get_mut(hijo) {
                *visibilidad = if con_espada {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

/// Tab cambia de personaje y conserva la posicion.
fn cambiar_personaje(
    teclas: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<(Entity, &Jugador, &Transform)>,
    mut inventario: ResMut<Inventario>,
    mut mensaje: ResMut<Mensaje>,
) {
    if !teclas.just_pressed(KeyCode::Tab) {
        return;
    }
    let Ok((entidad, jugador, transform)) = query.single() else {
        return;
    };

    let nueva = jugador.clase.siguiente();
    let personaje = personaje_de(nueva);

    let antes = (inventario.equipada, inventario.escudo);
    desequipar_incompatibles(&mut inventario, personaje);
    if (inventario.equipada, inventario.escudo) != antes {
        mensaje.0 = format!(
            "{} no puede usar ese equipo: equipo desequipado",
            nombre_clase(personaje)
        );
    }

    let posicion = transform.translation;
    commands.entity(entidad).despawn();
    spawn_jugador(&mut commands, &assets, &mut layouts, nueva, posicion);
}

fn seguir_camara(
    tiempo: Res<Time>,
    jugadores: Query<&Transform, With<Jugador>>,
    mut camaras: Query<&mut Transform, (With<Camera2d>, Without<Jugador>)>,
) {
    let Some(objetivo) = jugadores.iter().next().map(|t| t.translation.truncate()) else {
        return;
    };
    let factor = 1.0 - (-VELOCIDAD_CAMARA * tiempo.delta_secs()).exp();

    for mut camara in &mut camaras {
        let actual = camara.translation.truncate();
        let nuevo = actual.lerp(objetivo, factor);
        camara.translation.x = nuevo.x;
        camara.translation.y = nuevo.y;
    }
}

fn abrir_pausa(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn actualizar_recarga(time: Res<Time>, mut recarga: ResMut<Recarga>) {
    recarga.tick(time.delta_secs());
}

/// Update HUD bars based on player resources
fn actualizar_hud(
    vida: Res<VidaJugador>,
    estamina: Res<Estamina>,
    mana: Res<Mana>,
    progreso: Res<Progreso>,
    mut vida_nodos: Query<
        &mut Node,
        (With<HealthBarFill>, Without<StaminaBarFill>, Without<ManaBarFill>, Without<XpBarFill>),
    >,
    mut estamina_nodos: Query<
        &mut Node,
        (With<StaminaBarFill>, Without<HealthBarFill>, Without<ManaBarFill>, Without<XpBarFill>),
    >,
    mut mana_nodos: Query<
        &mut Node,
        (With<ManaBarFill>, Without<HealthBarFill>, Without<StaminaBarFill>, Without<XpBarFill>),
    >,
    mut xp_nodos: Query<
        &mut Node,
        (With<XpBarFill>, Without<HealthBarFill>, Without<StaminaBarFill>, Without<ManaBarFill>),
    >,
) {
    if vida.is_changed() {
        let ratio = if vida.maximo > 0 {
            (vida.actual as f32 / vida.maximo as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        for mut node in &mut vida_nodos {
            node.height = Val::Px(ORBE_H * ratio);
        }
    }

    if estamina.is_changed() {
        let ratio = if estamina.maximo > 0 {
            (estamina.actual / estamina.maximo as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        for mut node in &mut estamina_nodos {
            node.width = Val::Px(ESTAMINA_W * ratio);
        }
    }

    if mana.is_changed() {
        let ratio = if mana.maximo > 0 {
            (mana.actual as f32 / mana.maximo as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        for mut node in &mut mana_nodos {
            node.width = Val::Px(MANA_W * ratio);
        }
    }

    if progreso.is_changed() {
        let ratio = ratio_exp(progreso.exp, progreso.nivel);
        for mut node in &mut xp_nodos {
            node.width = Val::Px(MANA_W * ratio);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jugador_default_values_maintain_behavior() {
        let jugador = Jugador {
            clase: Clase::Chica,
            personaje: Personaje::Guerrero,
            disparo: false,
            golpe: false,
            accion: Accion::Quieto,
            moviendose: false,
            corriendo: false,
            mira_izquierda: false,
            nivel: 1,
            vida: 100,
            ataque: 0,
            defensa: 0,
        };
        assert_eq!(jugador.nivel, 1);
        assert_eq!(jugador.vida, 100);
        assert_eq!(jugador.ataque, 0);
        assert_eq!(jugador.defensa, 0);
    }

    #[test]
    fn test_drenar_estamina_clamp_30_fps() {
        let dt_30fps = 1.0 / 30.0;
        let actual = 100.0;
        let maximo = 100;
        let resultado = drenar_estamina(actual, dt_30fps, maximo);
        // 30/s * (1/30) = 1 unit drained
        assert_eq!(resultado, 99.0);
    }

    #[test]
    fn test_drenar_estamina_clamp_60_fps() {
        let dt_60fps = 1.0 / 60.0;
        let actual = 100.0;
        let maximo = 100;
        let resultado = drenar_estamina(actual, dt_60fps, maximo);
        // 30/s * (1/60) = 0.5 units drained
        assert_eq!(resultado, 99.5);
    }

    #[test]
    fn test_drenar_estamina_clamp_144_fps() {
        let dt_144fps = 1.0 / 144.0;
        let actual = 100.0;
        let maximo = 100;
        let resultado = drenar_estamina(actual, dt_144fps, maximo);
        // 30/s * (1/144) = 0.20833... units drained
        assert!((resultado - 99.7916667).abs() < 0.0001);
    }

    #[test]
    fn test_drenar_estamina_clamp_zero() {
        let dt = 1.0;
        let actual = 0.5;
        let maximo = 100;
        let resultado = drenar_estamina(actual, dt, maximo);
        // Cannot go below 0
        assert_eq!(resultado, 0.0);
    }

    #[test]
    fn test_regenerar_estamina_clamp_30_fps() {
        let dt_30fps = 1.0 / 30.0;
        let actual = 50.0;
        let maximo = 100;
        let resultado = regenerar_estamina(actual, dt_30fps, maximo);
        // 15/s * (1/30) = 0.5 units regenerated
        assert_eq!(resultado, 50.5);
    }

    #[test]
    fn test_regenerar_estamina_clamp_60_fps() {
        let dt_60fps = 1.0 / 60.0;
        let actual = 50.0;
        let maximo = 100;
        let resultado = regenerar_estamina(actual, dt_60fps, maximo);
        // 15/s * (1/60) = 0.25 units regenerated
        assert_eq!(resultado, 50.25);
    }

    #[test]
    fn test_regenerar_estamina_clamp_144_fps() {
        let dt_144fps = 1.0 / 144.0;
        let actual = 50.0;
        let maximo = 100;
        let resultado = regenerar_estamina(actual, dt_144fps, maximo);
        // 15/s * (1/144) = 0.1041666... units regenerated
        assert!((resultado - 50.1041667).abs() < 0.0001);
    }

    #[test]
    fn test_regenerar_estamina_clamp_max() {
        let dt = 1.0;
        let actual = 99.5;
        let maximo = 100;
        let resultado = regenerar_estamina(actual, dt, maximo);
        // Cannot exceed maximo
        assert_eq!(resultado, 100.0);
    }

    #[test]
    fn test_ratio_exp_exp_0() {
        let ratio = ratio_exp(0, 1);
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn test_ratio_exp_mitad() {
        let ratio = ratio_exp(50, 1);
        assert_eq!(ratio, 0.5);
    }

    #[test]
    fn test_ratio_exp_completo() {
        let ratio = ratio_exp(100, 1);
        assert_eq!(ratio, 1.0);
    }

    #[test]
    fn test_ratio_exp_mas_que_requerido() {
        let ratio = ratio_exp(150, 1);
        assert_eq!(ratio, 1.0); // Clamp a 1.0
    }

    #[test]
    fn test_ratio_exp_nivel_cero() {
        let ratio = ratio_exp(50, 0);
        assert_eq!(ratio, 0.0);
    }
}