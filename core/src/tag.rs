use crate::card::Card;
use crate::joker::Jokers;
use crate::planet::Planets;
use crate::spectral::Spectrals;
use crate::tarot::Tarots;
use pyo3::prelude::*;
use rand::seq::SliceRandom;

/// Tag types that can be obtained by skipping blinds or from special effects
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    // Ante 1 Tags (15 total)
    Uncommon,      // Shop has a free Uncommon Joker
    Rare,          // Shop has a free Rare Joker
    Foil,          // Next base edition shop Joker becomes Foil (+50 Chips) and free
    Holographic,   // Next base edition shop Joker becomes Holographic (+10 Mult) and free
    Polychrome,    // Next base edition shop Joker becomes Polychrome (X1.5 Mult) and free
    Investment,    // Gain $25 after defeating the next Boss Blind
    Voucher,       // Adds a Voucher to next shop
    Boss,          // Re-rolls the next Boss Blind
    Charm,         // Immediately open a free Mega Arcana Pack (choose 2 of 5)
    Coupon,        // Initial jokers, consumables, and packs are $0 in next shop
    Double,        // Gives a copy of the next Tag selected (excluding Double Tags)
    Juggle,        // +3 Hand Size for the next round only
    D6,            // Rerolls in next shop start at $0 (then +$1 each)
    Economy,       // Doubles your money (max of $40)
    Speed,         // $5 per skipped Blind this run

    // Ante 2+ Tags (9 additional)
    Negative,      // Next base edition shop Joker becomes Negative (+1 joker slot) and free
    Standard,      // Immediately open a free Mega Standard Pack (choose 2 of 5)
    Meteor,        // Immediately open a free Mega Celestial Pack (choose 2 of 5)
    Buffoon,       // Immediately open a free Mega Buffoon Pack (choose 2 of 4)
    Handy,         // $1 per played hand this run
    Garbage,       // $1 per unused discard this run
    Ethereal,      // Immediately open a free Spectral Pack (choose 1 of 2)
    TopUp,         // Creates up to 2 Common Jokers
    Orbital,       // Upgrade random poker hand by 3 levels
}

/// When a tag's effect triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagTrigger {
    Immediate,        // Activates as soon as obtained
    OnShopEnter,      // Activates when entering the next shop
    OnRoundStart,     // Activates at the start of next blind
    OnBossDefeated,   // Activates after defeating next boss
    OnTagObtained,    // Activates when next tag is obtained
    OnBossEncounter,  // Activates before facing next boss
}

impl Tag {
    /// All tags in the game
    pub const ALL: [Tag; 24] = [
        Tag::Uncommon,
        Tag::Rare,
        Tag::Foil,
        Tag::Holographic,
        Tag::Polychrome,
        Tag::Investment,
        Tag::Voucher,
        Tag::Boss,
        Tag::Charm,
        Tag::Coupon,
        Tag::Double,
        Tag::Juggle,
        Tag::D6,
        Tag::Economy,
        Tag::Speed,
        Tag::Negative,
        Tag::Standard,
        Tag::Meteor,
        Tag::Buffoon,
        Tag::Handy,
        Tag::Garbage,
        Tag::Ethereal,
        Tag::TopUp,
        Tag::Orbital,
    ];

    /// Returns the trigger timing for this tag
    pub fn trigger_type(&self) -> TagTrigger {
        match self {
            Tag::Charm
            | Tag::Buffoon
            | Tag::Meteor
            | Tag::Ethereal
            | Tag::Standard
            | Tag::Economy
            | Tag::Speed
            | Tag::Handy
            | Tag::Garbage
            | Tag::Orbital
            | Tag::TopUp => TagTrigger::Immediate,

            Tag::Uncommon
            | Tag::Rare
            | Tag::Foil
            | Tag::Holographic
            | Tag::Polychrome
            | Tag::Negative
            | Tag::Voucher
            | Tag::Coupon
            | Tag::D6 => TagTrigger::OnShopEnter,

            Tag::Juggle => TagTrigger::OnRoundStart,
            Tag::Investment => TagTrigger::OnBossDefeated,
            Tag::Double => TagTrigger::OnTagObtained,
            Tag::Boss => TagTrigger::OnBossEncounter,
        }
    }

    /// Returns true if this tag is available starting from Ante 1
    pub fn available_ante_1(&self) -> bool {
        !matches!(
            self,
            Tag::Negative
                | Tag::Standard
                | Tag::Meteor
                | Tag::Buffoon
                | Tag::Handy
                | Tag::Garbage
                | Tag::Ethereal
                | Tag::TopUp
                | Tag::Orbital
        )
    }

    /// Returns true if this tag is available at the given ante
    pub fn is_available_at_ante(&self, ante: usize) -> bool {
        if ante >= 2 {
            true
        } else {
            self.available_ante_1()
        }
    }

    /// Returns the name of this tag
    pub fn name(&self) -> &'static str {
        match self {
            Tag::Uncommon => "Uncommon Tag",
            Tag::Rare => "Rare Tag",
            Tag::Foil => "Foil Tag",
            Tag::Holographic => "Holographic Tag",
            Tag::Polychrome => "Polychrome Tag",
            Tag::Investment => "Investment Tag",
            Tag::Voucher => "Voucher Tag",
            Tag::Boss => "Boss Tag",
            Tag::Charm => "Charm Tag",
            Tag::Coupon => "Coupon Tag",
            Tag::Double => "Double Tag",
            Tag::Juggle => "Juggle Tag",
            Tag::D6 => "D6 Tag",
            Tag::Economy => "Economy Tag",
            Tag::Speed => "Speed Tag",
            Tag::Negative => "Negative Tag",
            Tag::Standard => "Standard Tag",
            Tag::Meteor => "Meteor Tag",
            Tag::Buffoon => "Buffoon Tag",
            Tag::Handy => "Handy Tag",
            Tag::Garbage => "Garbage Tag",
            Tag::Ethereal => "Ethereal Tag",
            Tag::TopUp => "Top-up Tag",
            Tag::Orbital => "Orbital Tag",
        }
    }

    /// Returns a description of what this tag does
    pub fn description(&self) -> &'static str {
        match self {
            Tag::Uncommon => "Shop has a free Uncommon Joker",
            Tag::Rare => "Shop has a free Rare Joker",
            Tag::Foil => "Next base edition shop Joker becomes Foil (+50 Chips) and free",
            Tag::Holographic => "Next base edition shop Joker becomes Holographic (+10 Mult) and free",
            Tag::Polychrome => "Next base edition shop Joker becomes Polychrome (X1.5 Mult) and free",
            Tag::Investment => "Gain $25 after defeating the next Boss Blind",
            Tag::Voucher => "Adds a Voucher to next shop",
            Tag::Boss => "Re-rolls the next Boss Blind",
            Tag::Charm => "Open a free Mega Arcana Pack (choose 2 of 5 Tarot cards)",
            Tag::Coupon => "Initial jokers, consumables, and packs are $0 in next shop",
            Tag::Double => "Gives a copy of the next Tag selected (excluding Double Tags)",
            Tag::Juggle => "+3 Hand Size for the next round only",
            Tag::D6 => "Rerolls in next shop start at $0 (then +$1 each)",
            Tag::Economy => "Doubles your money (max of $40)",
            Tag::Speed => "$5 per skipped Blind this run",
            Tag::Negative => "Next base edition shop Joker becomes Negative (+1 joker slot) and free",
            Tag::Standard => "Open a free Mega Standard Pack (choose 2 of 5 Playing cards)",
            Tag::Meteor => "Open a free Mega Celestial Pack (choose 2 of 5 Planet cards)",
            Tag::Buffoon => "Open a free Mega Buffoon Pack (choose 2 of 4 Jokers)",
            Tag::Handy => "$1 per played hand this run",
            Tag::Garbage => "$1 per unused discard this run",
            Tag::Ethereal => "Open a free Spectral Pack (choose 1 of 2 Spectral cards)",
            Tag::TopUp => "Creates up to 2 Common Jokers",
            Tag::Orbital => "Upgrade random poker hand by 3 levels",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tags_count() {
        assert_eq!(Tag::ALL.len(), 24, "Should have exactly 24 tags");
    }

    #[test]
    fn test_ante_1_tags() {
        let ante_1_tags: Vec<Tag> = Tag::ALL
            .iter()
            .filter(|tag| tag.available_ante_1())
            .copied()
            .collect();
        assert_eq!(ante_1_tags.len(), 15, "Should have 15 Ante 1 tags");

        // Verify specific Ante 1 tags
        assert!(Tag::Uncommon.available_ante_1());
        assert!(Tag::Rare.available_ante_1());
        assert!(Tag::Foil.available_ante_1());
        assert!(Tag::Investment.available_ante_1());
        assert!(Tag::Economy.available_ante_1());
    }

    #[test]
    fn test_ante_2_plus_tags() {
        let ante_2_plus_only: Vec<Tag> = Tag::ALL
            .iter()
            .filter(|tag| !tag.available_ante_1())
            .copied()
            .collect();
        assert_eq!(
            ante_2_plus_only.len(),
            9,
            "Should have 9 Ante 2+ exclusive tags"
        );

        // Verify specific Ante 2+ tags
        assert!(!Tag::Negative.available_ante_1());
        assert!(!Tag::Standard.available_ante_1());
        assert!(!Tag::Meteor.available_ante_1());
        assert!(!Tag::Buffoon.available_ante_1());
        assert!(!Tag::Handy.available_ante_1());
        assert!(!Tag::Garbage.available_ante_1());
        assert!(!Tag::Ethereal.available_ante_1());
        assert!(!Tag::TopUp.available_ante_1());
        assert!(!Tag::Orbital.available_ante_1());
    }

    #[test]
    fn test_is_available_at_ante() {
        // Ante 1 should only have 15 tags available
        assert!(Tag::Uncommon.is_available_at_ante(1));
        assert!(Tag::Economy.is_available_at_ante(1));
        assert!(!Tag::Negative.is_available_at_ante(1));
        assert!(!Tag::Buffoon.is_available_at_ante(1));

        // Ante 2+ should have all tags available
        for tag in Tag::ALL {
            assert!(
                tag.is_available_at_ante(2),
                "{:?} should be available at Ante 2",
                tag
            );
            assert!(
                tag.is_available_at_ante(8),
                "{:?} should be available at Ante 8",
                tag
            );
        }
    }

    #[test]
    fn test_immediate_triggers() {
        let immediate_tags = vec![
            Tag::Charm,
            Tag::Buffoon,
            Tag::Meteor,
            Tag::Ethereal,
            Tag::Standard,
            Tag::Economy,
            Tag::Speed,
            Tag::Handy,
            Tag::Garbage,
            Tag::Orbital,
            Tag::TopUp,
        ];

        for tag in immediate_tags {
            assert_eq!(
                tag.trigger_type(),
                TagTrigger::Immediate,
                "{:?} should trigger immediately",
                tag
            );
        }
    }

    #[test]
    fn test_shop_triggers() {
        let shop_tags = vec![
            Tag::Uncommon,
            Tag::Rare,
            Tag::Foil,
            Tag::Holographic,
            Tag::Polychrome,
            Tag::Negative,
            Tag::Voucher,
            Tag::Coupon,
            Tag::D6,
        ];

        for tag in shop_tags {
            assert_eq!(
                tag.trigger_type(),
                TagTrigger::OnShopEnter,
                "{:?} should trigger on shop enter",
                tag
            );
        }
    }

    #[test]
    fn test_special_triggers() {
        assert_eq!(Tag::Juggle.trigger_type(), TagTrigger::OnRoundStart);
        assert_eq!(Tag::Investment.trigger_type(), TagTrigger::OnBossDefeated);
        assert_eq!(Tag::Double.trigger_type(), TagTrigger::OnTagObtained);
        assert_eq!(Tag::Boss.trigger_type(), TagTrigger::OnBossEncounter);
    }

    #[test]
    fn test_tag_names() {
        assert_eq!(Tag::Uncommon.name(), "Uncommon Tag");
        assert_eq!(Tag::Economy.name(), "Economy Tag");
        assert_eq!(Tag::TopUp.name(), "Top-up Tag");
        assert_eq!(Tag::D6.name(), "D6 Tag");

        // All tags should have a name
        for tag in Tag::ALL {
            assert!(!tag.name().is_empty(), "{:?} should have a name", tag);
        }
    }

    #[test]
    fn test_tag_descriptions() {
        assert!(Tag::Uncommon
            .description()
            .contains("free Uncommon Joker"));
        assert!(Tag::Investment.description().contains("$25"));
        assert!(Tag::Economy.description().contains("Doubles your money"));

        // All tags should have a description
        for tag in Tag::ALL {
            assert!(
                !tag.description().is_empty(),
                "{:?} should have a description",
                tag
            );
        }
    }

    #[test]
    fn test_tag_trigger_coverage() {
        // Ensure all tags have a trigger type defined
        for tag in Tag::ALL {
            let _ = tag.trigger_type(); // Should not panic
        }
    }
}

/// Tag packs are special "Mega" packs received from tag effects
/// They are larger than regular packs and allow choosing multiple items
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagPack {
    /// Mega Arcana Pack: 5 Tarots, choose 2
    MegaArcana(Vec<Tarots>),
    /// Mega Celestial Pack: 5 Planets, choose 2
    MegaCelestial(Vec<Planets>),
    /// Mega Buffoon Pack: 4 Jokers, choose 2
    MegaBuffoon(Vec<Jokers>),
    /// Mega Standard Pack: 5 Playing Cards, choose 2
    MegaStandard(Vec<Card>),
    /// Spectral Pack: 2 Spectrals, choose 1
    Spectral(Vec<Spectrals>),
}

impl TagPack {
    /// Generate a new tag pack with random contents
    pub fn new_mega_arcana() -> Self {
        let all_tarots = Tarots::all();
        let selected: Vec<Tarots> = all_tarots
            .choose_multiple(&mut rand::thread_rng(), 5)
            .copied()
            .collect();
        TagPack::MegaArcana(selected)
    }

    pub fn new_mega_celestial() -> Self {
        let all_planets = Planets::all();
        let selected: Vec<Planets> = all_planets
            .choose_multiple(&mut rand::thread_rng(), 5)
            .copied()
            .collect();
        TagPack::MegaCelestial(selected)
    }

    pub fn new_mega_buffoon() -> Self {
        // For now, use all common jokers for Mega Buffoon pack
        let all_jokers = Jokers::all_common();
        let selected: Vec<Jokers> = all_jokers
            .choose_multiple(&mut rand::thread_rng(), 4)
            .cloned()
            .collect();
        TagPack::MegaBuffoon(selected)
    }

    pub fn new_mega_standard() -> Self {
        use crate::card::{Suit, Value};

        // Generate 5 random playing cards
        let mut cards = Vec::new();
        for _ in 0..5 {
            let suit = *[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade]
                .choose(&mut rand::thread_rng())
                .unwrap();
            let value = *[
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
            ]
            .choose(&mut rand::thread_rng())
            .unwrap();

            cards.push(Card::new(value, suit));
        }
        TagPack::MegaStandard(cards)
    }

    pub fn new_spectral() -> Self {
        let all_spectrals = Spectrals::all();
        let selected: Vec<Spectrals> = all_spectrals
            .choose_multiple(&mut rand::thread_rng(), 2)
            .cloned()
            .collect();
        TagPack::Spectral(selected)
    }

    /// How many selections the player must make from this pack
    pub fn num_selections(&self) -> usize {
        match self {
            TagPack::MegaArcana(_) => 2,
            TagPack::MegaCelestial(_) => 2,
            TagPack::MegaBuffoon(_) => 2,
            TagPack::MegaStandard(_) => 2,
            TagPack::Spectral(_) => 1,
        }
    }

    /// Get the number of items in this pack
    pub fn size(&self) -> usize {
        match self {
            TagPack::MegaArcana(items) => items.len(),
            TagPack::MegaCelestial(items) => items.len(),
            TagPack::MegaBuffoon(items) => items.len(),
            TagPack::MegaStandard(items) => items.len(),
            TagPack::Spectral(items) => items.len(),
        }
    }

    /// Get name of this pack type
    pub fn name(&self) -> &'static str {
        match self {
            TagPack::MegaArcana(_) => "Mega Arcana Pack",
            TagPack::MegaCelestial(_) => "Mega Celestial Pack",
            TagPack::MegaBuffoon(_) => "Mega Buffoon Pack",
            TagPack::MegaStandard(_) => "Mega Standard Pack",
            TagPack::Spectral(_) => "Spectral Pack",
        }
    }
}
