# balatro-rs: Project Documentation

**Last Updated:** 2025-11-12
**Version:** Core v0.0.1
**Test Suite:** 420+ tests passing

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Core Game Systems](#core-game-systems)
4. [Advanced Features](#advanced-features)
5. [Action Generation](#action-generation)
6. [Implementation Status](#implementation-status)
7. [Development Information](#development-information)

---

## Project Overview

balatro-rs is a Rust implementation of Balatro, a poker roguelike deckbuilder game. The project provides a complete game engine with move generation capabilities specifically designed for reinforcement learning applications. The implementation achieves approximately 82% feature parity with the full game.

### Project Structure

```
balatro-rs/
├── core/              Main game engine (balatro-rs crate)
│   ├── src/          Game logic, move generation, scoring
│   └── tests/        420+ comprehensive tests
├── pylatro/          Python bindings via PyO3
│   ├── examples/     Python simulation examples
│   └── gym/          OpenAI Gym environment wrapper
├── cli/              Command-line interface
└── docs/             Documentation
    ├── history/      Phase completion documents
    ├── sessions/     Development session summaries
    ├── design/       Implementation plans
    └── reference/    Game rules and feature references
```

### Key Capabilities

- **Exhaustive Move Generation**: Provides all legal actions at any game state
- **Fixed-Size Action Space**: 79-dimensional action space for RL agents
- **Fast Simulation**: Capable of 1000+ games per second
- **Python Bindings**: Full PyO3 integration for Python/RL frameworks
- **Comprehensive Testing**: Over 420 tests covering all major systems

---

## Architecture

### Game State Machine

The game progresses through a series of stages defined in `core/src/stage.rs`:

**Stage Flow:**
```
PreBlind → Blind → PostBlind → Shop → [repeat for next blind]
```

#### Stage Definitions

**PreBlind**
The preparation stage before each blind. Players can:
- View blind conditions and boss modifiers
- Choose to play the current blind (Small, Big, or Boss)
- Or skip blind for a tag reward (Small and Big only)

**Blind(Blind, Option<BossModifier>)**
The active gameplay stage where players:
- Play poker hands to accumulate score
- Must reach the target score within limited hands/discards
- Face optional boss modifiers on Boss blinds that add constraints

**PostBlind**
The reward collection stage after clearing a blind where players:
- Collect money rewards (Small: $3, Big: $4, Boss: $5)
- Optionally use consumable cards
- Receive interest on held money

**Shop**
The purchasing stage where players can:
- Buy jokers (permanent modifiers)
- Buy consumables (one-time use cards)
- Buy booster packs
- Buy vouchers (permanent upgrades)
- Sell jokers
- Reroll shop contents (costs money)

**End(End)**
Terminal stage indicating game over:
- Win: Successfully cleared Ante 8
- Lose: Failed to reach score requirement

### Core Data Structures

#### Game (core/src/game.rs)

The central game state containing:
- **Deck**: All cards in the deck
- **Hand**: Cards currently drawable/in hand
- **Available**: Cards that can be selected for play/discard
- **Stage**: Current game stage
- **Ante**: Current difficulty level (1-8)
- **Score**: Current score and target requirements
- **Money**: Current funds and interest tracking
- **Hands/Discards**: Remaining actions per blind
- **Jokers**: Active joker modifiers
- **Consumables**: Available consumable cards
- **Shop**: Shop state and inventory
- **Effect Registry**: Registered joker effects
- **Boss Modifier**: Active boss blind modifier
- **Modifiers**: Game rule modifiers from jokers
- **Tags**: Queue of pending tag rewards

#### Card (core/src/card.rs)

Cards are the fundamental unit with properties:
- **Value**: Rank (2-Ace)
- **Suit**: Suit (Spades, Hearts, Clubs, Diamonds)
- **Enhancement**: Optional modifier (Bonus, Mult, Stone, Glass, Steel, Gold)
- **Edition**: Visual effect with bonus (Base, Foil, Holographic, Polychrome, Negative)
- **Seal**: Special effect trigger (Gold, Red, Blue, Purple)
- **Face Down**: Boolean for boss modifier effects

#### Action (core/src/action.rs)

Actions represent all possible player moves:
- `SelectCard(Card)`: Select a card for play/discard
- `MoveCard(Direction, Card)`: Reorder cards in hand
- `Play()`: Play selected cards as a poker hand
- `Discard()`: Discard selected cards
- `CashOut(usize)`: Collect reward and proceed
- `BuyJoker(Jokers)`: Purchase a joker
- `BuyConsumable(Consumables)`: Purchase a consumable
- `UseConsumable(Consumables, Option<Vec<Card>>)`: Use a consumable with optional card targets
- `NextRound()`: Proceed to next stage
- `SelectBlind(Blind)`: Choose which blind to play
- `SkipBlind()`: Skip blind for tag reward
- `SelectFromTagPack(usize)`: Select reward from tag pack
- `SellJoker(Jokers)`: Sell a joker in shop

---

## Core Game Systems

### Poker Hand Detection

Location: `core/src/hand.rs`

The hand detection system identifies the best poker hand from selected cards. Balatro includes 13 hand ranks, including special hands not found in traditional poker.

#### Hand Ranks (Highest to Lowest)

1. **Flush Five**: 5 cards of same rank and suit (e.g., 5 Queens of Hearts)
2. **Flush House**: Full House where all cards share same suit
3. **Five of a Kind**: 5 cards of same rank
4. **Royal Flush**: Straight Flush with 10-J-Q-K-A
5. **Straight Flush**: 5 cards in sequence, all same suit
6. **Four of a Kind**: 4 cards of same rank
7. **Full House**: 3 of a kind + 2 of a kind
8. **Flush**: 5 cards of same suit
9. **Straight**: 5 cards in sequence
10. **Three of a Kind**: 3 cards of same rank
11. **Two Pair**: 2 pairs of same rank
12. **Pair**: 2 cards of same rank
13. **High Card**: No matching cards

#### Hand Context System

The `HandContext` struct allows game modifiers to affect hand detection:

```rust
pub struct HandContext<'a> {
    pub modifiers: &'a GameModifiers,
}
```

**Game Modifiers:**
- `four_card_straights`: Straights with only 4 cards count
- `four_card_flushes`: Flushes with only 4 cards count
- `all_cards_are_faces`: All cards treated as face cards
- `smeared_suits`: Hearts/Diamonds same, Spades/Clubs same
- `gap_straights`: Straights with 1-rank gaps allowed
- `all_cards_score`: All selected cards score (not just hand)

#### Implementation Details

The `SelectHand` type represents up to 5 selected cards and provides methods for hand detection:
- `best_hand_with_context()`: Detects best hand with modifier support
- `values_freq()`: Maps card ranks to cards for pair detection
- `suits_freq()`: Maps suits to cards for flush detection
- Face-down cards (from boss modifiers) are filtered out before detection

### Scoring System

Location: `core/src/game.rs` (calc_score method)

Scoring combines poker hand levels with card-specific bonuses and multipliers.

#### Scoring Formula

```
Final Score = (Base Chips + Card Chips) × (Base Mult + Card Mult) × Edition Multipliers
```

#### Scoring Process

1. **Base Score from Hand Level**
   Each poker hand type has a level that increases when upgraded by planet cards.
   - Base Chips: e.g., Pair Level 1 = 10 chips
   - Base Mult: e.g., Pair Level 1 = 2 mult

2. **Card Chips**
   Each card in the hand contributes chips based on rank:
   - Number cards (2-10): Face value in chips
   - Face cards (J/Q/K): 10 chips each
   - Aces: 11 chips each
   - Enhancements add bonus chips (Bonus: +30, Stone: +50)

3. **Card Mult**
   Enhancements can add to mult (Mult enhancement: +4)

4. **Edition Multipliers**
   Card editions apply multiplicative bonuses:
   - Foil: +50 chips
   - Holographic: +10 mult
   - Polychrome: ×1.5 mult multiplier
   - Negative: +1 joker slot (no scoring bonus)

5. **Joker Effects**
   After base scoring, registered joker effects fire via the effect registry, potentially modifying chips, mult, or multipliers.

6. **Final Calculation**
   `score = chips × mult × multipliers`

#### Hand Leveling

Planet cards upgrade poker hands permanently:
- Chips increase by formula: +30/+25/+20 (first 3 levels), then +20
- Mult increases by formula: +3/+2/+2 (first 3 levels), then +2
- Levels persist across rounds and antes

### Blind System

Location: `core/src/stage.rs`, `core/src/boss_modifier.rs`

Blinds are challenges that must be overcome by reaching a score threshold.

#### Blind Types

**Small Blind**
- Lowest score requirement
- Reward: $3
- Can be skipped for a tag

**Big Blind**
- Medium score requirement
- Reward: $4
- Can be skipped for a tag

**Boss Blind**
- Highest score requirement (×2.0 previous blind score, or ×2.5 for The Wall)
- Reward: $5
- Includes a random boss modifier
- Cannot be skipped

#### Score Requirements

Each ante multiplies the base score requirement:
- Ante 1: Base = 300
- Ante 2: Base × 1.5
- Ante 3: Base × 2.0
- Each subsequent ante increases further

Within each ante:
- Small: 1.0× ante requirement
- Big: 1.5× ante requirement
- Boss: 2.0× (or 2.5× for The Wall modifier)

### Boss Modifier System

Location: `core/src/boss_modifier.rs`

Boss modifiers add constraints and challenges to boss blinds. All 20 modifiers are fully implemented.

#### Category A: Simple Constraints

**The Wall**
Score requirement is ×2.5 instead of ×2.0, making the blind significantly harder.

**The Manacle**
Hand size is reduced by 1 for the entire blind, limiting card selection.

**The Water**
Player starts with 0 discards for this blind, no discarding allowed.

**The Needle**
Player can only play 1 hand total for this blind, must make it count.

**The Arm**
After each hand played, that poker hand type's level decreases by 1.

**The Tooth**
Player loses $1 for each card played during the blind.

#### Category B: Card Debuffing

**The Club / The Goad / The Window / The Head**
All cards of a specific suit (Clubs/Spades/Diamonds/Hearts respectively) are debuffed and don't contribute to scoring.

**The Plant**
All face cards (Jack, Queen, King) are debuffed.

**The Flint**
All chip and mult values are halved after calculation.

#### Category C: Hand Restrictions

**The Eye**
No poker hand type can be played more than once. Forces hand diversity.

**The Mouth**
Only one specific poker hand type can be played throughout the blind.

**The Serpent**
The first hand played always scores 0, regardless of cards.

**The Hook**
After each hand played, 2 random cards are discarded from the player's hand.

#### Category D: Complex Mechanics

**The Ox**
The leftmost card in hand is always face-down (no visible rank/suit).

**The House**
First hand of the blind is dealt with only 1 card instead of the full hand size.

**The Wheel**
Each card has a 1/7 chance to be dealt face-down randomly.

**The Pillar**
When playing cards, they are selected randomly instead of by player choice.

---

## Advanced Features

### Card Modifier System

Location: `core/src/card.rs`

Cards can have three types of modifiers that enhance their properties.

#### Enhancements

Enhancements modify a card's core properties. Implemented: 6/8

**Bonus** (+30 chips)
Card contributes +30 bonus chips when scored.

**Mult** (+4 mult)
Card contributes +4 to mult when scored.

**Stone** (+50 chips, no rank)
Card gives +50 chips but has no rank (doesn't count for pairs/straights).

**Glass** (×2 mult, 1/4 destroy chance)
Card doubles mult when scored, but has 25% chance to be destroyed after scoring.

**Steel** (×1.5 mult)
Card applies ×1.5 multiplier to mult when scored.

**Gold** (+$3 when played)
Card grants $3 when played or held in hand (boss blind).

**Wild** (not implemented)
Card acts as any suit for flush detection.

**Lucky** (not implemented)
Card provides probabilistic chip/mult bonuses.

#### Editions

Editions are visual effects that provide bonuses. Implemented: 4/4

**Foil** (+50 chips)
Adds +50 chips to card when scored.

**Holographic** (+10 mult)
Adds +10 mult when scored.

**Polychrome** (×1.5 mult)
Multiplies final score by 1.5.

**Negative** (+1 joker slot)
Adds an additional joker slot (no direct scoring bonus).

#### Seals

Seals trigger special effects when cards are played/discarded. Implemented: 4/4

**Red Seal**
Retriggers the card (scores it twice).

**Gold Seal**
Grants $3 when the card is played.

**Blue Seal**
Creates a random Planet card for the played hand when played.

**Purple Seal**
Creates a random Tarot card when discarded.

### Consumable System

Location: `core/src/consumable.rs`, `core/src/tarot.rs`, `core/src/planet.rs`, `core/src/spectral.rs`

Consumables are one-time use cards that modify the game state.

#### Consumable Trait

All consumables implement a common interface:
- `name()`: Display name
- `desc()`: Effect description
- `cost()`: Purchase price
- `requires_target()`: Whether card targets are needed
- `max_targets()`: Maximum cards that can be targeted
- `use_effect()`: Execute the consumable's effect
- `consumable_type()`: Tarot, Planet, or Spectral

#### Tarot Cards (22/22 Complete)

Tarot cards modify individual cards in the deck. Major categories:

**Enhancement Conversion**
- The Magician: Converts up to 2 cards to Lucky enhancement
- The High Priestess: Creates 2 random Planet cards
- The Empress: Converts up to 2 cards to Mult enhancement
- The Hierophant: Converts up to 2 cards to Bonus enhancement
- The Chariot: Converts 1 card to Steel enhancement
- The Devil: Converts 1 card to Gold enhancement
- The Tower: Converts 1 card to Stone enhancement
- The Lovers: Converts 1 card to Wild enhancement

**Suit Conversion**
- The Star: Converts up to 3 cards to Diamonds
- The Moon: Converts up to 3 cards to Clubs
- The Sun: Converts up to 3 cards to Hearts
- The World: Converts up to 3 cards to Spades

**Rank Modification**
- Strength: Increases rank of up to 2 cards by 1
- The Hanged Man: Destroys up to 2 selected cards
- Death: Converts 2 selected cards to 1 random card with same rank
- Temperance: Gives total sell value of all jokers (max $50)

**Card Generation**
- The Emperor: Creates 2-5 random Tarot cards
- Judgement: Creates a random Joker card
- The Hermit: Doubles money (max $20)
- The Wheel of Fortune: 1/4 chance to add Foil, Holographic, or Polychrome edition to 1 card

**Special Effects**
- The Fool: Repeats the last consumable used
- Justice: Creates a random Tarot card
- Temperance: Provides money based on joker values

#### Planet Cards (12/12 Complete)

Planet cards upgrade the level of their corresponding poker hand.

**Standard Planets**
- Pluto: High Card
- Mercury: Pair
- Venus: Two Pair
- Earth: Three of a Kind
- Mars: Straight
- Saturn: Flush
- Neptune: Full House

**Secret Planets**
- Ceres: Four of a Kind
- Eris: Straight Flush
- Planet X: Royal Flush
- Jupiter: Five of a Kind
- Uranus: Flush House

**Upgrade Formula**
First 3 levels: +30 chips/+3 mult, +25/+2, +20/+2
Subsequent levels: +20 chips/+2 mult

**Hand Leveling Example**
```
Pair Level 1: 10 chips, 2 mult
Pair Level 2: 40 chips, 5 mult (+30, +3)
Pair Level 3: 65 chips, 7 mult (+25, +2)
Pair Level 4: 85 chips, 9 mult (+20, +2)
```

#### Spectral Cards (18/18 Complete)

Spectral cards are high-impact consumables with powerful effects. Major categories:

**Deck Enhancement**
- Familiar: Destroys 1 random card, adds 3 random Enhanced face cards
- Grim: Destroys 1 random card, adds 3 random Enhanced Aces
- Incantation: Destroys 1 random card, adds 4 random Enhanced numbered cards

**Seal Addition**
- Talisman: Adds Gold Seal to 1 selected card
- Deja Vu: Adds Red Seal to 1 selected card
- Trance: Adds Blue Seal to 1 selected card
- Medium: Adds Purple Seal to 1 selected card

**Edition Effects**
- Aura: Adds Foil, Holographic, or Polychrome to 1-2 cards

**Deck Transformation**
- Sigil: Converts all cards in deck to 1 random suit
- Ouija: Converts all cards in deck to 1 random rank
- Immolate: Destroys 5 random cards, gains $20
- Cryptid: Creates 2 copies of 1 selected card

**Joker Manipulation**
- Wraith: Creates a random Rare joker (costs $0)
- Ankh: Creates a copy of a random joker
- Hex: Adds Polychrome to a random joker, destroys all other jokers
- Ectoplasm: Adds Negative to a random joker, sets hand size to 1
- The Soul: Creates a random Legendary joker (costs $0)

**Global Effects**
- Black Hole: Upgrades every poker hand by 1 level

### Shop System

Location: `core/src/shop.rs`, `core/src/voucher.rs`, `core/src/booster.rs`

The shop system allows purchasing cards, packs, and permanent upgrades.

#### Shop Configuration

Shops have configurable slots:
- Joker slots (default: 2)
- Consumable slots (default: 2)
- Pack slots (default: 2)
- Voucher slots (default: 1)

#### Pricing System

Base prices:
- Jokers: Rarity-based (Common: $5, Uncommon: $6-8, Rare: $10, Legendary: $20)
- Consumables: $3 (Tarots), $3 (Planets), $4 (Spectrals)
- Booster Packs: $4-6
- Vouchers: $10

Price modifiers from vouchers:
- Clearance Sale: -25%
- Liquidation: -50%

#### Shop Actions

**Reroll**
Regenerates all shop items for a cost:
- Base cost: $5
- Reroll Surplus voucher: -$2
- Reroll Glut voucher: -$5
- Cost increases by $1 each reroll per shop visit

**Purchase**
Buy items if player has sufficient funds. Items are removed from shop after purchase.

**Sell**
Sell jokers for half their purchase price (rounded up).

#### Voucher System (24/24 Complete)

Vouchers provide permanent upgrades organized in tier 1/tier 2 pairs.

**Shop Upgrades**
- Overstock/Overstock Plus: +1/+2 card slots in shop
- Clearance Sale/Liquidation: -25%/-50% prices
- Reroll Surplus/Reroll Glut: Rerolls cost -$2/-$5

**Consumable Enhancements**
- Crystal Ball/Illusion: +1/+2 consumable slots
- Telescope/Observatory: Celestial packs contain most-played hand / Planets give ×1.5

**Gameplay Bonuses**
- Grabber/Nacho Tong: +1/+2 hands per round
- Wasteful/Recyclomancy: +1/+2 discards per round

**Card Frequency**
- Tarot Merchant/Tarot Tycoon: Tarots 2×/4× more common
- Planet Merchant/Planet Tycoon: Planets 2×/4× more common
- Omen Globe/Seance: Spectrals enabled/2× more common
- Buffoon/Gros Michel: Buffoon packs 2×/4× more common
- Hone/Glow Up: Editions 2×/4× more common

#### Booster Packs (4/4 Complete)

Packs contain multiple cards, player chooses 1-2 to keep.

**Arcana Pack** ($4)
Contains 3 random Tarot cards. Choose 1.

**Celestial Pack** ($4)
Contains 3 random Planet cards. Choose 1.
With Telescope voucher: Always contains planet for most-played hand.

**Spectral Pack** ($4)
Contains 2 random Spectral cards. Choose 1.
Only available with Omen Globe voucher.

**Buffoon Pack** ($4)
Contains 2 random Jokers. Choose 1.

**Mega Packs**
Special packs from tags with more cards and choices:
- Mega Arcana: 5 cards, choose 2
- Mega Celestial: 5 cards, choose 2
- Mega Standard: 5 cards, choose 2
- Mega Buffoon: 4 jokers, choose 2

### Joker System

Location: `core/src/joker/`

Jokers are permanent modifiers that affect gameplay through an effect registry system.

#### Joker Categories

Jokers are organized by rarity:
- **Common**: ~70 jokers, $5
- **Uncommon**: ~40 jokers, $6-8
- **Rare**: ~35 jokers, $10
- **Legendary**: ~5 jokers, $20

#### Implementation Status

Approximately 120 fully functional jokers are implemented. About 30 additional jokers are present as stubs awaiting system support (retriggers, effect copying).

#### Effect System

Location: `core/src/effect.rs`

Jokers register effects that fire at specific game events:

**Effect Types:**
- `OnPlay`: Fires when cards are played
- `OnDiscard`: Fires when cards are discarded
- `OnScore`: Fires during score calculation
- `OnHandRank`: Fires when a hand rank is made
- `OnRoundBegin`: Fires at start of blind
- `OnRoundEnd`: Fires at end of blind
- `OnBlindSelect`: Fires when blind is selected
- `OnSell`: Fires when a joker is sold

**Effect Registration:**
```rust
pub struct EffectRegistry {
    pub on_play: Vec<Effects>,
    pub on_discard: Vec<Effects>,
    pub on_score: Vec<Effects>,
    // ... other effect types
}
```

When jokers are purchased, their effects are registered to the appropriate event hooks. When events occur, all registered effects fire in order.

#### Example Jokers
More joker rules can be found in `docs\reference\JOKERS.md`

**Jolly Joker** (Common, $5)
+4 mult when played. Simple OnScore effect.

**Greedy Joker** (Common, $5)
Cards with Diamond suit give +4 mult. OnScore effect checking card suits.

**Lusty Joker** (Common, $5)
Cards with Heart suit give +4 mult. OnScore effect checking card suits.

**Raised Fist** (Common, $5)
Adds double the lowest ranked card in hand to mult. OnScore effect reading game state dynamically.

**Four Fingers** (Uncommon, $7)
All Flushes and Straights can be made with 4 cards. Modifies game modifiers.

**Smeared Joker** (Uncommon, $6)
Hearts and Diamonds count as the same suit, Spades and Clubs count as the same suit. Modifies hand detection.

**Blueprint** (Rare, $10)
Copies the effect of the joker to the right. Dynamic effect copying.

#### Stateful Jokers

Some jokers maintain internal state that changes during gameplay:

**Green Joker** (Uncommon, $6)
Starts at +1 mult, increases by +1 per hand played, decreases by -1 per discard.

**Ice Cream** (Common, $5)
Starts at +100 chips, decreases by -5 per hand played.

**Popcorn** (Common, $5)
Starts at +20 mult, decreases by -4 per round.

**Constellation** (Uncommon, $6)
Gains ×0.1 mult per Planet card used.

### Tag System

Location: `core/src/tag.rs`

Tags are rewards obtained by skipping blinds or from special effects.

#### Obtaining Tags

**Skip Blind**
When skipping Small or Big blind, player receives a random tag.
Boss blinds cannot be skipped.

**Special Effects**
- Double Tag: Copies the next obtained tag
- Anaglyph Deck: Grants a tag after each boss blind

#### Tag Trigger System

Tags trigger at different times:

**Immediate Triggers**
Execute as soon as obtained:
- Charm, Buffoon, Meteor, Ethereal, Standard: Open mega packs
- Economy: Doubles money (max $40)
- Speed: $5 per blind skipped this run
- Handy: $1 per hand played this run
- Garbage: $1 per unused discard this run
- Orbital: Upgrades random hand by 3 levels
- Top Up: Creates 2 common jokers

**OnShopEnter Triggers**
Execute when entering the shop:
- Uncommon, Rare, Foil, Holographic, Polychrome, Negative: Shop joker effects
- Voucher: Adds a voucher to shop
- Coupon, D6: Shop price effects

**OnRoundStart Triggers**
Execute at start of next blind:
- Juggle: +3 hand size for next round only

**OnBossDefeated Triggers**
Execute after defeating next boss:
- Investment: Gain $25

**OnTagObtained Triggers**
Execute when next tag is obtained:
- Double: Copies the next tag (stacks additively with multiple Doubles)

**OnBossEncounter Triggers**
Execute before facing next boss:
- Boss: Reroll the boss blind modifier

#### Tag Queue System

Tags are stored in a FIFO queue and trigger in order based on their trigger type. The Double tag has special stacking behavior where multiple Doubles accumulate.

#### Ante Availability

15 tags available from Ante 1.
9 additional tags unlock at Ante 2+:
- Negative, Standard, Meteor, Buffoon, Handy, Garbage, Ethereal, Top Up, Orbital

### Alternative Deck System

Location: `core/src/alternative_deck.rs`

Alternative decks provide different starting conditions and modifiers.

#### Implementation Status

14/15 standard decks fully implemented. Plasma deck special scoring deferred.

#### Deck Types

**Red Deck**
+1 discard per round.

**Blue Deck**
+1 hand per round.

**Yellow Deck**
Start with $10 extra.

**Green Deck**
+1 hand and +1 discard per round, start with -$10.

**Black Deck**
+1 joker slot, -1 hand per round.

**Magic Deck**
Start with Crystal Ball voucher, Illusion voucher, and 2 Fool tarots.

**Nebula Deck**
Start with Planet Merchant voucher and Planet Tycoon voucher.

**Ghost Deck**
Start with 1 Hex spectral card.

**Abandoned Deck**
40 cards instead of 52, no face cards (only 2-10 and Ace).

**Checkered Deck**
52 cards with only Spades and Hearts (26 of each suit), 2 of each rank per suit.

**Zodiac Deck**
Start with Tarot Merchant voucher and Tarot Tycoon voucher.

**Painted Deck**
+1 hand size (can select more cards for play/discard).

**Anaglyph Deck**
Grants a random tag after defeating each boss blind.

**Erratic Deck**
52 completely random cards (any rank/suit combinations).

**Plasma Deck** (not fully implemented)
Special scoring formula: balance chips and mult.
Formula: min(chips, mult) + max(chips, mult) instead of chips × mult.
Requires scoring engine changes.

#### Deck Initialization

Decks are initialized through `Config::with_deck()` which:
1. Sets starting game parameters (hands, discards, money, joker slots)
2. Generates the starting deck with proper card distribution
3. Adds starting vouchers and consumables
4. Applies deck-specific modifiers

---

## Action Generation

The library provides two APIs for action generation, both implemented in `core/src/generator.rs`.

### Iterator API

For flexible gameplay and testing:

```rust
let actions: Vec<Action> = game.gen_moves().collect();
```

Returns an iterator over all legal `Action` enums at the current game state. Actions include:
- Card selection/movement during gameplay
- Play/discard actions
- Shop purchases
- Blind selection
- Consumable usage
- Cash out and round progression

### Fixed-Size Action Space API

For reinforcement learning agents requiring bounded discrete action spaces:

```rust
let action_space: ActionSpace = game.gen_action_space();
let mask: Vec<bool> = action_space.unmask();
```

Returns a fixed-size `ActionSpace` with 79 dimensions where each index maps to a potential action type:
- Indices 0-51: Card selections
- Indices 52-53: Play/Discard
- Indices 54-60: Joker/consumable purchases
- Remaining indices: Other game actions

The `unmask()` method returns a boolean vector indicating which actions are legal at the current state.

### Action Validation

Actions are validated before execution. Invalid actions return `GameError` with:
- `InvalidAction`: Action not legal in current stage
- `InsufficientFunds`: Not enough money for purchase
- `InvalidTarget`: Invalid card targets for consumable
- `Other`: Miscellaneous validation errors

---

## Implementation Status

### Core Systems: Complete

- Poker hand detection (all 13 ranks)
- Hand leveling and scoring
- Stage progression and game loop
- Blind system with score requirements
- Money and interest system
- Card dealing and hand management
- Action validation and execution

### Boss Modifiers: 20/20 Complete

All 20 boss blind modifiers fully implemented:
- Category A (Simple Constraints): 6/6
- Category B (Card Debuffing): 6/6
- Category C (Hand Restrictions): 4/4
- Category D (Complex Mechanics): 4/4

### Card Modifiers: 95% Complete

**Enhancements:** 6/8
- Implemented: Bonus, Mult, Stone, Glass, Steel, Gold
- Not implemented: Wild, Lucky

**Editions:** 4/4
- All implemented: Base, Foil, Holographic, Polychrome, Negative

**Seals:** 4/4
- All implemented: Gold, Red, Blue, Purple

### Consumables: 52/52 Complete

- Tarot cards: 22/22
- Planet cards: 12/12
- Spectral cards: 18/18

### Shop System: Complete

- Vouchers: 24/24 (12 tier 1, 12 tier 2)
- Booster packs: 4/4
- Shop mechanics: Purchase, sell, reroll
- Dynamic pricing with voucher modifiers

### Jokers: ~120 Functional

Approximately 120 jokers fully functional with effect system.
About 30 additional jokers present as stubs requiring:
- Retrigger system
- Effect copying
- Complex state management
- Additional lifecycle hooks

### Alternative Decks: 14/15 Complete

All standard decks except Plasma deck special scoring.

### Skip Blind & Tags: 24/24 Complete

All tag types implemented with trigger system and queue management.

### Not Implemented

**Stakes System** (0/8)
Difficulty levels not implemented.

**Wild/Lucky Enhancements**
Wild suit matching and Lucky probability effects deferred.

**Plasma Deck Scoring**
Special scoring formula requires scoring engine refactor.

**Advanced Joker Systems**
Some jokers require retrigger system, effect copying, or complex mechanics not yet implemented.

---

## Development Information

### Test Coverage

Total: 420+ tests passing

**Test Categories:**
- Core gameplay: ~80 tests
- Card modifiers: 11 tests
- Consumables: 81 tests
- Boss modifiers: 35 tests
- Shop system: 37 tests
- Alternative decks: 29 tests
- Tags: 18 tests
- Jokers: 100+ tests
- Hand detection: 20 tests
- Miscellaneous: ~9 tests

**Test Quality:**
- Unit tests for all core components
- Integration tests for game flows
- No flaky tests
- Full suite runs in <1 second

### Performance Characteristics

**Benchmarks:**
- Action generation: ~50-100μs per call
- Game simulation: ~1000 games/second
- Scoring calculation: <1μs per hand
- Test suite: <1 second for 420+ tests

**Memory Usage:**
- Game state: ~2-4 KB
- Deck: ~1 KB (52 cards)
- Action space: ~80 bytes (fixed size)
- Effect registry: <1 KB

### Code Statistics

**Total Production Code:** ~10,280 lines

By component:
- Core game logic: ~3,500 lines (34%)
- Consumables: ~1,680 lines (16%)
- Shop & acquisition: ~1,200 lines (12%)
- Boss modifiers: ~1,200 lines (12%)
- Alternative decks: ~700 lines (7%)
- Skip blind & tags: ~600 lines (6%)
- Jokers: ~400 lines (4%)
- Supporting code: ~1,000 lines (9%)

**Core Modules** (25 files):
action.rs, alternative_deck.rs, ante.rs, available.rs, booster.rs, boss_modifier.rs, card.rs, config.rs, consumable.rs, deck.rs, effect.rs, error.rs, game.rs, generator.rs, hand.rs, joker/, planet.rs, rank.rs, shop.rs, space.rs, spectral.rs, stage.rs, tag.rs, tarot.rs, voucher.rs

### Python Bindings

Status: Functional with PyO3

**Features:**
- Game state exposed to Python
- Action generation in Python
- OpenAI Gym environment wrapper
- Serialization support

**Usage Example:**
```python
import pylatro

game = pylatro.Game()
game.start()

while not game.is_over():
    actions = game.gen_moves()
    action = random.choice(actions)
    game.handle_action(action)

result = game.result()
```

**Gym Environment:**
```python
import gym
import pylatro.gym

env = gym.make('Balatro-v0')
obs = env.reset()

done = False
while not done:
    action = env.action_space.sample()
    obs, reward, done, info = env.step(action)
```

### Documentation

**Available Documentation:**
- `README.md`: Project overview
- `CLAUDE.md`: AI assistant instructions
- `PROJECT_STATUS.md`: This document
- `docs/reference/`: Game rules and feature references
- `docs/history/`: Phase completion documents
- `docs/sessions/`: Development session summaries
- `docs/design/`: Implementation plans

### Use Cases

**1. Reinforcement Learning**
Primary goal: Train RL agents to play Balatro optimally.

Supported features:
- Exhaustive action generation
- Fixed-size action space for neural networks
- Deterministic game logic
- Fast simulation (1000+ games/second)
- Rich state space with modifiers

**2. Game Analysis**
Monte Carlo simulations, strategy evaluation, balance testing, win rate analysis.

**3. Bot Development**
AI opponents for testing, heuristic strategy implementation, benchmark comparisons.

**4. Educational**
Learn Rust game development, study RL environments, explore game state machines.

### Building & Testing

**Build Entire Workspace:**
```bash
cargo build
```

**Run All Tests:**
```bash
cargo test
```

**Run Core Tests:**
```bash
cargo test -p balatro-rs
```

**Run Specific Test:**
```bash
cargo test test_game_gen_actions
```

**Run Benchmarks:**
```bash
cargo bench -p balatro-rs
```

**Python Development:**
```bash
cd pylatro
maturin develop
python examples/simulation.py
python test/main.py
```

**CLI:**
```bash
cargo run -p cli
```

### Feature Flags

The core crate supports optional features:
- `serde`: Enable serialization/deserialization
- `python`: Enable PyO3 bindings (default on)
- `colored`: Enable colored output for display

---

## Summary

balatro-rs is a comprehensive Rust implementation of Balatro with approximately 82% feature parity to the full game. The codebase provides:

**Complete Systems:**
- All core gameplay mechanics
- All 20 boss blind modifiers
- All 52 consumable cards
- Complete shop system with 24 vouchers
- 14/15 alternative decks
- All 24 tag types
- Approximately 120 functional jokers

**Key Strengths:**
- Solid architecture with clean abstractions
- Comprehensive test coverage (420+ tests)
- Fast performance (1000+ games/second)
- Ready for RL training and experimentation
- Complete Python bindings

**Suitable For:**
- Reinforcement learning research
- Game strategy analysis
- Bot development
- Educational purposes
- Balatro game engine applications

The codebase is well-documented, thoroughly tested, and actively maintained. With its exhaustive move generation and fixed-size action space API, it serves as an ideal platform for applying reinforcement learning techniques to Balatro gameplay.
