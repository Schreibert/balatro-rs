#[cfg(feature = "colored")]
use colored::Colorize;
use pyo3::pyclass;
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

// Useful balatro docs: https://balatrogame.fandom.com/wiki/Card_Ranks

/// Card rank or value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Value {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

/// Constant of all the values.
/// This is what `Value::values()` returns
const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    pub const fn values() -> [Self; 13] {
        VALUES
    }

    /// Raise the rank by 1 (for Strength tarot)
    /// Returns None if already at Ace (can't go higher)
    pub fn raise_rank(&self) -> Option<Self> {
        match self {
            Value::Two => Some(Value::Three),
            Value::Three => Some(Value::Four),
            Value::Four => Some(Value::Five),
            Value::Five => Some(Value::Six),
            Value::Six => Some(Value::Seven),
            Value::Seven => Some(Value::Eight),
            Value::Eight => Some(Value::Nine),
            Value::Nine => Some(Value::Ten),
            Value::Ten => Some(Value::Jack),
            Value::Jack => Some(Value::Queen),
            Value::Queen => Some(Value::King),
            Value::King => Some(Value::Ace),
            Value::Ace => None, // Can't go higher
        }
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
}

/// Enum for the four different suits.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Spade = 0,
    Club = 1,
    Heart = 2,
    Diamond = 3,
}

/// All of the `Suit`'s. This is what `Suit::suits()` returns.
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

impl Suit {
    pub const fn suits() -> [Self; 4] {
        SUITS
    }
    pub fn unicode(&self) -> &str {
        match self {
            Self::Spade => "♤",
            Self::Club => "♧",
            Self::Heart => "♡",
            Self::Diamond => "♢",
        }
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Spade => 's',
            Suit::Club => 'c',
            Suit::Heart => 'h',
            Suit::Diamond => 'd',
        }
    }
}

/// Enum for card  enhancements
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

/// Enum for card  editions
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Enum for card seals
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}

// Each card gets a unique id. Not sure this is strictly
// necessary but it makes identifying otherwise identical cards
// possible (i.e. for trashing, reordering, etc)
static CARD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub id: usize,
    pub edition: Edition,
    pub enhancement: Option<Enhancement>,
    pub seal: Option<Seal>,
    pub is_face_down: bool, // For The Ox, The Wheel boss modifiers
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        let id = CARD_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self {
            value,
            suit,
            id: id,
            edition: Edition::Base,
            enhancement: None,
            seal: None,
            is_face_down: false, // Default to face-up
        }
    }

    pub fn is_face(&self) -> bool {
        match self.value {
            Value::Jack | Value::Queen | Value::King => true,
            _ => false,
        }
    }

    pub fn is_even(&self) -> bool {
        self.value != Value::Ace && !self.is_face() && self.value as u16 % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.value == Value::Ace || !self.is_face() && self.value as u16 % 2 != 0
    }

    pub fn chips(&self) -> usize {
        let base_chips = match self.value {
            Value::Two => 1,
            Value::Three => 2,
            Value::Four => 3,
            Value::Five => 4,
            Value::Six => 5,
            Value::Seven => 6,
            Value::Eight => 7,
            Value::Nine => 8,
            Value::Ten => 9,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        };

        let mut chips = base_chips;

        // Add enhancement bonuses
        if let Some(enhancement) = self.enhancement {
            chips += match enhancement {
                Enhancement::Bonus => 30,
                Enhancement::Stone => 50,
                _ => 0,
            };
        }

        // Add edition bonuses
        chips += match self.edition {
            Edition::Foil => 50,
            _ => 0,
        };

        chips
    }

    /// Get mult contribution from this card (from enhancements and editions)
    pub fn mult(&self) -> usize {
        let mut mult = 0;

        // Add enhancement mult
        if let Some(enhancement) = self.enhancement {
            mult += match enhancement {
                Enhancement::Mult => 4,
                _ => 0,
            };
        }

        // Add edition mult
        mult += match self.edition {
            Edition::Holographic => 10,
            _ => 0,
        };

        mult
    }

    /// Get mult multiplier from this card (Glass, Steel, Polychrome)
    pub fn mult_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;

        // Enhancement multipliers
        if let Some(enhancement) = self.enhancement {
            multiplier *= match enhancement {
                Enhancement::Glass => 2.0,
                Enhancement::Steel => 1.5,
                _ => 1.0,
            };
        }

        // Edition multipliers
        multiplier *= match self.edition {
            Edition::Polychrome => 1.5,
            _ => 1.0,
        };

        multiplier
    }

    /// Check if this card should be destroyed (Glass has 1/4 chance)
    pub fn should_destroy(&self) -> bool {
        if let Some(Enhancement::Glass) = self.enhancement {
            use rand::Rng;
            return rand::thread_rng().gen_range(0..4) == 0; // 1/4 chance
        }
        false
    }

    /// Get money earned from this card's seal when played
    pub fn seal_money_on_play(&self) -> usize {
        if let Some(seal) = self.seal {
            match seal {
                Seal::Gold => 3,
                _ => 0,
            }
        } else {
            0
        }
    }

    /// Check if card has retrigger seal (Red seal)
    pub fn has_retrigger(&self) -> bool {
        matches!(self.seal, Some(Seal::Red))
    }

    /// Set the enhancement on this card (for Tarot effects)
    pub fn set_enhancement(&mut self, enhancement: Enhancement) {
        self.enhancement = Some(enhancement);
    }

    /// Set the suit on this card (for Tarot effects)
    pub fn set_suit(&mut self, suit: Suit) {
        self.suit = suit;
    }

    /// Set the rank on this card (for Strength tarot, Death tarot)
    pub fn set_rank(&mut self, rank: Value) {
        self.value = rank;
    }

    /// Set the seal on this card (for Spectral effects)
    pub fn set_seal(&mut self, seal: Seal) {
        self.seal = Some(seal);
    }

    /// Set the edition on this card (for Spectral/Tarot effects)
    pub fn set_edition(&mut self, edition: Edition) {
        self.edition = edition;
    }

    /// Set whether this card is face-down (for The Ox, The Wheel boss modifiers)
    pub fn set_face_down(&mut self, face_down: bool) {
        self.is_face_down = face_down;
    }

    /// Check if this card is visible (face-up)
    pub fn is_visible(&self) -> bool {
        !self.is_face_down
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "colored")]
        let suit = match self.suit {
            Suit::Spade => self.suit.unicode().bold(),
            Suit::Club => self.suit.unicode().green().bold(),
            Suit::Heart => self.suit.unicode().red().bold(),
            Suit::Diamond => self.suit.unicode().blue().bold(),
        };
        #[cfg(not(feature = "colored"))]
        let suit = self.suit.unicode();
        write!(f, "Card({}{})", char::from(self.value), suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "colored")]
        let suit = match self.suit {
            Suit::Spade => self.suit.unicode().bold(),
            Suit::Club => self.suit.unicode().green().bold(),
            Suit::Heart => self.suit.unicode().red().bold(),
            Suit::Diamond => self.suit.unicode().blue().bold(),
        };
        #[cfg(not(feature = "colored"))]
        let suit = self.suit.unicode();
        write!(f, "{}{}", char::from(self.value), suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let c = Card::new(Value::King, Suit::Heart);
        assert_eq!(Value::King, c.value);
        assert_eq!(Suit::Heart, c.suit);
    }

    #[test]
    fn test_face() {
        let king = Card::new(Value::King, Suit::Heart);
        assert_eq!(king.is_face(), true);
        let two = Card::new(Value::Two, Suit::Diamond);
        assert_eq!(two.is_face(), false);
    }

    #[test]
    fn test_even_odd() {
        // ace is odd
        let ace = Card::new(Value::Ace, Suit::Spade);
        assert_eq!(ace.is_even(), false);
        assert_eq!(ace.is_odd(), true);

        // two is even
        let two = Card::new(Value::Two, Suit::Diamond);
        assert_eq!(two.is_even(), true);
        assert_eq!(two.is_odd(), false);

        // three is odd
        let three = Card::new(Value::Three, Suit::Heart);
        assert_eq!(three.is_even(), false);
        assert_eq!(three.is_odd(), true);

        // ten is even
        let ten = Card::new(Value::Ten, Suit::Heart);
        assert_eq!(ten.is_even(), true);
        assert_eq!(ten.is_odd(), false);

        //king is neither odd nor even
        let king = Card::new(Value::King, Suit::Club);
        assert_eq!(king.is_even(), false);
        assert_eq!(king.is_odd(), false);
    }

    #[test]
    fn test_enhancement_bonus_chips() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Base chips: 4
        assert_eq!(card.chips(), 4);

        // Bonus enhancement: +30 chips
        card.enhancement = Some(Enhancement::Bonus);
        assert_eq!(card.chips(), 34);

        // Stone enhancement: +50 chips
        card.enhancement = Some(Enhancement::Stone);
        assert_eq!(card.chips(), 54);
    }

    #[test]
    fn test_enhancement_mult() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Base mult: 0
        assert_eq!(card.mult(), 0);

        // Mult enhancement: +4 mult
        card.enhancement = Some(Enhancement::Mult);
        assert_eq!(card.mult(), 4);
    }

    #[test]
    fn test_edition_bonus_chips() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Base chips: 4
        assert_eq!(card.chips(), 4);

        // Foil edition: +50 chips
        card.edition = Edition::Foil;
        assert_eq!(card.chips(), 54);
    }

    #[test]
    fn test_edition_mult() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Base mult: 0
        assert_eq!(card.mult(), 0);

        // Holographic edition: +10 mult
        card.edition = Edition::Holographic;
        assert_eq!(card.mult(), 10);
    }

    #[test]
    fn test_mult_multiplier_glass() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Base multiplier: 1.0
        assert_eq!(card.mult_multiplier(), 1.0);

        // Glass enhancement: ×2 mult
        card.enhancement = Some(Enhancement::Glass);
        assert_eq!(card.mult_multiplier(), 2.0);
    }

    #[test]
    fn test_mult_multiplier_steel() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        card.enhancement = Some(Enhancement::Steel);
        assert_eq!(card.mult_multiplier(), 1.5);
    }

    #[test]
    fn test_mult_multiplier_polychrome() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        card.edition = Edition::Polychrome;
        assert_eq!(card.mult_multiplier(), 1.5);
    }

    #[test]
    fn test_mult_multiplier_combined() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        // Glass (×2) + Polychrome (×1.5) = ×3
        card.enhancement = Some(Enhancement::Glass);
        card.edition = Edition::Polychrome;
        assert_eq!(card.mult_multiplier(), 3.0);
    }

    #[test]
    fn test_seal_money_gold() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        assert_eq!(card.seal_money_on_play(), 0);

        card.seal = Some(Seal::Gold);
        assert_eq!(card.seal_money_on_play(), 3);
    }

    #[test]
    fn test_retrigger_seal() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        assert_eq!(card.has_retrigger(), false);

        card.seal = Some(Seal::Red);
        assert_eq!(card.has_retrigger(), true);

        card.seal = Some(Seal::Gold);
        assert_eq!(card.has_retrigger(), false);
    }

    #[test]
    fn test_combined_bonus_mult_enhancement() {
        let mut card = Card::new(Value::Five, Suit::Heart);
        card.enhancement = Some(Enhancement::Bonus);
        card.edition = Edition::Holographic;

        // Should have both bonus chips and bonus mult
        assert_eq!(card.chips(), 34); // 4 + 30
        assert_eq!(card.mult(), 10); // 0 + 10
    }

    #[test]
    fn test_face_down_default() {
        let card = Card::new(Value::King, Suit::Heart);
        assert_eq!(card.is_face_down, false);
        assert_eq!(card.is_visible(), true);
    }

    #[test]
    fn test_set_face_down() {
        let mut card = Card::new(Value::King, Suit::Heart);
        assert_eq!(card.is_visible(), true);

        card.set_face_down(true);
        assert_eq!(card.is_face_down, true);
        assert_eq!(card.is_visible(), false);

        card.set_face_down(false);
        assert_eq!(card.is_face_down, false);
        assert_eq!(card.is_visible(), true);
    }

    #[test]
    fn test_face_down_preserves_properties() {
        let mut card = Card::new(Value::Ace, Suit::Spade);
        card.enhancement = Some(Enhancement::Mult);
        card.edition = Edition::Foil;

        // Make face-down
        card.set_face_down(true);

        // Properties should still exist even when face-down
        assert_eq!(card.value, Value::Ace);
        assert_eq!(card.suit, Suit::Spade);
        assert_eq!(card.enhancement, Some(Enhancement::Mult));
        assert_eq!(card.edition, Edition::Foil);
    }
}
