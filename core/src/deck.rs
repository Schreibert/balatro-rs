use crate::card::{Card, Suit, Value};
use rand::{seq::SliceRandom, thread_rng};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Create an empty deck (alias for new())
    pub fn empty() -> Self {
        Self::new()
    }
    pub(crate) fn draw(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n {
            return None;
        }
        return Some(self.cards.drain(0..n).collect());
    }
    pub(crate) fn len(&self) -> usize {
        self.cards.len()
    }

    pub(crate) fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub(crate) fn append(&mut self, other: &mut Vec<Card>) {
        self.cards.append(other);
    }

    pub(crate) fn extend(&mut self, other: Vec<Card>) {
        self.cards.extend(other);
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    /// Remove a specific card from the deck (for destruction, etc.)
    pub(crate) fn remove_card(&mut self, card: Card) {
        if let Some(index) = self.cards.iter().position(|c| c.id == card.id) {
            self.cards.remove(index);
        }
    }

    /// Modify a card in the deck by its ID (for Tarot effects)
    pub(crate) fn modify_card<F>(&mut self, card_id: usize, f: F) -> bool
    where
        F: FnOnce(&mut Card),
    {
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            f(card);
            return true;
        }
        false
    }

    /// Add a card to the deck (for Tarot/Spectral generation effects)
    pub(crate) fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    // // Loops through cards, assigning index to each equal to index in deck
    // pub(crate) fn index_cards(&mut self) {
    //     let mut i = 0;
    //     for card in &mut self.cards {
    //         card.index = Some(i);
    //         i += 1;
    //     }
    // }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for v in &Value::values() {
            for s in &Suit::suits() {
                let c = Card::new(*v, *s);
                cards.push(c);
            }
        }
        Self { cards }
    }
}
