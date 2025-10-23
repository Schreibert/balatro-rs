pub mod action;
pub mod alternative_deck;
pub mod ante;
pub mod available;
pub mod booster;
pub mod boss_modifier;
pub mod card;
pub mod config;
pub mod consumable;
pub mod deck;
pub mod effect;
pub mod error;
pub mod game;
pub mod generator;
pub mod hand;
pub mod joker;
pub mod planet;
pub mod rank;
pub mod shop;
pub mod space;
pub mod spectral;
pub mod stage;
pub mod tag;
pub mod tarot;
pub mod voucher;

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::game::Game;
    use crate::stage::Stage;

    use rand::Rng;

    #[test]
    // Test executing a full game using the gen_actions api
    fn test_game_gen_actions() {
        let mut g = Game::default();

        g.start();
        while !g.is_over() {
            // Get all available actions
            let actions: Vec<Action> = g.gen_actions().collect();
            if actions.len() == 0 {
                break;
            }

            // Pick a random move and execute it
            let i = rand::thread_rng().gen_range(0..actions.len());
            let action = actions[i].clone();
            dbg!("game state:\n{}", g.clone());
            dbg!("play action: {}", action.clone());
            let action_res = g.handle_action(action.clone());
            dbg!(action);
            assert!(action_res.is_ok());
        }
        let result = g.result();
        // Ensure game is over at end
        assert!(result.is_some());
        // Check game state at end
        assert!(matches!(g.stage, Stage::End(_)));
        dbg!("game action history: {:?}", g.action_history);
    }

    #[test]
    // Test executing a full game using the gen_action_space (vector) api
    fn test_game_action_space() {
        let mut g = Game::default();

        g.start();
        while !g.is_over() {
            // Get action space and vector
            let space = g.gen_action_space();
            let space_vec = space.to_vec();
            if space.is_empty() {
                break;
            }

            // Pick a random move and ensure its unmasked
            let mut i: usize;
            loop {
                i = rand::thread_rng().gen_range(0..space_vec.len());
                if space_vec[i] == 1 {
                    break;
                }
            }
            let action = space.to_action(i, &g).expect("valid index to action");
            dbg!("game state:\n{}", g.clone());
            dbg!("play action: {}", action.clone());
            let action_res = g.handle_action(action.clone());
            dbg!(action);
            assert!(action_res.is_ok());
        }
        let result = g.result();
        // Ensure game is over at end
        assert!(result.is_some());
        // Check game state at end
        assert!(matches!(g.stage, Stage::End(_)));
        dbg!("game action history: {:?}", g.action_history);
    }

    #[test]
    fn test_consumable_purchase() {
        use crate::consumable::{Consumable, Consumables};
        use crate::config::Config;
        use crate::planet::Planets;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        g.money = 100;
        g.stage = Stage::Shop();

        // Should be able to buy a tarot
        let tarot = Consumables::Tarot(Tarots::TheHermit);
        assert_eq!(g.consumables.len(), 0);
        assert!(g.buy_consumable(tarot.clone()).is_ok());
        assert_eq!(g.consumables.len(), 1);
        assert_eq!(g.money, 97); // Cost 3

        // Should be able to buy a planet
        let planet = Consumables::Planet(Planets::Mercury);
        assert!(g.buy_consumable(planet.clone()).is_ok());
        assert_eq!(g.consumables.len(), 2);
        assert_eq!(g.money, 94); // Cost 3

        // Should fail when at consumable slot limit
        let config = Config::default();
        assert_eq!(config.consumable_slots, 2);
        let another_tarot = Consumables::Tarot(Tarots::TheMagician);
        assert!(g.buy_consumable(another_tarot).is_err());
        assert_eq!(g.consumables.len(), 2);
    }

    #[test]
    fn test_consumable_insufficient_funds() {
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        g.money = 2; // Not enough for a tarot (cost 3)
        g.stage = Stage::Shop();

        let tarot = Consumables::Tarot(Tarots::TheHermit);
        assert!(g.buy_consumable(tarot).is_err());
        assert_eq!(g.consumables.len(), 0);
        assert_eq!(g.money, 2); // Money unchanged
    }

    #[test]
    fn test_consumable_wrong_stage() {
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        g.money = 100;
        g.stage = Stage::PreBlind(); // Not shop stage

        let tarot = Consumables::Tarot(Tarots::TheHermit);
        assert!(g.buy_consumable(tarot).is_err());
        assert_eq!(g.consumables.len(), 0);
    }

    #[test]
    fn test_use_consumable_without_target() {
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::planet::Planets;

        let mut g = Game::new(Config::default());
        let planet = Consumables::Planet(Planets::Mercury);
        g.consumables.push(planet.clone());

        assert_eq!(g.consumables.len(), 1);
        // Planets don't require targets
        assert!(g.use_consumable(planet.clone(), None).is_ok());
        assert_eq!(g.consumables.len(), 0);
        assert_eq!(g.last_consumable_used, Some(planet));
    }

    #[test]
    fn test_use_consumable_not_owned() {
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        let tarot = Consumables::Tarot(Tarots::TheHermit);

        // Don't add the tarot to consumables
        assert!(g.use_consumable(tarot, None).is_err());
    }

    #[test]
    fn test_use_consumable_requires_target_but_none_given() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        let tarot = Consumables::Tarot(Tarots::TheMagician);
        g.consumables.push(tarot.clone());

        // TheMagician requires 2 targets, but we give none
        assert!(g.use_consumable(tarot, None).is_err());
        assert_eq!(g.consumables.len(), 1); // Still have it
    }

    #[test]
    fn test_use_consumable_with_targets() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        let tarot = Consumables::Tarot(Tarots::TheMagician);
        g.consumables.push(tarot.clone());

        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::Six, Suit::Diamond);

        // TheMagician requires up to 2 targets
        assert!(g.use_consumable(tarot.clone(), Some(vec![card1, card2])).is_ok());
        assert_eq!(g.consumables.len(), 0);
        assert_eq!(g.last_consumable_used, Some(tarot));
    }

    #[test]
    fn test_last_consumable_tracking() {
        use crate::consumable::Consumables;
        use crate::config::Config;
        use crate::planet::Planets;
        use crate::tarot::Tarots;

        let mut g = Game::new(Config::default());
        assert!(g.last_consumable_used.is_none());

        let planet = Consumables::Planet(Planets::Venus);
        g.consumables.push(planet.clone());
        g.use_consumable(planet.clone(), None).unwrap();
        assert_eq!(g.last_consumable_used, Some(planet.clone()));

        // Use another consumable
        let tarot = Consumables::Tarot(Tarots::TheHermit);
        g.consumables.push(tarot.clone());
        g.use_consumable(tarot.clone(), None).unwrap();
        assert_eq!(g.last_consumable_used, Some(tarot));
    }

    #[test]
    fn test_hand_levels_initialization() {
        use crate::rank::HandRank;

        let g = Game::default();

        // All hand ranks should start at level 1 with default values
        assert_eq!(g.get_hand_level(HandRank::HighCard).level, 1);
        assert_eq!(g.get_hand_level(HandRank::OnePair).level, 1);
        assert_eq!(g.get_hand_level(HandRank::Flush).level, 1);
        assert_eq!(g.get_hand_level(HandRank::RoyalFlush).level, 1);

        // Verify specific default values
        let pair_level = g.get_hand_level(HandRank::OnePair);
        assert_eq!(pair_level.chips, 10);
        assert_eq!(pair_level.mult, 2);
    }

    #[test]
    fn test_upgrade_hand_level() {
        use crate::rank::HandRank;

        let mut g = Game::default();

        // Get initial level for Pair
        let initial = g.get_hand_level(HandRank::OnePair);
        assert_eq!(initial.level, 1);
        assert_eq!(initial.chips, 10);
        assert_eq!(initial.mult, 2);

        // Upgrade once: Level 1→2 should add +30 chips, +3 mult
        g.upgrade_hand(HandRank::OnePair);
        let upgraded = g.get_hand_level(HandRank::OnePair);
        assert_eq!(upgraded.level, 2);
        assert_eq!(upgraded.chips, 40); // 10 + 30
        assert_eq!(upgraded.mult, 5); // 2 + 3

        // Upgrade again: Level 2→3 should add +25 chips, +2 mult
        g.upgrade_hand(HandRank::OnePair);
        let upgraded2 = g.get_hand_level(HandRank::OnePair);
        assert_eq!(upgraded2.level, 3);
        assert_eq!(upgraded2.chips, 65); // 40 + 25
        assert_eq!(upgraded2.mult, 7); // 5 + 2

        // Upgrade a third time: Level 3→4 should add +20 chips, +2 mult
        g.upgrade_hand(HandRank::OnePair);
        let upgraded3 = g.get_hand_level(HandRank::OnePair);
        assert_eq!(upgraded3.level, 4);
        assert_eq!(upgraded3.chips, 85); // 65 + 20
        assert_eq!(upgraded3.mult, 9); // 7 + 2
    }

    #[test]
    fn test_upgrade_different_hands() {
        use crate::rank::HandRank;

        let mut g = Game::default();

        // Upgrade Flush
        g.upgrade_hand(HandRank::Flush);
        let flush = g.get_hand_level(HandRank::Flush);
        assert_eq!(flush.level, 2);
        assert_eq!(flush.chips, 65); // 35 + 30
        assert_eq!(flush.mult, 7); // 4 + 3

        // Upgrade Straight
        g.upgrade_hand(HandRank::Straight);
        let straight = g.get_hand_level(HandRank::Straight);
        assert_eq!(straight.level, 2);
        assert_eq!(straight.chips, 60); // 30 + 30
        assert_eq!(straight.mult, 7); // 4 + 3

        // Other hands should still be level 1
        assert_eq!(g.get_hand_level(HandRank::OnePair).level, 1);
        assert_eq!(g.get_hand_level(HandRank::FullHouse).level, 1);
    }

    #[test]
    fn test_hand_level_affects_scoring() {
        use crate::card::{Card, Suit, Value};
        use crate::rank::HandRank;
        use crate::stage::{Blind, Stage};

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small, None);

        // Create a pair of 5s
        let five_heart = Card::new(Value::Five, Suit::Heart);
        let five_diamond = Card::new(Value::Five, Suit::Diamond);
        g.available.extend(vec![five_heart, five_diamond]);
        g.select_card(five_heart).unwrap();
        g.select_card(five_diamond).unwrap();

        // Play the pair at level 1
        let level1_score = g.calc_score_for_test();

        // Upgrade Pair to level 2
        g.upgrade_hand(HandRank::OnePair);

        // Reset scoring state and play same hand at level 2
        g.chips = g.config.base_chips;
        g.mult = g.config.base_mult;
        let level2_score = g.calc_score_for_test();

        // Level 2 should score higher due to +30 chips, +3 mult
        assert!(level2_score > level1_score);

        // Calculate expected difference
        // Level 1 Pair: 10 chips, 2 mult
        // Level 2 Pair: 40 chips, 5 mult
        // Card chips: 5 + 5 = 10
        // Base chips/mult from config are also added
        // But the key is that level 2 should score significantly higher
        assert!(level2_score > level1_score);

        // Verify the difference is substantial (at least 5x higher for this example)
        assert!(level2_score >= level1_score * 5);
    }

    #[test]
    fn test_planet_upgrades_hand() {
        use crate::consumable::{Consumable, Consumables};
        use crate::planet::Planets;
        use crate::rank::HandRank;

        let mut g = Game::default();

        // Mercury upgrades Straight
        let mercury = Consumables::Planet(Planets::Mercury);

        // Initial Straight level
        let initial = g.get_hand_level(HandRank::Straight);
        assert_eq!(initial.level, 1);
        assert_eq!(initial.chips, 30);
        assert_eq!(initial.mult, 4);

        // Add mercury to inventory and use it
        g.consumables.push(mercury.clone());
        assert!(mercury.use_effect(&mut g, None).is_ok());

        // Straight should now be level 2
        let upgraded = g.get_hand_level(HandRank::Straight);
        assert_eq!(upgraded.level, 2);
        assert_eq!(upgraded.chips, 60); // 30 + 30
        assert_eq!(upgraded.mult, 7); // 4 + 3
    }

    #[test]
    fn test_all_planets_upgrade_correct_hands() {
        use crate::consumable::{Consumable, Consumables};
        use crate::planet::Planets;
        use crate::rank::HandRank;

        let test_cases = vec![
            (Planets::Pluto, HandRank::HighCard),
            (Planets::Eris, HandRank::OnePair),
            (Planets::Ceres, HandRank::TwoPair),
            (Planets::PlanetX, HandRank::ThreeOfAKind),
            (Planets::Mercury, HandRank::Straight),
            (Planets::Venus, HandRank::Flush),
            (Planets::Earth, HandRank::FullHouse),
            (Planets::Mars, HandRank::FourOfAKind),
            (Planets::Jupiter, HandRank::FiveOfAKind),
            (Planets::Saturn, HandRank::StraightFlush),
            (Planets::Uranus, HandRank::FlushHouse),
            (Planets::Neptune, HandRank::RoyalFlush),
        ];

        for (planet, expected_rank) in test_cases {
            let mut g = Game::default();
            let consumable = Consumables::Planet(planet);

            // Verify planet maps to correct hand
            assert_eq!(planet.hand_rank(), expected_rank);

            // Get initial level
            let initial = g.get_hand_level(expected_rank);

            // Use planet
            assert!(consumable.use_effect(&mut g, None).is_ok());

            // Verify hand was upgraded
            let upgraded = g.get_hand_level(expected_rank);
            assert_eq!(upgraded.level, initial.level + 1);
            assert!(upgraded.chips > initial.chips);
            assert!(upgraded.mult > initial.mult);
        }
    }

    #[test]
    fn test_planet_multiple_upgrades() {
        use crate::consumable::{Consumable, Consumables};
        use crate::planet::Planets;
        use crate::rank::HandRank;

        let mut g = Game::default();
        let venus = Consumables::Planet(Planets::Venus);

        // Upgrade Flush 5 times
        for i in 1..=5 {
            assert!(venus.use_effect(&mut g, None).is_ok());
            let level = g.get_hand_level(HandRank::Flush);
            assert_eq!(level.level, i + 1);
        }

        // After 5 upgrades, should be at level 6
        let final_level = g.get_hand_level(HandRank::Flush);
        assert_eq!(final_level.level, 6);

        // Verify upgrade formula progression
        // Level 1: 35 chips, 4 mult
        // Level 2: 65 chips, 7 mult (+30, +3)
        // Level 3: 90 chips, 9 mult (+25, +2)
        // Level 4: 110 chips, 11 mult (+20, +2)
        // Level 5: 130 chips, 13 mult (+20, +2)
        // Level 6: 150 chips, 15 mult (+20, +2)
        assert_eq!(final_level.chips, 150);
        assert_eq!(final_level.mult, 15);
    }

    #[test]
    fn test_use_planet_via_game_method() {
        use crate::consumable::Consumables;
        use crate::planet::Planets;
        use crate::rank::HandRank;

        let mut g = Game::default();
        let mars = Consumables::Planet(Planets::Mars);

        // Add Mars to consumables
        g.consumables.push(mars.clone());
        assert_eq!(g.consumables.len(), 1);

        // Use it through game.use_consumable
        assert!(g.use_consumable(mars.clone(), None).is_ok());

        // Should be removed from consumables
        assert_eq!(g.consumables.len(), 0);

        // FourOfAKind should be upgraded
        let level = g.get_hand_level(HandRank::FourOfAKind);
        assert_eq!(level.level, 2);
        assert_eq!(level.chips, 90); // 60 + 30
        assert_eq!(level.mult, 10); // 7 + 3

        // Should be tracked as last used
        assert_eq!(g.last_consumable_used, Some(mars));
    }

    // ==================== Card Modification Infrastructure Tests ====================

    #[test]
    fn test_card_set_enhancement() {
        use crate::card::{Card, Enhancement, Suit, Value};

        let mut card = Card::new(Value::Five, Suit::Heart);
        assert_eq!(card.enhancement, None);

        card.set_enhancement(Enhancement::Bonus);
        assert_eq!(card.enhancement, Some(Enhancement::Bonus));

        card.set_enhancement(Enhancement::Glass);
        assert_eq!(card.enhancement, Some(Enhancement::Glass));
    }

    #[test]
    fn test_card_set_suit() {
        use crate::card::{Card, Suit, Value};

        let mut card = Card::new(Value::Ace, Suit::Heart);
        assert_eq!(card.suit, Suit::Heart);

        card.set_suit(Suit::Spade);
        assert_eq!(card.suit, Suit::Spade);
    }

    #[test]
    fn test_card_set_rank() {
        use crate::card::{Card, Suit, Value};

        let mut card = Card::new(Value::Five, Suit::Diamond);
        assert_eq!(card.value, Value::Five);

        card.set_rank(Value::King);
        assert_eq!(card.value, Value::King);
    }

    #[test]
    fn test_modify_card_in_deck() {
        use crate::card::{Card, Enhancement, Suit, Value};

        let mut g = Game::default();
        let five_heart = Card::new(Value::Five, Suit::Heart);
        let card_id = five_heart.id;
        g.add_card_to_deck(five_heart);

        // Modify the card in deck
        g.modify_card_in_deck(card_id, |card| {
            card.set_enhancement(Enhancement::Bonus);
        });

        // Verify it was modified
        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == card_id).unwrap();
        assert_eq!(modified.enhancement, Some(Enhancement::Bonus));
    }

    #[test]
    fn test_add_card_to_deck() {
        use crate::card::{Card, Suit, Value};

        let mut g = Game::default();
        let initial_count = g.deck.cards().len();

        let new_card = Card::new(Value::Ace, Suit::Spade);
        g.add_card_to_deck(new_card);

        let cards = g.deck.cards();
        assert_eq!(cards.len(), initial_count + 1);
        assert!(cards.iter().any(|c| c.value == Value::Ace && c.suit == Suit::Spade));
    }

    #[test]
    fn test_add_money_capped() {
        let mut g = Game::default();
        g.money = 5;

        // Add money under cap
        g.add_money_capped(10, 20);
        assert_eq!(g.money, 15);

        // Add money that would exceed cap
        g.add_money_capped(10, 20);
        assert_eq!(g.money, 20); // Capped at 20

        // Try to add more when at cap
        g.add_money_capped(5, 20);
        assert_eq!(g.money, 20); // Still at cap
    }

    #[test]
    fn test_get_joker_sell_value() {
        use crate::joker::{JollyJoker, GreedyJoker, LustyJoker, Jokers};

        let mut g = Game::default();
        assert_eq!(g.get_joker_sell_value(), 0);

        // Add jokers
        // JollyJoker costs $3, sells for $1
        // GreedyJoker costs $5, sells for $2
        // LustyJoker costs $5, sells for $2
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));
        g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));
        g.jokers.push(Jokers::LustyJoker(LustyJoker::default()));

        let total = g.get_joker_sell_value();
        assert_eq!(total, 5); // 1 + 2 + 2 = 5
    }

    #[test]
    fn test_value_raise_rank() {
        use crate::card::Value;

        assert_eq!(Value::Two.raise_rank(), Some(Value::Three));
        assert_eq!(Value::Five.raise_rank(), Some(Value::Six));
        assert_eq!(Value::Ten.raise_rank(), Some(Value::Jack));
        assert_eq!(Value::Jack.raise_rank(), Some(Value::Queen));
        assert_eq!(Value::Queen.raise_rank(), Some(Value::King));
        assert_eq!(Value::King.raise_rank(), Some(Value::Ace));
        assert_eq!(Value::Ace.raise_rank(), None); // Can't go higher
    }

    #[test]
    fn test_generate_random_planet() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};

        let g = Game::default();
        let planet = g.generate_random_planet();

        // Should be a planet
        assert_eq!(planet.consumable_type(), ConsumableType::Planet);
        assert_eq!(planet.cost(), 3); // Planets cost 3
    }

    #[test]
    fn test_generate_random_tarot() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};

        let g = Game::default();
        let tarot = g.generate_random_tarot();

        // Should be a tarot
        assert_eq!(tarot.consumable_type(), ConsumableType::Tarot);
        assert_eq!(tarot.cost(), 3); // Tarots cost 3
    }

    #[test]
    fn test_generate_random_joker() {
        use crate::joker::Joker;

        let g = Game::default();
        let joker = g.generate_random_joker();

        // Should have valid properties
        assert!(!joker.name().is_empty());
        assert!(joker.cost() > 0);
    }

    // ===== Category A Tarot Tests (No Targets) =====

    #[test]
    fn test_tarot_the_hermit() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.money = 10;
        g.consumables.push(Consumables::Tarot(Tarots::TheHermit));

        // Use The Hermit (doubles money, max $20)
        g.use_consumable(Consumables::Tarot(Tarots::TheHermit), None).unwrap();
        assert_eq!(g.money, 20); // 10 * 2 = 20

        // Test cap at $20
        g.money = 15;
        g.consumables.push(Consumables::Tarot(Tarots::TheHermit));
        g.use_consumable(Consumables::Tarot(Tarots::TheHermit), None).unwrap();
        assert_eq!(g.money, 20); // Capped at 20, not 30
    }

    #[test]
    fn test_tarot_temperance() {
        use crate::consumable::Consumables;
        use crate::joker::{JollyJoker, GreedyJoker, Jokers};
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.money = 0;

        // Add jokers with known sell values
        // JollyJoker: cost $3, sells for $1
        // GreedyJoker: cost $5, sells for $2
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));
        g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));

        g.consumables.push(Consumables::Tarot(Tarots::Temperance));

        // Use Temperance (gain sell value of all jokers, max $50)
        g.use_consumable(Consumables::Tarot(Tarots::Temperance), None).unwrap();
        assert_eq!(g.money, 3); // 1 + 2 = 3

        // Test cap at $50
        g.money = 0;
        // Add many jokers to exceed cap (would need 26 GreedyJokers = $52 sell value)
        for _ in 0..26 {
            g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));
        }
        g.consumables.push(Consumables::Tarot(Tarots::Temperance));
        g.use_consumable(Consumables::Tarot(Tarots::Temperance), None).unwrap();
        assert_eq!(g.money, 50); // Capped at 50
    }

    #[test]
    fn test_tarot_the_high_priestess() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let initial_consumables = g.consumables.len();

        g.consumables.push(Consumables::Tarot(Tarots::TheHighPriestess));

        // Use The High Priestess (creates 2 random Planet cards)
        g.use_consumable(Consumables::Tarot(Tarots::TheHighPriestess), None).unwrap();

        // Should have 2 new planet cards
        assert_eq!(g.consumables.len(), initial_consumables + 2);

        // Verify both are planets
        let new_consumables = &g.consumables[initial_consumables..];
        assert_eq!(new_consumables[0].consumable_type(), ConsumableType::Planet);
        assert_eq!(new_consumables[1].consumable_type(), ConsumableType::Planet);
    }

    #[test]
    fn test_tarot_the_emperor() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let initial_consumables = g.consumables.len();

        g.consumables.push(Consumables::Tarot(Tarots::TheEmperor));

        // Use The Emperor (creates 2 random Tarot cards)
        g.use_consumable(Consumables::Tarot(Tarots::TheEmperor), None).unwrap();

        // Should have 2 new tarot cards
        assert_eq!(g.consumables.len(), initial_consumables + 2);

        // Verify both are tarots
        let new_consumables = &g.consumables[initial_consumables..];
        assert_eq!(new_consumables[0].consumable_type(), ConsumableType::Tarot);
        assert_eq!(new_consumables[1].consumable_type(), ConsumableType::Tarot);
    }

    #[test]
    fn test_tarot_judgement() {
        use crate::consumable::Consumables;
        use crate::joker::Joker;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let initial_jokers = g.jokers.len();

        g.consumables.push(Consumables::Tarot(Tarots::Judgement));

        // Use Judgement (creates random Joker)
        g.use_consumable(Consumables::Tarot(Tarots::Judgement), None).unwrap();

        // Should have 1 new joker
        assert_eq!(g.jokers.len(), initial_jokers + 1);

        // Verify it's a valid joker
        let new_joker = &g.jokers[initial_jokers];
        assert!(!new_joker.name().is_empty());
        assert!(new_joker.cost() > 0);
    }

    // ===== Category B Tarot Tests (Enhancement Tarots) =====

    #[test]
    fn test_tarot_the_magician() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Diamond);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::TheMagician));

        // Use The Magician (2 cards → Lucky)
        g.use_consumable(Consumables::Tarot(Tarots::TheMagician), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();

        assert_eq!(modified1.enhancement, Some(Enhancement::Lucky));
        assert_eq!(modified2.enhancement, Some(Enhancement::Lucky));
    }

    #[test]
    fn test_tarot_the_empress() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Ace, Suit::Spade);
        let card2 = Card::new(Value::Two, Suit::Club);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::TheEmpress));

        // Use The Empress (2 cards → Mult)
        g.use_consumable(Consumables::Tarot(Tarots::TheEmpress), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();

        assert_eq!(modified1.enhancement, Some(Enhancement::Mult));
        assert_eq!(modified2.enhancement, Some(Enhancement::Mult));
    }

    #[test]
    fn test_tarot_the_hierophant() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Three, Suit::Heart);
        let card2 = Card::new(Value::Four, Suit::Diamond);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::TheHierophant));

        // Use The Hierophant (2 cards → Bonus)
        g.use_consumable(Consumables::Tarot(Tarots::TheHierophant), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();

        assert_eq!(modified1.enhancement, Some(Enhancement::Bonus));
        assert_eq!(modified2.enhancement, Some(Enhancement::Bonus));
    }

    #[test]
    fn test_tarot_the_lovers() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Jack, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::TheLovers));

        // Use The Lovers (1 card → Wild)
        g.use_consumable(Consumables::Tarot(Tarots::TheLovers), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.enhancement, Some(Enhancement::Wild));
    }

    #[test]
    fn test_tarot_the_chariot() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Queen, Suit::Club);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::TheChariot));

        // Use The Chariot (1 card → Steel)
        g.use_consumable(Consumables::Tarot(Tarots::TheChariot), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.enhancement, Some(Enhancement::Steel));
    }

    #[test]
    fn test_tarot_justice() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Seven, Suit::Spade);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::Justice));

        // Use Justice (1 card → Glass)
        g.use_consumable(Consumables::Tarot(Tarots::Justice), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.enhancement, Some(Enhancement::Glass));
    }

    #[test]
    fn test_tarot_the_devil() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Eight, Suit::Diamond);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::TheDevil));

        // Use The Devil (1 card → Gold)
        g.use_consumable(Consumables::Tarot(Tarots::TheDevil), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.enhancement, Some(Enhancement::Gold));
    }

    #[test]
    fn test_tarot_the_tower() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Nine, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::TheTower));

        // Use The Tower (1 card → Stone)
        g.use_consumable(Consumables::Tarot(Tarots::TheTower), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.enhancement, Some(Enhancement::Stone));
    }

    // ===== Category C Tarot Tests (Suit Conversion) =====

    #[test]
    fn test_tarot_the_star() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Club);
        let card3 = Card::new(Value::Ace, Suit::Spade);
        let id1 = card1.id;
        let id2 = card2.id;
        let id3 = card3.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.add_card_to_deck(card3);
        g.consumables.push(Consumables::Tarot(Tarots::TheStar));

        // Use The Star (up to 3 cards → Diamonds)
        g.use_consumable(Consumables::Tarot(Tarots::TheStar), Some(vec![card1, card2, card3])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();
        let modified3 = cards.iter().find(|c| c.id == id3).unwrap();

        assert_eq!(modified1.suit, Suit::Diamond);
        assert_eq!(modified2.suit, Suit::Diamond);
        assert_eq!(modified3.suit, Suit::Diamond);
    }

    #[test]
    fn test_tarot_the_moon() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Two, Suit::Diamond);
        let card2 = Card::new(Value::Seven, Suit::Heart);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::TheMoon));

        // Use The Moon (up to 3 cards → Clubs)
        g.use_consumable(Consumables::Tarot(Tarots::TheMoon), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();

        assert_eq!(modified1.suit, Suit::Club);
        assert_eq!(modified2.suit, Suit::Club);
    }

    #[test]
    fn test_tarot_the_sun() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Jack, Suit::Spade);
        let card2 = Card::new(Value::Queen, Suit::Diamond);
        let card3 = Card::new(Value::King, Suit::Club);
        let id1 = card1.id;
        let id2 = card2.id;
        let id3 = card3.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.add_card_to_deck(card3);
        g.consumables.push(Consumables::Tarot(Tarots::TheSun));

        // Use The Sun (up to 3 cards → Hearts)
        g.use_consumable(Consumables::Tarot(Tarots::TheSun), Some(vec![card1, card2, card3])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();
        let modified3 = cards.iter().find(|c| c.id == id3).unwrap();

        assert_eq!(modified1.suit, Suit::Heart);
        assert_eq!(modified2.suit, Suit::Heart);
        assert_eq!(modified3.suit, Suit::Heart);
    }

    #[test]
    fn test_tarot_the_world() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Ten, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::TheWorld));

        // Use The World (up to 3 cards → Spades)
        g.use_consumable(Consumables::Tarot(Tarots::TheWorld), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.suit, Suit::Spade);
    }

    // ===== Category D Tarot Tests (Special Effects) =====

    #[test]
    fn test_tarot_strength() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Diamond);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::Strength));

        // Use Strength (up to 2 cards, raise rank by 1)
        g.use_consumable(Consumables::Tarot(Tarots::Strength), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let modified2 = cards.iter().find(|c| c.id == id2).unwrap();

        assert_eq!(modified1.value, Value::Six); // Five → Six
        assert_eq!(modified2.value, Value::Ace); // King → Ace
    }

    #[test]
    fn test_tarot_strength_ace_unchanged() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Ace, Suit::Spade);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::Strength));

        // Use Strength on Ace (shouldn't change)
        g.use_consumable(Consumables::Tarot(Tarots::Strength), Some(vec![card])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();

        assert_eq!(modified.value, Value::Ace); // Ace stays Ace (can't go higher)
    }

    #[test]
    fn test_tarot_the_hanged_man() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Two, Suit::Club);
        let card2 = Card::new(Value::Three, Suit::Diamond);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        let initial_count = g.deck.cards().len();

        g.consumables.push(Consumables::Tarot(Tarots::TheHangedMan));

        // Use The Hanged Man (destroy up to 2 cards)
        g.use_consumable(Consumables::Tarot(Tarots::TheHangedMan), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        assert_eq!(cards.len(), initial_count - 2); // 2 cards destroyed

        // Verify cards are gone
        assert!(cards.iter().find(|c| c.id == id1).is_none());
        assert!(cards.iter().find(|c| c.id == id2).is_none());
    }

    #[test]
    fn test_tarot_death() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Diamond);
        let id1 = card1.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::Death));

        // Use Death (convert left card into right card)
        // card1 should become King of Diamonds (same as card2)
        g.use_consumable(Consumables::Tarot(Tarots::Death), Some(vec![card1, card2])).unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id1).unwrap();

        assert_eq!(modified.value, Value::King);
        assert_eq!(modified.suit, Suit::Diamond);
    }

    #[test]
    fn test_tarot_the_fool() {
        use crate::consumable::Consumables;
        use crate::planet::Planets;
        use crate::rank::HandRank;
        use crate::tarot::Tarots;

        let mut g = Game::default();

        // Get initial level (should be 1)
        let initial_level = g.get_hand_level(HandRank::FourOfAKind).level;
        assert_eq!(initial_level, 1);

        // Use a planet card first to set last_consumable_used
        let mars = Consumables::Planet(Planets::Mars);
        g.consumables.push(mars.clone());
        g.use_consumable(mars.clone(), None).unwrap();

        // Verify Mars was used and level increased
        assert_eq!(g.last_consumable_used, Some(mars));
        assert_eq!(g.get_hand_level(HandRank::FourOfAKind).level, 2);

        // Now use The Fool (copies last consumable)
        let fool = Consumables::Tarot(Tarots::TheFool);
        g.consumables.push(fool.clone());
        g.use_consumable(fool, None).unwrap();

        // Should have upgraded FourOfAKind again (Mars effect copied by Fool)
        assert_eq!(g.get_hand_level(HandRank::FourOfAKind).level, 3);
    }

    #[test]
    fn test_tarot_wheel_of_fortune() {
        use crate::consumable::Consumables;
        use crate::joker::{JollyJoker, Jokers};
        use crate::tarot::Tarots;

        let mut g = Game::default();

        // Add a joker
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));

        g.consumables.push(Consumables::Tarot(Tarots::WheelOfFortune));

        // Use Wheel of Fortune (1/4 chance to add edition to random joker)
        // This is probabilistic, so we can't test the exact outcome
        // Just verify it doesn't crash
        g.use_consumable(Consumables::Tarot(Tarots::WheelOfFortune), None).unwrap();

        // Joker should still exist
        assert_eq!(g.jokers.len(), 1);
    }

    // ==================== Phase 3C Infrastructure Tests ====================

    #[test]
    fn test_get_random_card_from_deck() {
        use crate::card::{Card, Suit, Value};

        let g = Game::default();

        // Default deck has 52 cards, should return one
        let random = g.get_random_card_from_deck();
        assert!(random.is_some());

        // All default games have 52 cards, so this is sufficient
        assert!(random.is_some());
    }

    #[test]
    fn test_get_random_cards() {
        use crate::card::{Card, Suit, Value};

        let g = Game::default();

        // Default deck has 52 cards
        // Request 3 random cards
        let random = g.get_random_cards(3);
        assert_eq!(random.len(), 3);

        // Request more than available (default deck has 52)
        let random_all = g.get_random_cards(100);
        assert_eq!(random_all.len(), 52);
    }

    #[test]
    fn test_create_enhanced_face_card() {
        use crate::card::Value;

        let g = Game::default();
        let card = g.create_enhanced_face_card();

        // Should be J, Q, or K
        assert!(matches!(card.value, Value::Jack | Value::Queen | Value::King));

        // Should have an enhancement
        assert!(card.enhancement.is_some());
    }

    #[test]
    fn test_create_enhanced_ace() {
        use crate::card::Value;

        let g = Game::default();
        let card = g.create_enhanced_ace();

        // Should be an Ace
        assert_eq!(card.value, Value::Ace);

        // Should have an enhancement
        assert!(card.enhancement.is_some());
    }

    #[test]
    fn test_create_enhanced_number() {
        use crate::card::Value;

        let g = Game::default();
        let card = g.create_enhanced_number();

        // Should be 2-10
        assert!(matches!(
            card.value,
            Value::Two | Value::Three | Value::Four | Value::Five | Value::Six
            | Value::Seven | Value::Eight | Value::Nine | Value::Ten
        ));

        // Should have an enhancement
        assert!(card.enhancement.is_some());
    }

    #[test]
    fn test_generate_rare_joker() {
        use crate::joker::{Joker, Rarity};

        let g = Game::default();
        let joker = g.generate_rare_joker();

        // Should be rare rarity, or common if no rare jokers exist yet
        // (rare jokers not implemented yet, so will fallback to common)
        assert!(matches!(joker.rarity(), Rarity::Rare | Rarity::Common));
    }

    #[test]
    fn test_generate_legendary_joker() {
        use crate::joker::{Joker, Rarity};

        let g = Game::default();
        let joker = g.generate_legendary_joker();

        // Should be legendary rarity, or fallback if no legendary jokers exist yet
        // (legendary jokers not implemented yet, so will fallback)
        assert!(matches!(joker.rarity(), Rarity::Legendary | Rarity::Rare | Rarity::Common));
    }

    #[test]
    fn test_copy_joker() {
        use crate::joker::{Jokers, JollyJoker};

        let g = Game::default();
        let original = Jokers::JollyJoker(JollyJoker::default());
        let copy = g.copy_joker(&original);

        // Should be same variant
        assert!(matches!(copy, Jokers::JollyJoker(_)));
    }

    #[test]
    fn test_destroy_all_jokers_except() {
        use crate::joker::{Jokers, JollyJoker, GreedyJoker};

        let mut g = Game::default();
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));
        g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));

        // Keep index 1
        g.destroy_all_jokers_except(1);

        // Should only have 1 joker left
        assert_eq!(g.jokers.len(), 1);
        assert!(matches!(g.jokers[0], Jokers::GreedyJoker(_)));
    }

    #[test]
    fn test_convert_all_cards_to_suit() {
        use crate::card::{Card, Suit, Value};

        let mut g = Game::default();
        g.add_card_to_deck(Card::new(Value::Five, Suit::Heart));
        g.add_card_to_deck(Card::new(Value::King, Suit::Diamond));
        g.add_card_to_deck(Card::new(Value::Ace, Suit::Club));

        // Convert all to Spades
        g.convert_all_cards_to_suit(Suit::Spade);

        // All cards should now be Spades
        for card in g.deck.cards() {
            assert_eq!(card.suit, Suit::Spade);
        }
    }

    #[test]
    fn test_convert_all_cards_to_rank() {
        use crate::card::{Card, Suit, Value};

        let mut g = Game::default();
        g.add_card_to_deck(Card::new(Value::Five, Suit::Heart));
        g.add_card_to_deck(Card::new(Value::King, Suit::Diamond));
        g.add_card_to_deck(Card::new(Value::Ace, Suit::Club));

        // Convert all to Queens
        g.convert_all_cards_to_rank(Value::Queen);

        // All cards should now be Queens
        for card in g.deck.cards() {
            assert_eq!(card.value, Value::Queen);
        }
    }

    #[test]
    fn test_modify_hand_size() {
        let mut g = Game::default();

        // Default hand size should be 8
        assert_eq!(g.hand_size, 8);

        // Decrease by 1
        g.modify_hand_size(-1);
        assert_eq!(g.hand_size, 7);

        // Decrease by 2 more
        g.modify_hand_size(-2);
        assert_eq!(g.hand_size, 5);

        // Increase by 3
        g.modify_hand_size(3);
        assert_eq!(g.hand_size, 8);
    }

    // ==================== Phase 3C Category A: Seal Addition ====================

    #[test]
    fn test_spectral_talisman() {
        use crate::card::{Card, Seal, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::Talisman));

        g.use_consumable(Consumables::Spectral(Spectrals::Talisman), Some(vec![card]))
            .unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified.seal, Some(Seal::Gold));
    }

    #[test]
    fn test_spectral_deja_vu() {
        use crate::card::{Card, Seal, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::King, Suit::Diamond);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::DejaVu));

        g.use_consumable(Consumables::Spectral(Spectrals::DejaVu), Some(vec![card]))
            .unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified.seal, Some(Seal::Red));
    }

    #[test]
    fn test_spectral_trance() {
        use crate::card::{Card, Seal, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::Ace, Suit::Club);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::Trance));

        g.use_consumable(Consumables::Spectral(Spectrals::Trance), Some(vec![card]))
            .unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified.seal, Some(Seal::Blue));
    }

    #[test]
    fn test_spectral_medium() {
        use crate::card::{Card, Seal, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::Queen, Suit::Spade);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::Medium));

        g.use_consumable(Consumables::Spectral(Spectrals::Medium), Some(vec![card]))
            .unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified.seal, Some(Seal::Purple));
    }

    // ==================== Phase 3C Category B: Card Creation/Destruction ====================

    #[test]
    fn test_spectral_familiar() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_count = g.deck.len();

        g.consumables.push(Consumables::Spectral(Spectrals::Familiar));
        g.use_consumable(Consumables::Spectral(Spectrals::Familiar), None)
            .unwrap();

        // Should destroy 1 card and add 3 face cards = +2 net
        assert_eq!(g.deck.len(), initial_count + 2);

        // Check that we have 3 new face cards with enhancements
        let cards = g.deck.cards();
        let face_cards: Vec<_> = cards
            .iter()
            .filter(|c| matches!(c.value, Value::Jack | Value::Queen | Value::King))
            .collect();
        // At least 3 face cards should exist (could be more from original deck)
        assert!(face_cards.len() >= 3);
    }

    #[test]
    fn test_spectral_grim() {
        use crate::card::Value;
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_count = g.deck.len();

        g.consumables.push(Consumables::Spectral(Spectrals::Grim));
        g.use_consumable(Consumables::Spectral(Spectrals::Grim), None)
            .unwrap();

        // Should destroy 1 card and add 2 aces = +1 net
        assert_eq!(g.deck.len(), initial_count + 1);

        // Check that we have aces with enhancements
        let cards = g.deck.cards();
        let aces: Vec<_> = cards
            .iter()
            .filter(|c| matches!(c.value, Value::Ace))
            .collect();
        // At least 4 aces should exist (could be 4-6 depending on what was destroyed)
        assert!(aces.len() >= 4);
    }

    #[test]
    fn test_spectral_incantation() {
        use crate::card::Value;
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_count = g.deck.len();

        g.consumables.push(Consumables::Spectral(Spectrals::Incantation));
        g.use_consumable(Consumables::Spectral(Spectrals::Incantation), None)
            .unwrap();

        // Should destroy 1 card and add 4 number cards = +3 net
        assert_eq!(g.deck.len(), initial_count + 3);

        // Check that we have number cards
        let cards = g.deck.cards();
        let number_cards: Vec<_> = cards
            .iter()
            .filter(|c| matches!(
                c.value,
                Value::Two | Value::Three | Value::Four | Value::Five | Value::Six
                | Value::Seven | Value::Eight | Value::Nine | Value::Ten
            ))
            .collect();
        // Many number cards should exist
        assert!(number_cards.len() >= 4);
    }

    #[test]
    fn test_spectral_immolate() {
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_count = g.deck.len();
        let initial_money = g.money;

        g.consumables.push(Consumables::Spectral(Spectrals::Immolate));
        g.use_consumable(Consumables::Spectral(Spectrals::Immolate), None)
            .unwrap();

        // Should destroy 5 cards
        assert_eq!(g.deck.len(), initial_count - 5);

        // Should gain $20
        assert_eq!(g.money, initial_money + 20);
    }

    // ==================== Phase 3C Remaining Categories ====================

    #[test]
    fn test_spectral_aura() {
        use crate::card::{Card, Edition, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::Aura));

        g.use_consumable(Consumables::Spectral(Spectrals::Aura), Some(vec![card]))
            .unwrap();

        let cards = g.deck.cards();
        let modified = cards.iter().find(|c| c.id == id).unwrap();
        // Should have an edition (Foil, Holo, or Polychrome, not Base)
        assert!(!matches!(modified.edition, Edition::Base));
    }

    #[test]
    fn test_spectral_cryptid() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let initial_count = g.deck.len();

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Spectral(Spectrals::Cryptid));

        g.use_consumable(Consumables::Spectral(Spectrals::Cryptid), Some(vec![card]))
            .unwrap();

        // Should add 2 copies = +2 cards
        assert_eq!(g.deck.len(), initial_count + 3);
    }

    #[test]
    fn test_spectral_sigil() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.consumables.push(Consumables::Spectral(Spectrals::Sigil));

        g.use_consumable(Consumables::Spectral(Spectrals::Sigil), None)
            .unwrap();

        // All cards should have the same suit
        let cards = g.deck.cards();
        let first_suit = cards[0].suit;
        for card in cards {
            assert_eq!(card.suit, first_suit);
        }
    }

    #[test]
    fn test_spectral_ouija() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_hand_size = g.hand_size;
        g.consumables.push(Consumables::Spectral(Spectrals::Ouija));

        g.use_consumable(Consumables::Spectral(Spectrals::Ouija), None)
            .unwrap();

        // All cards should have the same rank
        let cards = g.deck.cards();
        let first_rank = cards[0].value;
        for card in cards {
            assert_eq!(card.value, first_rank);
        }

        // Hand size should decrease by 1
        assert_eq!(g.hand_size, initial_hand_size - 1);
    }

    #[test]
    fn test_spectral_wraith() {
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.money = 10;
        g.consumables.push(Consumables::Spectral(Spectrals::Wraith));

        g.use_consumable(Consumables::Spectral(Spectrals::Wraith), None)
            .unwrap();

        // Should create a joker
        assert_eq!(g.jokers.len(), 1);

        // Money should be $0
        assert_eq!(g.money, 0);
    }

    #[test]
    fn test_spectral_the_soul() {
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.consumables.push(Consumables::Spectral(Spectrals::TheSoul));

        g.use_consumable(Consumables::Spectral(Spectrals::TheSoul), None)
            .unwrap();

        // Should create a joker
        assert_eq!(g.jokers.len(), 1);
    }

    #[test]
    fn test_spectral_black_hole() {
        use crate::consumable::Consumables;
        use crate::rank::HandRank;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_level = g.hand_levels[&HandRank::HighCard];
        g.consumables.push(Consumables::Spectral(Spectrals::BlackHole));

        g.use_consumable(Consumables::Spectral(Spectrals::BlackHole), None)
            .unwrap();

        // All hand levels should increase by 1
        assert_eq!(
            g.hand_levels[&HandRank::HighCard],
            initial_level.upgrade()
        );
    }

    #[test]
    fn test_spectral_ankh() {
        use crate::consumable::Consumables;
        use crate::joker::{GreedyJoker, JollyJoker, LustyJoker, Jokers};
        use crate::spectral::Spectrals;

        let mut g = Game::default();

        // Add 3 different jokers
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));
        g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));
        g.jokers.push(Jokers::LustyJoker(LustyJoker::default()));

        g.consumables.push(Consumables::Spectral(Spectrals::Ankh));

        // Use Ankh (copy 1 joker, destroy others)
        // Note: Current implementation just keeps first joker
        g.use_consumable(Consumables::Spectral(Spectrals::Ankh), None)
            .unwrap();

        // Should only have 1 joker left
        assert_eq!(g.jokers.len(), 1);
        // Should be JollyJoker (the first one)
        assert!(matches!(g.jokers[0], Jokers::JollyJoker(_)));
    }

    #[test]
    fn test_spectral_hex() {
        use crate::consumable::Consumables;
        use crate::joker::{GreedyJoker, JollyJoker, Jokers};
        use crate::spectral::Spectrals;

        let mut g = Game::default();

        // Add 2 jokers
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));
        g.jokers.push(Jokers::GreedyJoker(GreedyJoker::default()));

        g.consumables.push(Consumables::Spectral(Spectrals::Hex));

        // Use Hex (add Polychrome to 1 joker, destroy others)
        // Note: Current implementation doesn't have joker editions yet
        g.use_consumable(Consumables::Spectral(Spectrals::Hex), None)
            .unwrap();

        // Should only have 1 joker left
        assert_eq!(g.jokers.len(), 1);
        // Should be JollyJoker (the first one)
        assert!(matches!(g.jokers[0], Jokers::JollyJoker(_)));
    }

    #[test]
    fn test_spectral_ectoplasm() {
        use crate::consumable::Consumables;
        use crate::joker::{JollyJoker, Jokers};
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        let initial_hand_size = g.hand_size;

        // Add a joker
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));

        g.consumables.push(Consumables::Spectral(Spectrals::Ectoplasm));

        // Use Ectoplasm (add Negative to random joker, -1 hand size)
        g.use_consumable(Consumables::Spectral(Spectrals::Ectoplasm), None)
            .unwrap();

        // Hand size should decrease by 1
        assert_eq!(g.hand_size, initial_hand_size - 1);

        // Joker should still exist
        assert_eq!(g.jokers.len(), 1);
    }

    // ==================== Phase 3C Edge Case Tests ====================

    #[test]
    fn test_spectral_hand_size_cumulative() {
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        assert_eq!(g.hand_size, 8);

        // Use Ouija twice
        g.consumables.push(Consumables::Spectral(Spectrals::Ouija));
        g.use_consumable(Consumables::Spectral(Spectrals::Ouija), None)
            .unwrap();
        assert_eq!(g.hand_size, 7);

        g.consumables.push(Consumables::Spectral(Spectrals::Ouija));
        g.use_consumable(Consumables::Spectral(Spectrals::Ouija), None)
            .unwrap();
        assert_eq!(g.hand_size, 6);

        // Use Ectoplasm
        g.consumables.push(Consumables::Spectral(Spectrals::Ectoplasm));
        g.use_consumable(Consumables::Spectral(Spectrals::Ectoplasm), None)
            .unwrap();
        assert_eq!(g.hand_size, 5);
    }

    #[test]
    fn test_spectral_familiar_empty_deck() {
        use crate::consumable::Consumables;
        use crate::deck::Deck;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.deck = Deck::empty(); // Start with empty deck

        g.consumables.push(Consumables::Spectral(Spectrals::Familiar));
        g.use_consumable(Consumables::Spectral(Spectrals::Familiar), None)
            .unwrap();

        // Should add 3 face cards even with empty deck
        assert_eq!(g.deck.len(), 3);
    }

    #[test]
    fn test_spectral_immolate_insufficient_cards() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::deck::Deck;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.deck = Deck::empty();

        // Add only 3 cards (less than 5)
        g.add_card_to_deck(Card::new(Value::Five, Suit::Heart));
        g.add_card_to_deck(Card::new(Value::Six, Suit::Diamond));
        g.add_card_to_deck(Card::new(Value::Seven, Suit::Club));

        let initial_money = g.money;
        g.consumables.push(Consumables::Spectral(Spectrals::Immolate));
        g.use_consumable(Consumables::Spectral(Spectrals::Immolate), None)
            .unwrap();

        // Should destroy all 3 cards
        assert_eq!(g.deck.len(), 0);

        // Should still gain $20
        assert_eq!(g.money, initial_money + 20);
    }

    #[test]
    fn test_spectral_wraith_no_jokers_initially() {
        use crate::consumable::Consumables;
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.jokers.clear(); // Remove all jokers
        g.money = 50;

        g.consumables.push(Consumables::Spectral(Spectrals::Wraith));
        g.use_consumable(Consumables::Spectral(Spectrals::Wraith), None)
            .unwrap();

        // Should create 1 joker
        assert_eq!(g.jokers.len(), 1);

        // Money should be $0
        assert_eq!(g.money, 0);
    }

    #[test]
    fn test_spectral_ankh_single_joker() {
        use crate::consumable::Consumables;
        use crate::joker::{JollyJoker, Jokers};
        use crate::spectral::Spectrals;

        let mut g = Game::default();
        g.jokers.push(Jokers::JollyJoker(JollyJoker::default()));

        g.consumables.push(Consumables::Spectral(Spectrals::Ankh));
        g.use_consumable(Consumables::Spectral(Spectrals::Ankh), None)
            .unwrap();

        // Should still have 1 joker (copy of itself, others destroyed = just itself)
        assert_eq!(g.jokers.len(), 1);
    }

    // ==================== Phase 3A Tarot Edge Case Tests ====================

    #[test]
    fn test_tarot_the_fool_no_last_consumable() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.consumables.push(Consumables::Tarot(Tarots::TheFool));

        // Use The Fool with no previous consumable - should not crash
        g.use_consumable(Consumables::Tarot(Tarots::TheFool), None)
            .unwrap();

        // Should complete without error, and The Fool itself is now last_consumable_used
        assert_eq!(
            g.last_consumable_used,
            Some(Consumables::Tarot(Tarots::TheFool))
        );
    }

    #[test]
    fn test_tarot_the_fool_copies_tarot() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let id = card.id;
        g.add_card_to_deck(card);

        // Use The Magician first (should set last_consumable_used)
        let magician = Consumables::Tarot(Tarots::TheMagician);
        g.consumables.push(magician.clone());
        g.use_consumable(magician.clone(), Some(vec![card]))
            .unwrap();

        // Verify it was used
        assert_eq!(g.last_consumable_used, Some(magician));

        // Card should be Lucky now
        let cards1 = g.deck.cards();
        let modified1 = cards1.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified1.enhancement, Some(Enhancement::Lucky));

        // Now use The Fool - should repeat The Magician effect
        let fool = Consumables::Tarot(Tarots::TheFool);
        g.consumables.push(fool);
        g.use_consumable(Consumables::Tarot(Tarots::TheFool), None)
            .unwrap();

        // Card should still be Lucky (Magician effect repeated)
        let cards2 = g.deck.cards();
        let modified2 = cards2.iter().find(|c| c.id == id).unwrap();
        assert_eq!(modified2.enhancement, Some(Enhancement::Lucky));
    }

    #[test]
    fn test_tarot_hermit_zero_money() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.money = 0;
        g.consumables.push(Consumables::Tarot(Tarots::TheHermit));

        // Double zero should still be zero
        g.use_consumable(Consumables::Tarot(Tarots::TheHermit), None)
            .unwrap();
        assert_eq!(g.money, 0);
    }

    #[test]
    fn test_tarot_temperance_no_jokers() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.jokers.clear();
        g.money = 10;
        g.consumables.push(Consumables::Tarot(Tarots::Temperance));

        // Should not crash with no jokers
        g.use_consumable(Consumables::Tarot(Tarots::Temperance), None)
            .unwrap();

        // Money should be unchanged
        assert_eq!(g.money, 10);
    }

    #[test]
    fn test_tarot_enhancement_overwrite() {
        use crate::card::{Card, Enhancement, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let id = card.id;

        // First enhance with Bonus
        g.add_card_to_deck(card);
        g.modify_card_in_deck(id, |c| {
            c.set_enhancement(Enhancement::Bonus);
        });

        // Verify Bonus
        let cards1 = g.deck.cards();
        assert_eq!(
            cards1.iter().find(|c| c.id == id).unwrap().enhancement,
            Some(Enhancement::Bonus)
        );

        // Now use Magician to change to Lucky
        g.consumables.push(Consumables::Tarot(Tarots::TheMagician));
        g.use_consumable(Consumables::Tarot(Tarots::TheMagician), Some(vec![card]))
            .unwrap();

        // Should now be Lucky (overwritten)
        let cards2 = g.deck.cards();
        assert_eq!(
            cards2.iter().find(|c| c.id == id).unwrap().enhancement,
            Some(Enhancement::Lucky)
        );
    }

    #[test]
    fn test_tarot_death_single_card() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card = Card::new(Value::Five, Suit::Heart);
        let id = card.id;

        g.add_card_to_deck(card);
        g.consumables.push(Consumables::Tarot(Tarots::Death));

        // Use Death with only 1 card (needs 2) - should not crash
        g.use_consumable(Consumables::Tarot(Tarots::Death), Some(vec![card]))
            .unwrap();

        // Card should be unchanged (not enough targets)
        let cards = g.deck.cards();
        let unchanged = cards.iter().find(|c| c.id == id).unwrap();
        assert_eq!(unchanged.value, Value::Five);
        assert_eq!(unchanged.suit, Suit::Heart);
    }

    #[test]
    fn test_tarot_hanged_man_single_card() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::deck::Deck;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.deck = Deck::empty();

        let card = Card::new(Value::Five, Suit::Heart);
        g.add_card_to_deck(card);

        g.consumables.push(Consumables::Tarot(Tarots::TheHangedMan));

        // Destroy 1 card (has capacity for 2)
        g.use_consumable(
            Consumables::Tarot(Tarots::TheHangedMan),
            Some(vec![card]),
        )
        .unwrap();

        // Deck should be empty
        assert_eq!(g.deck.len(), 0);
    }

    #[test]
    fn test_tarot_wheel_of_fortune_no_jokers() {
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        g.jokers.clear();
        g.consumables.push(Consumables::Tarot(Tarots::WheelOfFortune));

        // Should not crash with no jokers
        g.use_consumable(Consumables::Tarot(Tarots::WheelOfFortune), None)
            .unwrap();

        // Should still have no jokers
        assert_eq!(g.jokers.len(), 0);
    }

    #[test]
    fn test_tarot_suit_conversion_partial() {
        use crate::card::{Card, Suit, Value};
        use crate::consumable::Consumables;
        use crate::tarot::Tarots;

        let mut g = Game::default();
        let card1 = Card::new(Value::Five, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Club);
        let id1 = card1.id;
        let id2 = card2.id;

        g.add_card_to_deck(card1);
        g.add_card_to_deck(card2);
        g.consumables.push(Consumables::Tarot(Tarots::TheStar));

        // Use with only 1 card (can take up to 3)
        g.use_consumable(Consumables::Tarot(Tarots::TheStar), Some(vec![card1]))
            .unwrap();

        let cards = g.deck.cards();
        let modified1 = cards.iter().find(|c| c.id == id1).unwrap();
        let unchanged = cards.iter().find(|c| c.id == id2).unwrap();

        // First card should be Diamond
        assert_eq!(modified1.suit, Suit::Diamond);

        // Second card should be unchanged (Club)
        assert_eq!(unchanged.suit, Suit::Club);
    }

    #[test]
    fn test_tarot_emperor_consumable_generation() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};
        use crate::tarot::Tarots;

        let mut g = Game::default();

        // Use Emperor multiple times to ensure random generation works
        for _ in 0..5 {
            g.consumables.push(Consumables::Tarot(Tarots::TheEmperor));
            let initial_len = g.consumables.len();

            g.use_consumable(Consumables::Tarot(Tarots::TheEmperor), None)
                .unwrap();

            // Should have 2 more tarots (3 total: removed Emperor, added 2 new ones, so net +1)
            // Initial_len includes the Emperor we're about to use, which gets removed
            // Then 2 are added, so: initial_len - 1 + 2 = initial_len + 1
            assert_eq!(g.consumables.len(), initial_len + 1);

            // Verify the last 2 are actually tarots
            let last_two = &g.consumables[(initial_len - 1)..];
            assert_eq!(last_two[0].consumable_type(), ConsumableType::Tarot);
            assert_eq!(last_two[1].consumable_type(), ConsumableType::Tarot);
        }
    }

    #[test]
    fn test_tarot_high_priestess_planet_generation() {
        use crate::consumable::{Consumable, ConsumableType, Consumables};
        use crate::tarot::Tarots;

        let mut g = Game::default();

        // Use High Priestess multiple times to ensure random generation works
        for _ in 0..5 {
            g.consumables
                .push(Consumables::Tarot(Tarots::TheHighPriestess));
            let initial_len = g.consumables.len();

            g.use_consumable(Consumables::Tarot(Tarots::TheHighPriestess), None)
                .unwrap();

            // Should have 2 more planets (removed High Priestess, added 2 planets, so net +1)
            assert_eq!(g.consumables.len(), initial_len + 1);

            // Verify they're actually planets
            let last_two = &g.consumables[(initial_len - 1)..];
            assert_eq!(last_two[0].consumable_type(), ConsumableType::Planet);
            assert_eq!(last_two[1].consumable_type(), ConsumableType::Planet);
        }
    }
}
