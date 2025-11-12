// Common Rarity Jokers - 67 total
// These are the most basic and frequently available jokers

use super::*;


#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheJoker {}

impl Joker for TheJoker {
    fn name(&self) -> String {
        "Joker".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult".to_string()
    }
    fn cost(&self) -> usize {
        2
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.mult += 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreedyJoker {}

impl Joker for GreedyJoker {
    fn name(&self) -> String {
        "Greedy Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with diamond suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let diamonds = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Diamond)
                .count();
            g.mult += diamonds * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct LustyJoker {}

impl Joker for LustyJoker {
    fn name(&self) -> String {
        "Lusty Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with heart suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let hearts = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Heart)
                .count();
            g.mult += hearts * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WrathfulJoker {}

impl Joker for WrathfulJoker {
    fn name(&self) -> String {
        "Wrathful Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with spade suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let spades = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Spade)
                .count();
            g.mult += spades * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GluttonousJoker {}

impl Joker for GluttonousJoker {
    fn name(&self) -> String {
        "Gluttonous Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with club suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let clubs = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Club)
                .count();
            g.mult += clubs * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct JollyJoker {}

impl Joker for JollyJoker {
    fn name(&self) -> String {
        "Jolly Joker".to_string()
    }
    fn desc(&self) -> String {
        "+8 mult if played hand contains a pair".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.mult += 8
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ZanyJoker {}

impl Joker for ZanyJoker {
    fn name(&self) -> String {
        "Zany Joker".to_string()
    }
    fn desc(&self) -> String {
        "+12 mult if played hand contains a three of a kind".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.mult += 12
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MadJoker {}

impl Joker for MadJoker {
    fn name(&self) -> String {
        "Mad Joker".to_string()
    }
    fn desc(&self) -> String {
        "+10 mult if played hand contains a two pair".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.mult += 10
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CrazyJoker {}

impl Joker for CrazyJoker {
    fn name(&self) -> String {
        "Crazy Joker".to_string()
    }
    fn desc(&self) -> String {
        "+12 mult if played hand contains a straight".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_straight(&ctx).is_some() {
                g.mult += 12
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DrollJoker {}

impl Joker for DrollJoker {
    fn name(&self) -> String {
        "Droll Joker".to_string()
    }
    fn desc(&self) -> String {
        "+10 mult if played hand contains a flush".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_flush(&ctx).is_some() {
                g.mult += 10
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SlyJoker {}

impl Joker for SlyJoker {
    fn name(&self) -> String {
        "Sly Joker".to_string()
    }
    fn desc(&self) -> String {
        "+50 chips if played hand contains a pair".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_pair().is_some() {
                g.chips += 50
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WilyJoker {}

impl Joker for WilyJoker {
    fn name(&self) -> String {
        "Wily Joker".to_string()
    }
    fn desc(&self) -> String {
        "+100 chips if played hand contains a three of a kind".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_three_of_kind().is_some() {
                g.chips += 100
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CleverJoker {}

impl Joker for CleverJoker {
    fn name(&self) -> String {
        "Clever Joker".to_string()
    }
    fn desc(&self) -> String {
        "+80 chips if played hand contains a two pair".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.chips += 80
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DeviousJoker {}

impl Joker for DeviousJoker {
    fn name(&self) -> String {
        "Devious Joker".to_string()
    }
    fn desc(&self) -> String {
        "+100 chips if played hand contains a straight".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_straight(&ctx).is_some() {
                g.chips += 100
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CraftyJoker {}

impl Joker for CraftyJoker {
    fn name(&self) -> String {
        "Crafty Joker".to_string()
    }
    fn desc(&self) -> String {
        "+80 chips if played hand contains a flush".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_flush(&ctx).is_some() {
                g.chips += 80
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #16: Half Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct HalfJoker {}

impl Joker for HalfJoker {
    fn name(&self) -> String {
        "Half Joker".to_string()
    }
    fn desc(&self) -> String {
        "+20 Mult if played hand contains 3 or fewer cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.cards().len() <= 3 {
                g.mult += 20;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #17: Credit Card - allows going into debt
// Note: This is an Economy joker with passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CreditCard {}

impl Joker for CreditCard {
    fn name(&self) -> String {
        "Credit Card".to_string()
    }
    fn desc(&self) -> String {
        "Go up to -$20 in debt".to_string()
    }
    fn cost(&self) -> usize {
        1
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - handled in game logic
        vec![]
    }
}



// Joker #18: Banner
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Banner {}

impl Joker for Banner {
    fn name(&self) -> String {
        "Banner".to_string()
    }
    fn desc(&self) -> String {
        "+30 Chips for each remaining discard".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let discards_remaining = game.discards;
        fn apply(g: &mut Game, _hand: MadeHand, discards: usize) {
            g.chips += discards * 30;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, discards_remaining);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #19: Mystic Summit
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MysticSummit {}

impl Joker for MysticSummit {
    fn name(&self) -> String {
        "Mystic Summit".to_string()
    }
    fn desc(&self) -> String {
        "+15 Mult when discards remaining is 0".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let discards_remaining = game.discards;
        fn apply(g: &mut Game, _hand: MadeHand, discards: usize) {
            if discards == 0 {
                g.mult += 15;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, discards_remaining);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #20: Raised Fist
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RaisedFist {}

impl Joker for RaisedFist {
    fn name(&self) -> String {
        "Raised Fist".to_string()
    }
    fn desc(&self) -> String {
        "Adds double the rank of lowest ranked card held in hand to Mult".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::card::Value;

        fn apply(g: &mut Game, _hand: MadeHand) {
            // Calculate at score time, not registration time!
            let lowest_rank_value = g.hand.iter().map(|c| match c.value {
                Value::Two => 2,
                Value::Three => 3,
                Value::Four => 4,
                Value::Five => 5,
                Value::Six => 6,
                Value::Seven => 7,
                Value::Eight => 8,
                Value::Nine => 9,
                Value::Ten => 10,
                Value::Jack => 10,
                Value::Queen => 10,
                Value::King => 10,
                Value::Ace => 11,
            }).min().unwrap_or(0);
            let mult_bonus = lowest_rank_value * 2;
            g.mult += mult_bonus;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #21: Chaos the Clown - 1 free reroll per shop
// Note: Passive effect, handled in shop logic
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ChaosTheClown {}

impl Joker for ChaosTheClown {
    fn name(&self) -> String {
        "Chaos the Clown".to_string()
    }
    fn desc(&self) -> String {
        "1 free Reroll per shop".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - handled in shop logic
        vec![]
    }
}



// Joker #22: Scary Face
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ScaryFace {}

impl Joker for ScaryFace {
    fn name(&self) -> String {
        "Scary Face".to_string()
    }
    fn desc(&self) -> String {
        "+30 Chips for each face card played in scoring".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let face_count = hand.hand.cards().iter().filter(|c| c.is_face()).count();
            g.chips += face_count * 30;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #23: Abstract Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct AbstractJoker {}

impl Joker for AbstractJoker {
    fn name(&self) -> String {
        "Abstract Joker".to_string()
    }
    fn desc(&self) -> String {
        "+3 Mult for each Joker card (including self)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let joker_count = game.jokers.len();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            g.mult += count * 3;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, joker_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #24: Delayed Gratification - Economy joker
// Note: Effect handled at end of round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DelayedGratification {}

impl Joker for DelayedGratification {
    fn name(&self) -> String {
        "Delayed Gratification".to_string()
    }
    fn desc(&self) -> String {
        "Earn $2 per discard if no discards used by end of the round".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundEnd: Earn $2 per discard if no discards used
        fn on_round_end(g: &mut Game) {
            if g.discards_used == 0 {
                g.money += g.discards_total * 2;
            }
        }

        vec![Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))]
    }
}



// Joker #25: Gros Michel - +15 Mult with destruction chance
// Note: Destruction handled separately
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GrosMichel {}

impl Joker for GrosMichel {
    fn name(&self) -> String {
        "Gros Michel".to_string()
    }
    fn desc(&self) -> String {
        "+15 Mult (1 in 6 chance to be destroyed at end of round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.mult += 15;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #26: Even Steven
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct EvenSteven {}

impl Joker for EvenSteven {
    fn name(&self) -> String {
        "Even Steven".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each 10, 8, 6, 4, or 2 card in scored hand".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let even_count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Two | Value::Four | Value::Six | Value::Eight | Value::Ten))
                .count();
            g.mult += even_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #27: Odd Todd
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct OddTodd {}

impl Joker for OddTodd {
    fn name(&self) -> String {
        "Odd Todd".to_string()
    }
    fn desc(&self) -> String {
        "+31 Chips if played hand contains odd cards (A, 9, 7, 5, 3)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let has_odd = hand.hand.cards().iter()
                .any(|c| matches!(c.value, Value::Ace | Value::Three | Value::Five | Value::Seven | Value::Nine));
            if has_odd {
                g.chips += 31;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #28: Scholar
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Scholar {}

impl Joker for Scholar {
    fn name(&self) -> String {
        "Scholar".to_string()
    }
    fn desc(&self) -> String {
        "+20 Chips and +4 Mult per Ace played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus, Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let ace_count = hand.hand.cards().iter()
                .filter(|c| c.value == Value::Ace)
                .count();
            g.chips += ace_count * 20;
            g.mult += ace_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #29: Business Card - Economy joker
// Note: Random chance effect handled during scoring
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BusinessCard {}

impl Joker for BusinessCard {
    fn name(&self) -> String {
        "Business Card".to_string()
    }
    fn desc(&self) -> String {
        "Played face cards have 1 in 2 chance to give $2 when scored".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Played face cards have 1 in 2 chance to give $2
        fn on_score(g: &mut Game, hand: MadeHand) {
            let cards = hand.hand.cards();
            let face_count = cards.iter().filter(|c| c.is_face()).count();

            for _ in 0..face_count {
                if rand::random::<f32>() < 0.5 {
                    g.money += 2;
                }
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}



// Joker #30: Supernova
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Supernova {}

impl Joker for Supernova {
    fn name(&self) -> String {
        "Supernova".to_string()
    }
    fn desc(&self) -> String {
        "Adds the number of times poker hand has been played this run to Mult".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Clone the play counts HashMap for the closure
        let play_counts = game.hand_rank_play_counts.clone();

        fn apply(g: &mut Game, hand: MadeHand, counts: HashMap<HandRank, usize>) {
            let times_played = counts.get(&hand.rank).copied().unwrap_or(0);
            g.mult += times_played;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, play_counts.clone());
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #31: Ride the Bus - Stateful joker
// Note: State tracking needed - simplified implementation
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RideTheBus {}

impl Joker for RideTheBus {
    fn name(&self) -> String {
        "Ride the Bus".to_string()
    }
    fn desc(&self) -> String {
        "+1 Mult per consecutive hand without face cards (resets on face card)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = game.round_state.consecutive_hands_without_faces;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #32: Runner
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Runner {}

impl Joker for Runner {
    fn name(&self) -> String {
        "Runner".to_string()
    }
    fn desc(&self) -> String {
        "+15 Chips if played hand contains a Straight".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            use crate::hand::HandContext;
            let ctx = HandContext::default_context();
            if hand.hand.is_straight(&ctx).is_some() {
                g.chips += 15;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #33: Ice Cream - Stateful joker
// Note: Full implementation would track hands played
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct IceCream {}

impl Joker for IceCream {
    fn name(&self) -> String {
        "Ice Cream".to_string()
    }
    fn desc(&self) -> String {
        "+100 Chips (-5 Chips for each hand played)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let chips_bonus = 100_isize - (game.hands_played_this_blind as isize * 5);
        let chips_bonus = chips_bonus.max(0) as usize; // Don't go negative

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.chips += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, chips_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #34: Splash
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Splash {}

impl Joker for Splash {
    fn name(&self) -> String {
        "Splash".to_string()
    }
    fn desc(&self) -> String {
        "Every played card counts in scoring".to_string()
    }
    fn cost(&self) -> usize {
        3
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Note: This would need to be handled in the scoring logic itself
        // as it changes which cards count
        vec![]
    }
}



// Joker #35: Blue Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct BlueJoker {}

impl Joker for BlueJoker {
    fn name(&self) -> String {
        "Blue Joker".to_string()
    }
    fn desc(&self) -> String {
        "+2 Chips for each remaining card in deck".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let cards_in_deck = game.deck.cards().len();
        fn apply(g: &mut Game, _hand: MadeHand, deck_size: usize) {
            g.chips += deck_size * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_in_deck);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #36-61: Continuing with remaining jokers...
// Note: Many of these require complex state management or are better implemented
// with game-level hooks. For now, I'll provide implementations for the simpler ones
// and stubs for the complex ones.

// Joker #36: Sixth Sense - Complex (random chance, spectral card generation)
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SixthSense {}

impl Joker for SixthSense {
    fn name(&self) -> String {
        "Sixth Sense".to_string()
    }
    fn desc(&self) -> String {
        "1 in 6 chance to destroy played 6, create Spectral card if successful".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex effect - would need game-level handling
        vec![]
    }
}



// Joker #37: Constellation - Stateful
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Constellation {
    pub planet_cards_used: usize,
    pub bonus_mult: f32,
}

impl Default for Constellation {
    fn default() -> Self {
        Self {
            planet_cards_used: 0,
            bonus_mult: 1.0,
        }
    }
}

impl Eq for Constellation {}

impl std::hash::Hash for Constellation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.planet_cards_used.hash(state);
        self.bonus_mult.to_bits().hash(state);
    }
}

impl Constellation {
    pub fn on_planet_used(&mut self) {
        self.planet_cards_used += 1;
        self.bonus_mult = 1.0 + (self.planet_cards_used as f32 * 0.1);
    }
}

impl Joker for Constellation {
    fn name(&self) -> String {
        "Constellation".to_string()
    }
    fn desc(&self) -> String {
        format!("X{:.1} Mult ({} Planet cards used)", self.bonus_mult, self.planet_cards_used)
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
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



// Joker #38: Hiker - Modifies cards permanently
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Hiker {}

impl Joker for Hiker {
    fn name(&self) -> String {
        "Hiker".to_string()
    }
    fn desc(&self) -> String {
        "Every played card permanently gains +5 Chips when scored".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex - modifies cards permanently
        vec![]
    }
}



// Joker #39: Green Joker - Stateful
#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreenJoker {
    pub bonus_mult: isize,  // Accumulated mult bonus (can be negative)
}

impl Default for GreenJoker {
    fn default() -> Self {
        Self { bonus_mult: 0 }
    }
}

impl GreenJoker {
    pub fn on_hand_played(&mut self) {
        self.bonus_mult += 1;
    }

    pub fn on_discard_used(&mut self) {
        self.bonus_mult -= 1;
    }
}

impl Joker for GreenJoker {
    fn name(&self) -> String {
        "Green Joker".to_string()
    }
    fn desc(&self) -> String {
        format!("{:+} Mult (+1 per hand played; -1 per discard)", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_bonus = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: isize) {
            if bonus >= 0 {
                g.mult += bonus as usize;
            } else {
                g.mult = g.mult.saturating_sub((-bonus) as usize);
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #40: Superposition - Creates tarot cards
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Superposition {}

impl Joker for Superposition {
    fn name(&self) -> String {
        "Superposition".to_string()
    }
    fn desc(&self) -> String {
        "Create a Tarot card if poker hand contains Straight and Ace".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Create Tarot if hand contains Straight and Ace
        fn on_score(g: &mut Game, hand: MadeHand) {
            use crate::card::Value;
            use crate::rank::HandRank;

            // Check if hand is a Straight or Straight Flush
            let is_straight = matches!(hand.rank, HandRank::Straight | HandRank::StraightFlush);

            if !is_straight {
                return;
            }

            // Check if hand contains an Ace
            let cards = hand.hand.cards();
            let has_ace = cards.iter().any(|c| c.value == Value::Ace);

            if has_ace {
                g.create_random_tarot();
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}



// Joker #41: To Do List - Changes per round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ToDoList {}

impl Joker for ToDoList {
    fn name(&self) -> String {
        "To Do List".to_string()
    }
    fn desc(&self) -> String {
        "$5 if poker hand is listed type (hand changes each round)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let todo_hand = game.round_state.todo_hand;

        fn apply(g: &mut Game, hand: MadeHand, target: Option<HandRank>) {
            if let Some(target_rank) = target {
                if hand.rank == target_rank {
                    g.money += 5;
                }
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, todo_hand);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #42: Cavendish - Very rare destruction
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Cavendish {}

impl Joker for Cavendish {
    fn name(&self) -> String {
        "Cavendish".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult (1 in 1000 chance to be destroyed at end of round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: X3 Mult
        fn on_score(g: &mut Game, _hand: MadeHand) {
            g.mult *= 3;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
        // TODO: OnRoundEnd effect with 1 in 1000 chance to destroy this joker
    }
}



// Joker #43: Red Card - Triggers on pack skip
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RedCard {
    pub bonus_mult: usize,
}

impl RedCard {
    pub fn on_booster_skipped(&mut self) {
        self.bonus_mult += 3;
    }
}

impl Joker for RedCard {
    fn name(&self) -> String {
        "Red Card".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} Mult (gains +3 when Booster Pack skipped)", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_bonus = self.bonus_mult;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #44: Square Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SquareJoker {}

impl Joker for SquareJoker {
    fn name(&self) -> String {
        "Square Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +4 Chips if hand has exactly 4 cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.cards().len() == 4 {
                g.chips += 4;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Remaining jokers (#45-64) - These are unlockable and/or require complex state
// I'll add simplified implementations for completeness

// Joker #45: Riff-Raff
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RiffRaff {}

impl Joker for RiffRaff {
    fn name(&self) -> String {
        "Riff-Raff".to_string()
    }
    fn desc(&self) -> String {
        "When Blind selected, create 2 Common Jokers".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}



// Joker #46: Golden Ticket
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GoldenTicket {}

impl Joker for GoldenTicket {
    fn name(&self) -> String {
        "Golden Ticket".to_string()
    }
    fn desc(&self) -> String {
        "Played Gold cards earn $3 when scored".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}



// Joker #47: Swashbuckler
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Swashbuckler {}

impl Joker for Swashbuckler {
    fn name(&self) -> String {
        "Swashbuckler".to_string()
    }
    fn desc(&self) -> String {
        "Adds sell value of all Jokers to Mult (+1 Mult per card sold)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let total_sell_value: usize = game.jokers.iter()
            .map(|j| j.sell_value())
            .sum();
        fn apply(g: &mut Game, _hand: MadeHand, sell_value: usize) {
            g.mult += sell_value;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, total_sell_value);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #48: Smiley Face
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SmileyFace {}

impl Joker for SmileyFace {
    fn name(&self) -> String {
        "Smiley Face".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each face card played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let face_count = hand.hand.cards().iter().filter(|c| c.is_face()).count();
            g.mult += face_count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #49: Golden Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GoldenJoker {}

impl Joker for GoldenJoker {
    fn name(&self) -> String {
        "Golden Joker".to_string()
    }
    fn desc(&self) -> String {
        "Earn $3 at end of round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundEnd: Earn $3
        fn on_round_end(g: &mut Game) {
            g.money += 3;
        }

        vec![Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))]
    }
}



// Joker #50: Drunkard - Passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Drunkard {}

impl Joker for Drunkard {
    fn name(&self) -> String {
        "Drunkard".to_string()
    }
    fn desc(&self) -> String {
        "+1 discard per round".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - handled via GameModifiers.discard_bonus in update_modifiers()
        vec![]
    }
}



// Joker #51: Faceless Joker
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FacelessJoker {}

impl Joker for FacelessJoker {
    fn name(&self) -> String {
        "Faceless Joker".to_string()
    }
    fn desc(&self) -> String {
        "Earn $5 if 3+ face cards discarded at once".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn on_discard(g: &mut Game, hand: MadeHand) {
            let face_count = hand
                .hand
                .cards()
                .iter()
                .filter(|c| c.is_face())
                .count();

            if face_count >= 3 {
                g.money += 5;
            }
        }

        vec![Effects::OnDiscard(Arc::new(Mutex::new(on_discard)))]
    }
}



// Joker #52: Hanging Chad - Retrigger effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct HangingChad {}

impl Joker for HangingChad {
    fn name(&self) -> String {
        "Hanging Chad".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger first card used in scoring 2 additional times".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Retrigger]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}



// Joker #53: Popcorn - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Popcorn {}

impl Joker for Popcorn {
    fn name(&self) -> String {
        "Popcorn".to_string()
    }
    fn desc(&self) -> String {
        "+20 Mult (-4 Mult per round played)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = 20_isize - (game.round as isize * 4);
        let mult_bonus = mult_bonus.max(0) as usize; // Don't go negative

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #54: Walkie Talkie
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WalkieTalkie {}

impl Joker for WalkieTalkie {
    fn name(&self) -> String {
        "Walkie Talkie".to_string()
    }
    fn desc(&self) -> String {
        "+10 Chips and +4 Mult for each 10 or 4 played".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus, Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Ten | Value::Four))
                .count();
            g.chips += count * 10;
            g.mult += count * 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #55: Shoot the Moon
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ShootTheMoon {}

impl Joker for ShootTheMoon {
    fn name(&self) -> String {
        "Shoot the Moon".to_string()
    }
    fn desc(&self) -> String {
        "+13 Mult for each Queen held in hand".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::card::Value;

        fn apply(g: &mut Game, _hand: MadeHand) {
            // Calculate at score time, not registration time!
            let queen_count = g.hand.iter().filter(|c| c.value == Value::Queen).count();
            let mult_bonus = queen_count * 13;
            g.mult += mult_bonus;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #56: Fortune Teller - Stateful
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FortuneTeller {
    pub tarot_cards_used: usize,
}

impl FortuneTeller {
    pub fn on_tarot_used(&mut self) {
        self.tarot_cards_used += 1;
    }
}

impl Joker for FortuneTeller {
    fn name(&self) -> String {
        "Fortune Teller".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} Mult ({} Tarot cards used)", self.tarot_cards_used, self.tarot_cards_used)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let mult_bonus = self.tarot_cards_used;

        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}



// Joker #57: Juggler - Passive effect
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Juggler {}

impl Joker for Juggler {
    fn name(&self) -> String {
        "Juggler".to_string()
    }
    fn desc(&self) -> String {
        "+1 hand size".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
}



// Joker #58: Photograph
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Photograph {}

impl Joker for Photograph {
    fn name(&self) -> String {
        "Photograph".to_string()
    }
    fn desc(&self) -> String {
        "First played face card gives X2 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: First played face card gives X2 Mult
        fn on_score(g: &mut Game, hand: MadeHand) {
            let cards = hand.hand.cards();
            // Find first face card
            let has_face_card = cards.iter().any(|c| c.is_face());

            if has_face_card {
                g.mult *= 2;
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}



// Joker #59: Reserved Parking
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ReservedParking {}

impl Joker for ReservedParking {
    fn name(&self) -> String {
        "Reserved Parking".to_string()
    }
    fn desc(&self) -> String {
        "1 in 3 chance for each face card held in hand to give $1".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            // Calculate at score time, not registration time!
            let face_cards: Vec<_> = g.hand.iter().filter(|c| c.is_face()).collect();
            let mut money_bonus = 0;

            // Each face card has 1 in 3 chance to give $1
            for _ in &face_cards {
                if rand::random::<f32>() < 1.0 / 3.0 {
                    money_bonus += 1;
                }
            }

            g.money += money_bonus;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}



// Joker #60: Mail-In Rebate - Changes per round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MailInRebate {}

impl Joker for MailInRebate {
    fn name(&self) -> String {
        "Mail-In Rebate".to_string()
    }
    fn desc(&self) -> String {
        "Earn $3 for each discarded rank (rank changes each round)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Needs OnDiscard effect with access to discarded cards
        // For now, this joker's effect is handled directly in Game::discard_selected()
        vec![]
    }
}



// Joker #61: 8 Ball - Complex (random chance, tarot generation)
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct EightBall {}

impl Joker for EightBall {
    fn name(&self) -> String {
        "8 Ball".to_string()
    }
    fn desc(&self) -> String {
        "1 in 5 chance per 8 played to create Tarot (no 8s in deck)".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: 1 in 5 chance per 8 played to create Tarot
        fn on_score(g: &mut Game, hand: MadeHand) {
            use crate::card::Value;

            // Count 8s in played hand
            let cards = hand.hand.cards();
            let eights_played = cards.iter().filter(|c| c.value == Value::Eight).count();

            // For each 8, 1 in 5 chance to create Tarot
            for _ in 0..eights_played {
                if rand::random::<f32>() < 0.2 {
                    g.create_random_tarot();
                }
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}



// Joker #62: Misprint - Random mult
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Misprint {}

impl Joker for Misprint {
    fn name(&self) -> String {
        "Misprint".to_string()
    }
    fn desc(&self) -> String {
        "+0 to +23 Mult (random each time)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Add random mult between 0 and 23
        fn on_score(g: &mut Game, _hand: MadeHand) {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let bonus = rng.gen_range(0..=23);
            g.mult += bonus;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}



// Joker #63: Egg - Gains sell value over time
#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Egg {
    pub sell_value_bonus: usize,
}

impl Default for Egg {
    fn default() -> Self {
        Self { sell_value_bonus: 0 }
    }
}

impl Egg {
    pub fn on_round_end(&mut self) {
        self.sell_value_bonus += 3;
    }
}

impl Joker for Egg {
    fn name(&self) -> String {
        "Egg".to_string()
    }
    fn desc(&self) -> String {
        format!("Gains $3 sell value at end of round (Current: +${})", self.sell_value_bonus)
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        vec![]
    }
    fn sell_value(&self) -> usize {
        self.cost() / 2 + self.sell_value_bonus
    }
}



// Joker #64: Hit the Road - X0.5 Mult per Jack discarded this round
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct HitTheRoad {}

impl Joker for HitTheRoad {
    fn name(&self) -> String {
        "Hit the Road".to_string()
    }
    fn desc(&self) -> String {
        "X0.5 Mult for every Jack discarded this round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let jacks_discarded = game.round_state.jacks_discarded_this_round;

        fn apply(g: &mut Game, _hand: MadeHand, jack_count: usize) {
            // X0.5 for each jack: 0.5^jack_count
            let multiplier = 0.5_f32.powi(jack_count as i32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, jacks_discarded);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}



// Joker #65: Satellite - $1 at end of round per unique Planet card used
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Satellite {}

impl Joker for Satellite {
    fn name(&self) -> String {
        "Satellite".to_string()
    }
    fn desc(&self) -> String {
        "$1 at end of round per unique Planet card used this run".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        fn on_round_end(g: &mut Game) {
            let unique_count = g.unique_planets_used.len();
            g.money += unique_count;
        }

        vec![Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))]
    }
}



// Joker: Hallucination - 1 in 2 chance to create a Tarot card when any Booster Pack is opened
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Hallucination {}

impl Joker for Hallucination {
    fn name(&self) -> String {
        "Hallucination".to_string()
    }
    fn desc(&self) -> String {
        "1 in 2 chance to create a Tarot card when any Booster Pack is opened (Must have room)".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need OnPackOpen effect type (unique - only joker that triggers on pack opening)
        // TODO: Need Tarot card creation system
        // TODO: Need consumable slot availability checking
        vec![]
    }
}
