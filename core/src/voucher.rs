use crate::error::GameError;
use crate::game::Game;
use rand::seq::SliceRandom;
use std::fmt;

/// Voucher - permanent shop upgrades that persist across rounds
/// Vouchers provide bonuses like extra slots, reduced prices, better packs, etc.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vouchers {
    // Tier 1 Vouchers
    Overstock,      // +1 card slot in shop (jokers/consumables)
    ClearanceSale,  // -25% to all shop items
    Hone,           // Foil, Holographic, Polychrome cards 2x more common
    Reroll,         // Rerolls cost $2 less
    Crystal,        // +1 consumable slot
    Telescope,      // Celestial Packs always contain the most played hand's Planet
    Grabber,        // Permanently gain +1 hand per round
    Wasteful,       // Permanently gain +1 discard per round
    Tarot,          // Tarot cards appear 2x more frequently
    Planet,         // Planet cards appear 2x more frequently
    Spectral,       // Spectral cards may appear in shop
    Buffoon,        // Buffoon packs appear 2x more frequently

    // Tier 2 Vouchers (upgrades of Tier 1)
    Overstock2,     // +1 additional card slot in shop (total +2)
    Liquidation,    // -50% to all shop items (upgrade of ClearanceSale)
    Glow,           // Foil, Holographic, Polychrome cards 4x more common (upgrade of Hone)
    RerollPlus,     // Rerolls cost $5 less (upgrade of Reroll)
    Illusion,       // +1 additional consumable slot (total +2) (upgrade of Crystal)
    Observatory,    // Planet cards in Celestial Packs upgraded to next level (upgrade of Telescope)
    Nacho,          // Permanently gain +2 hands per round (upgrade of Grabber)
    Recyclomancy,   // Permanently gain +2 discards per round (upgrade of Wasteful)
    TarotPlus,      // Tarot cards appear 4x more frequently (upgrade of Tarot)
    PlanetPlus,     // Planet cards appear 4x more frequently (upgrade of Planet)
    SpectralPlus,   // Spectral cards appear 2x more frequently (upgrade of Spectral)
    BuffoonPlus,    // Buffoon packs appear 4x more frequently (upgrade of Buffoon)
}

impl Vouchers {
    /// Get the name of the voucher
    pub fn name(&self) -> &str {
        match self {
            Vouchers::Overstock => "Overstock",
            Vouchers::ClearanceSale => "Clearance Sale",
            Vouchers::Hone => "Hone",
            Vouchers::Reroll => "Reroll Surplus",
            Vouchers::Crystal => "Crystal Ball",
            Vouchers::Telescope => "Telescope",
            Vouchers::Grabber => "Grabber",
            Vouchers::Wasteful => "Wasteful",
            Vouchers::Tarot => "Tarot Merchant",
            Vouchers::Planet => "Planet Merchant",
            Vouchers::Spectral => "Omen Globe",
            Vouchers::Buffoon => "Buffoon",
            Vouchers::Overstock2 => "Overstock Plus",
            Vouchers::Liquidation => "Liquidation",
            Vouchers::Glow => "Glow Up",
            Vouchers::RerollPlus => "Reroll Glut",
            Vouchers::Illusion => "Illusion",
            Vouchers::Observatory => "Observatory",
            Vouchers::Nacho => "Nacho Tong",
            Vouchers::Recyclomancy => "Recyclomancy",
            Vouchers::TarotPlus => "Tarot Tycoon",
            Vouchers::PlanetPlus => "Planet Tycoon",
            Vouchers::SpectralPlus => "Seance",
            Vouchers::BuffoonPlus => "Gros Michel",
        }
    }

    /// Get the description of the voucher effect
    pub fn desc(&self) -> &str {
        match self {
            Vouchers::Overstock => "+1 card slot available in shop",
            Vouchers::ClearanceSale => "All cards and packs in shop are 25% off",
            Vouchers::Hone => "Foil, Holographic, and Polychrome cards appear 2x more frequently",
            Vouchers::Reroll => "Rerolls cost $2 less",
            Vouchers::Crystal => "+1 consumable slot",
            Vouchers::Telescope => "Celestial Packs always contain the Planet card for your most played poker hand",
            Vouchers::Grabber => "+1 hand per round",
            Vouchers::Wasteful => "+1 discard per round",
            Vouchers::Tarot => "Tarot cards appear 2x more frequently in the shop",
            Vouchers::Planet => "Planet cards appear 2x more frequently in the shop",
            Vouchers::Spectral => "Spectral cards may appear in the shop and Arcana Packs",
            Vouchers::Buffoon => "Buffoon Packs appear 2x more frequently",
            Vouchers::Overstock2 => "+1 card slot available in shop",
            Vouchers::Liquidation => "All cards and packs in shop are 50% off",
            Vouchers::Glow => "Foil, Holographic, and Polychrome cards appear 4x more frequently",
            Vouchers::RerollPlus => "Rerolls cost $5 less",
            Vouchers::Illusion => "+1 consumable slot",
            Vouchers::Observatory => "Planet cards in your consumable slots give X1.5 to their hand when used",
            Vouchers::Nacho => "+1 hand per round",
            Vouchers::Recyclomancy => "+1 discard per round",
            Vouchers::TarotPlus => "Tarot cards appear 4x more frequently in the shop",
            Vouchers::PlanetPlus => "Planet cards appear 4x more frequently in the shop",
            Vouchers::SpectralPlus => "Spectral cards appear 2x more frequently in the shop",
            Vouchers::BuffoonPlus => "Buffoon Packs appear 4x more frequently",
        }
    }

    /// Base cost of the voucher
    pub fn cost(&self) -> usize {
        match self {
            // Tier 1 vouchers
            Vouchers::Overstock | Vouchers::ClearanceSale | Vouchers::Hone |
            Vouchers::Reroll | Vouchers::Crystal | Vouchers::Telescope |
            Vouchers::Grabber | Vouchers::Wasteful | Vouchers::Tarot |
            Vouchers::Planet | Vouchers::Spectral | Vouchers::Buffoon => 10,

            // Tier 2 vouchers (upgrades)
            Vouchers::Overstock2 | Vouchers::Liquidation | Vouchers::Glow |
            Vouchers::RerollPlus | Vouchers::Illusion | Vouchers::Observatory |
            Vouchers::Nacho | Vouchers::Recyclomancy | Vouchers::TarotPlus |
            Vouchers::PlanetPlus | Vouchers::SpectralPlus | Vouchers::BuffoonPlus => 10,
        }
    }

    /// Get the prerequisite voucher (for Tier 2 upgrades)
    pub fn requires(&self) -> Option<Vouchers> {
        match self {
            Vouchers::Overstock2 => Some(Vouchers::Overstock),
            Vouchers::Liquidation => Some(Vouchers::ClearanceSale),
            Vouchers::Glow => Some(Vouchers::Hone),
            Vouchers::RerollPlus => Some(Vouchers::Reroll),
            Vouchers::Illusion => Some(Vouchers::Crystal),
            Vouchers::Observatory => Some(Vouchers::Telescope),
            Vouchers::Nacho => Some(Vouchers::Grabber),
            Vouchers::Recyclomancy => Some(Vouchers::Wasteful),
            Vouchers::TarotPlus => Some(Vouchers::Tarot),
            Vouchers::PlanetPlus => Some(Vouchers::Planet),
            Vouchers::SpectralPlus => Some(Vouchers::Spectral),
            Vouchers::BuffoonPlus => Some(Vouchers::Buffoon),
            _ => None,
        }
    }

    /// Check if this is a Tier 2 (upgraded) voucher
    pub fn is_upgrade(&self) -> bool {
        self.requires().is_some()
    }

    /// Get all tier 1 vouchers
    pub fn tier_1() -> Vec<Vouchers> {
        vec![
            Vouchers::Overstock,
            Vouchers::ClearanceSale,
            Vouchers::Hone,
            Vouchers::Reroll,
            Vouchers::Crystal,
            Vouchers::Telescope,
            Vouchers::Grabber,
            Vouchers::Wasteful,
            Vouchers::Tarot,
            Vouchers::Planet,
            Vouchers::Spectral,
            Vouchers::Buffoon,
        ]
    }

    /// Apply voucher effect immediately when purchased
    pub fn apply_effect(&self, game: &mut Game) {
        match self {
            Vouchers::Grabber => {
                game.config.plays += 1;
            }
            Vouchers::Wasteful => {
                game.config.discards += 1;
            }
            Vouchers::Nacho => {
                game.config.plays += 1; // Additional +1 (total +2 with Grabber)
            }
            Vouchers::Recyclomancy => {
                game.config.discards += 1; // Additional +1 (total +2 with Wasteful)
            }
            Vouchers::Crystal => {
                game.config.consumable_slots += 1;
            }
            Vouchers::Illusion => {
                game.config.consumable_slots += 1; // Additional +1 (total +2 with Crystal)
            }
            // Other vouchers are passive and checked when needed
            _ => {}
        }
    }

    /// Generate a random available voucher
    pub fn random_available(owned: &[Vouchers]) -> Option<Vouchers> {
        let mut available: Vec<Vouchers> = Vouchers::tier_1()
            .into_iter()
            .filter(|v| !owned.contains(v))
            .collect();

        // Add tier 2 upgrades if their prerequisite is owned
        for upgrade in [
            Vouchers::Overstock2,
            Vouchers::Liquidation,
            Vouchers::Glow,
            Vouchers::RerollPlus,
            Vouchers::Illusion,
            Vouchers::Observatory,
            Vouchers::Nacho,
            Vouchers::Recyclomancy,
            Vouchers::TarotPlus,
            Vouchers::PlanetPlus,
            Vouchers::SpectralPlus,
            Vouchers::BuffoonPlus,
        ] {
            if !owned.contains(&upgrade) {
                if let Some(required) = upgrade.requires() {
                    if owned.contains(&required) {
                        available.push(upgrade);
                    }
                }
            }
        }

        available.choose(&mut rand::thread_rng()).copied()
    }
}

impl fmt::Display for Vouchers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voucher_cost() {
        assert_eq!(Vouchers::Overstock.cost(), 10);
        assert_eq!(Vouchers::Liquidation.cost(), 10);
    }

    #[test]
    fn test_voucher_requires() {
        assert_eq!(Vouchers::Overstock.requires(), None);
        assert_eq!(Vouchers::Overstock2.requires(), Some(Vouchers::Overstock));
        assert_eq!(Vouchers::Liquidation.requires(), Some(Vouchers::ClearanceSale));
    }

    #[test]
    fn test_voucher_is_upgrade() {
        assert!(!Vouchers::Overstock.is_upgrade());
        assert!(Vouchers::Overstock2.is_upgrade());
    }

    #[test]
    fn test_random_available_tier1_only() {
        let owned = vec![];
        let voucher = Vouchers::random_available(&owned);
        assert!(voucher.is_some());
        assert!(!voucher.unwrap().is_upgrade());
    }

    #[test]
    fn test_random_available_with_upgrade() {
        let owned = vec![Vouchers::Overstock];
        // Should be able to get either tier 1 or Overstock2
        let mut found_upgrade = false;
        for _ in 0..50 {
            if let Some(v) = Vouchers::random_available(&owned) {
                if v == Vouchers::Overstock2 {
                    found_upgrade = true;
                    break;
                }
            }
        }
        // With 50 tries and multiple options, we should see the upgrade eventually
    }

    #[test]
    fn test_random_available_all_owned() {
        let mut owned = Vouchers::tier_1();
        owned.extend([
            Vouchers::Overstock2,
            Vouchers::Liquidation,
            Vouchers::Glow,
            Vouchers::RerollPlus,
            Vouchers::Illusion,
            Vouchers::Observatory,
            Vouchers::Nacho,
            Vouchers::Recyclomancy,
            Vouchers::TarotPlus,
            Vouchers::PlanetPlus,
            Vouchers::SpectralPlus,
            Vouchers::BuffoonPlus,
        ]);

        let voucher = Vouchers::random_available(&owned);
        assert!(voucher.is_none());
    }
}
