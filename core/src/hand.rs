use indexmap::IndexMap;
use itertools::Itertools;
use pyo3::pyclass;
use std::fmt;

use crate::card::Card;
use crate::card::Suit;
use crate::card::Value;
use crate::error::PlayHandError;
use crate::game::GameModifiers;
use crate::rank::HandRank;

/// Context object for hand detection, carrying modifiers and other game state
pub struct HandContext<'a> {
    pub modifiers: &'a GameModifiers,
}

impl<'a> HandContext<'a> {
    /// Create a default context with no modifiers enabled
    pub fn default_context() -> HandContext<'static> {
        static DEFAULT_MODS: GameModifiers = GameModifiers {
            four_card_straights: false,
            four_card_flushes: false,
            all_cards_are_faces: false,
            smeared_suits: false,
            gap_straights: false,
            all_cards_score: false,
            hand_size_bonus: 0,
            discard_bonus: 0,
            min_money: 0,
        };
        HandContext {
            modifiers: &DEFAULT_MODS,
        }
    }
}

// Hand, SelectHand and MadeHand are all representations of a collection of Card,
// just at different phases in the cycle of selecting, executing and scoring cards.
// Hand represents all drawn cards, cards available for action (play/discard).
// SelectHand represents (up to 5) cards user selects from hand for action.
// MadeHand represents actual poker hand level and associated cards from a selected hand.

// Hand represents all drawn cards, cards available for action (play/discard)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hand(Vec<Card>);

// MadeHand represents actual poker hand level and associated cards from a selected hand.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MadeHand {
    pub hand: SelectHand,
    pub rank: HandRank,
    pub all: Vec<Card>,
}

// SelectHand represents (up to 5) cards user selects from hand for action
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SelectHand(Vec<Card>);

impl SelectHand {
    pub fn new(cards: Vec<Card>) -> Self {
        // Filter out face-down cards - they don't participate in hand detection
        // Face-down cards are invisible for The Ox and The Wheel boss modifiers
        let visible_cards: Vec<Card> = cards.into_iter()
            .filter(|c| c.is_visible())
            .collect();
        Self(visible_cards)
    }
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    // Get all values in a hand. Sorted lowest to highest.
    fn values(&self) -> Vec<Value> {
        self.0.iter().map(|x| x.value).sorted().collect()
    }
    pub(crate) fn cards(&self) -> Vec<Card> {
        return self.0.clone();
    }

    // Get map of each value with corresponding cards.
    // For example, Ks, Ah, Jh, Jc, Jd -> {A: [Ah], K: [Ks], J: [Jh, Jc: Jd]}
    fn values_freq(&self) -> IndexMap<Value, Vec<Card>> {
        let mut counts: IndexMap<Value, Vec<Card>> = IndexMap::new();
        for card in self.0.clone() {
            if let Some(cards) = counts.get(&card.value) {
                let mut copy = cards.clone();
                copy.push(card);
                counts.insert(card.value, copy);
            } else {
                counts.insert(card.value, vec![card]);
            }
        }
        // Return sorted by value
        return counts
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .collect();
    }

    // Get all suits in a hand
    pub(crate) fn suits(&self) -> Vec<Suit> {
        self.0.iter().map(|x| x.suit).sorted().collect()
    }

    // Get map of each suit with corresponding cards.
    // For example, Ks, Ah, Jh, Jc, Jd -> {h: [Jh, Ah], s: [Ks], c: [Jc], d: [Jd]}
    pub(crate) fn suits_freq(&self) -> IndexMap<Suit, Vec<Card>> {
        let mut counts: IndexMap<Suit, Vec<Card>> = IndexMap::new();
        for card in self.0.clone() {
            if let Some(cards) = counts.get(&card.suit) {
                let mut copy = cards.clone();
                copy.push(card);
                counts.insert(card.suit, copy);
            } else {
                counts.insert(card.suit, vec![card]);
            }
        }
        // Return sorted by suit
        return counts
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .collect();
    }

    /// Can play any number of cards, it is our responsibility
    /// to determine the best hand. Higher tier hands take precedence
    /// over lower tier hands regardless of their level or scoring.
    /// For example, if hand is Kd Kd Kd Kd 2d, best hand will be a
    // Four of a Kind and never a Flush.
    //
    // Hand ranking:
    // FlushFive
    // FlushHouse
    // FiveOfAKind
    // RoyalFlush
    // StraightFlush
    // FourOfAKind
    // FullHouse
    // Flush
    // Straight
    // ThreeOfAKind
    // TwoPair
    // OnePair
    // HighCard
    //
    // This is the new API that accepts a HandContext for modifier support
    pub(crate) fn best_hand_with_context(
        &self,
        context: &HandContext,
    ) -> Result<MadeHand, PlayHandError> {
        if self.len() == 0 {
            return Err(PlayHandError::NoCards);
        }
        if self.len() > 5 {
            return Err(PlayHandError::TooManyCards);
        }

        // We start trying to evaluate best hands first, so we
        // can return best hand right when we find it.
        if let Some(hand) = self.is_flush_five(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::FlushFive,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_flush_house(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::FlushHouse,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_five_of_kind() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::FiveOfAKind,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_royal_flush(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::RoyalFlush,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_straight_flush(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::StraightFlush,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_four_of_kind() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::FourOfAKind,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_fullhouse() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::FullHouse,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_flush(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::Flush,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_straight(context) {
            return Ok(MadeHand {
                hand,
                rank: HandRank::Straight,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_three_of_kind() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::ThreeOfAKind,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_two_pair() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::TwoPair,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_pair() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::OnePair,
                all: self.cards(),
            });
        }
        if let Some(hand) = self.is_highcard() {
            return Ok(MadeHand {
                hand,
                rank: HandRank::HighCard,
                all: self.cards(),
            });
        }
        // We didn't match any known hand, oops...
        return Err(PlayHandError::UnknownHand);
    }

    /// Backward-compatible wrapper that uses default context
    pub(crate) fn best_hand(&self) -> Result<MadeHand, PlayHandError> {
        self.best_hand_with_context(&HandContext::default_context())
    }

    pub(crate) fn is_highcard(&self) -> Option<SelectHand> {
        if self.len() < 1 {
            return None;
        }
        if let Some((_value, cards)) = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 1)
        {
            return Some(SelectHand::new(cards));
        } else {
            return None;
        }
    }

    pub(crate) fn is_pair(&self) -> Option<SelectHand> {
        if self.len() < 2 {
            return None;
        }
        if let Some((_value, cards)) = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 2)
        {
            return Some(SelectHand::new(cards));
        } else {
            return None;
        }
    }

    pub(crate) fn is_two_pair(&self) -> Option<SelectHand> {
        if self.len() < 4 {
            return None;
        }

        // First find first pair
        let first = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 2);
        if first.is_none() {
            return None;
        }
        let first_val = first
            .as_ref()
            .unwrap()
            .1
            .first()
            .expect("values freq has empty Vec<card>")
            .value;

        // Next find second pair that isn't same value as first pair
        let second = self
            .values_freq()
            .into_iter()
            .find(|(key, val)| *key != first_val && val.len() >= 2);
        if second.is_none() {
            return None;
        }

        // Combine first and second pair
        let mut cards: Vec<Card> = Vec::new();
        cards.extend(first.unwrap().1);
        cards.extend(second.unwrap().1);
        return Some(SelectHand::new(cards));
    }

    pub(crate) fn is_three_of_kind(&self) -> Option<SelectHand> {
        if self.len() < 3 {
            return None;
        }
        if let Some((_value, cards)) = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 3)
        {
            return Some(SelectHand::new(cards));
        } else {
            return None;
        }
    }

    pub(crate) fn is_straight(&self, context: &HandContext) -> Option<SelectHand> {
        let min_cards = if context.modifiers.four_card_straights {
            4
        } else {
            5
        };

        if self.len() < min_cards {
            return None;
        }

        let values = self.values();

        // Check for consecutive sequences of the required length
        // Try from longest to shortest (5 down to min_cards)
        for window_size in (min_cards..=self.len().min(5)).rev() {
            // Check all possible windows of this size
            for i in 0..=(values.len().saturating_sub(window_size)) {
                let window = &values[i..i + window_size];

                // Check if this window is consecutive
                if window.windows(2).all(|v| (v[1] as u16 - v[0] as u16) == 1) {
                    // Build the straight from these values
                    let straight_cards: Vec<Card> = window
                        .iter()
                        .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                        .copied()
                        .collect();
                    return Some(SelectHand::new(straight_cards));
                }

                // Check for gap straights (Shortcut joker) - allows one gap in the sequence
                if context.modifiers.gap_straights {
                    let mut gap_count = 0;
                    let mut is_gap_straight = true;

                    for pair in window.windows(2) {
                        let diff = pair[1] as u16 - pair[0] as u16;
                        if diff == 1 {
                            // Consecutive - good
                            continue;
                        } else if diff == 2 && gap_count == 0 {
                            // One gap allowed
                            gap_count += 1;
                        } else {
                            // Too many gaps or gap too large
                            is_gap_straight = false;
                            break;
                        }
                    }

                    if is_gap_straight && gap_count == 1 {
                        // Valid gap straight - build the straight from these values
                        let straight_cards: Vec<Card> = window
                            .iter()
                            .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                            .copied()
                            .collect();
                        return Some(SelectHand::new(straight_cards));
                    }
                }
            }
        }

        // Special case for low ace straights (A, 2, 3, 4, 5) or (A, 2, 3, 4) with four_card modifier
        if values.contains(&Value::Ace) && values.contains(&Value::Two) {
            // Check if we have the required consecutive values starting from 2
            let needed_values: Vec<Value> = if min_cards == 4 {
                vec![Value::Two, Value::Three, Value::Four]
            } else {
                vec![Value::Two, Value::Three, Value::Four, Value::Five]
            };

            if needed_values.iter().all(|v| values.contains(v)) {
                let mut straight_cards: Vec<Card> = needed_values
                    .iter()
                    .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                    .copied()
                    .collect();

                // Add the ace
                if let Some(ace) = self.0.iter().find(|c| c.value == Value::Ace) {
                    straight_cards.push(*ace);
                }

                return Some(SelectHand::new(straight_cards));
            }

            // Gap straight with low ace (A, 2, 3, 5) or (A, 2, 4, 5)
            if context.modifiers.gap_straights {
                if min_cards == 4 {
                    // Check for A,2,3,5 (missing 4) or A,2,4,5 (missing 3)
                    if values.contains(&Value::Five) {
                        if values.contains(&Value::Three) && !values.contains(&Value::Four) {
                            // A,2,3,5 gap straight
                            let gap_values = vec![Value::Two, Value::Three, Value::Five];
                            let mut straight_cards: Vec<Card> = gap_values
                                .iter()
                                .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                                .copied()
                                .collect();
                            if let Some(ace) = self.0.iter().find(|c| c.value == Value::Ace) {
                                straight_cards.push(*ace);
                            }
                            return Some(SelectHand::new(straight_cards));
                        } else if values.contains(&Value::Four) && !values.contains(&Value::Three) {
                            // A,2,4,5 gap straight
                            let gap_values = vec![Value::Two, Value::Four, Value::Five];
                            let mut straight_cards: Vec<Card> = gap_values
                                .iter()
                                .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                                .copied()
                                .collect();
                            if let Some(ace) = self.0.iter().find(|c| c.value == Value::Ace) {
                                straight_cards.push(*ace);
                            }
                            return Some(SelectHand::new(straight_cards));
                        }
                    }
                } else {
                    // 5-card with gap: A,2,3,4,6 or A,2,3,5,6 or A,2,4,5,6
                    if values.contains(&Value::Six) {
                        // Check various gap combinations
                        let gap_combos = vec![
                            vec![Value::Two, Value::Three, Value::Four, Value::Six], // Missing 5
                            vec![Value::Two, Value::Three, Value::Five, Value::Six], // Missing 4
                            vec![Value::Two, Value::Four, Value::Five, Value::Six],  // Missing 3
                        ];

                        for combo in gap_combos {
                            if combo.iter().all(|v| values.contains(v)) {
                                let mut straight_cards: Vec<Card> = combo
                                    .iter()
                                    .filter_map(|v| self.0.iter().find(|c| c.value == *v))
                                    .copied()
                                    .collect();
                                if let Some(ace) = self.0.iter().find(|c| c.value == Value::Ace) {
                                    straight_cards.push(*ace);
                                }
                                return Some(SelectHand::new(straight_cards));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub(crate) fn is_flush(&self, context: &HandContext) -> Option<SelectHand> {
        let min_cards = if context.modifiers.four_card_flushes {
            4
        } else {
            5
        };

        if self.len() < min_cards {
            return None;
        }

        // Check normal suit matching
        if let Some((_suit, cards)) = self
            .suits_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= min_cards)
        {
            // If 4-card flush, take only the best 4 cards
            if context.modifiers.four_card_flushes && cards.len() > 4 {
                let best_4: Vec<Card> = cards
                    .iter()
                    .sorted_by_key(|c| c.value)
                    .rev()
                    .take(4)
                    .copied()
                    .collect();
                return Some(SelectHand::new(best_4));
            }
            return Some(SelectHand::new(cards));
        }

        // Check smeared suits (Hearts/Diamonds count as same, Spades/Clubs count as same)
        if context.modifiers.smeared_suits {
            // Hearts + Diamonds count as same suit
            let red_cards: Vec<Card> = self
                .0
                .iter()
                .filter(|c| c.suit == Suit::Heart || c.suit == Suit::Diamond)
                .copied()
                .collect();

            if red_cards.len() >= min_cards {
                let flush_cards: Vec<Card> = if context.modifiers.four_card_flushes && red_cards.len() > 4 {
                    red_cards
                        .iter()
                        .sorted_by_key(|c| c.value)
                        .rev()
                        .take(4)
                        .copied()
                        .collect()
                } else {
                    red_cards.into_iter().take(min_cards).collect()
                };
                return Some(SelectHand::new(flush_cards));
            }

            // Spades + Clubs count as same suit
            let black_cards: Vec<Card> = self
                .0
                .iter()
                .filter(|c| c.suit == Suit::Spade || c.suit == Suit::Club)
                .copied()
                .collect();

            if black_cards.len() >= min_cards {
                let flush_cards: Vec<Card> = if context.modifiers.four_card_flushes && black_cards.len() > 4 {
                    black_cards
                        .iter()
                        .sorted_by_key(|c| c.value)
                        .rev()
                        .take(4)
                        .copied()
                        .collect()
                } else {
                    black_cards.into_iter().take(min_cards).collect()
                };
                return Some(SelectHand::new(flush_cards));
            }
        }

        None
    }

    pub(crate) fn is_fullhouse(&self) -> Option<SelectHand> {
        if self.len() < 5 {
            return None;
        }

        // First find 3ok
        let three = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 3);
        if three.is_none() {
            return None;
        }
        let three_val = three
            .as_ref()
            .unwrap()
            .1
            .first()
            .expect("values freq has empty Vec<card>")
            .value;

        // Next find 2ok that isn't same value as 3ok
        let two = self
            .values_freq()
            .into_iter()
            .find(|(key, val)| *key != three_val && val.len() >= 2);
        if two.is_none() {
            return None;
        }

        // Combine 3ok and 2ok
        let mut cards: Vec<Card> = Vec::new();
        cards.extend(three.unwrap().1);
        cards.extend(two.unwrap().1);
        return Some(SelectHand::new(cards));
    }

    pub(crate) fn is_four_of_kind(&self) -> Option<SelectHand> {
        if self.len() < 4 {
            return None;
        }
        if let Some((_value, cards)) = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 4)
        {
            return Some(SelectHand::new(cards));
        } else {
            return None;
        }
    }

    pub(crate) fn is_straight_flush(&self, context: &HandContext) -> Option<SelectHand> {
        if self.is_flush(context).is_some() && self.is_straight(context).is_some() {
            return Some(self.clone());
        }
        return None;
    }

    pub(crate) fn is_royal_flush(&self, context: &HandContext) -> Option<SelectHand> {
        if self.is_straight_flush(context).is_some()
            && self.values().into_iter().eq(vec![
                Value::Ten,
                Value::Jack,
                Value::Queen,
                Value::King,
                Value::Ace,
            ])
        {
            return Some(self.clone());
        }
        return None;
    }

    pub(crate) fn is_five_of_kind(&self) -> Option<SelectHand> {
        if self.len() < 5 {
            return None;
        }
        if let Some((_value, cards)) = self
            .values_freq()
            .into_iter()
            .find(|(_key, val)| val.len() >= 5)
        {
            return Some(SelectHand::new(cards));
        } else {
            return None;
        }
    }

    pub(crate) fn is_flush_house(&self, context: &HandContext) -> Option<SelectHand> {
        if self.is_flush(context).is_some() && self.is_fullhouse().is_some() {
            return Some(self.clone());
        }
        return None;
    }

    pub(crate) fn is_flush_five(&self, context: &HandContext) -> Option<SelectHand> {
        if self.is_flush(context).is_some() && self.is_five_of_kind().is_some() {
            return Some(self.clone());
        }
        return None;
    }
}

impl Default for SelectHand {
    fn default() -> Self {
        let cards: Vec<Card> = Vec::new();
        Self(cards)
    }
}

impl fmt::Display for SelectHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for card in &self.0 {
            write!(f, "{}", card)?;
        }
        write!(f, "]")?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create default context for tests
    fn ctx() -> HandContext<'static> {
        HandContext::default_context()
    }

    #[test]
    fn test_values() {
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Diamond);
        let c5 = Card::new(Value::Jack, Suit::Heart);
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::Ace, Suit::Spade);

        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let values = hand.values();

        // Should have 5 values
        assert_eq!(values.len(), 5);

        // Expect sorted (2, 3, J, K, A)
        assert_eq!(values[0], Value::Two);
        assert_eq!(values[1], Value::Three);
        assert_eq!(values[2], Value::Jack);
        assert_eq!(values[3], Value::King);
        assert_eq!(values[4], Value::Ace);
    }

    #[test]
    fn test_values_freq() {
        let c1 = Card::new(Value::Two, Suit::Heart);
        let c2 = Card::new(Value::Three, Suit::Diamond);
        let c3 = Card::new(Value::Four, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Heart);
        let c5 = Card::new(Value::King, Suit::Spade);

        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let freq = hand.values_freq();

        // Should have 4 values (K, 2, 3, 4)
        assert_eq!(freq.len(), 4);

        // Expect 2 kings and 1 each of 2, 3, 4
        assert_eq!(freq.get(&Value::King).unwrap().len(), 2);
        assert_eq!(freq.get(&Value::Two).unwrap().len(), 1);
        assert_eq!(freq.get(&Value::Three).unwrap().len(), 1);
        assert_eq!(freq.get(&Value::Four).unwrap().len(), 1);

        // No extra cards
        assert_eq!(freq.get(&Value::Five), None);
        assert_eq!(freq.get(&Value::Nine), None);

        // Can also check the cards in the vec are as expected
        assert_eq!(freq.get(&Value::King).unwrap()[0].value, Value::King);
        assert_eq!(freq.get(&Value::King).unwrap()[1].value, Value::King);
        assert_eq!(freq.get(&Value::Two).unwrap()[0].value, Value::Two);
        assert_eq!(freq.get(&Value::Three).unwrap()[0].value, Value::Three);
        assert_eq!(freq.get(&Value::Four).unwrap()[0].value, Value::Four);

        // Check ordered by value
        assert_eq!(freq.into_iter().nth(0).unwrap().0, Value::King)
    }

    #[test]
    fn test_suits_freq() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Diamond);
        let c5 = Card::new(Value::Four, Suit::Heart);

        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let freq = hand.suits_freq();

        // Should have 3 values (heart, spade, diamond)
        assert_eq!(freq.len(), 3);

        // Expect 3 hearts and 1 each of spade and diamond
        assert_eq!(freq.get(&Suit::Heart).unwrap().len(), 3);
        assert_eq!(freq.get(&Suit::Spade).unwrap().len(), 1);
        assert_eq!(freq.get(&Suit::Diamond).unwrap().len(), 1);

        // No clubs to be found
        assert_eq!(freq.get(&Suit::Club), None);

        // Can also check the cards in the vec are as expected
        assert_eq!(freq.get(&Suit::Heart).unwrap()[0].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Heart).unwrap()[1].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Heart).unwrap()[2].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Spade).unwrap()[0].suit, Suit::Spade);
        assert_eq!(freq.get(&Suit::Diamond).unwrap()[0].suit, Suit::Diamond);
    }

    #[test]
    fn test_best_hand() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Diamond);

        // Best hand is flush five (Ah, Ah, Ah, Ah, Ah)
        let hand = SelectHand::new(vec![c1, c1, c1, c1, c1]);
        let best = hand.best_hand().expect("is best hand");
        assert_eq!(best.rank, HandRank::FlushFive);
        assert_eq!(best.hand.len(), 5);

        // 4ok is better than flush (Ah, Ah, Ah, Ah, 2h)
        let hand = SelectHand::new(vec![c1, c1, c1, c1, c2]);
        let best = hand.best_hand().expect("is best hand");
        assert_eq!(best.clone().rank, HandRank::FourOfAKind);
        assert_eq!(best.hand.len(), 4);

        // Two pair is better than pair (Ah, Ah, 2h, 2h, 3d)
        let hand = SelectHand::new(vec![c1, c1, c2, c2, c3]);
        let best = hand.best_hand().expect("is best hand");
        assert_eq!(best.clone().rank, HandRank::TwoPair);
        assert_eq!(best.hand.len(), 4);

        // At worst, we get a high card (Ah, 2h, 3d)
        let hand = SelectHand::new(vec![c1, c2, c3]);
        let best = hand.best_hand().expect("is best hand");
        assert_eq!(best.clone().rank, HandRank::HighCard);
        assert_eq!(best.hand.len(), 1);
    }

    #[test]
    fn test_highcard() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Diamond);
        let c4 = Card::new(Value::Four, Suit::Diamond);
        let c5 = Card::new(Value::Five, Suit::Diamond);
        let c6 = Card::new(Value::Six, Suit::Diamond);

        // Valid 5 (A, K, 3, 4, 5)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::Ace);

        // Valid 5 (K, A, 3, 4, 5)
        let hand = SelectHand::new(vec![c2, c1, c3, c4, c5]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::Ace);

        // Valid 5 (K, 3, 4, 5, 6)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, c6]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::King);

        // Valid 4 (K, 3, 4, 5)
        let hand = SelectHand::new(vec![c2, c3, c4, c5]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::King);

        // Valid 3 (K, 3, 4)
        let hand = SelectHand::new(vec![c2, c3, c4]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::King);

        // Valid 2 (K, 3)
        let hand = SelectHand::new(vec![c2, c3]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::King);

        // Valid 1 (K)
        let hand = SelectHand::new(vec![c2]);
        let hc = hand.is_highcard();
        assert_eq!(hc.clone().unwrap().len(), 1);
        assert_eq!(hc.unwrap().0[0].value, Value::King);
    }

    #[test]
    fn test_pair() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Diamond);
        let c3 = Card::new(Value::Three, Suit::Diamond);
        let c4 = Card::new(Value::Four, Suit::Diamond);
        let c5 = Card::new(Value::Five, Suit::Diamond);
        let c6 = Card::new(Value::Six, Suit::Diamond);

        // Valid 5 (K, K, 3, 4, 5)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2.unwrap().len(), 2);

        // Valid 4 (K, K, 3, 4)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2.unwrap().len(), 2);

        // Valid 3 (K, K, 3)
        let hand = SelectHand::new(vec![c1, c2, c3]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2.unwrap().len(), 2);

        // Valid 2 (K, K)
        let hand = SelectHand::new(vec![c1, c2]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2.unwrap().len(), 2);

        // Invalid 1 (K)
        let hand = SelectHand::new(vec![c1]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2, None);

        // Invalid 2 (K, 3)
        let hand = SelectHand::new(vec![c1, c3]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2, None);

        // Invalid 3 (K, 3, 4)
        let hand = SelectHand::new(vec![c1, c3, c4]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2, None);

        // Invalid 4 (K, 3, 4, 5)
        let hand = SelectHand::new(vec![c1, c3, c4, c5]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2, None);

        // Invalid 5 (K, 3, 4, 5, 6)
        let hand = SelectHand::new(vec![c1, c3, c4, c5, c6]);
        let is_2 = hand.is_pair();
        assert_eq!(is_2, None);
    }

    #[test]
    fn test_two_pair() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Four, Suit::Diamond);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::Three, Suit::Heart);

        // Valid 5 (K, K, 4, 4, 2)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not1]);
        let tp = hand.is_two_pair();
        assert_eq!(tp.unwrap().len(), 4);

        // Valid 4 (K, K, 4, 4)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let tp = hand.is_two_pair();
        assert_eq!(tp.unwrap().len(), 4);

        // Invalid 5 (K, K, K, K, 2)
        let hand = SelectHand::new(vec![c1, c1, c2, c2, not1]);
        let tp = hand.is_two_pair();
        assert_eq!(tp, None);

        // Invalid 5 (K, 4, 3, 2, 2)
        let hand = SelectHand::new(vec![c1, c4, not1, not2, not2]);
        let tp = hand.is_two_pair();
        assert_eq!(tp, None);

        // Invalid 5 (K, K, 4, 3, 2)
        let hand = SelectHand::new(vec![c1, c1, c4, not1, not2]);
        let tp = hand.is_two_pair();
        assert_eq!(tp, None);

        // Invalid 4 (K, K, 4, 2)
        let hand = SelectHand::new(vec![c1, c2, c4, not1]);
        let tp = hand.is_two_pair();
        assert_eq!(tp, None);
    }

    #[test]
    fn test_three_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let not1 = Card::new(Value::Ace, Suit::Heart);
        let not2 = Card::new(Value::Two, Suit::Heart);

        // Valid 5 (K, K, K, A, 2)
        let hand = SelectHand::new(vec![c1, c2, c3, not1, not2]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3.unwrap().len(), 3);

        // Valid 4 (K, K, K, A)
        let hand = SelectHand::new(vec![c1, c2, c3, not1]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3.unwrap().len(), 3);

        // Valid 3 (K, K, K)
        let hand = SelectHand::new(vec![c1, c2, c3]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3.unwrap().len(), 3);

        // Invalid 3 (K, K, A)
        let hand = SelectHand::new(vec![c1, c2, not1]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3, None);

        // Invalid 4 (K, K, A, A),
        let hand = SelectHand::new(vec![c1, c2, not1, not1]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3, None);

        // Invalid 5 (K, K, A, A, 2),
        let hand = SelectHand::new(vec![c1, c2, not1, not1, not2]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3, None);

        // Invalid 2 (K, K)
        let hand = SelectHand::new(vec![c1, c2]);
        let is_3 = hand.is_three_of_kind();
        assert_eq!(is_3, None);
    }

    #[test]
    fn test_straight() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let c5 = Card::new(Value::Five, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Diamond);
        let c7 = Card::new(Value::Seven, Suit::Diamond);

        // Valid 5 (2, 3, 4 ,5 ,6)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, c6]);
        let straight = hand.is_straight(&ctx());
        assert_eq!(straight.unwrap().len(), 5);

        // Valid 5 with low ace (A, 2, 3, 4 ,5)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let straight = hand.is_straight(&ctx());
        assert_eq!(straight.unwrap().len(), 5);

        // Invalid 5 (2, 3, 4, 5, 7)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, c7]);
        let straight = hand.is_straight(&ctx());
        assert_eq!(straight, None);

        // Invalid 5 with low ace (A, 2, 3, 4, 7)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c7]);
        let straight = hand.is_straight(&ctx());
        assert_eq!(straight, None);

        // Invalid 4 (2, 3, 4, 5)
        let hand = SelectHand::new(vec![c2, c3, c4, c5]);
        let straight = hand.is_straight(&ctx());
        assert_eq!(straight, None);
    }

    #[test]
    fn test_flush() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::Queen, Suit::Heart);
        let c3 = Card::new(Value::Jack, Suit::Heart);
        let c4 = Card::new(Value::Seven, Suit::Heart);
        let c5 = Card::new(Value::Eight, Suit::Heart);
        let not = Card::new(Value::Ace, Suit::Diamond);

        // Valid 5 (h, h, h, h, h)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let flush = hand.is_flush(&ctx());
        assert_eq!(flush.unwrap().len(), 5);

        // Valid 5 from 7 cards (h, h, h, h, h, d, d)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5, not, not]);
        let flush = hand.is_flush(&ctx());
        assert_eq!(flush.unwrap().len(), 5);

        // Invalid 5 (h, h, h, h, d)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not]);
        let flush = hand.is_flush(&ctx());
        assert_eq!(flush, None);

        // Invalid 4 (h, h, h, h)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let flush = hand.is_flush(&ctx());
        assert_eq!(flush, None);
    }

    #[test]
    fn test_fullhouse() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Diamond);
        let c5 = Card::new(Value::Four, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::Three, Suit::Heart);

        // Valid 5 (K, K, K, 4, 4)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let is_fh = hand.is_fullhouse();
        assert_eq!(is_fh.unwrap().len(), 5);

        // Valid 5 from 7 cards (K, K, K, 4, 4, 2, 3)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5, not1, not2]);
        let is_fh = hand.is_fullhouse();
        assert_eq!(is_fh.unwrap().len(), 5);

        // Invalid 5 (K, K, K, K, 2)
        let hand = SelectHand::new(vec![c1, c2, c3, c3, not1]);
        let is_fh = hand.is_fullhouse();
        assert_eq!(is_fh, None);

        // Invalid 5 (K, K, 4, 4, 2)
        let hand = SelectHand::new(vec![c1, c2, c4, c5, not1]);
        let is_fh = hand.is_fullhouse();
        assert_eq!(is_fh, None);

        // Invalid 4 (K, K, 4, 4)
        let hand = SelectHand::new(vec![c1, c2, c4, c5]);
        let is_fh = hand.is_fullhouse();
        assert_eq!(is_fh, None);
    }

    #[test]
    fn test_four_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Diamond);
        let not = Card::new(Value::Ace, Suit::Heart);

        // Valid 4 (K, K, K, K)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not]);
        let is_4 = hand.is_four_of_kind();
        assert_eq!(is_4.unwrap().len(), 4);

        // Valid 4 from 7 cards (K, K, K, K, A, A, A)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not, not, not]);
        let is_4 = hand.is_four_of_kind();
        assert_eq!(is_4.unwrap().len(), 4);

        // Invalid 4 (K, K, K, A)
        let hand = SelectHand::new(vec![c1, c2, c3, not]);
        let is_4 = hand.is_four_of_kind();
        assert_eq!(is_4, None);

        // Invalid 3 (K, K, K)
        let hand = SelectHand::new(vec![c1, c2, c3]);
        let is_4 = hand.is_four_of_kind();
        assert_eq!(is_4, None);
    }

    #[test]
    fn test_straight_flush() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let c5 = Card::new(Value::Five, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Heart);
        let not1 = Card::new(Value::Seven, Suit::Heart);
        let not2 = Card::new(Value::Six, Suit::Diamond);

        // Valid 5 (2h, 3h, 4h, 5h ,6h)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, c6]);
        let sf = hand.is_straight_flush(&ctx());
        assert_eq!(sf.unwrap().len(), 5);

        // Valid 5 with low ace (Ah, 2h, 3h, 4h, 5h)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let sf = hand.is_straight_flush(&ctx());
        assert_eq!(sf.unwrap().len(), 5);

        // Invalid 5, wrong value (2h, 3h, 4h, 5h, 7h)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, not1]);
        let sf = hand.is_straight_flush(&ctx());
        assert_eq!(sf, None);

        // Invalid 5, wrong suit (2h, 3h, 4h, 5h, 6d)
        let hand = SelectHand::new(vec![c2, c3, c4, c5, not2]);
        let sf = hand.is_straight_flush(&ctx());
        assert_eq!(sf, None);

        // Invalid 4 (2h, 3h, 4h, 5h)
        let hand = SelectHand::new(vec![c2, c3, c4, c5]);
        let sf = hand.is_straight_flush(&ctx());
        assert_eq!(sf, None);
    }

    #[test]
    fn test_royal_flush() {
        let c1 = Card::new(Value::Ten, Suit::Spade);
        let c2 = Card::new(Value::Jack, Suit::Spade);
        let c3 = Card::new(Value::Queen, Suit::Spade);
        let c4 = Card::new(Value::King, Suit::Spade);
        let c5 = Card::new(Value::Ace, Suit::Spade);
        let not1 = Card::new(Value::Nine, Suit::Spade);
        let not2 = Card::new(Value::Ace, Suit::Diamond);

        // Valid 5 (10s, Js, Qs, Ks, As)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let rf = hand.is_royal_flush(&ctx());
        assert_eq!(rf.unwrap().len(), 5);

        // Valid 5, scrambled input order (Js, 10s, Ks, Qs, As)
        let hand = SelectHand::new(vec![c2, c1, c4, c3, c5]);
        let rf = hand.is_royal_flush(&ctx());
        assert_eq!(rf.unwrap().len(), 5);

        // Invalid 5, wrong value (9s, Js, Qs, Ks, As)
        let hand = SelectHand::new(vec![not1, c2, c3, c4, c5]);
        let rf = hand.is_royal_flush(&ctx());
        assert_eq!(rf, None);

        // Invalid 5, wrong suit (10s, Js, Qs, Ks, Ad)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not2]);
        let rf = hand.is_royal_flush(&ctx());
        assert_eq!(rf, None);

        // Invalid 4 (2h, 3h, 4h, 5h)
        let hand = SelectHand::new(vec![c2, c3, c4, c5]);
        let rf = hand.is_royal_flush(&ctx());
        assert_eq!(rf, None);
    }

    #[test]
    fn test_five_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Diamond);
        let c5 = Card::new(Value::King, Suit::Heart);
        let not = Card::new(Value::Ace, Suit::Heart);

        // Valid 5 (K, K, K, K, K)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let is_5 = hand.is_five_of_kind();
        assert_eq!(is_5.unwrap().len(), 5);

        // Valid 5 from 7 cards (K, K, K, K, K, A, A)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5, not, not]);
        let is_5 = hand.is_five_of_kind();
        assert_eq!(is_5.unwrap().len(), 5);

        // Invalid 5 (K, K, K, K, A)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not]);
        let is_5 = hand.is_five_of_kind();
        assert_eq!(is_5, None);

        // Invalid 4 (K, K, K, K)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let is_5 = hand.is_five_of_kind();
        assert_eq!(is_5, None);
    }

    #[test]
    fn test_flush_house() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Heart);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::Ace, Suit::Heart);
        let c5 = Card::new(Value::Ace, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::Ace, Suit::Diamond);

        // Valid 5 (Kh, Kh, Kh, Ah, Ah)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let fh = hand.is_flush_house(&ctx());
        assert_eq!(fh.unwrap().len(), 5);

        // Invalid 5 (Kh, Kh, Kh, Ah, 2h)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not1]);
        let fh = hand.is_flush_house(&ctx());
        assert_eq!(fh, None);

        // Invalid 5 (Kh, Kh, Kh, Ah, Ad)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not2]);
        let fh = hand.is_flush_house(&ctx());
        assert_eq!(fh, None);

        // Invalid 4 (Kh, Kh, Kh, Ah)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let fh = hand.is_flush_house(&ctx());
        assert_eq!(fh, None);
    }

    #[test]
    fn test_flush_five() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Heart);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Heart);
        let c5 = Card::new(Value::King, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::King, Suit::Diamond);

        // Valid 5 (Kh, Kh, Kh, Kh, Kh)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, c5]);
        let ff = hand.is_flush_five(&ctx());
        assert_eq!(ff.unwrap().len(), 5);

        // Invalid 5 (Kh, Kh, Kh, Kh, 2h)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not1]);
        let ff = hand.is_flush_five(&ctx());
        assert_eq!(ff, None);

        // Invalid 5 (Kh, Kh, Kh, Kh, Kd)
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not2]);
        let ff = hand.is_flush_five(&ctx());
        assert_eq!(ff, None);

        // Invalid 4 (Kh, Kh, Kh, Kh)
        let hand = SelectHand::new(vec![c1, c2, c3, c4]);
        let ff = hand.is_flush_five(&ctx());
        assert_eq!(ff, None);
    }

    #[test]
    fn test_gap_straight_normal() {
        // Test gap straight with Shortcut joker modifier
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c5 = Card::new(Value::Five, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Diamond);
        let c7 = Card::new(Value::Seven, Suit::Diamond);

        // 2, 3, 5, 6, 7 - missing 4, should be gap straight with modifier
        let hand = SelectHand::new(vec![c2, c3, c5, c6, c7]);

        // Without modifier - should not be a straight
        let ctx = HandContext::default_context();
        let straight = hand.is_straight(&ctx);
        assert_eq!(straight, None);

        // With gap_straights modifier - should be a straight
        let mods = GameModifiers {
            gap_straights: true,
            ..Default::default()
        };
        let ctx = HandContext { modifiers: &mods };
        let straight = hand.is_straight(&ctx);
        assert!(straight.is_some());
        assert_eq!(straight.unwrap().len(), 5);
    }

    #[test]
    fn test_gap_straight_ace_low() {
        // Test gap straight with low ace: A, 2, 3, 5 (missing 4)
        let ace = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Diamond);
        let c5 = Card::new(Value::Five, Suit::Club);

        let hand = SelectHand::new(vec![ace, c2, c3, c5]);

        // Without modifier - should not be a straight
        let ctx = HandContext::default_context();
        let straight = hand.is_straight(&ctx);
        assert_eq!(straight, None);

        // With gap_straights and four_card_straights modifiers
        let mods = GameModifiers {
            gap_straights: true,
            four_card_straights: true,
            ..Default::default()
        };
        let ctx = HandContext { modifiers: &mods };
        let straight = hand.is_straight(&ctx);
        assert!(straight.is_some());
        assert_eq!(straight.unwrap().len(), 4);
    }

    #[test]
    fn test_gap_straight_multiple_gaps_fails() {
        // Test that multiple gaps don't work - 2, 3, 6, 7 (missing 4 and 5)
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Diamond);
        let c7 = Card::new(Value::Seven, Suit::Diamond);
        let c8 = Card::new(Value::Eight, Suit::Club);

        let hand = SelectHand::new(vec![c2, c3, c6, c7, c8]);

        // Even with gap_straights modifier - should not be a straight (2 gaps)
        let mods = GameModifiers {
            gap_straights: true,
            ..Default::default()
        };
        let ctx = HandContext { modifiers: &mods };
        let straight = hand.is_straight(&ctx);
        assert_eq!(straight, None);
    }

    #[test]
    fn test_four_card_flush_modifier() {
        // Test 4-card flush with Four Fingers joker
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::Queen, Suit::Heart);
        let c3 = Card::new(Value::Jack, Suit::Heart);
        let c4 = Card::new(Value::Seven, Suit::Heart);
        let not = Card::new(Value::Ace, Suit::Diamond);

        // 4 hearts + 1 diamond
        let hand = SelectHand::new(vec![c1, c2, c3, c4, not]);

        // Without modifier - should not be a flush
        let ctx = HandContext::default_context();
        let flush = hand.is_flush(&ctx);
        assert_eq!(flush, None);

        // With four_card_flushes modifier - should be a flush
        let mods = GameModifiers {
            four_card_flushes: true,
            ..Default::default()
        };
        let ctx = HandContext { modifiers: &mods };
        let flush = hand.is_flush(&ctx);
        assert!(flush.is_some());
        assert_eq!(flush.unwrap().len(), 4);
    }

    #[test]
    fn test_smeared_suits_flush() {
        // Test smeared suits with Smeared Joker
        let h2 = Card::new(Value::Two, Suit::Heart);
        let h3 = Card::new(Value::Three, Suit::Heart);
        let d5 = Card::new(Value::Five, Suit::Diamond);
        let d7 = Card::new(Value::Seven, Suit::Diamond);
        let d9 = Card::new(Value::Nine, Suit::Diamond);

        // 2 hearts + 3 diamonds = 5 "red" cards
        let hand = SelectHand::new(vec![h2, h3, d5, d7, d9]);

        // Without modifier - should not be a flush
        let ctx = HandContext::default_context();
        let flush = hand.is_flush(&ctx);
        assert_eq!(flush, None);

        // With smeared_suits modifier - should be a flush
        let mods = GameModifiers {
            smeared_suits: true,
            ..Default::default()
        };
        let ctx = HandContext { modifiers: &mods };
        let flush = hand.is_flush(&ctx);
        assert!(flush.is_some());
        assert_eq!(flush.unwrap().len(), 5);
    }
}
