use crate::card::Card;
use crate::consumable::{Consumable, ConsumableType};
use crate::error::GameError;
use crate::game::Game;
use crate::rank::HandRank;
use pyo3::pyclass;
use strum::{EnumIter, IntoEnumIterator};

/// The 12 Planet cards that upgrade poker hands
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, Hash)]
pub enum Planets {
    Pluto,   // High Card
    Mercury, // Straight
    Venus,   // Flush
    Earth,   // Full House
    Mars,    // Four of a Kind
    Jupiter, // Five of a Kind (secret)
    Saturn,  // Straight Flush
    Uranus,  // Flush House (secret)
    Neptune, // Royal Flush
    Ceres,   // Two Pair (secret)
    Eris,    // Pair (secret)
    PlanetX, // Special hands (secret)
}

impl Planets {
    /// Get the hand rank this planet upgrades
    pub fn hand_rank(&self) -> HandRank {
        match self {
            Self::Pluto => HandRank::HighCard,
            Self::Eris => HandRank::OnePair,
            Self::Ceres => HandRank::TwoPair,
            Self::PlanetX => HandRank::ThreeOfAKind, // Placeholder
            Self::Mercury => HandRank::Straight,
            Self::Venus => HandRank::Flush,
            Self::Earth => HandRank::FullHouse,
            Self::Mars => HandRank::FourOfAKind,
            Self::Jupiter => HandRank::FiveOfAKind,
            Self::Saturn => HandRank::StraightFlush,
            Self::Uranus => HandRank::FlushHouse,
            Self::Neptune => HandRank::RoyalFlush,
        }
    }

    /// Check if this planet is a secret planet
    pub fn is_secret(&self) -> bool {
        matches!(
            self,
            Self::Jupiter | Self::Uranus | Self::Ceres | Self::Eris | Self::PlanetX
        )
    }

    /// Get all planet cards
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}

impl Consumable for Planets {
    fn name(&self) -> String {
        match self {
            Self::Pluto => "Pluto".to_string(),
            Self::Mercury => "Mercury".to_string(),
            Self::Venus => "Venus".to_string(),
            Self::Earth => "Earth".to_string(),
            Self::Mars => "Mars".to_string(),
            Self::Jupiter => "Jupiter".to_string(),
            Self::Saturn => "Saturn".to_string(),
            Self::Uranus => "Uranus".to_string(),
            Self::Neptune => "Neptune".to_string(),
            Self::Ceres => "Ceres".to_string(),
            Self::Eris => "Eris".to_string(),
            Self::PlanetX => "Planet X".to_string(),
        }
    }

    fn desc(&self) -> String {
        format!("Level up {}", self.hand_rank())
    }

    fn cost(&self) -> usize {
        3 // Standard planet cost
    }

    fn requires_target(&self) -> bool {
        false // Planets don't need targets
    }

    fn max_targets(&self) -> usize {
        0
    }

    fn use_effect(&self, game: &mut Game, _targets: Option<Vec<Card>>) -> Result<(), GameError> {
        // Upgrade the hand rank associated with this planet
        game.upgrade_hand(self.hand_rank());
        Ok(())
    }

    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }
}

impl std::fmt::Display for HandRank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandRank::HighCard => write!(f, "High Card"),
            HandRank::OnePair => write!(f, "Pair"),
            HandRank::TwoPair => write!(f, "Two Pair"),
            HandRank::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandRank::Straight => write!(f, "Straight"),
            HandRank::Flush => write!(f, "Flush"),
            HandRank::FullHouse => write!(f, "Full House"),
            HandRank::FourOfAKind => write!(f, "Four of a Kind"),
            HandRank::StraightFlush => write!(f, "Straight Flush"),
            HandRank::RoyalFlush => write!(f, "Royal Flush"),
            HandRank::FiveOfAKind => write!(f, "Five of a Kind"),
            HandRank::FlushHouse => write!(f, "Flush House"),
            HandRank::FlushFive => write!(f, "Flush Five"),
        }
    }
}
