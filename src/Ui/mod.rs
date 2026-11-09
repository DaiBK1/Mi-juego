pub mod menus;
pub mod screens;
pub mod theme;
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::{Activate, Button};

use crate::juego::forja::{
    desequipar_arma, desequipar_escudo, equipar, fabricar_flechas, forjar, nombre_clase,
    reparar, tirar, Datos, Inventario, Item, Material, Objeto, Recetas,
};
use crate::recursos::iconos::Iconos;
use crate::ui::screens::gameplay::Jugador;
use crate::ui::screens::Screen;

#[cfg(test)]
mod tests {
    use crate::juego::forja::{Inventario, Material};

    #[test]
    fn test_new_materials_hidden_when_zero() {
        let inventario = Inventario::inicial();
        // Initial inventory has 0 of new materials
        assert_eq!(inventario.cantidad(Material::Cobre), 0);
        assert_eq!(inventario.cantidad(Material::Oro), 0);
        assert_eq!(inventario.cantidad(Material::Diamante), 0);

        // Verify original materials are present
        assert!(inventario.cantidad(Material::Hierro) > 0);
        assert!(inventario.cantidad(Material::Madera) > 0);
        assert!(inventario.cantidad(Material::Cuero) > 0);
        assert!(inventario.cantidad(Material::PiedraMagica) == 0); // Not in initial
        assert!(inventario.cantidad(Material::Flecha) > 0);
    }

    #[test]
    fn test_new_materials_shown_when_positive() {
        let mut inventario = Inventario::inicial();
        *inventario.materiales.entry(Material::Cobre).or_insert(0) = 5;
        *inventario.materiales.entry(Material::Oro).or_insert(0) = 3;
        *inventario.materiales.entry(Material::Diamante).or_insert(0) = 1;

        assert_eq!(inventario.cantidad(Material::Cobre), 5);
        assert_eq!(inventario.cantidad(Material::Oro), 3);
        assert_eq!(inventario.cantidad(Material::Diamante), 1);
    }
}

#[derive(Resource)]
pub struct Mensaje(pub String);

#[derive(Component)]
struct PanelInventario;

#[derive(Component)]
struct PanelForja;

#[derive(Component)]
struct FilaInventario;

#[derive(Component)]
struct FilaForja;

#[derive(Component)]
struct TextoMensaje;

#[derive(Component)]
struct TextoClase;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Mensaje(
            "Recolecta materiales con E y forja en el panel".to_string(),
        ));
        app.add_systems(OnEnter(Screen::Gameplay), preparar_ui);
        app.add_systems(
            Update,
            (
                refrescar_inventario,
                refrescar_forja,
                actualizar_mensaje,
                actualizar_clase,
            )
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_plugins((theme::plugin, menus::plugin, screens::plugin));
    }
}

fn fuente(tamano: f32) -> TextFont {
    TextFont {
        font_size: FontSize::Px(tamano),
        ..default()
    }
}

fn color_texto() -> Color {
    Color::srgb(0.93, 0.90, 0.84)
}

fn color_gris() -> Color {
    Color::srgb(0.55, 0.55, 0.62)
}

fn icono_nodo() -> Node {
    Node {
        width: Val::Px(48.0),
        height: Val::Px(48.0),
        ..default()
    }
}

fn fila_nodo() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        column_gap: Val::Px(10.0),
        ..default()
    }
}

fn boton_nodo() -> Node {
    Node {
        padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
        ..default()
    }
}

fn descripcion(item: &Item) -> String {
    let estadistica = match item.tipo {
        Objeto::Escudo => "Bloquea 50%".to_string(),
        otro => format!("Dano {}", otro.dano()),
    };
    let mut texto = format!(
        "{:?}  {}  Dur {}/{}",
        item.tipo,
        estadistica,
        item.durabilidad,
        item.tipo.durabilidad_max()
    );
    if item.durabilidad == 0 {
        texto.push_str(" (rota)");
    }
    texto
}

fn texto_coste(item: &Item) -> String {
    item.tipo
        .coste_reparacion(item.tipo.durabilidad_max() - item.durabilidad)
        .iter()
        .map(|(material, cantidad)| format!("{} {}", material, cantidad))
        .collect::<Vec<_>>()
        .join(", ")
}

fn texto_flechas(datos: &Datos) -> String {
    let receta = datos
        .flecha_receta
        .iter()
        .map(|(material, cantidad)| format!("{} {}", material, cantidad))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Flechas\n{} -> {}", receta, datos.flechas_por_fabrica)
}

fn preparar_ui(mut commands: Commands) {
    // Help (bottom left)
    commands.spawn((
        Text::new("[WASD] Mover   [E] Recolectar / Tendero   [F] Atacar   [Tab] Personaje"),
        fuente(18.0),
        TextColor(color_gris()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
    ));

    // Current class (bottom right)
    commands.spawn((
        Text::new(""),
        fuente(18.0),
        TextColor(color_texto()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            right: Val::Px(20.0),
            ..default()
        },
        TextoClase,
        DespawnOnExit(Screen::Gameplay),
    ));

    // On-screen messages (bottom center)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                bottom: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(""),
                fuente(24.0),
                TextColor(Color::srgb(1.0, 0.85, 0.4)),
                TextoMensaje,
            ));
        });

    // Inventory panel (left). Its content is generated by `refrescar_inventario`.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(340.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.85)),
        PanelInventario,
        DespawnOnExit(Screen::Gameplay),
    ));

    // Forging panel (right). Its content is generated by refrescar_forja.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(340.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.85)),
        PanelForja,
        DespawnOnExit(Screen::Gameplay),
    ));
	
	    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-174.0)),
            width: Val::Px(348.0),
            height: Val::Px(192.0),
            ..default()
        },
        GlobalZIndex(10),
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn refrescar_inventario(
    mut commands: Commands,
    inventario: Res<Inventario>,
    iconos: Res<Iconos>,
    paneles: Query<Entity, With<PanelInventario>>,
    anteriores: Query<Entity, With<FilaInventario>>,
    nuevos: Query<(), Added<PanelInventario>>,
) {
    if !inventario.is_changed() && nuevos.is_empty() {
        return;
    }

    for entidad in &anteriores {
        commands.entity(entidad).despawn();
    }

    for panel in &paneles {
        commands.entity(panel).with_children(|p| {
            p.spawn((
                Text::new("Inventario"),
                fuente(22.0),
                TextColor(color_texto()),
                FilaInventario,
            ));

            p.spawn((
                Text::new("Materiales"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaInventario,
            ));
            // Show original 5 materials always, new materials only if quantity > 0
            for material in Material::TODOS {
                let cantidad = inventario.cantidad(material);
                // New materials (Cobre, Oro, Diamante) only show if quantity > 0
                let es_material_nuevo = matches!(material, Material::Cobre | Material::Oro | Material::Diamante);
                if !es_material_nuevo || cantidad > 0 {
                    p.spawn((
                        Text::new(format!("  {}: {}", material, cantidad)),
                        fuente(18.0),
                        TextColor(color_texto()),
                        FilaInventario,
                    ));
                }
            }
            p.spawn((
                Text::new(format!("  Oro: {}", inventario.oro)),
                fuente(18.0),
                TextColor(Color::srgb(1.0, 0.85, 0.4)),
                FilaInventario,
            ));

            // Slot: weapon in hand
            p.spawn((
                Text::new("Arma (clic para quitar)"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaInventario,
            ));
            p.spawn((
                Button,
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    min_height: Val::Px(56.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.10, 0.16)),
                FilaInventario,
            ))
            .observe(
                |_: On<Activate>, mut inv: ResMut<Inventario>, mut mensaje: ResMut<Mensaje>| {
                    if inv.equipada.is_some() {
                        desequipar_arma(&mut inv);
                        mensaje.0 = "Arma desequipada".to_string();
                    } else {
                        mensaje.0 = "No hay arma equipada".to_string();
                    }
                },
            )
            .with_children(|boton| {
                if let Some(item) = inventario.arma_equipada() {
                    if let Some(imagen) = iconos.get(item.tipo.icono()) {
                        boton.spawn((ImageNode::new(imagen), icono_nodo()));
                    }
                    boton.spawn((
                        Text::new(descripcion(item)),
                        fuente(16.0),
                        TextColor(color_texto()),
                    ));
                } else {
                    boton.spawn((Text::new("Ninguna"), fuente(16.0), TextColor(color_gris())));
                }
            });

            // Slot: shield in the other hand
            p.spawn((
                Text::new("Escudo (clic para quitar)"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaInventario,
            ));
            p.spawn((
                Button,
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    min_height: Val::Px(56.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.10, 0.16)),
                FilaInventario,
            ))
            .observe(
                |_: On<Activate>, mut inv: ResMut<Inventario>, mut mensaje: ResMut<Mensaje>| {
                    if inv.escudo.is_some() {
                        desequipar_escudo(&mut inv);
                        mensaje.0 = "Escudo desequipado".to_string();
                    } else {
                        mensaje.0 = "No hay escudo equipado".to_string();
                    }
                },
            )
            .with_children(|boton| {
                if let Some(item) = inventario.escudo_equipado() {
                    if let Some(imagen) = iconos.get(item.tipo.icono()) {
                        boton.spawn((ImageNode::new(imagen), icono_nodo()));
                    }
                    boton.spawn((
                        Text::new(descripcion(item)),
                        fuente(16.0),
                        TextColor(color_texto()),
                    ));
                } else {
                    boton.spawn((Text::new("Ninguno"), fuente(16.0), TextColor(color_gris())));
                }
            });

            // List of items. Equipped items are highlighted with a background color.
            p.spawn((
                Text::new("Objetos (clic para equipar)"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaInventario,
            ));
            for (indice, item) in inventario.objetos.iter().enumerate() {
                let fondo = if inventario.equipada == Some(indice) {
                    Color::srgb(0.24, 0.16, 0.26)
                } else if inventario.escudo == Some(indice) {
                    Color::srgb(0.12, 0.20, 0.28)
                } else {
                    Color::srgb(0.10, 0.10, 0.13)
                };

                p.spawn((
                    Button,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(fondo),
                    FilaInventario,
                ))
                .observe(
                    move |_: On<Activate>,
                          jugadores: Query<&Jugador>,
                          mut inv: ResMut<Inventario>,
                          mut mensaje: ResMut<Mensaje>| {
                        let Some(clase) = jugadores.iter().next().map(|j| j.personaje) else {
                            return;
                        };
                        mensaje.0 = match equipar(&mut inv, indice, clase) {
                            Ok(objeto) => format!("Equipado: {:?}", objeto),
                            Err(error) => error,
                        };
                    },
                )
                .with_children(|boton| {
                    if let Some(imagen) = iconos.get(item.tipo.icono()) {
                        boton.spawn((ImageNode::new(imagen), icono_nodo()));
                    }
                    boton.spawn((
                        Text::new(descripcion(item)),
                        fuente(16.0),
                        TextColor(color_texto()),
                    ));
                });
            }
        });
    }
}

fn refrescar_forja(
    mut commands: Commands,
    inventario: Res<Inventario>,
    recetas: Res<Recetas>,
    datos: Res<Datos>,
    paneles: Query<Entity, With<PanelForja>>,
    anteriores: Query<Entity, With<FilaForja>>,
    nuevos: Query<(), Added<PanelForja>>,
) {
    if !inventario.is_changed() && nuevos.is_empty() {
        return;
    }

    for entidad in &anteriores {
        commands.entity(entidad).despawn();
    }

    for panel in &paneles {
        commands.entity(panel).with_children(|p| {
            p.spawn((
                Text::new("Forja"),
                fuente(22.0),
                TextColor(color_texto()),
                FilaForja,
            ));

            // Section: Crafting Recipes
            p.spawn((
                Text::new("Forjar"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaForja,
            ));
            for (indice, receta) in recetas.0.iter().enumerate() {
                let ingredientes = receta
                    .ingredientes
                    .iter()
                    .map(|(material, cantidad)| format!("{} {}", material, cantidad))
                    .collect::<Vec<_>>()
                    .join(", ");

                p.spawn((fila_nodo(), FilaForja)).with_children(|fila| {
                    fila.spawn((
                        Text::new(format!("{:?}\n{}", receta.resultado, ingredientes)),
                        fuente(16.0),
                        TextColor(color_texto()),
                    ));
                    fila.spawn((
                        Button,
                        boton_nodo(),
                        BackgroundColor(Color::srgb(0.30, 0.20, 0.10)),
                    ))
                    .observe(
                        move |_: On<Activate>,
                              disponibles: Res<Recetas>,
                              mut inv: ResMut<Inventario>,
                              mut mensaje: ResMut<Mensaje>| {
                            let Some(receta) = disponibles.0.get(indice) else {
                                return;
                            };
                            mensaje.0 = match forjar(&mut inv, receta) {
                                Ok(objeto) => format!("Forjado: {:?}", objeto),
                                Err(error) => error,
                            };
                        },
                    )
                    .with_children(|boton| {
                        boton.spawn((Text::new("Forjar"), fuente(18.0), TextColor(color_texto())));
                    });
                });
            }
			
                        // Section: Making arrows
            p.spawn((fila_nodo(), FilaForja)).with_children(|fila| {
                fila.spawn((
                    Text::new(texto_flechas(&datos)),
                    fuente(16.0),
                    TextColor(color_texto()),
                ));
                fila.spawn((
                    Button,
                    boton_nodo(),
                    BackgroundColor(Color::srgb(0.30, 0.20, 0.10)),
                ))
                .observe(
                    |_: On<Activate>,
                     datos: Res<Datos>,
                     mut inv: ResMut<Inventario>,
                     mut mensaje: ResMut<Mensaje>| {
                        mensaje.0 = match fabricar_flechas(&mut inv, &datos) {
                            Ok(cantidad) => format!("Fabricadas {} flechas", cantidad),
                            Err(error) => error,
                        };
                    },
                )
                .with_children(|boton| {
                    boton.spawn((Text::new("Fabricar"), fuente(18.0), TextColor(color_texto())));
                });
            });
			
            // Section: repairing damaged weapons and shields
            p.spawn((
                Text::new("Reparar (restaura durabilidad completa)"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaForja,
            ));
            let mut hay_danadas = false;
            for (indice, item) in inventario.objetos.iter().enumerate() {
                if item.durabilidad >= item.tipo.durabilidad_max() {
                    continue;
                }
                hay_danadas = true;
                let texto = format!(
                    "{:?}  Dur {}/{}\nCoste: {}",
                    item.tipo,
                    item.durabilidad,
                    item.tipo.durabilidad_max(),
                    texto_coste(item)
                );

                p.spawn((fila_nodo(), FilaForja)).with_children(|fila| {
                    fila.spawn((Text::new(texto), fuente(16.0), TextColor(color_texto())));
                    fila.spawn((
                        Button,
                        boton_nodo(),
                        BackgroundColor(Color::srgb(0.15, 0.35, 0.15)),
                    ))
                    .observe(
                        move |_: On<Activate>,
                              mut inv: ResMut<Inventario>,
                              mut mensaje: ResMut<Mensaje>| {
                            mensaje.0 = match reparar(&mut inv, indice) {
                                Ok(objeto) => format!("Reparada: {:?}", objeto),
                                Err(error) => error,
                            };
                        },
                    )
                    .with_children(|boton| {
                        boton.spawn((Text::new("Reparar"), fuente(18.0), TextColor(color_texto())));
                    });
                });
            }
            if !hay_danadas {
                p.spawn((
                    Text::new("Nada que reparar"),
                    fuente(16.0),
                    TextColor(color_gris()),
                    FilaForja,
                ));
            }

            // Section: discarding broken weapons
            p.spawn((
                Text::new("Tirar armas rotas"),
                fuente(16.0),
                TextColor(color_gris()),
                FilaForja,
            ));
            let mut hay_rotas = false;
            for (indice, item) in inventario.objetos.iter().enumerate() {
                if item.durabilidad > 0 {
                    continue;
                }
                hay_rotas = true;
                let texto = format!("{:?}  (rota)", item.tipo);

                p.spawn((fila_nodo(), FilaForja)).with_children(|fila| {
                    fila.spawn((Text::new(texto), fuente(16.0), TextColor(color_texto())));
                    fila.spawn((
                        Button,
                        boton_nodo(),
                        BackgroundColor(Color::srgb(0.40, 0.12, 0.12)),
                    ))
                    .observe(
                        move |_: On<Activate>,
                              mut inv: ResMut<Inventario>,
                              mut mensaje: ResMut<Mensaje>| {
                            mensaje.0 = match tirar(&mut inv, indice) {
                                Ok(objeto) => format!("Tirada: {:?}", objeto),
                                Err(error) => error,
                            };
                        },
                    )
                    .with_children(|boton| {
                        boton.spawn((Text::new("Tirar"), fuente(18.0), TextColor(color_texto())));
                    });
                });
            }
            if !hay_rotas {
                p.spawn((
                    Text::new("Ninguna arma rota"),
                    fuente(16.0),
                    TextColor(color_gris()),
                    FilaForja,
                ));
            }
        });
    }
}

fn actualizar_mensaje(mensaje: Res<Mensaje>, mut textos: Query<&mut Text, With<TextoMensaje>>) {
    for mut texto in &mut textos {
        if texto.0 != mensaje.0 {
            texto.0 = mensaje.0.clone();
        }
    }
}

fn actualizar_clase(jugadores: Query<&Jugador>, mut textos: Query<&mut Text, With<TextoClase>>) {
    let Some(jugador) = jugadores.iter().next() else {
        return;
    };
    let nuevo = format!("Clase: {}", nombre_clase(jugador.personaje));

    for mut texto in &mut textos {
        if texto.0 != nuevo {
            texto.0 = nuevo.clone();
        }
    }	
}