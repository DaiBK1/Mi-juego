//! Datos de los personajes jugables: hojas, celdas y rangos de frames por accion.

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Clase {
    Chica,
    Arquero,
}

impl Clase {
    pub fn siguiente(self) -> Clase {
        match self {
            Clase::Chica => Clase::Arquero,
            Clase::Arquero => Clase::Chica,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Accion {
    Quieto,
    Caminar,
    Correr,
    Atacar,
	}

/// Frames (inicio, fin), ambos incluidos, dentro de la hoja.
pub fn rango(clase: Clase, accion: Accion) -> (usize, usize) {
    match (clase, accion) {
        // Chica (10 columnas): fila 0 quieta, fila 1 caminar, fila 2 correr, fila 5 ataque.
        // Su fila de dano aun no esta confirmada: usa la quieta.
        (Clase::Chica, Accion::Quieto) => (0, 4),
        (Clase::Chica, Accion::Caminar) => (10, 17),
        (Clase::Chica, Accion::Correr) => (20, 27),
        (Clase::Chica, Accion::Atacar) => (50, 55),
        // Arquero (11 columnas): fila 0 quieto, fila 1 disparo, fila 2 caminar, fila 3 dano.
        (Clase::Arquero, Accion::Quieto) => (0, 4),
        (Clase::Arquero, Accion::Caminar) => (22, 29),
        (Clase::Arquero, Accion::Correr) => (22, 29),
        (Clase::Arquero, Accion::Atacar) => (11, 21),
    }
}

/// Celda, columnas, filas y capas en orden: la ultima queda encima.
pub fn capas(clase: Clase) -> (UVec2, u32, u32, &'static [&'static str]) {
    match clase {
        Clase::Chica => (
            UVec2::new(80, 64),
            10,
            7,
            &[
                "personajes/chica/Female_Skin1.png",
                "personajes/chica/Boots.png",
                "personajes/chica/Skirt.png",
                "personajes/chica/Blue_Corset_v2.png",
                "personajes/chica/Female_Hair1.png",
                "personajes/chica/Female_Sword.png",
            ],
        ),
        Clase::Arquero => (
            UVec2::new(64, 64),
            11,
            5,
            &["personajes/arquero/arquero_sheet.png"],
        ),
    }
}