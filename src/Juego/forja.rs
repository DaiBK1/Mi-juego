use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

use crate::juego::personajes::Personaje;

#[allow(dead_code)]
pub const PRECIO_PIEDRA: u32 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[allow(dead_code)]
pub enum Material {
    Cobre,
    Oro,
    Hierro,
    Diamante,
    Madera,
    Cuero,
    PiedraMagica,
    Flecha,
}

impl Material {
    pub const TODOS: [Material; 8] = [
        Material::Cobre,
        Material::Oro,
        Material::Hierro,
        Material::Diamante,
        Material::Madera,
        Material::Cuero,
        Material::PiedraMagica,
        Material::Flecha,
    ];
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            Material::Cobre => "Cobre",
            Material::Oro => "Oro (mineral)",
            Material::Hierro => "Hierro",
            Material::Diamante => "Diamante",
            Material::Madera => "Madera",
            Material::Cuero => "Cuero",
            Material::PiedraMagica => "Piedra magica",
            Material::Flecha => "Flecha",
        };
        write!(f, "{}", nombre)
    }
}

/// Valores editables desde assets/datos.ron
#[derive(Resource, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Datos {
    #[allow(dead_code)]
    pub flecha_precio_oro: u32,
    #[allow(dead_code)]
    pub flechas_por_compra: u32,
    pub flechas_por_fabrica: u32,
    pub flecha_receta: Vec<(Material, u32)>,
}

pub fn cargar_datos() -> Datos {
    let ruta = concat!(env!("CARGO_MANIFEST_DIR"), "/Assets/Datos.ron");
    let texto = std::fs::read_to_string(ruta)
        .expect("No se pudo leer Assets/Datos.ron");
    ron::from_str(&texto).expect("Assets/Datos.ron tiene un error de formato")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Objeto {
    Espada,
    Escudo,
    Hacha,
    Arco,
    Lanza,
}

/// Rarity tier for weapons. Default (0) maintains current behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum Rareza {
    #[default]
    Comun,
    PocoComun,
    Raro,
    Epico,
    Legendario,
}

/// Upgrade level for weapons. Default (0) maintains current behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum Mejora {
    #[default]
    Base,
    Mas1,
    Mas2,
    Mas3,
}

impl Objeto {
    ///Damage from a melee strike. For the bow, it is the damage of each arrow.
    pub fn dano(&self) -> u32 {
        match self {
            Objeto::Espada => 20,
            Objeto::Escudo => 0,
            Objeto::Hacha => 35,
            Objeto::Arco => 15,
            Objeto::Lanza => 25,
        }
    }

    pub fn durabilidad_max(&self) -> u32 {
        match self {
            Objeto::Espada => 30,
            Objeto::Escudo => 50,
            Objeto::Hacha => 15,
            Objeto::Arco => 20,
            Objeto::Lanza => 25,
        }
    }

    pub fn icono(&self) -> &'static str {
        match self {
            Objeto::Espada => "espada.png",
            Objeto::Escudo => "escudo.png",
            Objeto::Hacha => "hacha.png",
            Objeto::Arco => "arco.png",
            Objeto::Lanza => "lanza.png",
        }
    }


    ///Durability that restores every repaired section.
    pub fn tramo_reparacion(&self) -> u32 {
        match self {
            Objeto::Espada => 10,
            Objeto::Escudo => 10,
            Objeto::Hacha => 5,
            Objeto::Arco => 5,
            Objeto::Lanza => 5,
        }
    }

    /// Materials for a repair section.
    fn coste_tramo(&self) -> Vec<(Material, u32)> {
        match self {
            Objeto::Espada => vec![(Material::Hierro, 2), (Material::Madera, 1)],
            Objeto::Escudo => vec![(Material::Madera, 2), (Material::Cuero, 1)],
            Objeto::Hacha => vec![(Material::Hierro, 2), (Material::Madera, 1)],
            Objeto::Arco => vec![(Material::Madera, 2), (Material::Cuero, 1)],
            Objeto::Lanza => vec![(Material::Hierro, 2), (Material::Madera, 1)],
        }
    }

    /// Cost to repair `missing` durability points. Charged per full increment.
    pub fn coste_reparacion(&self, faltan: u32) -> Vec<(Material, u32)> {
        if faltan == 0 {
            return Vec::new();
        }
        let tramo = self.tramo_reparacion();
        let tramos = (faltan + tramo - 1) / tramo;
        self.coste_tramo()
            .into_iter()
            .map(|(material, cantidad)| (material, cantidad * tramos))
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub tipo: Objeto,
    pub durabilidad: u32,
    /// Rarity tier. Default is Comun, which maintains current behavior.
    #[allow(dead_code)]
    pub rareza: Rareza,
    /// Upgrade level. Default is Base, which maintains current behavior.
    #[allow(dead_code)]
    pub mejora: Mejora,
}

impl Item {
    fn nuevo(tipo: Objeto) -> Self {
        Self {
            tipo,
            durabilidad: tipo.durabilidad_max(),
            rareza: Rareza::default(),
            mejora: Mejora::default(),
        }
    }
}

pub struct Receta {
    pub resultado: Objeto,
    pub ingredientes: Vec<(Material, u32)>,
    /// Crafting station required. Default is Yunque (Anvil), which maintains current behavior.
    #[allow(dead_code)]
    pub estacion: EstacionForja,
    /// Upgrade level for crafted item. Default is Base, which maintains current behavior.
    #[allow(dead_code)]
    pub mejora: Mejora,
    /// Success probability (0.0 to 1.0). Default is 1.0 (always succeeds), which maintains current behavior.
    #[allow(dead_code)]
    pub probabilidad_exito: f32,
}

/// Crafting station for recipes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EstacionForja {
    Yunque,
    Martillo,
}

impl Default for EstacionForja {
    fn default() -> Self {
        EstacionForja::Yunque
    }
}

#[derive(Resource)]
pub struct Recetas(pub Vec<Receta>);

pub fn recetas() -> Vec<Receta> {
    vec![
        Receta {
            resultado: Objeto::Espada,
            ingredientes: vec![(Material::Hierro, 2), (Material::Madera, 1)],
            estacion: EstacionForja::default(),
            mejora: Mejora::default(),
            probabilidad_exito: 1.0,
        },
        Receta {
            resultado: Objeto::Escudo,
            ingredientes: vec![(Material::Madera, 2), (Material::Cuero, 2)],
            estacion: EstacionForja::default(),
            mejora: Mejora::default(),
            probabilidad_exito: 1.0,
        },
        Receta {
            resultado: Objeto::Hacha,
            ingredientes: vec![(Material::Hierro, 4), (Material::Madera, 2)],
            estacion: EstacionForja::default(),
            mejora: Mejora::default(),
            probabilidad_exito: 1.0,
        },
        Receta {
            resultado: Objeto::Arco,
            ingredientes: vec![(Material::Madera, 4), (Material::Cuero, 2)],
            estacion: EstacionForja::default(),
            mejora: Mejora::default(),
            probabilidad_exito: 1.0,
        },
        Receta {
            resultado: Objeto::Lanza,
            ingredientes: vec![(Material::Hierro, 3), (Material::Madera, 2)],
            estacion: EstacionForja::default(),
            mejora: Mejora::default(),
            probabilidad_exito: 1.0,
        },
    ]
}

#[derive(Resource)]
pub struct Inventario {
    pub materiales: HashMap<Material, u32>,
    pub objetos: Vec<Item>,
    /// Index in `objetos` of the weapon in hand (sword, axe, spear, or bow).
    pub equipada: Option<usize>,
    /// Index in `objetos` of the shield in the other hand.
    pub escudo: Option<usize>,
    pub oro: u32,
}

impl Inventario {
    pub fn inicial() -> Self {
        let mut materiales = HashMap::new();
        materiales.insert(Material::Hierro, 5);
        materiales.insert(Material::Madera, 3);
        materiales.insert(Material::Cuero, 2);
        materiales.insert(Material::Flecha, 10);
        Self {
            materiales,
            objetos: Vec::new(),
            equipada: None,
            escudo: None,
            oro: 0,
        }
    }

    pub fn cantidad(&self, material: Material) -> u32 {
        self.materiales.get(&material).copied().unwrap_or(0)
    }

    pub fn arma_equipada(&self) -> Option<&Item> {
        self.equipada.and_then(|i| self.objetos.get(i))
    }

    pub fn escudo_equipado(&self) -> Option<&Item> {
        self.escudo.and_then(|i| self.objetos.get(i))
    }
}

fn comprobar(inv: &Inventario, ingredientes: &[(Material, u32)]) -> Result<(), String> {
    let faltan: Vec<String> = ingredientes
        .iter()
        .filter(|(material, cantidad)| inv.cantidad(*material) < *cantidad)
        .map(|(material, cantidad)| {
            let tiene = inv.cantidad(*material);
            format!("{} (faltan {})", material, *cantidad - tiene)
        })
        .collect();

    if faltan.is_empty() {
        Ok(())
    } else {
        Err(format!("Faltan materiales: {}", faltan.join(", ")))
    }
}

fn consumir(inv: &mut Inventario, ingredientes: &[(Material, u32)]) {
    for (material, cantidad) in ingredientes {
        if let Some(stock) = inv.materiales.get_mut(material) {
            *stock -= *cantidad;
        }
    }
}

pub fn forjar(inv: &mut Inventario, receta: &Receta) -> Result<Objeto, String> {
    comprobar(inv, &receta.ingredientes)?;
    consumir(inv, &receta.ingredientes);

    inv.objetos.push(Item::nuevo(receta.resultado));
    Ok(receta.resultado)
}

pub fn reparar(inv: &mut Inventario, indice: usize) -> Result<Objeto, String> {
    let item = *inv
        .objetos
        .get(indice)
        .ok_or_else(|| "Objeto no encontrado".to_string())?;

    if item.durabilidad >= item.tipo.durabilidad_max() {
        return Err(format!("{:?} ya esta en buen estado", item.tipo));
    }

    let coste = item.tipo.coste_reparacion(item.tipo.durabilidad_max() - item.durabilidad);
    comprobar(inv, &coste)?;
    consumir(inv, &coste);

    if let Some(objeto) = inv.objetos.get_mut(indice) {
        objeto.durabilidad = item.tipo.durabilidad_max();
    }
    Ok(item.tipo)
}

/// Craft arrows using the recipe from datos.ron.
pub fn fabricar_flechas(inv: &mut Inventario, datos: &Datos) -> Result<u32, String> {
    comprobar(inv, &datos.flecha_receta)?;
    consumir(inv, &datos.flecha_receta);
    *inv.materiales.entry(Material::Flecha).or_insert(0) += datos.flechas_por_fabrica;
    Ok(datos.flechas_por_fabrica)
}

/// Buy arrows from the shopkeeper with gold.
#[allow(dead_code)]
pub fn comprar_flechas(inv: &mut Inventario, datos: &Datos) -> Result<u32, String> {
    if inv.oro < datos.flecha_precio_oro {
        return Err(format!(
            "Necesitas {} de oro para comprar flechas",
            datos.flecha_precio_oro
        ));
    }
    inv.oro -= datos.flecha_precio_oro;
    *inv.materiales.entry(Material::Flecha).or_insert(0) += datos.flechas_por_compra;
    Ok(datos.flechas_por_compra)
}

/// Sell all the magic stones. Return the gold obtained.
#[allow(dead_code)]
pub fn vender_piedras(inv: &mut Inventario) -> Result<u32, String> {
    let cantidad = inv.cantidad(Material::PiedraMagica);
    if cantidad == 0 {
        return Err("No tienes piedras magicas para vender".to_string());
    }

    inv.materiales.insert(Material::PiedraMagica, 0);
    let ganado = cantidad * PRECIO_PIEDRA;
    inv.oro += ganado;
    Ok(ganado)
}

/// Recalculates an item index after deleting the object `borrado` from the list.
fn ajustar_indice(equipado: Option<usize>, borrado: usize) -> Option<usize> {
    match equipado {
        Some(actual) if actual == borrado => None,
        Some(actual) if actual > borrado => Some(actual - 1),
        otro => otro,
    }
}

pub fn tirar(inv: &mut Inventario, indice: usize) -> Result<Objeto, String> {
    let item = *inv
        .objetos
        .get(indice)
        .ok_or_else(|| "Objeto no encontrado".to_string())?;

    if item.durabilidad > 0 {
        return Err(format!("{:?} no esta rota", item.tipo));
    }

    inv.objetos.remove(indice);
    inv.equipada = ajustar_indice(inv.equipada, indice);
    inv.escudo = ajustar_indice(inv.escudo, indice);
    Ok(item.tipo)
}

/// Bow only for the Archer. Shield, sword, axe, and spear only for the Warrior.
pub fn puede_equipar(objeto: Objeto, clase: Personaje) -> bool {
    match objeto {
        Objeto::Arco => matches!(clase, Personaje::Arquero),
        _ => matches!(clase, Personaje::Guerrero),
    }
}

pub fn nombre_clase(clase: Personaje) -> &'static str {
    match clase {
        Personaje::Arquero => "Arquero",
        Personaje::Guerrero => "Guerrero",
    }
}

pub fn equipar(inv: &mut Inventario, indice: usize, clase: Personaje) -> Result<Objeto, String> {
    let item = *inv
        .objetos
        .get(indice)
        .ok_or_else(|| "Objeto no encontrado".to_string())?;

    if !puede_equipar(item.tipo, clase) {
        return Err(format!(
            "{} no puede usar {:?}. Cambia de clase con Tab",
            nombre_clase(clase),
            item.tipo
        ));
    }
    if item.durabilidad == 0 {
        return Err(format!("{:?} esta rota", item.tipo));
    }

    if item.tipo == Objeto::Escudo {
        inv.escudo = Some(indice);
    } else {
        inv.equipada = Some(indice);
    }
    Ok(item.tipo)
}

/// Switching classes using the Tab key: unequip items that the new class cannot use.
pub fn desequipar_incompatibles(inv: &mut Inventario, clase: Personaje) {
    let tipo_arma = inv.arma_equipada().map(|i| i.tipo);
    if let Some(tipo) = tipo_arma {
        if !puede_equipar(tipo, clase) {
            inv.equipada = None;
        }
    }

    let tipo_escudo = inv.escudo_equipado().map(|i| i.tipo);
    if let Some(tipo) = tipo_escudo {
        if !puede_equipar(tipo, clase) {
            inv.escudo = None;
        }
    }
}

pub fn desequipar_arma(inv: &mut Inventario) {
    inv.equipada = None;
}

pub fn desequipar_escudo(inv: &mut Inventario) {
    inv.escudo = None;
}

/// Returns the damage of the hit, and indicates if the weapon has just broken.
/// With no weapon equipped, the hit deals 1 damage (fists).
pub fn golpear(inv: &mut Inventario) -> (u32, bool) {
    let Some(indice) = inv.equipada else {
        return (1, false);
    };
    let Some(item) = inv.objetos.get_mut(indice) else {
        inv.equipada = None;
        return (1, false);
    };

    item.durabilidad = item.durabilidad.saturating_sub(1);
    let dano = item.tipo.dano();
    let rota = item.durabilidad == 0;

    if rota {
        inv.equipada = None;
    }
    (dano, rota)
}

/// Consumes one arrow and 1 point of bow durability. Returns the arrow's damage, as well as whether the bow broke.
#[allow(dead_code)]
pub fn disparar_arco(inv: &mut Inventario) -> Result<(u32, bool), String> {
    let indice = match inv.equipada {
        Some(i) if inv.objetos.get(i).map(|o| o.tipo) == Some(Objeto::Arco) => i,
        _ => return Err("No llevas el arco equipado".to_string()),
    };

    let flechas = inv.cantidad(Material::Flecha);
    if flechas == 0 {
        return Err("No te quedan flechas".to_string());
    }
    inv.materiales.insert(Material::Flecha, flechas - 1);

    let item = &mut inv.objetos[indice];
    item.durabilidad = item.durabilidad.saturating_sub(1);
    let dano = item.tipo.dano();
    let rota = item.durabilidad == 0;

    if rota {
        inv.equipada = None;
    }
    Ok((dano, rota))
}

/// Apply the shield to an incoming hit: it halves the damage and consumes 1 point of durability.
/// Return the final damage, and indicate whether the shield has just broken.
pub fn bloquear(inv: &mut Inventario, dano: u32) -> (u32, bool) {
    let Some(indice) = inv.escudo else {
        return (dano, false);
    };
    let Some(item) = inv.objetos.get_mut(indice) else {
        inv.escudo = None;
        return (dano, false);
    };

    item.durabilidad = item.durabilidad.saturating_sub(1);
    let rota = item.durabilidad == 0;

    if rota {
        inv.escudo = None;
    }
    (dano / 2, rota)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_default_values_maintain_behavior() {
        let item = Item::nuevo(Objeto::Espada);
        assert_eq!(item.tipo, Objeto::Espada);
        assert_eq!(item.durabilidad, Objeto::Espada.durabilidad_max());
        assert_eq!(item.rareza, Rareza::Comun);
        assert_eq!(item.mejora, Mejora::Base);
    }

    #[test]
    fn test_rareza_default() {
        assert_eq!(Rareza::default(), Rareza::Comun);
    }

    #[test]
    fn test_mejora_default() {
        assert_eq!(Mejora::default(), Mejora::Base);
    }

    #[test]
    fn test_estacion_forja_default() {
        assert_eq!(EstacionForja::default(), EstacionForja::Yunque);
    }

    #[test]
    fn test_receta_default_values_maintain_behavior() {
        let recetas = recetas();
        for receta in &recetas {
            assert_eq!(receta.estacion, EstacionForja::Yunque);
            assert_eq!(receta.mejora, Mejora::Base);
            assert_eq!(receta.probabilidad_exito, 1.0);
        }
    }

    #[test]
    fn test_material_display_new_materials() {
        assert_eq!(Material::Cobre.to_string(), "Cobre");
        assert_eq!(Material::Oro.to_string(), "Oro (mineral)");
        assert_eq!(Material::Diamante.to_string(), "Diamante");
    }

    #[test]
    fn test_material_display_existing_materials() {
        assert_eq!(Material::Hierro.to_string(), "Hierro");
        assert_eq!(Material::Madera.to_string(), "Madera");
        assert_eq!(Material::Cuero.to_string(), "Cuero");
        assert_eq!(Material::PiedraMagica.to_string(), "Piedra magica");
        assert_eq!(Material::Flecha.to_string(), "Flecha");
    }
}