use crate::card::{Card, Suit, Value};
use crate::config::Config;
use crate::consumable::Consumables;
use crate::joker::Jokers;
use crate::voucher::Vouchers;
use rand::{seq::SliceRandom, thread_rng};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyo3::pyclass(eq))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckType {
    // Standard decks (15 total)
    RedDeck,
    BlueDeck,
    YellowDeck,
    GreenDeck,
    BlackDeck,
    MagicDeck,
    NebulaDeck,
    GhostDeck,
    AbandonedDeck,
    CheckeredDeck,
    ZodiacDeck,
    PaintedDeck,
    AnaglyPhDeck,
    PlasmaDeck,
    ErraticDeck,
}

impl DeckType {
    /// Get all available standard deck types
    pub fn all_standard_decks() -> Vec<DeckType> {
        vec![
            DeckType::RedDeck,
            DeckType::BlueDeck,
            DeckType::YellowDeck,
            DeckType::GreenDeck,
            DeckType::BlackDeck,
            DeckType::MagicDeck,
            DeckType::NebulaDeck,
            DeckType::GhostDeck,
            DeckType::AbandonedDeck,
            DeckType::CheckeredDeck,
            DeckType::ZodiacDeck,
            DeckType::PaintedDeck,
            DeckType::AnaglyPhDeck,
            DeckType::PlasmaDeck,
            DeckType::ErraticDeck,
        ]
    }

    /// Get the name of this deck
    pub fn name(&self) -> &'static str {
        match self {
            DeckType::RedDeck => "Red Deck",
            DeckType::BlueDeck => "Blue Deck",
            DeckType::YellowDeck => "Yellow Deck",
            DeckType::GreenDeck => "Green Deck",
            DeckType::BlackDeck => "Black Deck",
            DeckType::MagicDeck => "Magic Deck",
            DeckType::NebulaDeck => "Nebula Deck",
            DeckType::GhostDeck => "Ghost Deck",
            DeckType::AbandonedDeck => "Abandoned Deck",
            DeckType::CheckeredDeck => "Checkered Deck",
            DeckType::ZodiacDeck => "Zodiac Deck",
            DeckType::PaintedDeck => "Painted Deck",
            DeckType::AnaglyPhDeck => "Anaglyph Deck",
            DeckType::PlasmaDeck => "Plasma Deck",
            DeckType::ErraticDeck => "Erratic Deck",
        }
    }

    /// Apply this deck's modifiers to a Config
    pub fn apply_to_config(&self, config: &mut Config) {
        match self {
            DeckType::RedDeck => {
                // +1 discard every round (total: 4 discards)
                config.discards += 1;
            }
            DeckType::BlueDeck => {
                // +1 hand every round (total: 5 hands)
                config.plays += 1;
            }
            DeckType::YellowDeck => {
                // Start with extra $10 (total: $14 if base is $4)
                config.money_start += 10;
            }
            DeckType::GreenDeck => {
                // Earn $2 per remaining hand, $1 per remaining discard
                // Interest disabled
                config.interest_rate = 0.0;
                config.interest_max = 0;
                config.money_per_hand = 2;
                // Note: money_per_discard would need to be added to Config
            }
            DeckType::BlackDeck => {
                // +1 joker slot (total: 6), -1 hand (total: 3)
                config.joker_slots += 1;
                config.joker_slots_max += 1;
                config.plays -= 1;
            }
            DeckType::MagicDeck => {
                // Starting items handled separately
                // No config changes
            }
            DeckType::NebulaDeck => {
                // -1 consumable slot (total: 1)
                config.consumable_slots -= 1;
                config.consumable_slots_max = config.consumable_slots_max.saturating_sub(1);
                // Starting voucher handled separately
            }
            DeckType::GhostDeck => {
                // Spectral cards can appear in shop
                // This is a shop modifier, handled elsewhere
                // Starting spectral card handled separately
            }
            DeckType::AbandonedDeck => {
                // No face cards in deck
                // Deck generation handled separately
            }
            DeckType::CheckeredDeck => {
                // Only Spades and Hearts in deck
                // Deck generation handled separately
            }
            DeckType::ZodiacDeck => {
                // Starting vouchers handled separately
            }
            DeckType::PaintedDeck => {
                // +2 hand size (total: 10), -1 joker slot (total: 4)
                config.available += 2;
                config.joker_slots -= 1;
                config.joker_slots_max = config.joker_slots_max.saturating_sub(1);
            }
            DeckType::AnaglyPhDeck => {
                // Gain Double Tag after defeating each Boss Blind
                // This is a game mechanic, not a config change
            }
            DeckType::PlasmaDeck => {
                // Balances chips and mult
                // This affects scoring, not config
                // Blinds are 2x base size would need to be in blind generation
            }
            DeckType::ErraticDeck => {
                // All ranks and suits randomized
                // Deck generation handled separately
            }
        }
    }

    /// Get starting vouchers for this deck
    pub fn starting_vouchers(&self) -> Vec<Vouchers> {
        match self {
            DeckType::MagicDeck => {
                vec![Vouchers::Crystal] // Crystal Ball
            }
            DeckType::NebulaDeck => {
                vec![Vouchers::Telescope]
            }
            DeckType::ZodiacDeck => {
                vec![
                    Vouchers::Tarot,      // Tarot Merchant
                    Vouchers::Planet,     // Planet Merchant
                    Vouchers::Overstock,
                ]
            }
            _ => vec![],
        }
    }

    /// Get starting consumables for this deck
    pub fn starting_consumables(&self) -> Vec<Consumables> {
        use crate::spectral::Spectrals;
        use crate::tarot::Tarots;

        match self {
            DeckType::MagicDeck => {
                vec![
                    Consumables::Tarot(Tarots::TheFool),
                    Consumables::Tarot(Tarots::TheFool),
                ]
            }
            DeckType::GhostDeck => {
                vec![Consumables::Spectral(Spectrals::Hex)]
            }
            _ => vec![],
        }
    }

    /// Get starting jokers for this deck
    pub fn starting_jokers(&self) -> Vec<Jokers> {
        // Currently no standard decks start with jokers
        // Challenge decks would be implemented separately
        vec![]
    }

    /// Generate a deck of cards for this deck type
    pub fn generate_cards(&self) -> Vec<Card> {
        match self {
            DeckType::AbandonedDeck => {
                // No face cards (J, Q, K) - only 2-10 and Aces
                let mut cards = Vec::new();
                let values = vec![
                    Value::Two,
                    Value::Three,
                    Value::Four,
                    Value::Five,
                    Value::Six,
                    Value::Seven,
                    Value::Eight,
                    Value::Nine,
                    Value::Ten,
                    Value::Ace,
                ];
                for v in &values {
                    for s in &Suit::suits() {
                        cards.push(Card::new(*v, *s));
                    }
                }
                cards
            }
            DeckType::CheckeredDeck => {
                // Only Spades and Hearts (26 of each = 52 total)
                let mut cards = Vec::new();
                let suits = vec![Suit::Spade, Suit::Heart];
                // For each value, create one of each suit
                for v in &Value::values() {
                    for s in &suits {
                        cards.push(Card::new(*v, *s));
                    }
                }
                // Wait - this should work. Let me check Card::new
                // Actually, this logic looks correct - 13 values × 2 suits = 26 cards
                // But the spec says 52 cards (26 Spades + 26 Hearts)
                // That means 26 cards of EACH suit, not 13 of each
                cards.clear();
                // Create 26 Spades (2 of each rank)
                for _ in 0..2 {
                    for v in &Value::values() {
                        cards.push(Card::new(*v, Suit::Spade));
                    }
                }
                // Create 26 Hearts (2 of each rank)
                for _ in 0..2 {
                    for v in &Value::values() {
                        cards.push(Card::new(*v, Suit::Heart));
                    }
                }
                cards
            }
            DeckType::ErraticDeck => {
                // 52 random cards (any rank, any suit)
                let mut rng = thread_rng();
                let values = Value::values();
                let suits = Suit::suits();
                let mut cards = Vec::new();
                for _ in 0..52 {
                    let random_value = values.choose(&mut rng).unwrap();
                    let random_suit = suits.choose(&mut rng).unwrap();
                    cards.push(Card::new(*random_value, *random_suit));
                }
                cards
            }
            _ => {
                // Standard 52-card deck
                let mut cards = Vec::new();
                for v in &Value::values() {
                    for s in &Suit::suits() {
                        cards.push(Card::new(*v, *s));
                    }
                }
                cards
            }
        }
    }

    /// Check if this deck disables interest (Green Deck)
    pub fn disables_interest(&self) -> bool {
        matches!(self, DeckType::GreenDeck)
    }

    /// Check if this deck uses special scoring (Plasma Deck)
    pub fn uses_plasma_scoring(&self) -> bool {
        matches!(self, DeckType::PlasmaDeck)
    }

    /// Check if this deck allows spectrals in shop (Ghost Deck)
    pub fn allows_spectrals_in_shop(&self) -> bool {
        matches!(self, DeckType::GhostDeck)
    }

    /// Check if this deck grants Double Tag after boss blinds (Anaglyph Deck)
    pub fn grants_double_tag(&self) -> bool {
        matches!(self, DeckType::AnaglyPhDeck)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_standard_decks() {
        let decks = DeckType::all_standard_decks();
        assert_eq!(decks.len(), 15, "Should have 15 standard decks");
    }

    #[test]
    fn test_deck_names() {
        assert_eq!(DeckType::RedDeck.name(), "Red Deck");
        assert_eq!(DeckType::BlueDeck.name(), "Blue Deck");
        assert_eq!(DeckType::YellowDeck.name(), "Yellow Deck");
        assert_eq!(DeckType::GreenDeck.name(), "Green Deck");
        assert_eq!(DeckType::BlackDeck.name(), "Black Deck");
        assert_eq!(DeckType::MagicDeck.name(), "Magic Deck");
        assert_eq!(DeckType::NebulaDeck.name(), "Nebula Deck");
        assert_eq!(DeckType::GhostDeck.name(), "Ghost Deck");
        assert_eq!(DeckType::AbandonedDeck.name(), "Abandoned Deck");
        assert_eq!(DeckType::CheckeredDeck.name(), "Checkered Deck");
        assert_eq!(DeckType::ZodiacDeck.name(), "Zodiac Deck");
        assert_eq!(DeckType::PaintedDeck.name(), "Painted Deck");
        assert_eq!(DeckType::AnaglyPhDeck.name(), "Anaglyph Deck");
        assert_eq!(DeckType::PlasmaDeck.name(), "Plasma Deck");
        assert_eq!(DeckType::ErraticDeck.name(), "Erratic Deck");
    }

    #[test]
    fn test_red_deck_config() {
        let mut config = Config::default();
        let base_discards = config.discards;
        DeckType::RedDeck.apply_to_config(&mut config);
        assert_eq!(config.discards, base_discards + 1, "Red Deck should add 1 discard");
    }

    #[test]
    fn test_blue_deck_config() {
        let mut config = Config::default();
        let base_plays = config.plays;
        DeckType::BlueDeck.apply_to_config(&mut config);
        assert_eq!(config.plays, base_plays + 1, "Blue Deck should add 1 hand");
    }

    #[test]
    fn test_yellow_deck_config() {
        let mut config = Config::default();
        let base_money = config.money_start;
        DeckType::YellowDeck.apply_to_config(&mut config);
        assert_eq!(config.money_start, base_money + 10, "Yellow Deck should add $10 starting money");
    }

    #[test]
    fn test_green_deck_config() {
        let mut config = Config::default();
        DeckType::GreenDeck.apply_to_config(&mut config);
        assert_eq!(config.interest_rate, 0.0, "Green Deck should disable interest");
        assert_eq!(config.interest_max, 0, "Green Deck should disable interest");
        assert_eq!(config.money_per_hand, 2, "Green Deck should pay $2 per remaining hand");
        assert!(DeckType::GreenDeck.disables_interest());
    }

    #[test]
    fn test_black_deck_config() {
        let mut config = Config::default();
        let base_joker_slots = config.joker_slots;
        let base_plays = config.plays;
        DeckType::BlackDeck.apply_to_config(&mut config);
        assert_eq!(config.joker_slots, base_joker_slots + 1, "Black Deck should add 1 joker slot");
        assert_eq!(config.plays, base_plays - 1, "Black Deck should remove 1 hand");
    }

    #[test]
    fn test_painted_deck_config() {
        let mut config = Config::default();
        let base_available = config.available;
        let base_joker_slots = config.joker_slots;
        DeckType::PaintedDeck.apply_to_config(&mut config);
        assert_eq!(config.available, base_available + 2, "Painted Deck should add 2 hand size");
        assert_eq!(config.joker_slots, base_joker_slots - 1, "Painted Deck should remove 1 joker slot");
    }

    #[test]
    fn test_nebula_deck_config() {
        let mut config = Config::default();
        let base_consumable_slots = config.consumable_slots;
        DeckType::NebulaDeck.apply_to_config(&mut config);
        assert_eq!(config.consumable_slots, base_consumable_slots - 1, "Nebula Deck should remove 1 consumable slot");
    }

    #[test]
    fn test_magic_deck_starting_items() {
        let vouchers = DeckType::MagicDeck.starting_vouchers();
        assert_eq!(vouchers.len(), 1, "Magic Deck should start with 1 voucher");
        assert_eq!(vouchers[0], Vouchers::Crystal); // Crystal Ball

        let consumables = DeckType::MagicDeck.starting_consumables();
        assert_eq!(consumables.len(), 2, "Magic Deck should start with 2 consumables");
        // Both should be The Fool tarots
    }

    #[test]
    fn test_nebula_deck_starting_items() {
        let vouchers = DeckType::NebulaDeck.starting_vouchers();
        assert_eq!(vouchers.len(), 1, "Nebula Deck should start with 1 voucher");
        assert_eq!(vouchers[0], Vouchers::Telescope);
    }

    #[test]
    fn test_ghost_deck_starting_items() {
        let consumables = DeckType::GhostDeck.starting_consumables();
        assert_eq!(consumables.len(), 1, "Ghost Deck should start with 1 consumable");
        // Should be Hex spectral
        assert!(DeckType::GhostDeck.allows_spectrals_in_shop());
    }

    #[test]
    fn test_zodiac_deck_starting_items() {
        let vouchers = DeckType::ZodiacDeck.starting_vouchers();
        assert_eq!(vouchers.len(), 3, "Zodiac Deck should start with 3 vouchers");
        assert!(vouchers.contains(&Vouchers::Tarot)); // Tarot Merchant
        assert!(vouchers.contains(&Vouchers::Planet)); // Planet Merchant
        assert!(vouchers.contains(&Vouchers::Overstock));
    }

    #[test]
    fn test_abandoned_deck_generation() {
        let cards = DeckType::AbandonedDeck.generate_cards();
        assert_eq!(cards.len(), 40, "Abandoned Deck should have 40 cards (no face cards)");

        // Check no face cards
        for card in &cards {
            assert!(
                card.value != Value::Jack
                && card.value != Value::Queen
                && card.value != Value::King,
                "Abandoned Deck should not contain face cards"
            );
        }

        // Should have all suits
        let has_spades = cards.iter().any(|c| c.suit == Suit::Spade);
        let has_hearts = cards.iter().any(|c| c.suit == Suit::Heart);
        let has_clubs = cards.iter().any(|c| c.suit == Suit::Club);
        let has_diamonds = cards.iter().any(|c| c.suit == Suit::Diamond);
        assert!(has_spades && has_hearts && has_clubs && has_diamonds,
            "Abandoned Deck should contain all 4 suits");
    }

    #[test]
    fn test_checkered_deck_generation() {
        let cards = DeckType::CheckeredDeck.generate_cards();
        assert_eq!(cards.len(), 52, "Checkered Deck should have 52 cards");

        // Check only Spades and Hearts
        for card in &cards {
            assert!(
                card.suit == Suit::Spade || card.suit == Suit::Heart,
                "Checkered Deck should only contain Spades and Hearts"
            );
        }

        // Count each suit
        let spades = cards.iter().filter(|c| c.suit == Suit::Spade).count();
        let hearts = cards.iter().filter(|c| c.suit == Suit::Heart).count();
        assert_eq!(spades, 26, "Checkered Deck should have 26 Spades");
        assert_eq!(hearts, 26, "Checkered Deck should have 26 Hearts");
    }

    #[test]
    fn test_erratic_deck_generation() {
        let cards = DeckType::ErraticDeck.generate_cards();
        assert_eq!(cards.len(), 52, "Erratic Deck should have 52 cards");

        // Each card should be random, but we can't predict the exact distribution
        // Just verify we have 52 valid cards
        for card in &cards {
            assert!(Value::values().contains(&card.value));
            assert!(Suit::suits().contains(&card.suit));
        }
    }

    #[test]
    fn test_standard_deck_generation() {
        // Test that non-special decks generate standard 52-card decks
        let decks_to_test = vec![
            DeckType::RedDeck,
            DeckType::BlueDeck,
            DeckType::YellowDeck,
            DeckType::GreenDeck,
            DeckType::BlackDeck,
            DeckType::MagicDeck,
            DeckType::NebulaDeck,
            DeckType::GhostDeck,
            DeckType::ZodiacDeck,
            DeckType::PaintedDeck,
            DeckType::AnaglyPhDeck,
            DeckType::PlasmaDeck,
        ];

        for deck_type in decks_to_test {
            let cards = deck_type.generate_cards();
            assert_eq!(cards.len(), 52, "{} should generate 52 cards", deck_type.name());

            // Verify all standard cards present (13 ranks × 4 suits)
            for value in &Value::values() {
                for suit in &Suit::suits() {
                    let found = cards.iter().any(|c| c.value == *value && c.suit == *suit);
                    assert!(found, "{} should contain {:?} of {:?}",
                        deck_type.name(), value, suit);
                }
            }
        }
    }

    #[test]
    fn test_plasma_deck_special_scoring() {
        assert!(DeckType::PlasmaDeck.uses_plasma_scoring());
        assert!(!DeckType::RedDeck.uses_plasma_scoring());
    }

    #[test]
    fn test_anaglyph_deck_double_tag() {
        assert!(DeckType::AnaglyPhDeck.grants_double_tag());
        assert!(!DeckType::RedDeck.grants_double_tag());
    }

    // ==================== Integration Tests with Game ====================

    #[test]
    fn test_game_with_red_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::RedDeck);
        let game = Game::new(config);

        // Red Deck should have +1 discard (base 4 + 1 = 5)
        assert_eq!(game.discards, 5);
        assert_eq!(game.deck.len(), 52);
    }

    #[test]
    fn test_game_with_blue_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::BlueDeck);
        let game = Game::new(config);

        // Blue Deck should have +1 hand (base 4 + 1 = 5)
        assert_eq!(game.plays, 5);
        assert_eq!(game.deck.len(), 52);
    }

    #[test]
    fn test_game_with_yellow_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::YellowDeck);
        let game = Game::new(config);

        // Yellow Deck should have +$10 starting money (base $4 + $10 = $14)
        assert_eq!(game.money, 14);
        assert_eq!(game.deck.len(), 52);
    }

    #[test]
    fn test_game_with_abandoned_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::AbandonedDeck);
        let game = Game::new(config);

        // Abandoned Deck should have 40 cards (no face cards)
        assert_eq!(game.deck.len(), 40);

        // Verify no face cards
        for card in game.deck.cards() {
            assert!(
                card.value != crate::card::Value::Jack
                && card.value != crate::card::Value::Queen
                && card.value != crate::card::Value::King
            );
        }
    }

    #[test]
    fn test_game_with_checkered_deck() {
        use crate::card::Suit;
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::CheckeredDeck);
        let game = Game::new(config);

        // Checkered Deck should have 52 cards (26 Spades + 26 Hearts)
        assert_eq!(game.deck.len(), 52);

        // Verify only Spades and Hearts
        for card in game.deck.cards() {
            assert!(card.suit == Suit::Spade || card.suit == Suit::Heart);
        }
    }

    #[test]
    fn test_game_with_magic_deck_starting_items() {
        use crate::config::Config;
        use crate::consumable::Consumables;
        use crate::game::Game;
        use crate::tarot::Tarots;

        let config = Config::with_deck(DeckType::MagicDeck);
        let game = Game::new(config);

        // Magic Deck should start with Crystal Ball voucher
        assert_eq!(game.vouchers.len(), 1);
        assert_eq!(game.vouchers[0], crate::voucher::Vouchers::Crystal);

        // Should start with 2 The Fool tarots
        assert_eq!(game.consumables.len(), 2);
        for consumable in &game.consumables {
            if let Consumables::Tarot(tarot) = consumable {
                assert_eq!(*tarot, Tarots::TheFool);
            } else {
                panic!("Expected Tarot consumable");
            }
        }
    }

    #[test]
    fn test_game_with_zodiac_deck_starting_vouchers() {
        use crate::config::Config;
        use crate::game::Game;
        use crate::voucher::Vouchers;

        let config = Config::with_deck(DeckType::ZodiacDeck);
        let game = Game::new(config);

        // Zodiac Deck should start with 3 vouchers
        assert_eq!(game.vouchers.len(), 3);
        assert!(game.vouchers.contains(&Vouchers::Tarot));
        assert!(game.vouchers.contains(&Vouchers::Planet));
        assert!(game.vouchers.contains(&Vouchers::Overstock));
    }

    #[test]
    fn test_game_with_painted_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::PaintedDeck);
        let game = Game::new(config);

        // Painted Deck should have +2 hand size (base 8 + 2 = 10)
        assert_eq!(game.hand_size, 10);
        // Should have -1 joker slot (base 5 - 1 = 4)
        assert_eq!(game.config.joker_slots, 4);
    }

    #[test]
    fn test_game_with_black_deck() {
        use crate::config::Config;
        use crate::game::Game;

        let config = Config::with_deck(DeckType::BlackDeck);
        let game = Game::new(config);

        // Black Deck should have +1 joker slot (base 5 + 1 = 6)
        assert_eq!(game.config.joker_slots, 6);
        // Should have -1 hand (base 4 - 1 = 3)
        assert_eq!(game.plays, 3);
    }

    #[test]
    fn test_default_game_uses_standard_deck() {
        use crate::game::Game;

        let game = Game::default();

        // Default game should have standard 52-card deck
        assert_eq!(game.deck.len(), 52);
        // Should have no starting vouchers/consumables
        assert_eq!(game.vouchers.len(), 0);
        assert_eq!(game.consumables.len(), 0);
        assert_eq!(game.jokers.len(), 0);
    }
}
