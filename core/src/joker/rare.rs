// Rare Rarity Jokers - 17 total
// These are rare jokers with powerful and unique effects

use super::*;

// Joker: The Duo - X2 Mult if played hand contains a Pair
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheDuo {}

impl Joker for TheDuo {
    fn name(&self) -> String {
        "The Duo".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand contains a Pair".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Trio - X3 Mult if played hand contains Three of a Kind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheTrio {}

impl Joker for TheTrio {
    fn name(&self) -> String {
        "The Trio".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if played hand contains Three of a Kind".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Family - X4 Mult if played hand contains Four of a Kind
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheFamily {}

impl Joker for TheFamily {
    fn name(&self) -> String {
        "The Family".to_string()
    }
    fn desc(&self) -> String {
        "X4 Mult if played hand contains Four of a Kind".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_four_of_kind().is_some() {
                g.mult = g.mult * 4;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Order - X3 Mult if played hand contains Straight
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheOrder {}

impl Joker for TheOrder {
    fn name(&self) -> String {
        "The Order".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if played hand contains Straight".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_straight(&ctx).is_some() {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: The Tribe - X2 Mult if played hand contains Flush
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheTribe {}

impl Joker for TheTribe {
    fn name(&self) -> String {
        "The Tribe".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand contains Flush".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_flush(&ctx).is_some() {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Baron - Each King held in hand gives X1.5 Mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Baron {}
impl Joker for Baron {
    fn name(&self) -> String {
        "Baron".to_string()
    }
    fn desc(&self) -> String {
        "Each King held in hand gives X1.5 Mult".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::card::Value;

        fn apply(g: &mut Game, _hand: MadeHand) {
            // Calculate at score time, not registration time!
            let king_count = g.hand.iter().filter(|c| c.value == Value::King).count();
            let mult_multiplier = 1.5_f32.powi(king_count as i32);
            g.mult = (g.mult as f32 * mult_multiplier) as usize;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Blueprint - Copies ability of Joker to the right
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Blueprint {}
impl Joker for Blueprint {
    fn name(&self) -> String {
        "Blueprint".to_string()
    }
    fn desc(&self) -> String {
        "Copies ability of Joker to the right".to_string()
    }
    fn cost(&self) -> usize {
        10
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // Complex - would need to dynamically copy another joker's effects
        vec![]
    }
}

// Joker: WeeJoker - Gains +8 Chips when each played 2 is scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WeeJoker {}
impl Joker for WeeJoker {
    fn name(&self) -> String {
        "Wee Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +8 Chips when each played 2 is scored".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let twos = hand
                .hand
                .cards()
                .iter()
                .filter(|c| c.value == Value::Two)
                .count();
            g.chips += twos * 8;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: BaseballCard - Uncommon Jokers each give X1.5 Mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BaseballCard {}
impl Joker for BaseballCard {
    fn name(&self) -> String {
        "Baseball Card".to_string()
    }
    fn desc(&self) -> String {
        "Uncommon Jokers each give X1.5 Mult".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let uncommon_count = game
            .jokers
            .iter()
            .filter(|j| j.rarity() == Rarity::Uncommon)
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            // X1.5 per uncommon = multiply by (1.5 ^ count)
            let multiplier = 1.5_f32.powi(count as i32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, uncommon_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: AncientJoker - Each played card with [suit] gives X1.5 Mult when scored
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct AncientJoker {}
impl Joker for AncientJoker {
    fn name(&self) -> String {
        "Ancient Joker".to_string()
    }
    fn desc(&self) -> String {
        "Each played card with [suit] gives X1.5 Mult when scored; suit changes at end of round".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let ancient_suit = game.round_state.ancient_suit;

        fn apply(g: &mut Game, hand: MadeHand, suit: Option<Suit>) {
            if let Some(target_suit) = suit {
                let matching_cards = hand.all.iter().filter(|c| c.suit == target_suit).count();
                if matching_cards > 0 {
                    let multiplier = 1.5_f32.powi(matching_cards as i32);
                    g.mult = (g.mult as f32 * multiplier) as usize;
                }
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, ancient_suit);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Stuntman - +250 Chips; +3 hand size
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Stuntman {}
impl Joker for Stuntman {
    fn name(&self) -> String {
        "Stuntman".to_string()
    }
    fn desc(&self) -> String {
        "+250 Chips; +3 hand size".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips, Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.chips += 250;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Vagabond - Create Tarot card if hand played with $4 or less
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Vagabond {}
impl Joker for Vagabond {
    fn name(&self) -> String {
        "Vagabond".to_string()
    }
    fn desc(&self) -> String {
        "Create Tarot card if hand played with $4 or less".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Create Tarot if hand played with $4 or less
        fn on_score(g: &mut Game, _hand: MadeHand) {
            if g.money <= 4 {
                g.create_random_tarot();
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}

// Joker: Driver's License - X3 Mult if full deck has at least 16 Enhanced cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DriverLicense {}
impl Joker for DriverLicense {
    fn name(&self) -> String {
        "Driver's License".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if full deck has at least 16 Enhanced cards".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // Count enhanced cards in full deck (deck + available + discarded)
        let enhanced_count = game.deck.cards().iter()
            .chain(game.available.cards().iter())
            .chain(game.discarded.iter())
            .filter(|c| c.enhancement.is_some())
            .count();

        if enhanced_count >= 16 {
            let effect = Effects::OnScore(Arc::new(Mutex::new(|g: &mut Game, _hand: MadeHand| {
                g.mult *= 3;
            })));
            vec![effect]
        } else {
            vec![]
        }
    }
}

// Joker: Burnt Joker - Upgrade level of first discarded poker hand each round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BurntJoker {}
impl Joker for BurntJoker {
    fn name(&self) -> String {
        "Burnt Joker".to_string()
    }
    fn desc(&self) -> String {
        "Upgrade level of first discarded poker hand each round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need OnDiscard effect to track first discard of round
        // TODO: Need hand level upgrade system
        vec![]
    }
}

// Joker: Invisible Joker - After 2 rounds, sell this to duplicate random Joker
#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct InvisibleJoker {
    pub rounds_remaining: usize,
}
impl Default for InvisibleJoker {
    fn default() -> Self {
        Self { rounds_remaining: 2 }
    }
}
impl Joker for InvisibleJoker {
    fn name(&self) -> String {
        "Invisible Joker".to_string()
    }
    fn desc(&self) -> String {
        format!("After {} rounds, sell this to duplicate random Joker", self.rounds_remaining)
    }
    fn cost(&self) -> usize {
        10
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundEnd: Decrement rounds_remaining
        fn on_round_end(g: &mut Game) {
            for joker in g.jokers.iter_mut() {
                if let Jokers::InvisibleJoker(ref mut ij) = joker {
                    ij.rounds_remaining = ij.rounds_remaining.saturating_sub(1);
                    break;
                }
            }
        }

        // OnSell: If rounds_remaining == 0, duplicate a random other joker
        fn on_sell(g: &mut Game) {
            // Check if InvisibleJoker being sold has rounds_remaining == 0
            // Find InvisibleJoker in jokers list
            let mut should_duplicate = false;
            for joker in g.jokers.iter() {
                if let Jokers::InvisibleJoker(ref ij) = joker {
                    if ij.rounds_remaining == 0 {
                        should_duplicate = true;
                    }
                    break;
                }
            }

            if should_duplicate && g.jokers.len() > 1 {
                // Get all jokers except InvisibleJoker itself
                use rand::seq::SliceRandom;
                let other_jokers: Vec<Jokers> = g.jokers.iter()
                    .filter(|j| !matches!(j, Jokers::InvisibleJoker(_)))
                    .cloned()
                    .collect();

                if !other_jokers.is_empty() {
                    // Pick a random joker to duplicate
                    let to_duplicate = other_jokers.choose(&mut rand::thread_rng()).unwrap().clone();

                    // Add it if there's space
                    if g.jokers.len() < g.max_joker_slots() {
                        g.jokers.push(to_duplicate);
                        // Re-register joker effects
                        g.effect_registry = crate::effect::EffectRegistry::new();
                        g.effect_registry.register_jokers(g.jokers.clone(), &g.clone());
                    }
                }
            }
        }

        vec![
            Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end))),
            Effects::OnSell(Arc::new(Mutex::new(on_sell)))
        ]
    }
}

// Joker: Brainstorm - Copies ability of leftmost Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Brainstorm {}
impl Joker for Brainstorm {
    fn name(&self) -> String {
        "Brainstorm".to_string()
    }
    fn desc(&self) -> String {
        "Copies ability of leftmost Joker".to_string()
    }
    fn cost(&self) -> usize {
        10
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need dynamic effect copying system (similar to Blueprint which copies rightmost)
        // TODO: Find leftmost joker and copy its effects
        vec![]
    }
}

// Joker: DNA - If first hand of round has only 1 card, add permanent copy to deck and draw it to hand
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DNA {}
impl Joker for DNA {
    fn name(&self) -> String {
        "DNA".to_string()
    }
    fn desc(&self) -> String {
        "If first hand of round has only 1 card, add permanent copy to deck and draw it to hand".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need OnPlay effect to check if first hand of round
        // TODO: Need deck modification system (add card permanently)
        // TODO: Need card duplication and draw system
        vec![]
    }
}
