// Uncommon Rarity Jokers - 62 total
// These are moderately rare jokers with more complex effects

use super::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Throwback {}

impl Joker for Throwback {
    fn name(&self) -> String {
        "Throwback".to_string()
    }
    fn desc(&self) -> String {
        "X0.25 Mult per blind skipped this run".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let skips = game.blinds_skipped_count;

        fn apply(g: &mut Game, _hand: MadeHand, skip_count: usize) {
            // X0.25 for each skip: 1.0 + (0.25 * skip_count)
            let multiplier = 1.0 + (0.25 * skip_count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, skips);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct LoyaltyCard {
    pub hands_until_bonus: usize,
}

impl Default for LoyaltyCard {
    fn default() -> Self {
        Self {
            hands_until_bonus: 6,
        }
    }
}

impl Joker for LoyaltyCard {
    fn name(&self) -> String {
        "Loyalty Card".to_string()
    }
    fn desc(&self) -> String {
        format!("X4 Mult every 6 hands played ({}/6)", 6 - self.hands_until_bonus)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let should_trigger = self.hands_until_bonus == 0;

        fn apply(g: &mut Game, _hand: MadeHand, trigger: bool) {
            if trigger {
                g.mult = (g.mult as f32 * 4.0) as usize;
            }
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, should_trigger);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl LoyaltyCard {
    pub fn on_hand_played(&mut self) {
        if self.hands_until_bonus == 0 {
            self.hands_until_bonus = 6;
        }
        self.hands_until_bonus -= 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Campfire {
    pub cards_sold: usize,
}

impl Default for Campfire {
    fn default() -> Self {
        Self { cards_sold: 0 }
    }
}

impl Joker for Campfire {
    fn name(&self) -> String {
        "Campfire".to_string()
    }
    fn desc(&self) -> String {
        let mult = 1.0 + (0.25 * self.cards_sold as f32);
        format!("X{:.2} Mult (X0.25 per card sold, resets on boss)", mult)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let cards_sold = self.cards_sold;

        fn apply(g: &mut Game, _hand: MadeHand, sold_count: usize) {
            let multiplier = 1.0 + (0.25 * sold_count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_sold);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl Campfire {
    pub fn on_card_sold(&mut self) {
        self.cards_sold += 1;
    }

    pub fn reset_on_boss(&mut self) {
        self.cards_sold = 0;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Hologram {
    pub cards_added: usize,
}

impl Default for Hologram {
    fn default() -> Self {
        Self { cards_added: 0 }
    }
}

impl Joker for Hologram {
    fn name(&self) -> String {
        "Hologram".to_string()
    }
    fn desc(&self) -> String {
        let mult = 1.0 + (0.25 * self.cards_added as f32);
        format!("X{:.2} Mult (X0.25 when card added to deck)", mult)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let cards_added = self.cards_added;

        fn apply(g: &mut Game, _hand: MadeHand, added_count: usize) {
            let multiplier = 1.0 + (0.25 * added_count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_added);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl Hologram {
    pub fn on_card_added(&mut self) {
        self.cards_added += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Obelisk {
    pub consecutive_count: usize,
}

impl Default for Obelisk {
    fn default() -> Self {
        Self {
            consecutive_count: 0,
        }
    }
}

impl Joker for Obelisk {
    fn name(&self) -> String {
        "Obelisk".to_string()
    }
    fn desc(&self) -> String {
        let mult = 1.0 + (0.2 * self.consecutive_count as f32);
        format!(
            "X{:.1} Mult (X0.2 per consecutive hand without most-played hand)",
            mult
        )
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let consecutive = self.consecutive_count;

        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            let multiplier = 1.0 + (0.2 * count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, consecutive);
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl Obelisk {
    pub fn on_hand_played(&mut self, rank: crate::rank::HandRank, most_played: Option<crate::rank::HandRank>) {
        if let Some(most) = most_played {
            if rank == most {
                self.consecutive_count = 0;
            } else {
                self.consecutive_count += 1;
            }
        } else {
            // No most-played hand yet (first hand of run)
            self.consecutive_count = 0;
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheIdol {}

impl Joker for TheIdol {
    fn name(&self) -> String {
        "The Idol".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult for each [rank] of [suit] in hand (changes per round)".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let idol_rank = game.round_state.idol_rank;
        let idol_suit = game.round_state.idol_suit;
        let hand_cards = game.hand.clone();

        fn apply(g: &mut Game, _hand: MadeHand, rank: Option<Value>, suit: Option<Suit>, hand: Vec<Card>) {
            if let (Some(r), Some(s)) = (rank, suit) {
                let matching_count = hand.iter()
                    .filter(|c| c.value == r && c.suit == s)
                    .count();

                if matching_count > 0 {
                    let multiplier = 2.0_f32.powi(matching_count as i32);
                    g.mult = (g.mult as f32 * multiplier) as usize;
                }
            }
        }

        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, idol_rank, idol_suit, hand_cards.clone());
        };

        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SpaceJoker {}

impl Joker for SpaceJoker {
    fn name(&self) -> String {
        "Space Joker".to_string()
    }
    fn desc(&self) -> String {
        "1 in 4 chance to upgrade level of played poker hand".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use rand::Rng;
        fn apply(g: &mut Game, hand: MadeHand) {
            if rand::thread_rng().gen_bool(0.25) {
                // Upgrade the hand rank that was just played
                g.upgrade_hand(hand.rank);
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Burglar {}

impl Joker for Burglar {
    fn name(&self) -> String {
        "Burglar".to_string()
    }
    fn desc(&self) -> String {
        "When Blind is selected, gain +3 Hands and lose all discards".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        fn on_blind_select(g: &mut Game) {
            g.plays += 3;
            g.discards = 0;
        }

        vec![Effects::OnBlindSelect(Arc::new(Mutex::new(on_blind_select)))]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Rocket {
    pub payout: usize,
}

impl Default for Rocket {
    fn default() -> Self {
        Self { payout: 1 }
    }
}

impl Joker for Rocket {
    fn name(&self) -> String {
        "Rocket".to_string()
    }
    fn desc(&self) -> String {
        format!("Earn ${} at end of round (+$2 per Boss defeated)", self.payout)
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundEnd: Earn payout amount (read live from game state)
        fn on_round_end(g: &mut Game) {
            // Find the Rocket joker and read its current payout value
            for joker in &g.jokers {
                if let Jokers::Rocket(rocket) = joker {
                    g.money += rocket.payout;
                    break;
                }
            }
        }

        vec![Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))]
        // TODO: Need OnBossDefeat effect to increment payout
    }
}

impl Rocket {
    pub fn on_boss_defeated(&mut self) {
        self.payout += 2;
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MerryAndy {}

impl Joker for MerryAndy {
    fn name(&self) -> String {
        "Merry Andy".to_string()
    }
    fn desc(&self) -> String {
        "+3 discards each round, -1 hand size".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // Passive effect - handled in game logic for discards and hand size
        vec![]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct OopsAll6s {}

impl Joker for OopsAll6s {
    fn name(&self) -> String {
        "Oops! All 6s".to_string()
    }
    fn desc(&self) -> String {
        "Doubles all probabilities".to_string()
    }
    fn cost(&self) -> usize {
        4
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // Passive effect - checked in shop probability generation
        // Full implementation requires comprehensive probability system
        // Currently only affects joker rarity probabilities
        vec![]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Ramen {
    pub cards_discarded: usize,
}

impl Default for Ramen {
    fn default() -> Self {
        Self {
            cards_discarded: 0,
        }
    }
}

impl Joker for Ramen {
    fn name(&self) -> String {
        "Ramen".to_string()
    }
    fn desc(&self) -> String {
        let mult = 2.0 - (0.01 * self.cards_discarded as f32);
        format!("X{:.2} Mult (loses X0.01 per card discarded)", mult.max(0.0))
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let discarded = self.cards_discarded;
        fn apply(g: &mut Game, _hand: MadeHand, disc: usize) {
            let multiplier = (2.0 - (0.01 * disc as f32)).max(0.0);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, discarded);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl Ramen {
    pub fn on_cards_discarded(&mut self, count: usize) {
        self.cards_discarded += count;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Castle {
    pub bonus_chips: usize,
}

impl Default for Castle {
    fn default() -> Self {
        Self { bonus_chips: 0 }
    }
}

impl Joker for Castle {
    fn name(&self) -> String {
        "Castle".to_string()
    }
    fn desc(&self) -> String {
        format!(
            "+{} Chips (gains +3 per discarded suit card, suit changes each round)",
            self.bonus_chips
        )
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let chips = self.bonus_chips;
        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.chips += bonus;
        }
        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, chips);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl Castle {
    pub fn on_suit_card_discarded(&mut self) {
        self.bonus_chips += 3;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GlassJoker {
    pub glass_destroyed: usize,
}

impl Default for GlassJoker {
    fn default() -> Self {
        Self {
            glass_destroyed: 0,
        }
    }
}

impl Joker for GlassJoker {
    fn name(&self) -> String {
        "Glass Joker".to_string()
    }
    fn desc(&self) -> String {
        let mult = 1.0 + (0.75 * self.glass_destroyed as f32);
        format!("X{:.2} Mult (X0.75 per Glass Card destroyed)", mult)
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let destroyed = self.glass_destroyed;
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            let multiplier = 1.0 + (0.75 * count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, destroyed);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl GlassJoker {
    pub fn on_glass_card_destroyed(&mut self) {
        self.glass_destroyed += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct LuckyCat {
    pub lucky_triggers: usize,
}

impl Default for LuckyCat {
    fn default() -> Self {
        Self {
            lucky_triggers: 0,
        }
    }
}

impl Joker for LuckyCat {
    fn name(&self) -> String {
        "Lucky Cat".to_string()
    }
    fn desc(&self) -> String {
        let mult = 1.0 + (0.25 * self.lucky_triggers as f32);
        format!("X{:.2} Mult (X0.25 per Lucky card trigger)", mult)
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let triggers = self.lucky_triggers;
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            let multiplier = 1.0 + (0.25 * count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, triggers);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

impl LuckyCat {
    pub fn on_lucky_trigger(&mut self) {
        self.lucky_triggers += 1;
    }
}

// UNCOMMON JOKERS

// Joker: Fibonacci - Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Fibonacci {}

impl Joker for Fibonacci {
    fn name(&self) -> String {
        "Fibonacci".to_string()
    }
    fn desc(&self) -> String {
        "Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Value;
        fn apply(g: &mut Game, hand: MadeHand) {
            let fib_count = hand.hand.cards().iter()
                .filter(|c| matches!(c.value, Value::Ace | Value::Two | Value::Three | Value::Five | Value::Eight))
                .count();
            g.mult += fib_count * 8;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Spare Trousers - Gains +2 Mult if played hand contains Two Pair

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SpareTrousers {}

impl Joker for SpareTrousers {
    fn name(&self) -> String {
        "Spare Trousers".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult if played hand contains Two Pair".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            if hand.hand.is_two_pair().is_some() {
                g.mult += 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Acrobat - X3 Mult on final hand of round
// Note: Requires tracking if this is the final hand

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Acrobat {}

impl Joker for Acrobat {
    fn name(&self) -> String {
        "Acrobat".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult on final hand of round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let is_final_hand = game.plays == 1;
        fn apply(g: &mut Game, _hand: MadeHand, final_hand: bool) {
            if final_hand {
                g.mult = g.mult * 3;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, is_final_hand);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Onyx Agate - +7 Mult for each Club card played

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct OnyxAgate {}

impl Joker for OnyxAgate {
    fn name(&self) -> String {
        "Onyx Agate".to_string()
    }
    fn desc(&self) -> String {
        "+7 Mult for each Club card played".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
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
            g.mult += clubs * 7;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Arrowhead - Played Spade cards give +50 Chips when scored

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Arrowhead {}

impl Joker for Arrowhead {
    fn name(&self) -> String {
        "Arrowhead".to_string()
    }
    fn desc(&self) -> String {
        "Played Spade cards give +50 Chips when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let spades = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Spade)
                .count();
            g.chips += spades * 50;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// RARE JOKERS

// Joker: The Duo - X2 Mult if played hand contains a Pair

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bloodstone {}

impl Joker for Bloodstone {
    fn name(&self) -> String {
        "Bloodstone".to_string()
    }
    fn desc(&self) -> String {
        "1 in 2 chance for Hearts to give X1.5 Mult when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use rand::Rng;
        fn apply(g: &mut Game, hand: MadeHand) {
            let hearts_count = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Heart)
                .count();

            for _ in 0..hearts_count {
                if rand::thread_rng().gen_bool(0.5) {
                    g.mult = (g.mult as f32 * 1.5) as usize;
                }
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Rough Gem - Played Diamond cards earn $1 when scored

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct RoughGem {}

impl Joker for RoughGem {
    fn name(&self) -> String {
        "Rough Gem".to_string()
    }
    fn desc(&self) -> String {
        "Played Diamond cards earn $1 when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let diamonds = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Diamond)
                .count();
            g.money += diamonds;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Flash Card - Gains +2 Mult per reroll in shop
// Note: Requires tracking shop rerolls

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FlashCard {}

impl Joker for FlashCard {
    fn name(&self) -> String {
        "Flash Card".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult per reroll in shop".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Count rerolls from shop
        let reroll_count = game.shop.rerolls_this_round;
        fn apply(g: &mut Game, _hand: MadeHand, rerolls: usize) {
            g.mult += rerolls * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, reroll_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Stone Joker - Gains +25 Chips for each Stone Card in full deck

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct StoneJoker {}

impl Joker for StoneJoker {
    fn name(&self) -> String {
        "Stone Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains +25 Chips for each Stone Card in full deck".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Enhancement;
        let stone_count = game
            .deck
            .cards()
            .iter()
            .filter(|c| c.enhancement == Some(Enhancement::Stone))
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, stones: usize) {
            g.chips += stones * 25;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, stone_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Bull - +2 Chips for each $1 you have

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bull {}

impl Joker for Bull {
    fn name(&self) -> String {
        "Bull".to_string()
    }
    fn desc(&self) -> String {
        "+2 Chips for each $1 you have".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Chips]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let money = game.money;
        fn apply(g: &mut Game, _hand: MadeHand, money: usize) {
            g.chips += money * 2;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, money);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Erosion - +4 Mult for each card below 52 in full deck

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Erosion {}

impl Joker for Erosion {
    fn name(&self) -> String {
        "Erosion".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult for each card below 52 in full deck".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let cards_below = 52_usize.saturating_sub(game.deck.cards().len());
        fn apply(g: &mut Game, _hand: MadeHand, missing: usize) {
            g.mult += missing * 4;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, cards_below);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: The Family - X4 Mult if played hand contains Four of a Kind

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FourFingers {}
impl Joker for FourFingers {
    fn name(&self) -> String {
        "Four Fingers".to_string()
    }
    fn desc(&self) -> String {
        "All Flushes and Straights can be made with 4 cards".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in hand detection logic
        vec![]
    }
}

// Joker: Mime - Retrigger all card held in hand abilities

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Mime {}
impl Joker for Mime {
    fn name(&self) -> String {
        "Mime".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger all card held in hand abilities".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Retrigger]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Complex - would need to retrigger cards in hand (not played cards)
        vec![]
    }
}

// Joker: Marble Joker - Adds one Stone card to deck when Blind selected

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MarbleJoker {}
impl Joker for MarbleJoker {
    fn name(&self) -> String {
        "Marble Joker".to_string()
    }
    fn desc(&self) -> String {
        "Adds one Stone card to deck when Blind selected".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnBlindSelect: Add one Stone card to deck (modify random card to Stone enhancement)
        fn on_blind_select(g: &mut Game) {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();

            // Get all cards in deck without Stone enhancement
            let non_stone_cards: Vec<usize> = g.deck.cards().iter()
                .filter(|c| c.enhancement != Some(crate::card::Enhancement::Stone))
                .map(|c| c.id)
                .collect();

            // Pick a random card and convert it to Stone
            if let Some(&card_id) = non_stone_cards.choose(&mut rng) {
                g.deck.modify_card(card_id, |c| {
                    c.enhancement = Some(crate::card::Enhancement::Stone);
                });
            }
        }

        vec![Effects::OnBlindSelect(Arc::new(Mutex::new(on_blind_select)))]
    }
}

// Joker: Steel Joker - Gains X0.2 Mult for each Steel Card in full deck

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SteelJoker {}
impl Joker for SteelJoker {
    fn name(&self) -> String {
        "Steel Joker".to_string()
    }
    fn desc(&self) -> String {
        "Gains X0.2 Mult for each Steel Card in full deck".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        use crate::card::Edition;
        let steel_count = game
            .deck
            .cards()
            .iter()
            .filter(|c| c.edition == Edition::Foil)
            .count();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            // X0.2 per steel card = multiply by (1.0 + 0.2 * count)
            let multiplier = 1.0 + (0.2 * count as f32);
            g.mult = (g.mult as f32 * multiplier) as usize;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, steel_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Pareidolia - All cards considered face cards

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Pareidolia {}
impl Joker for Pareidolia {
    fn name(&self) -> String {
        "Pareidolia".to_string()
    }
    fn desc(&self) -> String {
        "All cards considered face cards".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in card.is_face() logic
        vec![]
    }
}

// Joker: Blackboard - X3 Mult if all cards held in hand are Spades or Clubs

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Blackboard {}
impl Joker for Blackboard {
    fn name(&self) -> String {
        "Blackboard".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if all cards held in hand are Spades or Clubs".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            // Calculate at score time, not registration time!
            let all_black = g.hand.iter().all(|c| c.suit == Suit::Spade || c.suit == Suit::Club);
            let mult_multiplier = if all_black { 3 } else { 1 };
            g.mult = g.mult * mult_multiplier;
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Smeared Joker - Hearts and Diamonds count as same suit; Spades and Clubs count as same suit

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SmearedJoker {}
impl Joker for SmearedJoker {
    fn name(&self) -> String {
        "Smeared Joker".to_string()
    }
    fn desc(&self) -> String {
        "Hearts and Diamonds count as same suit; Spades and Clubs count as same suit".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        // Passive effect - would need to be handled in flush detection logic
        vec![]
    }
}

// Joker: Flower Pot - X3 Mult if hand contains Diamond, Club, Heart, and Spade cards

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct FlowerPot {}
impl Joker for FlowerPot {
    fn name(&self) -> String {
        "Flower Pot".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if hand contains Diamond, Club, Heart, and Spade cards".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Suit;
        fn apply(g: &mut Game, hand: MadeHand) {
            // Check all played cards, not just the made hand
            let has_diamond = hand.all.iter().any(|c| c.suit == Suit::Diamond);
            let has_club = hand.all.iter().any(|c| c.suit == Suit::Club);
            let has_heart = hand.all.iter().any(|c| c.suit == Suit::Heart);
            let has_spade = hand.all.iter().any(|c| c.suit == Suit::Spade);

            if has_diamond && has_club && has_heart && has_spade {
                g.mult = g.mult * 3;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Seeing Double - X2 Mult if played hand has Club card and any other suit card

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SeeingDouble {}
impl Joker for SeeingDouble {
    fn name(&self) -> String {
        "Seeing Double".to_string()
    }
    fn desc(&self) -> String {
        "X2 Mult if played hand has Club card and any other suit card".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::card::Suit;
        fn apply(g: &mut Game, hand: MadeHand) {
            // Check all played cards, not just the made hand
            let has_club = hand.all.iter().any(|c| c.suit == Suit::Club);
            let has_other = hand.all.iter().any(|c| c.suit != Suit::Club);

            if has_club && has_other {
                g.mult = g.mult * 2;
            }
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

// Joker: Baron - Each King held in hand gives X1.5 Mult

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct JokerStencil {}
impl Joker for JokerStencil {
    fn name(&self) -> String {
        "Joker Stencil".to_string()
    }
    fn desc(&self) -> String {
        "X1 Mult for each empty Joker slot (counts itself as empty)".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Max joker slots is typically 5, count empty slots
        let max_slots: usize = 5;
        let current_jokers = game.jokers.len();
        let empty_slots = max_slots.saturating_sub(current_jokers).saturating_add(1); // +1 because it counts itself as empty
        fn apply(g: &mut Game, _hand: MadeHand, slots: usize) {
            // X1 per slot means multiply by (1 * slots), which is just slots
            g.mult = g.mult * slots;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, empty_slots);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Showman - +4 Mult for Joker, Tarot, Planet, or Spectral cards remaining in consumable slots

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Showman {}
impl Joker for Showman {
    fn name(&self) -> String {
        "Showman".to_string()
    }
    fn desc(&self) -> String {
        "Gains +4 Mult for Joker, Tarot, Planet, or Spectral cards remaining in consumable slots".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let consumable_count = game.consumables.len();
        fn apply(g: &mut Game, _hand: MadeHand, count: usize) {
            g.mult += count * 4;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, consumable_count);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Bootstraps - Gains +2 Mult for every $5 you have

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Bootstraps {}
impl Joker for Bootstraps {
    fn name(&self) -> String {
        "Bootstraps".to_string()
    }
    fn desc(&self) -> String {
        "Gains +2 Mult for every $5 you have".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let mult_bonus = (game.money / 5) * 2;
        fn apply(g: &mut Game, _hand: MadeHand, bonus: usize) {
            g.mult += bonus;
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, mult_bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Cloud9 - Earn $1 for each 9 in full deck at end of round

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Cloud9 {}
impl Joker for Cloud9 {
    fn name(&self) -> String {
        "Cloud 9".to_string()
    }
    fn desc(&self) -> String {
        "Earn $1 for each 9 in full deck at end of round".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundEnd: Earn $1 for each 9 in full deck
        fn on_round_end(g: &mut Game) {
            use crate::card::Value;
            let nine_count = g.deck.cards().iter().filter(|c| c.value == Value::Nine).count();
            g.money += nine_count;
        }

        vec![Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))]
    }
}

// Joker: WeeJoker - Gains +8 Chips when each played 2 is scored

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CardSharp {}

impl Joker for CardSharp {
    fn name(&self) -> String {
        "Card Sharp".to_string()
    }
    fn desc(&self) -> String {
        "X3 Mult if poker hand has already been played this round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        let hands_played = game.round_state.hands_played_this_round.clone();

        fn apply(g: &mut Game, hand: MadeHand, played: std::collections::HashSet<HandRank>) {
            if played.contains(&hand.rank) {
                g.mult = g.mult * 3;
            }
        }
        let apply_closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, hands_played.clone());
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(apply_closure)))]
    }
}

// Joker: Chicot - Disables effect of every Boss Blind

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Shortcut {}
impl Joker for Shortcut {
    fn name(&self) -> String {
        "Shortcut".to_string()
    }
    fn desc(&self) -> String {
        "Allows Straights to be made with gaps of 1 rank".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: This joker enables game.modifiers.gap_straights
        // The hand detection already supports gap straights (implemented in previous session)
        // Need to add OnBuy effect to enable the modifier
        vec![]
    }
}

// Joker: Troubadour - +2 hand size; -1 hand per round

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Troubadour {
    pub hands_remaining: i32,
}
impl Default for Troubadour {
    fn default() -> Self {
        Self { hands_remaining: 5 } // Starts with +5 hands (total +2 after -1 per round for 5 rounds)
    }
}
impl Joker for Troubadour {
    fn name(&self) -> String {
        "Troubadour".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} hand size", self.hands_remaining.max(0))
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundBegin: Calculate and apply hand_size bonus, then decrement counter
        fn on_round_begin(g: &mut Game) {
            // Find all Troubadour jokers and apply their bonuses
            let mut total_bonus = 0;
            for joker in g.jokers.iter_mut() {
                if let Jokers::Troubadour(troub) = joker {
                    if troub.hands_remaining > 0 {
                        total_bonus += troub.hands_remaining as usize;
                        troub.hands_remaining -= 1;
                    }
                }
            }
            // Apply the bonus to hand_size (this is additive to base 8)
            if total_bonus > 0 {
                g.hand_size += total_bonus;
            }
        }

        vec![Effects::OnRoundBegin(Arc::new(Mutex::new(on_round_begin)))]
    }
}

// Joker: Turtle Bean - Gains +5 hand size; decreases by 1 per round

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TurtleBean {
    pub hand_size_bonus: i32,
}
impl Default for TurtleBean {
    fn default() -> Self {
        Self { hand_size_bonus: 5 }
    }
}
impl Joker for TurtleBean {
    fn name(&self) -> String {
        "Turtle Bean".to_string()
    }
    fn desc(&self) -> String {
        format!("+{} hand size", self.hand_size_bonus.max(0))
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnRoundBegin: Calculate and apply hand_size bonus, then decrement counter
        fn on_round_begin(g: &mut Game) {
            // Find all TurtleBean jokers and apply their bonuses
            let mut total_bonus = 0;
            for joker in g.jokers.iter_mut() {
                if let Jokers::TurtleBean(bean) = joker {
                    if bean.hand_size_bonus > 0 {
                        total_bonus += bean.hand_size_bonus as usize;
                        bean.hand_size_bonus -= 1;
                    }
                }
            }
            // Apply the bonus to hand_size (this is additive to base 8)
            if total_bonus > 0 {
                g.hand_size += total_bonus;
            }
        }

        vec![Effects::OnRoundBegin(Arc::new(Mutex::new(on_round_begin)))]
    }
}

// Joker: Trading Card - If first discard contains 1 card, destroy it and earn $3

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TradingCard {}
impl Joker for TradingCard {
    fn name(&self) -> String {
        "Trading Card".to_string()
    }
    fn desc(&self) -> String {
        "If first discard contains 1 card, destroy it and earn $3".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        fn on_discard(g: &mut Game, hand: crate::hand::MadeHand) {
            // Check if this is the first discard of the blind and exactly 1 card was discarded
            // Use hand.all which contains ALL selected cards, not just the ones used in best hand
            if g.discards_this_blind == 1 && hand.all.len() == 1 {
                // Get the discarded card
                let card = hand.all[0];

                // Remove from discarded pile and add to destroyed pile
                if let Some(pos) = g.discarded.iter().rposition(|c| c == &card) {
                    g.discarded.remove(pos);
                    g.destroyed.push(card);
                    g.money += 3;
                }
            }
        }

        vec![Effects::OnDiscard(Arc::new(Mutex::new(on_discard)))]
    }
}

// Joker: Matador - Earn $8 if played hand triggers Boss Blind ability

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Matador {}
impl Joker for Matador {
    fn name(&self) -> String {
        "Matador".to_string()
    }
    fn desc(&self) -> String {
        "Earn $8 if played hand triggers Boss Blind ability".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need boss blind trigger detection system
        // TODO: Need OnBossBlindTrigger effect type
        vec![]
    }
}

// Joker: To the Moon - Earn $1 per $5 in excess of $20; excess lowers by $5 after round

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct ToTheMoon {
    pub excess_money: usize,
}
impl Default for ToTheMoon {
    fn default() -> Self {
        Self { excess_money: 0 }
    }
}
impl Joker for ToTheMoon {
    fn name(&self) -> String {
        "To the Moon".to_string()
    }
    fn desc(&self) -> String {
        format!("Earn $1 per $5 in excess of $20 (current excess: ${})", self.excess_money)
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Earn money based on excess over $20
        fn on_score(g: &mut Game, _hand: MadeHand) {
            let current_excess = if g.money > 20 {
                g.money.saturating_sub(20)
            } else {
                0
            };

            // Earn $1 per $5 of excess
            let money_earned = current_excess / 5;
            g.money += money_earned;
        }

        // OnRoundEnd: Decrease the internal excess tracker
        fn on_round_end(g: &mut Game) {
            // Find this joker and update its state
            for joker in g.jokers.iter_mut() {
                if let Jokers::ToTheMoon(ttm) = joker {
                    // Calculate current excess
                    let current_excess = if g.money > 20 {
                        g.money.saturating_sub(20)
                    } else {
                        0
                    };

                    // Update internal excess tracker and decrease by $5
                    ttm.excess_money = current_excess.saturating_sub(5);
                    break;
                }
            }
        }

        vec![
            Effects::OnScore(Arc::new(Mutex::new(on_score))),
            Effects::OnRoundEnd(Arc::new(Mutex::new(on_round_end)))
        ]
    }
}

// Joker: Vagabond - Create Tarot card if hand played with $4 or less

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Seance {}
impl Joker for Seance {
    fn name(&self) -> String {
        "Séance".to_string()
    }
    fn desc(&self) -> String {
        "If poker hand is Straight Flush, create random Planet card".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Create Planet if hand is Straight Flush
        fn on_score(g: &mut Game, hand: MadeHand) {
            if hand.rank == crate::rank::HandRank::StraightFlush {
                g.create_random_planet();
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}

// Joker: Mr. Bones - Prevents death if chips scored >= 25% of required chips; self-destructs

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MrBones {}
impl Joker for MrBones {
    fn name(&self) -> String {
        "Mr. Bones".to_string()
    }
    fn desc(&self) -> String {
        "Prevents death if chips scored >= 25% of required chips; self-destructs".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need death prevention system
        // TODO: Need OnBlindEnd effect to check score vs requirement
        // TODO: Need joker self-destruction system
        vec![]
    }
}

// Joker: Luchador - Sell this to disable current Boss Blind

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Luchador {}
impl Joker for Luchador {
    fn name(&self) -> String {
        "Luchador".to_string()
    }
    fn desc(&self) -> String {
        "Sell this to disable current Boss Blind".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::stage::{Blind, Stage};
        use std::sync::{Arc, Mutex};

        // OnSell: Disable the current Boss Blind modifier
        let on_sell = Arc::new(Mutex::new(|g: &mut Game| {
            // Check if we're in a Boss Blind
            if let Stage::Blind(Blind::Boss, ref mut modifier) = g.stage {
                // Disable the boss modifier
                *modifier = None;
            }
        }));

        vec![Effects::OnSell(on_sell)]
    }
}

// Joker: Diet Cola - Sell this to create free Double Tag

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct DietCola {}
impl Joker for DietCola {
    fn name(&self) -> String {
        "Diet Cola".to_string()
    }
    fn desc(&self) -> String {
        "Sell this to create free Double Tag".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::tag::Tag;
        use std::sync::{Arc, Mutex};

        // OnSell: Create a free Double Tag
        let on_sell = Arc::new(Mutex::new(|g: &mut Game| {
            g.add_tag(Tag::Double);
        }));

        vec![Effects::OnSell(on_sell)]
    }
}

// Joker: Ceremonial Dagger - When Blind selected, destroys Joker to the right; adds double sell value to Mult

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct CeremonialDagger {
    pub bonus_mult: usize,
}
impl Default for CeremonialDagger {
    fn default() -> Self {
        Self { bonus_mult: 0 }
    }
}
impl Joker for CeremonialDagger {
    fn name(&self) -> String {
        "Ceremonial Dagger".to_string()
    }
    fn desc(&self) -> String {
        format!("When Blind selected, destroys Joker to the right; +{} Mult", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
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
        // TODO: Need OnBlindSelect effect to destroy joker to the right
        // TODO: Need joker destruction system
    }
}

// Joker: Cartomancer - Create Tarot card when Blind selected; requires empty consumable slot

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Cartomancer {}
impl Joker for Cartomancer {
    fn name(&self) -> String {
        "Cartomancer".to_string()
    }
    fn desc(&self) -> String {
        "Create Tarot card when Blind selected; requires empty consumable slot".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnBlindSelect: Create Tarot if there's room
        fn on_blind_select(g: &mut Game) {
            // Check if there's room for another consumable (max 2)
            if g.consumables.len() < 2 {
                g.create_random_tarot();
            }
        }

        vec![Effects::OnBlindSelect(Arc::new(Mutex::new(on_blind_select)))]
    }
}

// Joker: Astronomer - All Planet cards and Celestial Packs in shop are free

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Astronomer {}
impl Joker for Astronomer {
    fn name(&self) -> String {
        "Astronomer".to_string()
    }
    fn desc(&self) -> String {
        "All Planet cards and Celestial Packs in shop are free".to_string()
    }
    fn cost(&self) -> usize {
        8
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need shop price modification system
        // Passive effect - would be checked in shop purchase logic
        vec![]
    }
}

// Joker: Vampire - Gains X0.2 Mult per Enhanced card played; removes enhancement

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Vampire {
    pub bonus_mult: f32,
}
impl Eq for Vampire {}
impl std::hash::Hash for Vampire {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash as u32 bits for deterministic hashing
        self.bonus_mult.to_bits().hash(state);
    }
}
impl Default for Vampire {
    fn default() -> Self {
        Self { bonus_mult: 1.0 }
    }
}
impl Joker for Vampire {
    fn name(&self) -> String {
        "Vampire".to_string()
    }
    fn desc(&self) -> String {
        format!("X{:.1} Mult; gains X0.2 per Enhanced card played", self.bonus_mult)
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultMult]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        let multiplier = self.bonus_mult;

        // OnScore: Apply X mult multiplier
        fn on_score(g: &mut Game, _hand: MadeHand, mult: f32) {
            g.mult = (g.mult as f32 * mult) as usize;
        }
        let on_score_closure = move |g: &mut Game, hand: MadeHand| {
            on_score(g, hand, multiplier);
        };

        // OnPlay: Detect enhanced cards, increment bonus, remove enhancements
        fn on_play(g: &mut Game, hand: MadeHand) {
            // Get IDs of enhanced cards in the hand
            let enhanced_card_ids: Vec<usize> = hand.all.iter()
                .filter(|c| c.enhancement.is_some())
                .map(|c| c.id)
                .collect();

            let enhanced_count = enhanced_card_ids.len();

            // Increment Vampire's bonus_mult
            if enhanced_count > 0 {
                for joker in g.jokers.iter_mut() {
                    if let Jokers::Vampire(vampire) = joker {
                        vampire.bonus_mult += enhanced_count as f32 * 0.2;
                        break;
                    }
                }

                // Remove enhancements from these cards
                for card_id in enhanced_card_ids {
                    // Try deck
                    if g.deck.modify_card(card_id, |c| {
                        c.enhancement = None;
                    }) {
                        continue;
                    }
                    // Try available
                    if g.available.modify_card(card_id, |c| {
                        c.enhancement = None;
                    }) {
                        continue;
                    }
                    // Try discarded
                    if let Some(card) = g.discarded.iter_mut().find(|c| c.id == card_id) {
                        card.enhancement = None;
                    }
                }
            }
        }

        vec![
            Effects::OnScore(Arc::new(Mutex::new(on_score_closure))),
            Effects::OnPlay(Arc::new(Mutex::new(on_play)))
        ]
    }
}

// Joker: Driver's License - X3 Mult if full deck has at least 16 Enhanced cards

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Hack {}
impl Joker for Hack {
    fn name(&self) -> String {
        "Hack".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger each played 2, 3, 4, or 5".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need RETRIGGER SYSTEM
        // TODO: This is a major feature needed by multiple jokers
        vec![]
    }
}

// Joker: Dusk - Retrigger all played cards in final hand of round

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Dusk {}
impl Joker for Dusk {
    fn name(&self) -> String {
        "Dusk".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger all played cards in final hand of round".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need RETRIGGER SYSTEM
        // TODO: Need to detect final hand of round
        vec![]
    }
}

// Joker: Sock and Buskin - Retrigger all played face cards

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct SockAndBuskin {}
impl Joker for SockAndBuskin {
    fn name(&self) -> String {
        "Sock and Buskin".to_string()
    }
    fn desc(&self) -> String {
        "Retrigger all played face cards".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need RETRIGGER SYSTEM
        vec![]
    }
}

// Joker: Seltzer - Retrigger all played cards for next 10 hands

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Seltzer {
    pub hands_remaining: usize,
}
impl Default for Seltzer {
    fn default() -> Self {
        Self { hands_remaining: 10 }
    }
}
impl Joker for Seltzer {
    fn name(&self) -> String {
        "Seltzer".to_string()
    }
    fn desc(&self) -> String {
        format!("Retrigger all played cards for next {} hands", self.hands_remaining)
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnPlay: Decrement hands_remaining counter after each hand played
        fn on_play(g: &mut Game, _hand: MadeHand) {
            for joker in g.jokers.iter_mut() {
                if let Jokers::Seltzer(seltzer) = joker {
                    if seltzer.hands_remaining > 0 {
                        seltzer.hands_remaining -= 1;
                    }
                }
            }
        }

        vec![Effects::OnPlay(Arc::new(Mutex::new(on_play)))]
    }
}

// Joker: Midas Mask - All face cards become Gold cards when scored

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct MidasMask {}
impl Joker for MidasMask {
    fn name(&self) -> String {
        "Midas Mask".to_string()
    }
    fn desc(&self) -> String {
        "All face cards become Gold cards when scored".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        // OnScore: Convert all face cards to Gold enhancement
        fn on_score(g: &mut Game, hand: MadeHand) {
            // Get face card IDs from the hand
            let face_card_ids: Vec<usize> = hand.all.iter()
                .filter(|c| c.is_face())
                .map(|c| c.id)
                .collect();

            // Modify each face card to have Gold enhancement
            for card_id in face_card_ids {
                // Try deck first
                if g.deck.modify_card(card_id, |c| {
                    c.enhancement = Some(crate::card::Enhancement::Gold);
                }) {
                    continue;
                }
                // Try available
                if g.available.modify_card(card_id, |c| {
                    c.enhancement = Some(crate::card::Enhancement::Gold);
                }) {
                    continue;
                }
                // Try discarded
                if let Some(card) = g.discarded.iter_mut().find(|c| c.id == card_id) {
                    card.enhancement = Some(crate::card::Enhancement::Gold);
                }
            }
        }

        vec![Effects::OnScore(Arc::new(Mutex::new(on_score)))]
    }
}

// Joker: Madness - When Small or Big Blind selected, destroy random Joker and create 2 free Jokers

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Madness {}
impl Joker for Madness {
    fn name(&self) -> String {
        "Madness".to_string()
    }
    fn desc(&self) -> String {
        "When Small or Big Blind selected, destroy random Joker and create 2 free Jokers".to_string()
    }
    fn cost(&self) -> usize {
        7
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // TODO: Need OnBlindSelect effect type
        // TODO: Need joker destruction system
        // TODO: Need joker creation system
        vec![]
    }
}

// ============================================================================
// FINAL 3 JOKERS - Completing the full 150
// ============================================================================

// Joker: Certificate - When round begins, add random playing card with random seal to hand

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Certificate {}
impl Joker for Certificate {
    fn name(&self) -> String {
        "Certificate".to_string()
    }
    fn desc(&self) -> String {
        "When round begins, add random playing card with random seal to hand".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Effect]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        use crate::effect::Effects;
        use std::sync::{Arc, Mutex};

        fn on_round_begin(g: &mut Game) {
            use crate::card::{Card, Seal, Suit, Value};
            use rand::seq::SliceRandom;

            // Generate a random playing card
            let all_values = [
                Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five,
                Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten,
                Value::Jack, Value::Queen, Value::King
            ];
            let all_suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
            let all_seals = [Seal::Gold, Seal::Red, Seal::Blue, Seal::Purple];

            let value = all_values.choose(&mut rand::thread_rng()).unwrap();
            let suit = all_suits.choose(&mut rand::thread_rng()).unwrap();
            let seal = all_seals.choose(&mut rand::thread_rng()).unwrap();

            let mut card = Card::new(*value, *suit);
            card.seal = Some(*seal);

            // Add the card to the player's hand
            g.hand.push(card);
        }

        vec![Effects::OnRoundBegin(Arc::new(Mutex::new(on_round_begin)))]
    }
}

// Joker: Gift Card - Add $1 of sell value to every Joker and Consumable card at end of round

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GiftCard {}
impl Joker for GiftCard {
    fn name(&self) -> String {
        "Gift Card".to_string()
    }
    fn desc(&self) -> String {
        "Add $1 of sell value to every Joker and Consumable card at end of round".to_string()
    }
    fn cost(&self) -> usize {
        6
    }
    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::Economy]
    }
    fn effects(&self, _game: &Game) -> Vec<Effects> {
        // The actual sell value incrementing happens in Game::trigger_round_end()
        // because it needs to mutate all joker structs
        // This is just a marker effect
        vec![]
    }
}
