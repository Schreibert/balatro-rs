use crate::card::Card;
use crate::consumable::{Consumable, ConsumableType};
use crate::error::GameError;
use crate::game::Game;
use pyo3::pyclass;
use strum::{EnumIter, IntoEnumIterator};

/// The 22 Tarot cards (Major Arcana)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, EnumIter, Eq, PartialEq, Hash)]
pub enum Tarots {
    TheFool,          // 0 - Copy last Tarot/Planet used
    TheMagician,      // I - 2 cards → Lucky
    TheHighPriestess, // II - Create up to 2 Planet cards
    TheEmpress,       // III - 2 cards → Mult
    TheEmperor,       // IV - Create up to 2 Tarot cards
    TheHierophant,    // V - 2 cards → Bonus
    TheLovers,        // VI - 1 card → Wild
    TheChariot,       // VII - 1 card → Steel
    Justice,          // VIII - 1 card → Glass
    TheHermit,        // IX - Double money (max $20)
    WheelOfFortune,   // X - 1/4 chance add edition to random Joker
    Strength,         // XI - Up to 2 cards, raise rank by 1
    TheHangedMan,     // XII - Destroy up to 2 cards
    Death,            // XIII - Convert left card into right card
    Temperance,       // XIV - Gain sell value of all Jokers (max $50)
    TheDevil,         // XV - 1 card → Gold
    TheTower,         // XVI - 1 card → Stone
    TheStar,          // XVII - Up to 3 cards → Diamonds
    TheMoon,          // XVIII - Up to 3 cards → Clubs
    TheSun,           // XIX - Up to 3 cards → Hearts
    Judgement,        // XX - Create random Joker
    TheWorld,         // XXI - Up to 3 cards → Spades
}

impl Tarots {
    /// Get all tarot cards
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}

impl Consumable for Tarots {
    fn name(&self) -> String {
        match self {
            Self::TheFool => "The Fool".to_string(),
            Self::TheMagician => "The Magician".to_string(),
            Self::TheHighPriestess => "The High Priestess".to_string(),
            Self::TheEmpress => "The Empress".to_string(),
            Self::TheEmperor => "The Emperor".to_string(),
            Self::TheHierophant => "The Hierophant".to_string(),
            Self::TheLovers => "The Lovers".to_string(),
            Self::TheChariot => "The Chariot".to_string(),
            Self::Justice => "Justice".to_string(),
            Self::TheHermit => "The Hermit".to_string(),
            Self::WheelOfFortune => "Wheel of Fortune".to_string(),
            Self::Strength => "Strength".to_string(),
            Self::TheHangedMan => "The Hanged Man".to_string(),
            Self::Death => "Death".to_string(),
            Self::Temperance => "Temperance".to_string(),
            Self::TheDevil => "The Devil".to_string(),
            Self::TheTower => "The Tower".to_string(),
            Self::TheStar => "The Star".to_string(),
            Self::TheMoon => "The Moon".to_string(),
            Self::TheSun => "The Sun".to_string(),
            Self::Judgement => "Judgement".to_string(),
            Self::TheWorld => "The World".to_string(),
        }
    }

    fn desc(&self) -> String {
        match self {
            Self::TheFool => "Creates a copy of the last Tarot or Planet card used".to_string(),
            Self::TheMagician => "Enhances 2 selected cards to Lucky Cards".to_string(),
            Self::TheHighPriestess => "Creates up to 2 random Planet cards".to_string(),
            Self::TheEmpress => "Enhances 2 selected cards to Mult Cards".to_string(),
            Self::TheEmperor => "Creates up to 2 random Tarot cards".to_string(),
            Self::TheHierophant => "Enhances 2 selected cards to Bonus Cards".to_string(),
            Self::TheLovers => "Enhances 1 selected card to a Wild Card".to_string(),
            Self::TheChariot => "Enhances 1 selected card to a Steel Card".to_string(),
            Self::Justice => "Enhances 1 selected card to a Glass Card".to_string(),
            Self::TheHermit => "Doubles your money (max $20)".to_string(),
            Self::WheelOfFortune => {
                "1 in 4 chance to add edition to a random Joker".to_string()
            }
            Self::Strength => "Increases rank of up to 2 selected cards by 1".to_string(),
            Self::TheHangedMan => "Destroys up to 2 selected cards".to_string(),
            Self::Death => "Select 2 cards; converts left card into right card".to_string(),
            Self::Temperance => {
                "Gives the total sell value of all current Jokers (max $50)".to_string()
            }
            Self::TheDevil => "Enhances 1 selected card to a Gold Card".to_string(),
            Self::TheTower => "Enhances 1 selected card to a Stone Card".to_string(),
            Self::TheStar => "Converts up to 3 selected cards to Diamonds".to_string(),
            Self::TheMoon => "Converts up to 3 selected cards to Clubs".to_string(),
            Self::TheSun => "Converts up to 3 selected cards to Hearts".to_string(),
            Self::Judgement => "Creates a random Joker".to_string(),
            Self::TheWorld => "Converts up to 3 selected cards to Spades".to_string(),
        }
    }

    fn cost(&self) -> usize {
        3 // Standard tarot cost
    }

    fn requires_target(&self) -> bool {
        !matches!(
            self,
            Self::TheFool
                | Self::TheHighPriestess
                | Self::TheEmperor
                | Self::TheHermit
                | Self::WheelOfFortune
                | Self::Temperance
                | Self::Judgement
        )
    }

    fn max_targets(&self) -> usize {
        match self {
            Self::TheLovers | Self::TheChariot | Self::Justice | Self::TheDevil | Self::TheTower => 1,
            Self::TheMagician | Self::TheEmpress | Self::TheHierophant | Self::Strength
            | Self::TheHangedMan | Self::Death => 2,
            Self::TheStar | Self::TheMoon | Self::TheSun | Self::TheWorld => 3,
            _ => 0,
        }
    }

    fn use_effect(&self, game: &mut Game, targets: Option<Vec<Card>>) -> Result<(), GameError> {
        use crate::card::Enhancement;

        match self {
            // Category A: No targets needed
            Self::TheHermit => {
                // Double money (max $20)
                let doubled = game.money * 2;
                game.money = doubled.min(20);
                Ok(())
            }
            Self::Temperance => {
                // Gain sell value of all Jokers (max $50)
                let sell_value = game.get_joker_sell_value();
                game.add_money_capped(sell_value, 50);
                Ok(())
            }
            Self::TheHighPriestess => {
                // Create 2 random Planet cards
                let planet1 = game.generate_random_planet();
                let planet2 = game.generate_random_planet();
                game.consumables.push(planet1);
                game.consumables.push(planet2);
                Ok(())
            }
            Self::TheEmperor => {
                // Create 2 random Tarot cards
                let tarot1 = game.generate_random_tarot();
                let tarot2 = game.generate_random_tarot();
                game.consumables.push(tarot1);
                game.consumables.push(tarot2);
                Ok(())
            }
            Self::Judgement => {
                // Create random Joker
                let joker = game.generate_random_joker();
                game.jokers.push(joker);
                Ok(())
            }

            // Category B: Enhancement Tarots (require targets)
            Self::TheMagician => {
                // 2 cards → Lucky
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Lucky);
                        });
                    }
                }
                Ok(())
            }
            Self::TheEmpress => {
                // 2 cards → Mult
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Mult);
                        });
                    }
                }
                Ok(())
            }
            Self::TheHierophant => {
                // 2 cards → Bonus
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Bonus);
                        });
                    }
                }
                Ok(())
            }
            Self::TheLovers => {
                // 1 card → Wild
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Wild);
                        });
                    }
                }
                Ok(())
            }
            Self::TheChariot => {
                // 1 card → Steel
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Steel);
                        });
                    }
                }
                Ok(())
            }
            Self::Justice => {
                // 1 card → Glass
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Glass);
                        });
                    }
                }
                Ok(())
            }
            Self::TheDevil => {
                // 1 card → Gold
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Gold);
                        });
                    }
                }
                Ok(())
            }
            Self::TheTower => {
                // 1 card → Stone
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_enhancement(Enhancement::Stone);
                        });
                    }
                }
                Ok(())
            }

            // Category C: Suit Conversion Tarots
            Self::TheStar => {
                // Up to 3 cards → Diamonds
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_suit(crate::card::Suit::Diamond);
                        });
                    }
                }
                Ok(())
            }
            Self::TheMoon => {
                // Up to 3 cards → Clubs
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_suit(crate::card::Suit::Club);
                        });
                    }
                }
                Ok(())
            }
            Self::TheSun => {
                // Up to 3 cards → Hearts
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_suit(crate::card::Suit::Heart);
                        });
                    }
                }
                Ok(())
            }
            Self::TheWorld => {
                // Up to 3 cards → Spades
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            c.set_suit(crate::card::Suit::Spade);
                        });
                    }
                }
                Ok(())
            }

            // Category D: Special Effect Tarots
            Self::Strength => {
                // Up to 2 cards, raise rank by 1
                if let Some(cards) = targets {
                    for card in cards {
                        game.modify_card_in_deck(card.id, |c| {
                            if let Some(new_rank) = c.value.raise_rank() {
                                c.set_rank(new_rank);
                            }
                        });
                    }
                }
                Ok(())
            }
            Self::TheHangedMan => {
                // Destroy up to 2 cards
                if let Some(cards) = targets {
                    for card in cards {
                        game.destroy_card(card);
                    }
                }
                Ok(())
            }
            Self::Death => {
                // Convert left card into right card
                if let Some(cards) = targets {
                    if cards.len() >= 2 {
                        let source_id = cards[0].id;
                        let target_value = cards[1].value;
                        let target_suit = cards[1].suit;

                        game.modify_card_in_deck(source_id, |c| {
                            c.set_rank(target_value);
                            c.set_suit(target_suit);
                        });
                    }
                }
                Ok(())
            }
            Self::TheFool => {
                // Copy last Tarot/Planet used
                if let Some(last_consumable) = game.last_consumable_used.clone() {
                    last_consumable.use_effect(game, None)?;
                }
                Ok(())
            }
            Self::WheelOfFortune => {
                // 1/4 chance to add edition to random Joker
                use rand::Rng;
                if rand::thread_rng().gen_range(0..4) == 0 {
                    // Success! Add random edition to random joker
                    if !game.jokers.is_empty() {
                        use crate::card::Edition;
                        use rand::seq::SliceRandom;

                        let editions = vec![Edition::Foil, Edition::Holographic, Edition::Polychrome];
                        let edition = editions.choose(&mut rand::thread_rng()).unwrap();

                        // Note: Jokers don't have editions in current implementation
                        // This is a placeholder - would need to add edition field to Jokers
                        // For now, just don't crash
                    }
                }
                Ok(())
            }
        }
    }

    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
}
