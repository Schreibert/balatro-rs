// Legendary Rarity Jokers - 5 total
// These are the rarest and most powerful jokers in the game

use super::*;

// Joker: Triboulet - Played Kings and Queens each give X2 Mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Triboulet {}

impl Joker for Triboulet {
    fn name(&self) -> String {
        "Triboulet".to_string()
    }
    fn desc(&self) -> String {
        "Played Kings and Queens each give X2 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let royal_count = hand
                .hand
                .cards()
                .iter()
                .filter(|c| matches!(c.value, Value::King | Value::Queen))
                .count();

            for _ in 0..royal_count {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Canio - X Mult (gains X1 Mult when a face card is destroyed)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Canio {
    pub bonus_mult: f32,  // Accumulated X mult multiplier (starts at 1.0)
}

// Manual implementations for Eq and Hash since f32 doesn't support them
impl Eq for Canio {}

impl std::hash::Hash for Canio {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the bits of the f32 for deterministic hashing
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Default for Canio {
    fn default() -> Self {
        Self { bonus_mult: 1.0 }
    }
}

impl PartialEq for Canio {
    fn eq(&self, other: &Self) -> bool {
        self.bonus_mult.to_bits() == other.bonus_mult.to_bits()
    }
}

impl Canio {
    pub fn on_face_card_destroyed(&mut self) {
        self.bonus_mult += 1.0;
    }
}

impl Joker for Canio {
    fn name(&self) -> String {
        "Canio".to_string()
    }
    fn desc(&self) -> String {
        format!("X{} Mult (gains X1 Mult when a face card is destroyed)", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_multiplier = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Yorick - Gains X1 Mult every 23 cards discarded
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Yorick {
    pub cards_discarded: usize,  // Total cards discarded
    pub bonus_mult: f32,          // Accumulated X mult (starts at 1.0)
}

impl Default for Yorick {
    fn default() -> Self {
        Self {
            cards_discarded: 0,
            bonus_mult: 1.0,
        }
    }
}

impl Eq for Yorick {}

impl PartialEq for Yorick {
    fn eq(&self, other: &Self) -> bool {
        self.cards_discarded == other.cards_discarded
            && self.bonus_mult.to_bits() == other.bonus_mult.to_bits()
    }
}

impl std::hash::Hash for Yorick {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cards_discarded.hash(state);
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Yorick {
    pub fn on_cards_discarded(&mut self, count: usize) {
        self.cards_discarded += count;
        // Every 23 cards, gain X1 mult
        let levels = self.cards_discarded / 23;
        self.bonus_mult = 1.0 + levels as f32;
    }
}

impl Joker for Yorick {
    fn name(&self) -> String {
        "Yorick".to_string()
    }
    fn desc(&self) -> String {
        format!("X{} Mult ({}/23 cards discarded for next level)", self.bonus_mult, self.cards_discarded % 23)
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_multiplier = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, multiplier: f32) {
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_multiplier);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Chicot - Disables effect of every Boss Blind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Chicot {}

impl Joker for Chicot {
    fn name(&self) -> String {
        "Chicot".to_string()
    }
    fn desc(&self) -> String {
        "Disables effect of every Boss Blind".to_string()
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would be checked in Boss Blind logic
        vec![]
    }
}

// Joker: Perkeo - Creates Negative copy of 1 random consumable at end of shop
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Perkeo {}

impl Joker for Perkeo {
    fn name(&self) -> String {
        "Perkeo".to_string()
    }
    fn desc(&self) -> String {
        "Creates Negative copy of 1 random consumable at end of shop".to_string()
    }
    fn cost(&self) -> usize {
        0
    }
    fn rarity(&self) -> Rarity {
        Rarity::Legendary
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        vec![Effects::OnShopEnd(Arc::new(Mutex::new(|game: &mut Game| {
            // Only duplicate if we have consumables and space for more
            if game.consumables.is_empty() {
                return;
            }
            if game.consumables.len() >= game.config.consumable_slots {
                return;
            }

            // Pick a random consumable to duplicate
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            if let Some(consumable) = game.consumables.choose(&mut rng) {
                // TODO: Full implementation should create a "Negative" edition consumable
                // which provides +1 consumable slot. For now, just duplicate if space available.
                game.consumables.push(consumable.clone());
            }
        })))]
    }
}
