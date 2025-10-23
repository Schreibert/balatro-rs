use crate::card::Card;
use crate::consumable::{Consumable, ConsumableType};
use crate::error::GameError;
use crate::game::Game;
use pyo3::pyclass;
use strum::{EnumIter, IntoEnumIterator};

/// The 18 Spectral cards with high-impact effects
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, EnumIter, Eq, PartialEq, Hash)]
pub enum Spectrals {
    // Deck enhancement (3)
    Familiar,    // Destroy 1, add 3 enhanced face cards
    Grim,        // Destroy 1, add 2 enhanced Aces
    Incantation, // Destroy 1, add 4 enhanced number cards

    // Seal addition (4)
    Talisman, // Add Gold Seal to 1 card
    DejaVu,   // Add Red Seal to 1 card
    Trance,   // Add Blue Seal to 1 card
    Medium,   // Add Purple Seal to 1 card

    // Edition addition (1)
    Aura, // Add random edition to 1 card

    // Deck transformation (4)
    Sigil,   // All cards → same random suit
    Ouija,   // All cards → same rank, -1 hand size
    Immolate, // Destroy 5 random cards, gain $20
    Cryptid, // Create 2 copies of 1 card

    // Joker manipulation (5)
    Wraith,    // Create Rare Joker, set money to $0
    Ankh,      // Copy 1 Joker, destroy others
    Hex,       // Add Polychrome to 1 Joker, destroy others
    Ectoplasm, // Add Negative to random Joker, -1 hand size
    TheSoul,   // Create Legendary Joker

    // Global effect (1)
    BlackHole, // Upgrade every poker hand
}

impl Consumable for Spectrals {
    fn name(&self) -> String {
        match self {
            Self::Familiar => "Familiar".to_string(),
            Self::Grim => "Grim".to_string(),
            Self::Incantation => "Incantation".to_string(),
            Self::Talisman => "Talisman".to_string(),
            Self::Aura => "Aura".to_string(),
            Self::Wraith => "Wraith".to_string(),
            Self::Sigil => "Sigil".to_string(),
            Self::Ouija => "Ouija".to_string(),
            Self::Ectoplasm => "Ectoplasm".to_string(),
            Self::Immolate => "Immolate".to_string(),
            Self::Ankh => "Ankh".to_string(),
            Self::DejaVu => "Deja Vu".to_string(),
            Self::Hex => "Hex".to_string(),
            Self::Trance => "Trance".to_string(),
            Self::Medium => "Medium".to_string(),
            Self::Cryptid => "Cryptid".to_string(),
            Self::TheSoul => "The Soul".to_string(),
            Self::BlackHole => "Black Hole".to_string(),
        }
    }

    fn desc(&self) -> String {
        match self {
            Self::Familiar => "Destroy 1 random card, add 3 random Enhanced Face cards".to_string(),
            Self::Grim => "Destroy 1 random card, add 2 random Enhanced Aces".to_string(),
            Self::Incantation => {
                "Destroy 1 random card, add 4 random Enhanced numbered cards".to_string()
            }
            Self::Talisman => "Add a Gold Seal to 1 selected card".to_string(),
            Self::Aura => "Add Foil, Holographic, or Polychrome to 1 selected card".to_string(),
            Self::Wraith => "Create a Rare Joker, set money to $0".to_string(),
            Self::Sigil => "Convert all cards in hand to a single random suit".to_string(),
            Self::Ouija => "Convert all cards to a single random rank, -1 hand size".to_string(),
            Self::Ectoplasm => "Add Negative to a random Joker, -1 hand size".to_string(),
            Self::Immolate => "Destroy 5 random cards, gain $20".to_string(),
            Self::Ankh => "Create a copy of 1 Joker, destroy all other Jokers".to_string(),
            Self::DejaVu => "Add a Red Seal to 1 selected card".to_string(),
            Self::Hex => "Add Polychrome to 1 Joker, destroy all other Jokers".to_string(),
            Self::Trance => "Add a Blue Seal to 1 selected card".to_string(),
            Self::Medium => "Add a Purple Seal to 1 selected card".to_string(),
            Self::Cryptid => "Create 2 copies of 1 selected card".to_string(),
            Self::TheSoul => "Create a Legendary Joker".to_string(),
            Self::BlackHole => "Upgrade every poker hand".to_string(),
        }
    }

    fn cost(&self) -> usize {
        4 // Spectrals are more expensive
    }

    fn requires_target(&self) -> bool {
        matches!(
            self,
            Self::Talisman
                | Self::Aura
                | Self::DejaVu
                | Self::Trance
                | Self::Medium
                | Self::Cryptid
                // Ankh and Hex target jokers, not cards - for now they don't require targets
                // | Self::Ankh
                // | Self::Hex
        )
    }

    fn max_targets(&self) -> usize {
        match self {
            Self::Talisman
            | Self::Aura
            | Self::DejaVu
            | Self::Trance
            | Self::Medium
            | Self::Cryptid => 1,
            // Ankh and Hex would target jokers, not cards - for now they don't use targets
            // | Self::Ankh
            // | Self::Hex
            _ => 0,
        }
    }

    fn use_effect(&self, game: &mut Game, targets: Option<Vec<Card>>) -> Result<(), GameError> {
        use crate::card::{Edition, Enhancement, Seal, Suit, Value};

        match self {
            // ==================== Category A: Seal Addition ====================
            Self::Talisman => {
                // Add Gold Seal to 1 card
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_seal(Seal::Gold);
                        });
                    }
                }
                Ok(())
            }
            Self::DejaVu => {
                // Add Red Seal to 1 card
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_seal(Seal::Red);
                        });
                    }
                }
                Ok(())
            }
            Self::Trance => {
                // Add Blue Seal to 1 card
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_seal(Seal::Blue);
                        });
                    }
                }
                Ok(())
            }
            Self::Medium => {
                // Add Purple Seal to 1 card
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_seal(Seal::Purple);
                        });
                    }
                }
                Ok(())
            }

            // ==================== Category B: Card Creation/Destruction ====================
            Self::Familiar => {
                // Destroy 1 random card, add 3 enhanced face cards
                if let Some(random_card) = game.get_random_card_from_deck() {
                    game.destroy_card(random_card);
                }
                for _ in 0..3 {
                    let face_card = game.create_enhanced_face_card();
                    game.add_card_to_deck(face_card);
                }
                Ok(())
            }
            Self::Grim => {
                // Destroy 1 random card, add 2 enhanced Aces
                if let Some(random_card) = game.get_random_card_from_deck() {
                    game.destroy_card(random_card);
                }
                for _ in 0..2 {
                    let ace = game.create_enhanced_ace();
                    game.add_card_to_deck(ace);
                }
                Ok(())
            }
            Self::Incantation => {
                // Destroy 1 random card, add 4 enhanced number cards
                if let Some(random_card) = game.get_random_card_from_deck() {
                    game.destroy_card(random_card);
                }
                for _ in 0..4 {
                    let number_card = game.create_enhanced_number();
                    game.add_card_to_deck(number_card);
                }
                Ok(())
            }
            Self::Immolate => {
                // Destroy 5 random cards, gain $20
                let cards_to_destroy = game.get_random_cards(5);
                for card in cards_to_destroy {
                    game.destroy_card(card);
                }
                game.money += 20;
                Ok(())
            }

            // ==================== Category C: Edition/Enhancement ====================
            Self::Aura => {
                // Add random edition (Foil/Holo/Poly) to 1 card
                if let Some(cards) = targets {
                    use rand::seq::SliceRandom;
                    let editions = vec![Edition::Foil, Edition::Holographic, Edition::Polychrome];
                    let edition = *editions.choose(&mut rand::thread_rng()).unwrap();

                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_edition(edition);
                        });
                    }
                }
                Ok(())
            }
            Self::Cryptid => {
                // Create 2 copies of 1 card
                if let Some(cards) = targets {
                    if let Some(card) = cards.first() {
                        // Add 2 copies of the card
                        for _ in 0..2 {
                            let mut copy = Card::new(card.value, card.suit);
                            copy.edition = card.edition;
                            copy.enhancement = card.enhancement;
                            copy.seal = card.seal;
                            game.add_card_to_deck(copy);
                        }
                    }
                }
                Ok(())
            }

            // ==================== Category D: Joker Manipulation ====================
            Self::Wraith => {
                // Create Rare Joker, set money to $0
                let joker = game.generate_rare_joker();
                game.jokers.push(joker);
                game.money = 0;
                Ok(())
            }
            Self::TheSoul => {
                // Create Legendary Joker
                let joker = game.generate_legendary_joker();
                game.jokers.push(joker);
                Ok(())
            }
            Self::Ankh => {
                // Copy 1 Joker, destroy others
                // Note: targets should be joker indices, but we're using Card
                // This is a simplified implementation - just keep first joker
                if !game.jokers.is_empty() {
                    let joker_to_copy = game.jokers[0].clone();
                    game.jokers.clear();
                    game.jokers.push(joker_to_copy);
                }
                Ok(())
            }
            Self::Hex => {
                // Add Polychrome to 1 Joker, destroy others
                // Note: Jokers don't have editions yet, this is a placeholder
                // Just destroy all but one joker for now
                if !game.jokers.is_empty() {
                    let kept_joker = game.jokers[0].clone();
                    game.jokers.clear();
                    game.jokers.push(kept_joker);
                }
                Ok(())
            }
            Self::Ectoplasm => {
                // Add Negative to random Joker, -1 hand size
                // Note: Jokers don't have editions yet, this is a placeholder
                // Just decrease hand size for now
                game.modify_hand_size(-1);
                Ok(())
            }

            // ==================== Category E: Bulk Operations ====================
            Self::Sigil => {
                // Convert all cards to same random suit
                use rand::seq::SliceRandom;
                let suits = vec![Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
                let chosen_suit = *suits.choose(&mut rand::thread_rng()).unwrap();
                game.convert_all_cards_to_suit(chosen_suit);
                Ok(())
            }
            Self::Ouija => {
                // Convert all cards to same rank, -1 hand size
                use rand::seq::SliceRandom;
                let ranks = vec![
                    Value::Two, Value::Three, Value::Four, Value::Five, Value::Six,
                    Value::Seven, Value::Eight, Value::Nine, Value::Ten,
                    Value::Jack, Value::Queen, Value::King, Value::Ace
                ];
                let chosen_rank = *ranks.choose(&mut rand::thread_rng()).unwrap();
                game.convert_all_cards_to_rank(chosen_rank);
                game.modify_hand_size(-1);
                Ok(())
            }

            // ==================== Category F: Universal Upgrade ====================
            Self::BlackHole => {
                // Upgrade all poker hands
                use crate::rank::HandRank;
                for hand_rank in [
                    HandRank::HighCard,
                    HandRank::OnePair,
                    HandRank::TwoPair,
                    HandRank::ThreeOfAKind,
                    HandRank::Straight,
                    HandRank::Flush,
                    HandRank::FullHouse,
                    HandRank::FourOfAKind,
                    HandRank::StraightFlush,
                    HandRank::RoyalFlush,
                    HandRank::FiveOfAKind,
                    HandRank::FlushHouse,
                    HandRank::FlushFive,
                ] {
                    if let Some(level) = game.hand_levels.get_mut(&hand_rank) {
                        *level = level.upgrade();
                    }
                }
                Ok(())
            }
        }
    }

    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }
}

impl Spectrals {
    /// Get all spectral cards
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}
