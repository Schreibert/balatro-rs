use crate::consumable::Consumables;
use crate::joker::Jokers;
use crate::planet::Planets;
use crate::spectral::Spectrals;
use crate::tarot::Tarots;
use rand::seq::SliceRandom;
use std::fmt;

/// Booster Pack Types
/// Packs are purchased from the shop and opened to receive consumables or jokers
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackType {
    Arcana,    // Contains Tarot cards
    Celestial, // Contains Planet cards
    Spectral,  // Contains Spectral cards
    Buffoon,   // Contains Jokers
}

impl PackType {
    pub fn name(&self) -> &str {
        match self {
            PackType::Arcana => "Arcana Pack",
            PackType::Celestial => "Celestial Pack",
            PackType::Spectral => "Spectral Pack",
            PackType::Buffoon => "Buffoon Pack",
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            PackType::Arcana => "Choose 1 of up to 3 Tarot cards to be used immediately",
            PackType::Celestial => "Choose 1 of up to 3 Planet cards to be used immediately",
            PackType::Spectral => "Choose 1 of up to 3 Spectral cards to be used immediately",
            PackType::Buffoon => "Choose 1 of up to 2 Joker cards",
        }
    }

    pub fn base_cost(&self) -> usize {
        match self {
            PackType::Arcana => 4,
            PackType::Celestial => 4,
            PackType::Spectral => 4,
            PackType::Buffoon => 4,
        }
    }

    /// Number of cards in the pack
    pub fn card_count(&self) -> usize {
        match self {
            PackType::Arcana => 3,
            PackType::Celestial => 3,
            PackType::Spectral => 3,
            PackType::Buffoon => 2,
        }
    }

    /// How many cards the player can choose from the pack
    pub fn choices(&self) -> usize {
        match self {
            PackType::Arcana => 1,
            PackType::Celestial => 1,
            PackType::Spectral => 1,
            PackType::Buffoon => 1,
        }
    }
}

impl fmt::Display for PackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A booster pack instance with its contents
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pack {
    pub pack_type: PackType,
    pub contents: PackContents,
}

/// Contents of a booster pack
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackContents {
    Tarots(Vec<Tarots>),
    Planets(Vec<Planets>),
    Spectrals(Vec<Spectrals>),
    Jokers(Vec<Jokers>),
}

impl Pack {
    /// Create a new pack with randomly generated contents
    pub fn new(pack_type: PackType) -> Self {
        let contents = match pack_type {
            PackType::Arcana => {
                let count = pack_type.card_count();
                let all_tarots = Tarots::all();
                let selected: Vec<Tarots> = all_tarots
                    .choose_multiple(&mut rand::thread_rng(), count)
                    .copied()
                    .collect();
                PackContents::Tarots(selected)
            }
            PackType::Celestial => {
                let count = pack_type.card_count();
                let all_planets = Planets::all();
                let selected: Vec<Planets> = all_planets
                    .choose_multiple(&mut rand::thread_rng(), count)
                    .copied()
                    .collect();
                PackContents::Planets(selected)
            }
            PackType::Spectral => {
                let count = pack_type.card_count();
                let all_spectrals = Spectrals::all();
                let selected: Vec<Spectrals> = all_spectrals
                    .choose_multiple(&mut rand::thread_rng(), count)
                    .cloned()
                    .collect();
                PackContents::Spectrals(selected)
            }
            PackType::Buffoon => {
                let count = pack_type.card_count();
                let all_jokers = Jokers::all_common(); // For now, only common jokers
                let selected: Vec<Jokers> = all_jokers
                    .choose_multiple(&mut rand::thread_rng(), count)
                    .cloned()
                    .collect();
                PackContents::Jokers(selected)
            }
        };

        Pack {
            pack_type,
            contents,
        }
    }

    /// Get the items from this pack as consumables or jokers
    pub fn get_tarots(&self) -> Option<&Vec<Tarots>> {
        match &self.contents {
            PackContents::Tarots(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_planets(&self) -> Option<&Vec<Planets>> {
        match &self.contents {
            PackContents::Planets(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_spectrals(&self) -> Option<&Vec<Spectrals>> {
        match &self.contents {
            PackContents::Spectrals(items) => Some(items),
            _ => None,
        }
    }

    pub fn get_jokers(&self) -> Option<&Vec<Jokers>> {
        match &self.contents {
            PackContents::Jokers(items) => Some(items),
            _ => None,
        }
    }

    /// Select an item from the pack by index
    pub fn select(&self, index: usize) -> Option<PackSelection> {
        match &self.contents {
            PackContents::Tarots(items) => {
                items.get(index).map(|t| PackSelection::Tarot(*t))
            }
            PackContents::Planets(items) => {
                items.get(index).map(|p| PackSelection::Planet(*p))
            }
            PackContents::Spectrals(items) => {
                items.get(index).map(|s| PackSelection::Spectral(s.clone()))
            }
            PackContents::Jokers(items) => {
                items.get(index).map(|j| PackSelection::Joker(j.clone()))
            }
        }
    }
}

/// Result of selecting from a pack
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackSelection {
    Tarot(Tarots),
    Planet(Planets),
    Spectral(Spectrals),
    Joker(Jokers),
}

impl PackSelection {
    /// Convert to a consumable if applicable
    pub fn to_consumable(&self) -> Option<Consumables> {
        match self {
            PackSelection::Tarot(t) => Some(Consumables::Tarot(*t)),
            PackSelection::Planet(p) => Some(Consumables::Planet(*p)),
            PackSelection::Spectral(s) => Some(Consumables::Spectral(s.clone())),
            PackSelection::Joker(_) => None,
        }
    }

    /// Get joker if this is a joker selection
    pub fn to_joker(&self) -> Option<Jokers> {
        match self {
            PackSelection::Joker(j) => Some(j.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_type_properties() {
        assert_eq!(PackType::Arcana.name(), "Arcana Pack");
        assert_eq!(PackType::Arcana.base_cost(), 4);
        assert_eq!(PackType::Arcana.card_count(), 3);
        assert_eq!(PackType::Arcana.choices(), 1);

        assert_eq!(PackType::Buffoon.card_count(), 2);
    }

    #[test]
    fn test_pack_creation_arcana() {
        let pack = Pack::new(PackType::Arcana);
        assert_eq!(pack.pack_type, PackType::Arcana);

        let tarots = pack.get_tarots();
        assert!(tarots.is_some());
        assert_eq!(tarots.unwrap().len(), 3);
    }

    #[test]
    fn test_pack_creation_celestial() {
        let pack = Pack::new(PackType::Celestial);
        assert_eq!(pack.pack_type, PackType::Celestial);

        let planets = pack.get_planets();
        assert!(planets.is_some());
        assert_eq!(planets.unwrap().len(), 3);
    }

    #[test]
    fn test_pack_creation_spectral() {
        let pack = Pack::new(PackType::Spectral);
        assert_eq!(pack.pack_type, PackType::Spectral);

        let spectrals = pack.get_spectrals();
        assert!(spectrals.is_some());
        assert_eq!(spectrals.unwrap().len(), 3);
    }

    #[test]
    fn test_pack_creation_buffoon() {
        let pack = Pack::new(PackType::Buffoon);
        assert_eq!(pack.pack_type, PackType::Buffoon);

        let jokers = pack.get_jokers();
        assert!(jokers.is_some());
        assert_eq!(jokers.unwrap().len(), 2);
    }

    #[test]
    fn test_pack_selection() {
        let pack = Pack::new(PackType::Arcana);
        let selection = pack.select(0);
        assert!(selection.is_some());

        match selection.unwrap() {
            PackSelection::Tarot(_) => {} // Expected
            _ => panic!("Expected Tarot selection"),
        }
    }

    #[test]
    fn test_pack_selection_out_of_bounds() {
        let pack = Pack::new(PackType::Arcana);
        let selection = pack.select(10);
        assert!(selection.is_none());
    }

    #[test]
    fn test_pack_selection_to_consumable() {
        let pack = Pack::new(PackType::Arcana);
        let selection = pack.select(0).unwrap();
        let consumable = selection.to_consumable();
        assert!(consumable.is_some());

        match consumable.unwrap() {
            Consumables::Tarot(_) => {} // Expected
            _ => panic!("Expected Tarot consumable"),
        }
    }

    #[test]
    fn test_pack_joker_selection() {
        let pack = Pack::new(PackType::Buffoon);
        let selection = pack.select(0).unwrap();
        let joker = selection.to_joker();
        assert!(joker.is_some());
    }
}
