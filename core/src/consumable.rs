use crate::card::Card;
use crate::error::GameError;
use crate::game::Game;
use crate::planet::Planets;
use crate::spectral::Spectrals;
use crate::tarot::Tarots;

/// Trait for all consumable items (Tarots, Planets, Spectrals)
pub trait Consumable: std::fmt::Debug + Clone {
    /// Get the name of this consumable
    fn name(&self) -> String;

    /// Get the description of this consumable's effect
    fn desc(&self) -> String;

    /// Get the cost to purchase this consumable in the shop
    fn cost(&self) -> usize;

    /// Check if this consumable requires card targets
    fn requires_target(&self) -> bool;

    /// Get the maximum number of cards that can be targeted
    fn max_targets(&self) -> usize;

    /// Get the minimum number of cards that must be targeted
    fn min_targets(&self) -> usize {
        if self.requires_target() {
            1
        } else {
            0
        }
    }

    /// Execute the consumable's effect
    /// Returns Ok(()) if successful, Err if the effect failed
    fn use_effect(&self, game: &mut Game, targets: Option<Vec<Card>>) -> Result<(), GameError>;

    /// Get the type of consumable (for categorization)
    fn consumable_type(&self) -> ConsumableType;
}

/// Type of consumable
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsumableType {
    Tarot,
    Planet,
    Spectral,
}

impl std::fmt::Display for ConsumableType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tarot => write!(f, "Tarot"),
            Self::Planet => write!(f, "Planet"),
            Self::Spectral => write!(f, "Spectral"),
        }
    }
}

/// Unified enum for all consumables (similar to Jokers enum)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyo3::pyclass(eq))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Consumables {
    Tarot(Tarots),
    Planet(Planets),
    Spectral(Spectrals),
}

impl Consumable for Consumables {
    fn name(&self) -> String {
        match self {
            Self::Tarot(t) => t.name(),
            Self::Planet(p) => p.name(),
            Self::Spectral(s) => s.name(),
        }
    }

    fn desc(&self) -> String {
        match self {
            Self::Tarot(t) => t.desc(),
            Self::Planet(p) => p.desc(),
            Self::Spectral(s) => s.desc(),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Self::Tarot(t) => t.cost(),
            Self::Planet(p) => p.cost(),
            Self::Spectral(s) => s.cost(),
        }
    }

    fn requires_target(&self) -> bool {
        match self {
            Self::Tarot(t) => t.requires_target(),
            Self::Planet(p) => p.requires_target(),
            Self::Spectral(s) => s.requires_target(),
        }
    }

    fn max_targets(&self) -> usize {
        match self {
            Self::Tarot(t) => t.max_targets(),
            Self::Planet(p) => p.max_targets(),
            Self::Spectral(s) => s.max_targets(),
        }
    }

    fn min_targets(&self) -> usize {
        match self {
            Self::Tarot(t) => t.min_targets(),
            Self::Planet(p) => p.min_targets(),
            Self::Spectral(s) => s.min_targets(),
        }
    }

    fn use_effect(&self, game: &mut Game, targets: Option<Vec<Card>>) -> Result<(), GameError> {
        match self {
            Self::Tarot(t) => t.use_effect(game, targets),
            Self::Planet(p) => p.use_effect(game, targets),
            Self::Spectral(s) => s.use_effect(game, targets),
        }
    }

    fn consumable_type(&self) -> ConsumableType {
        match self {
            Self::Tarot(_) => ConsumableType::Tarot,
            Self::Planet(_) => ConsumableType::Planet,
            Self::Spectral(_) => ConsumableType::Spectral,
        }
    }
}

impl std::fmt::Display for Consumables {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planet::Planets;
    use crate::spectral::Spectrals;
    use crate::tarot::Tarots;

    #[test]
    fn test_tarot_consumable_trait() {
        let tarot = Consumables::Tarot(Tarots::TheMagician);
        assert_eq!(tarot.name(), "The Magician");
        assert_eq!(tarot.cost(), 3);
        assert!(tarot.requires_target());
        assert_eq!(tarot.max_targets(), 2);
        assert_eq!(tarot.consumable_type(), ConsumableType::Tarot);
    }

    #[test]
    fn test_planet_consumable_trait() {
        let planet = Consumables::Planet(Planets::Mercury);
        assert_eq!(planet.name(), "Mercury");
        assert_eq!(planet.cost(), 3);
        assert!(!planet.requires_target());
        assert_eq!(planet.max_targets(), 0);
        assert_eq!(planet.consumable_type(), ConsumableType::Planet);
    }

    #[test]
    fn test_spectral_consumable_trait() {
        let spectral = Consumables::Spectral(Spectrals::BlackHole);
        assert_eq!(spectral.name(), "Black Hole");
        assert_eq!(spectral.cost(), 4);
        assert!(!spectral.requires_target());
        assert_eq!(spectral.max_targets(), 0);
        assert_eq!(spectral.consumable_type(), ConsumableType::Spectral);
    }

    #[test]
    fn test_tarot_targeting_requirements() {
        // No target tarots
        let hermit = Consumables::Tarot(Tarots::TheHermit);
        assert!(!hermit.requires_target());
        assert_eq!(hermit.max_targets(), 0);

        // Single target tarots
        let lovers = Consumables::Tarot(Tarots::TheLovers);
        assert!(lovers.requires_target());
        assert_eq!(lovers.max_targets(), 1);
        assert_eq!(lovers.min_targets(), 1);

        // Dual target tarots
        let magician = Consumables::Tarot(Tarots::TheMagician);
        assert!(magician.requires_target());
        assert_eq!(magician.max_targets(), 2);
        assert_eq!(magician.min_targets(), 1);

        // Triple target tarots
        let star = Consumables::Tarot(Tarots::TheStar);
        assert!(star.requires_target());
        assert_eq!(star.max_targets(), 3);
        assert_eq!(star.min_targets(), 1);
    }

    #[test]
    fn test_all_tarots_have_descriptions() {
        for tarot in Tarots::all() {
            let consumable = Consumables::Tarot(tarot);
            assert!(!consumable.name().is_empty());
            assert!(!consumable.desc().is_empty());
            assert!(consumable.cost() > 0);
        }
    }

    #[test]
    fn test_all_planets_have_descriptions() {
        for planet in Planets::all() {
            let consumable = Consumables::Planet(planet);
            assert!(!consumable.name().is_empty());
            assert!(!consumable.desc().is_empty());
            assert!(consumable.cost() > 0);
            assert!(!consumable.requires_target());
        }
    }

    #[test]
    fn test_all_spectrals_have_descriptions() {
        for spectral in Spectrals::all() {
            let consumable = Consumables::Spectral(spectral);
            assert!(!consumable.name().is_empty());
            assert!(!consumable.desc().is_empty());
            assert!(consumable.cost() > 0);
        }
    }

    #[test]
    fn test_secret_planets() {
        assert!(Planets::Jupiter.is_secret());
        assert!(Planets::Uranus.is_secret());
        assert!(Planets::Ceres.is_secret());
        assert!(Planets::Eris.is_secret());
        assert!(Planets::PlanetX.is_secret());

        assert!(!Planets::Mercury.is_secret());
        assert!(!Planets::Venus.is_secret());
        assert!(!Planets::Earth.is_secret());
    }

    #[test]
    fn test_consumables_equality() {
        let tarot1 = Consumables::Tarot(Tarots::TheFool);
        let tarot2 = Consumables::Tarot(Tarots::TheFool);
        let tarot3 = Consumables::Tarot(Tarots::TheMagician);

        assert_eq!(tarot1, tarot2);
        assert_ne!(tarot1, tarot3);
    }

    #[test]
    fn test_consumable_display() {
        let tarot = Consumables::Tarot(Tarots::TheWorld);
        assert_eq!(format!("{}", tarot), "The World");

        let planet = Consumables::Planet(Planets::Jupiter);
        assert_eq!(format!("{}", planet), "Jupiter");

        let spectral = Consumables::Spectral(Spectrals::Wraith);
        assert_eq!(format!("{}", spectral), "Wraith");
    }
}

