use crate::card::Card;
use pyo3::pyclass;
use strum::{EnumIter, IntoEnumIterator};

/// The 20 Boss Blind modifiers that add constraints and challenges
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum BossModifier {
    // Category A: Simple Constraints (6)
    TheWall,    // ×2.5 score requirement instead of ×2
    TheManacle, // -1 hand size for this blind
    TheWater,   // Start with 0 discards
    TheNeedle,  // Can only play 1 hand this blind
    TheArm,     // Decrease level of played hand by 1 after each play
    TheTooth,   // Lose $1 per card played

    // Category B: Card Debuffing (6)
    TheClub,   // All Clubs are debuffed
    TheGoad,   // All Spades are debuffed
    TheWindow, // All Diamonds are debuffed
    TheHead,   // All Hearts are debuffed
    ThePlant,  // All face cards (J/Q/K) are debuffed
    TheFlint,  // Chips and mult are halved

    // Category C: Hand/Card Restrictions (4)
    TheEye,     // No hand type can be repeated
    TheMouth,   // Only 1 specific hand type can be played
    TheSerpent, // First hand played always scores 0
    TheHook,    // Discard 2 random cards after each hand played

    // Category D: Complex Mechanics (4)
    TheOx,     // Leftmost card is played face-down (no rank/suit)
    TheHouse,  // First hand is dealt with only 1 card
    TheWheel,  // Each card has 1/7 chance to be face-down
    ThePillar, // Cards are selected randomly for play
}

impl BossModifier {
    /// Get the name of this boss modifier
    pub fn name(&self) -> &str {
        match self {
            Self::TheWall => "The Wall",
            Self::TheManacle => "The Manacle",
            Self::TheWater => "The Water",
            Self::TheNeedle => "The Needle",
            Self::TheArm => "The Arm",
            Self::TheTooth => "The Tooth",
            Self::TheClub => "The Club",
            Self::TheGoad => "The Goad",
            Self::TheWindow => "The Window",
            Self::TheHead => "The Head",
            Self::ThePlant => "The Plant",
            Self::TheFlint => "The Flint",
            Self::TheEye => "The Eye",
            Self::TheMouth => "The Mouth",
            Self::TheSerpent => "The Serpent",
            Self::TheHook => "The Hook",
            Self::TheOx => "The Ox",
            Self::TheHouse => "The House",
            Self::TheWheel => "The Wheel",
            Self::ThePillar => "The Pillar",
        }
    }

    /// Get the description of this boss modifier
    pub fn description(&self) -> &str {
        match self {
            Self::TheWall => "Boss blind requires ×2.5 score instead of ×2",
            Self::TheManacle => "-1 hand size for this blind",
            Self::TheWater => "Start with 0 discards",
            Self::TheNeedle => "Only 1 hand can be played",
            Self::TheArm => "Played hand's level decreases by 1",
            Self::TheTooth => "Lose $1 per card played",
            Self::TheClub => "All Clubs are debuffed",
            Self::TheGoad => "All Spades are debuffed",
            Self::TheWindow => "All Diamonds are debuffed",
            Self::TheHead => "All Hearts are debuffed",
            Self::ThePlant => "All face cards (J/Q/K) are debuffed",
            Self::TheFlint => "Chips and mult are halved",
            Self::TheEye => "No hand type can be repeated",
            Self::TheMouth => "Only 1 specific hand type allowed",
            Self::TheSerpent => "First hand always scores 0",
            Self::TheHook => "Discard 2 random cards after each hand",
            Self::TheOx => "Leftmost card is face-down",
            Self::TheHouse => "First hand dealt with only 1 card",
            Self::TheWheel => "1/7 chance for cards to be face-down",
            Self::ThePillar => "Cards selected randomly",
        }
    }

    /// Returns the score multiplier for this boss modifier
    /// Most bosses use 2.0x, The Wall uses 2.5x
    pub fn score_multiplier(&self) -> f64 {
        match self {
            Self::TheWall => 2.5,
            _ => 2.0,
        }
    }

    /// Returns hand size modifier for this boss
    /// The Manacle reduces by 1
    pub fn hand_size_modifier(&self) -> i32 {
        match self {
            Self::TheManacle => -1,
            _ => 0,
        }
    }

    /// Returns starting discards modifier
    /// The Water sets discards to 0
    pub fn discard_modifier(&self) -> i32 {
        match self {
            Self::TheWater => i32::MIN, // Sets to 0 (subtract all)
            _ => 0,
        }
    }

    /// Returns max hands allowed for this blind (None = unlimited)
    /// The Needle allows only 1 hand
    pub fn max_hands(&self) -> Option<usize> {
        match self {
            Self::TheNeedle => Some(1),
            _ => None,
        }
    }

    /// Check if a card is debuffed by this modifier
    /// Debuffed cards don't contribute to scoring
    pub fn is_card_debuffed(&self, card: &Card) -> bool {
        use crate::card::{Suit, Value};

        match self {
            Self::TheClub => card.suit == Suit::Club,
            Self::TheGoad => card.suit == Suit::Spade,
            Self::TheWindow => card.suit == Suit::Diamond,
            Self::TheHead => card.suit == Suit::Heart,
            Self::ThePlant => matches!(card.value, Value::Jack | Value::Queen | Value::King),
            _ => false,
        }
    }

    /// Returns true if this modifier halves the final score
    pub fn halves_score(&self) -> bool {
        matches!(self, Self::TheFlint)
    }

    /// Returns true if this modifier decreases hand level after play
    pub fn decreases_hand_level(&self) -> bool {
        matches!(self, Self::TheArm)
    }

    /// Returns money cost per card played (0 = no cost)
    pub fn money_per_card(&self) -> usize {
        match self {
            Self::TheTooth => 1,
            _ => 0,
        }
    }

    /// Returns true if this modifier prevents hand type repeats
    pub fn prevents_repeats(&self) -> bool {
        matches!(self, Self::TheEye)
    }

    /// Returns true if this modifier makes first hand score 0
    pub fn first_hand_scores_zero(&self) -> bool {
        matches!(self, Self::TheSerpent)
    }

    /// Returns number of cards to discard after each hand (0 = none)
    pub fn cards_to_discard_after_play(&self) -> usize {
        match self {
            Self::TheHook => 2,
            _ => 0,
        }
    }

    /// Returns true if this modifier restricts to only one hand type
    pub fn restricts_to_one_hand_type(&self) -> bool {
        matches!(self, Self::TheMouth)
    }

    /// Returns true if leftmost card should be face-down (Category D)
    pub fn leftmost_face_down(&self) -> bool {
        matches!(self, Self::TheOx)
    }

    /// Returns true if first hand should be dealt with 1 card (Category D)
    pub fn first_hand_one_card(&self) -> bool {
        matches!(self, Self::TheHouse)
    }

    /// Returns probability (0.0-1.0) that each card is face-down (Category D)
    pub fn face_down_probability(&self) -> f64 {
        match self {
            Self::TheWheel => 1.0 / 7.0,
            _ => 0.0,
        }
    }

    /// Returns true if cards should be randomly selected for play (Category D)
    pub fn random_card_selection(&self) -> bool {
        matches!(self, Self::ThePillar)
    }

    /// Get all boss modifiers
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }

    /// Get a random boss modifier (using deterministic RNG from game state)
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        use rand::seq::SliceRandom;
        let all = Self::all();
        *all.choose(rng).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_all_modifiers_have_names() {
        for modifier in BossModifier::all() {
            assert!(!modifier.name().is_empty());
            assert!(!modifier.description().is_empty());
        }
    }

    #[test]
    fn test_the_wall_score_multiplier() {
        assert_eq!(BossModifier::TheWall.score_multiplier(), 2.5);
        assert_eq!(BossModifier::TheNeedle.score_multiplier(), 2.0);
    }

    #[test]
    fn test_the_manacle_hand_size() {
        assert_eq!(BossModifier::TheManacle.hand_size_modifier(), -1);
        assert_eq!(BossModifier::TheWall.hand_size_modifier(), 0);
    }

    #[test]
    fn test_the_water_discards() {
        assert_eq!(BossModifier::TheWater.discard_modifier(), i32::MIN);
        assert_eq!(BossModifier::TheWall.discard_modifier(), 0);
    }

    #[test]
    fn test_the_needle_max_hands() {
        assert_eq!(BossModifier::TheNeedle.max_hands(), Some(1));
        assert_eq!(BossModifier::TheWall.max_hands(), None);
    }

    #[test]
    fn test_suit_debuffs() {
        let club = Card::new(Value::Ace, Suit::Club);
        let spade = Card::new(Value::Ace, Suit::Spade);
        let diamond = Card::new(Value::Ace, Suit::Diamond);
        let heart = Card::new(Value::Ace, Suit::Heart);

        assert!(BossModifier::TheClub.is_card_debuffed(&club));
        assert!(!BossModifier::TheClub.is_card_debuffed(&spade));

        assert!(BossModifier::TheGoad.is_card_debuffed(&spade));
        assert!(!BossModifier::TheGoad.is_card_debuffed(&club));

        assert!(BossModifier::TheWindow.is_card_debuffed(&diamond));
        assert!(!BossModifier::TheWindow.is_card_debuffed(&club));

        assert!(BossModifier::TheHead.is_card_debuffed(&heart));
        assert!(!BossModifier::TheHead.is_card_debuffed(&club));
    }

    #[test]
    fn test_the_plant_face_card_debuff() {
        let jack = Card::new(Value::Jack, Suit::Heart);
        let queen = Card::new(Value::Queen, Suit::Diamond);
        let king = Card::new(Value::King, Suit::Club);
        let ace = Card::new(Value::Ace, Suit::Spade);
        let ten = Card::new(Value::Ten, Suit::Heart);

        assert!(BossModifier::ThePlant.is_card_debuffed(&jack));
        assert!(BossModifier::ThePlant.is_card_debuffed(&queen));
        assert!(BossModifier::ThePlant.is_card_debuffed(&king));
        assert!(!BossModifier::ThePlant.is_card_debuffed(&ace));
        assert!(!BossModifier::ThePlant.is_card_debuffed(&ten));
    }

    #[test]
    fn test_the_flint_halves_score() {
        assert!(BossModifier::TheFlint.halves_score());
        assert!(!BossModifier::TheWall.halves_score());
    }

    #[test]
    fn test_the_arm_decreases_level() {
        assert!(BossModifier::TheArm.decreases_hand_level());
        assert!(!BossModifier::TheWall.decreases_hand_level());
    }

    #[test]
    fn test_the_tooth_money_cost() {
        assert_eq!(BossModifier::TheTooth.money_per_card(), 1);
        assert_eq!(BossModifier::TheWall.money_per_card(), 0);
    }

    #[test]
    fn test_the_eye_prevents_repeats() {
        assert!(BossModifier::TheEye.prevents_repeats());
        assert!(!BossModifier::TheWall.prevents_repeats());
    }

    #[test]
    fn test_the_serpent_first_hand_zero() {
        assert!(BossModifier::TheSerpent.first_hand_scores_zero());
        assert!(!BossModifier::TheWall.first_hand_scores_zero());
    }

    #[test]
    fn test_the_hook_discard_count() {
        assert_eq!(BossModifier::TheHook.cards_to_discard_after_play(), 2);
        assert_eq!(BossModifier::TheWall.cards_to_discard_after_play(), 0);
    }

    #[test]
    fn test_random_modifier_generation() {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

        let modifier1 = BossModifier::random(&mut rng);
        let modifier2 = BossModifier::random(&mut rng);

        // Just verify they're valid modifiers
        assert!(BossModifier::all().contains(&modifier1));
        assert!(BossModifier::all().contains(&modifier2));
    }

    #[test]
    fn test_all_20_modifiers_exist() {
        let all = BossModifier::all();
        assert_eq!(all.len(), 20, "Should have exactly 20 boss modifiers");
    }
}
