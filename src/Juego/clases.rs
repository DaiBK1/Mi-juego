use bevy::prelude::*;
use crate::juego::personajes::Personaje;

/// Level requirement for the base class (Aprendiz).
const NIVEL_REQUISITO_BASE: u32 = 1;
/// Level requirement for tier 1 classes (Guerrero, Arquero).
const NIVEL_REQUISITO_TIER1: u32 = 5;

/// Base health for all classes.
const VIDA_BASE: u32 = 100;
/// Base attack for all classes.
const ATAQUE_BASE: u32 = 10;

/// Character class with name, tier, and level requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ClaseJugador {
    /// Base class for all characters.
    Aprendiz,
    /// Tier 1 melee combat class.
    Guerrero,
    /// Tier 1 ranged combat class.
    Arquero,
}

#[allow(dead_code)]
impl ClaseJugador {
    /// Returns the display name of the class.
    pub fn nombre(&self) -> &'static str {
        match self {
            ClaseJugador::Aprendiz => "Aprendiz",
            ClaseJugador::Guerrero => "Guerrero",
            ClaseJugador::Arquero => "Arquero",
        }
    }

    /// Returns the tier of the class (0 for base, 1 for tier 1).
    pub fn tier(&self) -> u32 {
        match self {
            ClaseJugador::Aprendiz => 0,
            ClaseJugador::Guerrero | ClaseJugador::Arquero => 1,
        }
    }

    /// Returns the level required to switch to this class.
    pub fn requisito_nivel(&self) -> u32 {
        match self {
            ClaseJugador::Aprendiz => NIVEL_REQUISITO_BASE,
            ClaseJugador::Guerrero | ClaseJugador::Arquero => NIVEL_REQUISITO_TIER1,
        }
    }

    /// Returns the base health for this class.
    pub fn vida_base(&self) -> u32 {
        VIDA_BASE
    }

    /// Returns the base attack for this class.
    pub fn ataque_base(&self) -> u32 {
        ATAQUE_BASE
    }

    /// Maps this class to the corresponding Personaje type for equipment rules.
    pub fn personaje(&self) -> Personaje {
        match self {
            ClaseJugador::Aprendiz => Personaje::Guerrero,
            ClaseJugador::Guerrero => Personaje::Guerrero,
            ClaseJugador::Arquero => Personaje::Arquero,
        }
    }
}

/// Player's current class resource.
#[derive(Resource, Clone, Copy)]
#[allow(dead_code)]
pub struct ClaseActual {
    pub clase: ClaseJugador,
}

impl Default for ClaseActual {
    fn default() -> Self {
        Self {
            clase: ClaseJugador::Aprendiz,
        }
    }
}

/// Checks if the player can change to the target class based on level and current class.
#[allow(dead_code)]
pub fn puede_cambiar_clase(
    nivel_actual: u32,
    clase_actual: ClaseJugador,
    clase_objetivo: ClaseJugador,
) -> bool {
    // Cannot change to the same class
    if clase_actual == clase_objetivo {
        return false;
    }

    // Must meet level requirement
    if nivel_actual < clase_objetivo.requisito_nivel() {
        return false;
    }

    // TODO: Design decision pending - should players be able to switch directly between tier 1 classes?
    // Currently allowed, but this may need to be restricted based on game balance.
    // Options:
    // 1. Disallow direct tier 1 to tier 1 switching (must return to base class first)
    // 2. Allow direct switching with a cooldown or cost
    // 3. Allow direct switching with no restrictions (current implementation)

    true
}

/// Changes the player's class and returns the new base stats.
#[allow(dead_code)]
pub fn cambiar_clase(clase_objetivo: ClaseJugador) -> (u32, u32) {
    let vida = clase_objetivo.vida_base();
    let ataque = clase_objetivo.ataque_base();
    (vida, ataque)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clase_nombre() {
        assert_eq!(ClaseJugador::Aprendiz.nombre(), "Aprendiz");
        assert_eq!(ClaseJugador::Guerrero.nombre(), "Guerrero");
        assert_eq!(ClaseJugador::Arquero.nombre(), "Arquero");
    }

    #[test]
    fn test_clase_tier() {
        assert_eq!(ClaseJugador::Aprendiz.tier(), 0);
        assert_eq!(ClaseJugador::Guerrero.tier(), 1);
        assert_eq!(ClaseJugador::Arquero.tier(), 1);
    }

    #[test]
    fn test_requisito_nivel() {
        assert_eq!(ClaseJugador::Aprendiz.requisito_nivel(), NIVEL_REQUISITO_BASE);
        assert_eq!(ClaseJugador::Guerrero.requisito_nivel(), NIVEL_REQUISITO_TIER1);
        assert_eq!(ClaseJugador::Arquero.requisito_nivel(), NIVEL_REQUISITO_TIER1);
    }

    #[test]
    fn test_estadisticas_base() {
        assert_eq!(ClaseJugador::Aprendiz.vida_base(), VIDA_BASE);
        assert_eq!(ClaseJugador::Aprendiz.ataque_base(), ATAQUE_BASE);
        assert_eq!(ClaseJugador::Guerrero.vida_base(), VIDA_BASE);
        assert_eq!(ClaseJugador::Guerrero.ataque_base(), ATAQUE_BASE);
        assert_eq!(ClaseJugador::Arquero.vida_base(), VIDA_BASE);
        assert_eq!(ClaseJugador::Arquero.ataque_base(), ATAQUE_BASE);
    }

    #[test]
    fn test_puede_cambiar_clase_misma_clase() {
        assert!(!puede_cambiar_clase(1, ClaseJugador::Aprendiz, ClaseJugador::Aprendiz));
        assert!(!puede_cambiar_clase(5, ClaseJugador::Guerrero, ClaseJugador::Guerrero));
    }

    #[test]
    fn test_puede_cambiar_clase_nivel_insuficiente() {
        assert!(!puede_cambiar_clase(1, ClaseJugador::Aprendiz, ClaseJugador::Guerrero));
        assert!(!puede_cambiar_clase(4, ClaseJugador::Aprendiz, ClaseJugador::Arquero));
    }

    #[test]
    fn test_puede_cambiar_clase_nivel_suficiente() {
        assert!(puede_cambiar_clase(5, ClaseJugador::Aprendiz, ClaseJugador::Guerrero));
        assert!(puede_cambiar_clase(5, ClaseJugador::Aprendiz, ClaseJugador::Arquero));
        assert!(puede_cambiar_clase(10, ClaseJugador::Aprendiz, ClaseJugador::Guerrero));
    }

    #[test]
    fn test_puede_cambiar_clase_entre_tier1() {
        // Now allowed (design decision pending)
        assert!(puede_cambiar_clase(10, ClaseJugador::Guerrero, ClaseJugador::Arquero));
        assert!(puede_cambiar_clase(10, ClaseJugador::Arquero, ClaseJugador::Guerrero));
    }

    #[test]
    fn test_cambiar_clase() {
        let (vida, ataque) = cambiar_clase(ClaseJugador::Aprendiz);
        assert_eq!(vida, VIDA_BASE);
        assert_eq!(ataque, ATAQUE_BASE);

        let (vida, ataque) = cambiar_clase(ClaseJugador::Guerrero);
        assert_eq!(vida, VIDA_BASE);
        assert_eq!(ataque, ATAQUE_BASE);

        let (vida, ataque) = cambiar_clase(ClaseJugador::Arquero);
        assert_eq!(vida, VIDA_BASE);
        assert_eq!(ataque, ATAQUE_BASE);
    }

    #[test]
    fn test_clase_actual_default() {
        let clase_actual = ClaseActual::default();
        assert_eq!(clase_actual.clase, ClaseJugador::Aprendiz);
    }

    #[test]
    fn test_mapeo_personaje() {
        assert_eq!(ClaseJugador::Aprendiz.personaje(), Personaje::Guerrero);
        assert_eq!(ClaseJugador::Guerrero.personaje(), Personaje::Guerrero);
        assert_eq!(ClaseJugador::Arquero.personaje(), Personaje::Arquero);
    }
}
