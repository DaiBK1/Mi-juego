use std::collections::HashMap;
use std::f32::consts::{PI, TAU};

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::juego::forja::Objeto;
use crate::recursos::iconos::{color_arte, ESCUDO_ARTE, ESPADA_ARTE};

/// The side of each animation cell. It should be larger than the character so that the reach of their weapons doesn't get cut off.
pub const CELDA: usize = 160;
/// Positioning of the drawing within the cell: the figure is centered (32 + 48 = 80 = center).
const DESPLAZAMIENTO: i32 = 48;
/// Inventory icon side.
pub const LADO_ICONO: usize = 48;
/// Size of the sword in hand: each pixel of the icon occupies 2x2 pixels of the cell.
const ESCALA_ESPADA: f32 = 2.0;

const RADIO_ARCO: f32 = 14.0;
const LARGO_LANZA: f32 = 34.0;

const PIEL: [u8; 4] = [232, 188, 156, 255];
const PELO: [u8; 4] = [222, 224, 236, 255];
const CAPA: [u8; 4] = [58, 44, 96, 255];
const TUNICA: [u8; 4] = [40, 60, 92, 255];
const PANTALON: [u8; 4] = [34, 32, 48, 255];
const BOTAS: [u8; 4] = [60, 40, 30, 255];
const CUERO: [u8; 4] = [120, 80, 44, 255];
const PLUMAS: [u8; 4] = [196, 60, 60, 255];
const OJOS_ARQUERO: [u8; 4] = [90, 60, 170, 255];
const MADERA_ARCO: [u8; 4] = [150, 96, 52, 255];
const CUERDA: [u8; 4] = [240, 230, 190, 255];
const ARMADURA: [u8; 4] = [60, 100, 170, 255];
const ARMADURA_CLARA: [u8; 4] = [90, 130, 200, 255];
const ACERO: [u8; 4] = [150, 165, 185, 255];
const ACERO_OSCURO: [u8; 4] = [78, 90, 112, 255];
const ORO: [u8; 4] = [230, 180, 60, 255];
const VISERA: [u8; 4] = [25, 28, 40, 255];
const MADERA: [u8; 4] = [120, 78, 40, 255];
const FILO: [u8; 4] = [225, 235, 245, 255];

#[derive(Clone, Copy, PartialEq)]
pub enum Estado {
    Idle,
    Correr,
    Disparar,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Personaje {
    Arquero,
    Guerrero,
}

impl Personaje {
    pub fn frames_totales(&self) -> u32 {
        match self {
            Personaje::Arquero => 19,
            Personaje::Guerrero => 12,
        }
    }

    pub fn rango(&self, estado: Estado) -> (usize, usize) {
        match (self, estado) {
            (Personaje::Arquero, Estado::Idle) => (0, 5),
            (Personaje::Arquero, Estado::Correr) => (6, 9),
            (Personaje::Arquero, Estado::Disparar) => (10, 17),
            (Personaje::Guerrero, Estado::Idle) => (0, 3),
            (Personaje::Guerrero, Estado::Correr) => (4, 7),
            (Personaje::Guerrero, Estado::Disparar) => (8, 11),
        }
    }

 }

/// What the character is holding in their hands. Each combination has its own sprite.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Equipo {
    pub arma: Option<Objeto>,
    pub escudo: bool,
}

pub fn preparar_personajes(_commands: Commands, mut imagenes: ResMut<Assets<Image>>) {
    let mut arquero = HashMap::new();
    for arma in [None, Some(Objeto::Arco)] {
        let equipo = Equipo { arma, escudo: false };
        arquero.insert(equipo, generar(Personaje::Arquero, equipo, &mut imagenes));
    }
  }

/// Hand-drawn inventory icons for weapons (bow, axe, spear).
/// The sword and shield use their pixel art versions in icons.rs.
pub fn generar_icono(objeto: Objeto, imagenes: &mut Assets<Image>) -> Handle<Image> {
    let mut datos = vec![0u8; LADO_ICONO * LADO_ICONO * 4];
    {
        let mut p = Pintor {
            datos: &mut datos,
            ancho: LADO_ICONO,
            x0: 0,
            lado: LADO_ICONO as i32,
            desp: 0,
        };
        match objeto {
            Objeto::Arco => {
                let agarre = (24.0, 24.0);
                let nock = (agarre.0 - 7.0, agarre.1);
                dibujar_arco(&mut p, agarre, nock, 0.0);
            }
            Objeto::Espada | Objeto::Escudo => {}
            otro => dibujar_arma(&mut p, otro, (14.0, 36.0), 2.3, false),
        }
    }

    imagenes.add(Image::new(
        Extent3d {
            width: LADO_ICONO as u32,
            height: LADO_ICONO as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        datos,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ))
}

fn generar(personaje: Personaje, equipo: Equipo, imagenes: &mut Assets<Image>) -> Handle<Image> {
    let frames = personaje.frames_totales() as usize;
    let ancho = CELDA * frames;
    let mut datos = vec![0u8; ancho * CELDA * 4];

    for f in 0..frames {
        let (estado, t) = estado_de(personaje, f);
        let mut p = Pintor {
            datos: &mut datos,
            ancho,
            x0: f * CELDA,
            lado: CELDA as i32,
            desp: DESPLAZAMIENTO,
        };
        match personaje {
            Personaje::Arquero => arquero(&mut p, estado, t, &equipo),
            Personaje::Guerrero => guerrero(&mut p, estado, t, &equipo),
        }
    }

    imagenes.add(Image::new(
        Extent3d {
            width: ancho as u32,
            height: CELDA as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        datos,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ))
}

fn estado_de(personaje: Personaje, f: usize) -> (Estado, f32) {
    for estado in [Estado::Idle, Estado::Correr, Estado::Disparar] {
        let (inicio, fin) = personaje.rango(estado);
        if f >= inicio && f <= fin {
            let largo = (fin - inicio + 1) as f32;
            return (estado, (f - inicio) as f32 / largo);
        }
    }
    (Estado::Idle, 0.0)
}

struct Pose {
    rebote: f32,
    pierna_i: f32,
    pierna_d: f32,
    brazo_i: f32,
    brazo_d: f32,
}

fn calcular_pose(estado: Estado, t: f32) -> Pose {
    let fase = t * TAU;
    let s = fase.sin();
    match estado {
        Estado::Idle => Pose {
            rebote: if t < 0.5 { 0.0 } else { 1.0 },
            pierna_i: 0.08,
            pierna_d: -0.08,
            brazo_i: 0.15,
            brazo_d: -0.15,
        },
        Estado::Correr => Pose {
            rebote: (1.0 - (fase * 2.0).cos()) * 0.5,
            pierna_i: 0.7 * s,
            pierna_d: -0.7 * s,
            brazo_i: -0.6 * s,
            brazo_d: 0.6 * s,
        },
        Estado::Disparar => Pose {
            rebote: 0.0,
            pierna_i: 0.1,
            pierna_d: -0.1,
            brazo_i: 0.0,
            brazo_d: 0.0,
        },
    }
}

/// Bowstring tension: increases to 1 and is released at t = 0.6.
fn tension_arco(t: f32) -> f32 {
    if t < 0.6 { t / 0.6 } else { 0.0 }
}

/// Sword or axe blow: raised to the shoulder, then brought down and forward.
fn angulo_golpe(t: f32) -> f32 {
    if t < 0.25 {
        return 2.6;
    }
    let u = ((t - 0.25) / 0.5).clamp(0.0, 1.0);
    2.6 - 2.6 * u
}

fn extremo(origen: (f32, f32), largo: f32, angulo: f32) -> (f32, f32) {
    (origen.0 + largo * angulo.sin(), origen.1 + largo * angulo.cos())
}

fn r(v: f32) -> i32 {
    v.round() as i32
}

fn signo(q: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    (q.0 - b.0) * (a.1 - b.1) - (a.0 - b.0) * (q.1 - b.1)
}

struct Pintor<'a> {
    datos: &'a mut [u8],
    ancho: usize,
    x0: usize,
    lado: i32,
    desp: i32,
}

impl Pintor<'_> {
    fn punto(&mut self, x: i32, y: i32, color: [u8; 4]) {
        let x = x + self.desp;
        let y = y + self.desp;
        if x < 0 || y < 0 || x >= self.lado || y >= self.lado {
            return;
        }
        let i = (y as usize * self.ancho + self.x0 + x as usize) * 4;
        self.datos[i..i + 4].copy_from_slice(&color);
    }

    fn rect(&mut self, x: i32, y: i32, ancho: i32, alto: i32, color: [u8; 4]) {
        for dy in 0..alto {
            for dx in 0..ancho {
                self.punto(x + dx, y + dy, color);
            }
        }
    }

    fn circulo(&mut self, cx: f32, cy: f32, radio: f32, color: [u8; 4]) {
        for y in (cy - radio).floor() as i32..=(cy + radio).ceil() as i32 {
            for x in (cx - radio).floor() as i32..=(cx + radio).ceil() as i32 {
                let dx = x as f32 + 0.5 - cx;
                let dy = y as f32 + 0.5 - cy;
                if dx * dx + dy * dy <= radio * radio {
                    self.punto(x, y, color);
                }
            }
        }
    }

    fn triangulo(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32), color: [u8; 4]) {
        let x0 = a.0.min(b.0).min(c.0).floor() as i32;
        let x1 = a.0.max(b.0).max(c.0).ceil() as i32;
        let y0 = a.1.min(b.1).min(c.1).floor() as i32;
        let y1 = a.1.max(b.1).max(c.1).ceil() as i32;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let q = (x as f32 + 0.5, y as f32 + 0.5);
                let d1 = signo(q, a, b);
                let d2 = signo(q, b, c);
                let d3 = signo(q, c, a);
                let negativo = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
                let positivo = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
                if !(negativo && positivo) {
                    self.punto(x, y, color);
                }
            }
        }
    }

    fn trazo(&mut self, a: (f32, f32), b: (f32, f32), grosor: i32, color: [u8; 4]) {
        let pasos = ((b.0 - a.0).abs().max((b.1 - a.1).abs()).ceil() as i32).max(1);
        for i in 0..=pasos {
            let t = i as f32 / pasos as f32;
            let x = r(a.0 + (b.0 - a.0) * t);
            let y = r(a.1 + (b.1 - a.1) * t);
            self.rect(x - grosor / 2, y - grosor / 2, grosor, grosor, color);
        }
    }
}

/// Draws rotated and scaled pixel art. `anchor` is the point in the artwork that aligns with `destination`;
/// `m` is the rotation matrix [a, b, c, d], and `scale` is the number of pixels each pixel of the artwork occupies.
/// Inverse sampling: no gaps are left.
fn dibujar_arte(
    p: &mut Pintor,
    arte: &[&str],
    ancla: (f32, f32),
    destino: (f32, f32),
    m: [f32; 4],
    escala: f32,
) {
    let radio = (16.0 * escala).ceil() as i32;
    for dy in -radio..=radio {
        for dx in -radio..=radio {
            let x = dx as f32 + 0.5;
            let y = dy as f32 + 0.5;
            let du = (m[0] * x + m[2] * y) / escala;
            let dv = (m[1] * x + m[3] * y) / escala;
            let ax = (ancla.0 + du).floor();
            let ay = (ancla.1 + dv).floor();
            if ax < 0.0 || ay < 0.0 {
                continue;
            }
            let Some(fila) = arte.get(ay as usize) else {
                continue;
            };
            let Some(c) = fila.chars().nth(ax as usize) else {
                continue;
            };
            let color = color_arte(c);
            if color[3] == 0 {
                continue;
            }
            p.punto(r(destino.0) + dx, r(destino.1) + dy, color);
        }
    }
}

/// Rotation matrix for a weapon whose handle points in the direction of the angle.
/// In the artwork, the blade points upward; here, the blade follows the direction (sin, cos).
fn matriz_arma(angulo: f32) -> [f32; 4] {
    let (s, c) = (angulo.sin(), angulo.cos());
    [-c, -s, s, -c]
}

/// Bow with a string. The "nock" is the point where the string is attached.
/// When under tension, draw the nocked arrow.
fn dibujar_arco(p: &mut Pintor, agarre: (f32, f32), nock: (f32, f32), tension: f32) {
    let c = (agarre.0 - RADIO_ARCO, agarre.1);

    let mut previo: Option<(f32, f32)> = None;
    let mut ang = -1.05_f32;
    while ang <= 1.05 {
        let punto = (c.0 + RADIO_ARCO * ang.cos(), c.1 + RADIO_ARCO * ang.sin());
        if let Some(a) = previo {
            p.trazo(a, punto, 2, MADERA_ARCO);
        }
        previo = Some(punto);
        ang += 0.15;
    }

    let extremo_sup = (
        c.0 + RADIO_ARCO * (-1.05_f32).cos(),
        c.1 + RADIO_ARCO * (-1.05_f32).sin(),
    );
    let extremo_inf = (
        c.0 + RADIO_ARCO * 1.05_f32.cos(),
        c.1 + RADIO_ARCO * 1.05_f32.sin(),
    );

    p.trazo(extremo_sup, nock, 1, CUERDA);
    p.trazo(nock, extremo_inf, 1, CUERDA);

    if tension > 0.05 {
        let madera_flecha = [170, 120, 70, 255];
        let punta = (nock.0 + 22.0, nock.1);
        p.trazo(nock, punta, 2, madera_flecha);
        p.triangulo(
            (punta.0 - 1.0, punta.1 - 2.5),
            (punta.0 - 1.0, punta.1 + 2.5),
            (punta.0 + 3.0, punta.1),
            ACERO,
        );
        p.trazo((nock.0 - 1.0, nock.1 - 2.0), (nock.0 + 2.0, nock.1), 1, PLUMAS);
        p.trazo((nock.0 - 1.0, nock.1 + 2.0), (nock.0 + 2.0, nock.1), 1, PLUMAS);
    }
}

/// Shield held in the left hand. Same style as the icon, not rotated.
fn dibujar_escudo(p: &mut Pintor, mano: (f32, f32)) {
    dibujar_arte(p, ESCUDO_ARTE, (7.5, 7.5), mano, [1.0, 0.0, 0.0, 1.0], 1.0);
}

/// Weapon in the right hand. The sword uses the same artwork as its icon, scaled up to double size.
/// The axe and the spear are hand-drawn. `attacking` flips the orientation of the axe blade
/// so that the cutting edge faces downward during the strike.
fn dibujar_arma(p: &mut Pintor, arma: Objeto, mano: (f32, f32), angulo: f32, atacando: bool) {
    let dir = (angulo.sin(), angulo.cos());
    let perp = (angulo.cos(), -angulo.sin());

    match arma {
        Objeto::Espada => {
            dibujar_arte(
                p,
                ESPADA_ARTE,
                (8.5, 12.5),
                mano,
                matriz_arma(angulo),
                ESCALA_ESPADA,
            );
        }
        Objeto::Hacha => {
            let signo_filo = if atacando { -1.0 } else { 1.0 };
            let perp = (perp.0 * signo_filo, perp.1 * signo_filo);

            let base = (mano.0 - dir.0 * 4.0, mano.1 - dir.1 * 4.0);
            let cima = (mano.0 + dir.0 * 14.0, mano.1 + dir.1 * 14.0);
            p.trazo(base, cima, 3, MADERA);

            let a = (cima.0 - dir.0 * 6.0, cima.1 - dir.1 * 6.0);
            let b = (cima.0 + dir.0 * 6.0, cima.1 + dir.1 * 6.0);
            let c = (
                cima.0 + perp.0 * 11.0 + dir.0 * 3.0,
                cima.1 + perp.1 * 11.0 + dir.1 * 3.0,
            );
            let d = (
                cima.0 + perp.0 * 11.0 - dir.0 * 4.0,
                cima.1 + perp.1 * 11.0 - dir.1 * 4.0,
            );
            p.triangulo(a, b, c, ACERO);
            p.triangulo(a, c, d, ACERO);
            p.trazo(c, d, 2, FILO);
            p.trazo(a, d, 1, ACERO_OSCURO);
        }
        Objeto::Lanza => {
            let base = (mano.0 - dir.0 * 8.0, mano.1 - dir.1 * 8.0);
            let punta = (mano.0 + dir.0 * LARGO_LANZA, mano.1 + dir.1 * LARGO_LANZA);
            p.trazo(base, punta, 3, MADERA);

            let cabeza_a = (punta.0 + perp.0 * 3.5, punta.1 + perp.1 * 3.5);
            let cabeza_b = (punta.0 - perp.0 * 3.5, punta.1 - perp.1 * 3.5);
            let cima = (punta.0 + dir.0 * 8.0, punta.1 + dir.1 * 8.0);
            p.triangulo(cabeza_a, cabeza_b, cima, FILO);
        }
        _ => {}
    }
}

fn arquero(p: &mut Pintor, estado: Estado, t: f32, equipo: &Equipo) {
    let pose = calcular_pose(estado, t);
    let b = -pose.rebote * 2.0;
    let con_arco = equipo.arma == Some(Objeto::Arco);
    let tension = if estado == Estado::Disparar && con_arco {
        tension_arco(t)
    } else {
        0.0
    };

    // Long legs
    let cadera = (32.0, 42.0 + b);
    for pierna in [pose.pierna_i, pose.pierna_d] {
        let pie = extremo(cadera, 22.0, pierna);
        p.trazo(cadera, pie, 4, PANTALON);
        p.circulo(pie.0, pie.1 + 1.0, 2.5, BOTAS);
    }

    // Cloak and quiver (behind the body)
    p.rect(22, r(21.0 + b), 3, 22, CAPA);
    p.rect(17, r(18.0 + b), 5, 22, CUERO);
    p.rect(18, r(14.0 + b), 1, 4, PLUMAS);
    p.rect(20, r(14.0 + b), 1, 4, PLUMAS);

    // Torso and belt
    p.rect(24, r(21.0 + b), 16, 21, TUNICA);
    p.rect(24, r(40.0 + b), 16, 2, CUERO);

    // Small head: silver hair and a face
    p.circulo(32.0, 13.0 + b, 6.5, PELO);
    p.circulo(33.0, 16.0 + b, 4.5, PIEL);
    p.rect(26, r(11.0 + b), 11, 2, PELO);
    p.rect(30, r(15.0 + b), 2, 2, OJOS_ARQUERO);
    p.rect(36, r(15.0 + b), 2, 2, OJOS_ARQUERO);

    let hombro_i = (24.0, 23.0 + b);
    let hombro_d = (40.0, 23.0 + b);
    let (mano_i, mano_d) = if con_arco {
        let agarre = (37.0, 25.0 + b);
        let nock = (agarre.0 - 7.0 - 10.0 * tension, agarre.1);
        (agarre, nock)
    } else {
        (
            extremo(hombro_i, 13.0, pose.brazo_i),
            extremo(hombro_d, 13.0, pose.brazo_d),
        )
    };

    p.trazo(hombro_i, mano_i, 3, TUNICA);
    p.trazo(hombro_d, mano_d, 3, TUNICA);

    if con_arco {
        dibujar_arco(p, mano_i, mano_d, tension);
    }

    p.circulo(mano_i.0, mano_i.1, 1.8, PIEL);
    p.circulo(mano_d.0, mano_d.1, 1.8, PIEL);
}

fn guerrero(p: &mut Pintor, estado: Estado, t: f32, equipo: &Equipo) {
    let pose = calcular_pose(estado, t);
    let b = -pose.rebote * 2.0;
    let disparando = estado == Estado::Disparar;

    // Long legs
    let cadera = (32.0, 42.0 + b);
    for pierna in [pose.pierna_i, pose.pierna_d] {
        let pie = extremo(cadera, 22.0, pierna);
        p.trazo(cadera, pie, 6, ACERO_OSCURO);
    }

    // Torso and belt
    p.rect(22, r(21.0 + b), 20, 21, ARMADURA);
    p.rect(22, r(21.0 + b), 3, 21, ARMADURA_CLARA);
    p.rect(22, r(40.0 + b), 20, 3, ORO);

    //Small helmet
    p.circulo(32.0, 13.0 + b, 6.5, ACERO);
    p.rect(26, r(12.0 + b), 12, 3, VISERA);
    p.rect(31, r(5.0 + b), 2, 3, ORO);

    let hombro_i = (22.0, 23.0 + b);
    let hombro_d = (42.0, 23.0 + b);

    // Left arm and shield (only if equipped)
    let mano_i = extremo(hombro_i, 13.0, pose.brazo_i);
    p.trazo(hombro_i, mano_i, 4, ARMADURA);
    p.circulo(mano_i.0, mano_i.1, 2.2, ACERO);
    if equipo.escudo {
        dibujar_escudo(p, mano_i);
    }

// Right arm: the angle and reach depend on the attack and the weapon
let (angulo_d, alcance) = if disparando {
        match equipo.arma {
            Some(Objeto::Lanza) => (1.5, 13.0 + 8.0 * (PI * t).sin()),
            _ => (angulo_golpe(t), 13.0),
        }
    } else {
        (pose.brazo_d + 0.3, 13.0)
    };

    let mano_d = extremo(hombro_d, alcance, angulo_d);
    p.trazo(hombro_d, mano_d, 4, ARMADURA);
    p.circulo(mano_d.0, mano_d.1, 2.2, ACERO);

    if let Some(arma) = equipo.arma {
        dibujar_arma(p, arma, mano_d, angulo_d, disparando);
    }
}