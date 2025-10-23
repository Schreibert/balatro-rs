# Stateful Jokers Implementation Plan

This document outlines the implementation strategy for jokers that require state tracking or game logic modifications.

## Overview

Currently, **91/150 jokers** are implemented. Of the remaining 59 jokers, approximately 40 require some form of state tracking or game logic changes. This document breaks down the implementation into 3 phases based on complexity.

---

## Phase 1: Hand Access & Round State (Quick Wins)

**Difficulty:** ⭐⭐ Medium
**Estimated Jokers Unlocked:** 15-20
**Timeline:** 1-2 hours

### Requirements

1. **Add `hand` field to Game struct** - Track current cards in player's hand
2. **Add `RoundState` struct** - Track per-round temporary state
3. **Update game logic** - Populate hand when dealt, update round state at appropriate times

### Changes Needed

#### Game Struct Addition
```rust
pub struct Game {
    // ... existing fields ...
    pub hand: Vec<Card>,  // Current cards in player's hand
    pub round_state: RoundState,
}

pub struct RoundState {
    // Random selections that change each round
    pub idol_rank: Option<Value>,
    pub idol_suit: Option<Suit>,
    pub ancient_suit: Option<Suit>,
    pub todo_hand: Option<HandRank>,
    pub mail_rebate_rank: Option<Value>,

    // Round tracking
    pub hands_played_this_round: HashSet<HandRank>,
    pub consecutive_hands_without_faces: usize,
}
```

#### Initialization
- Initialize `hand` as empty vec in `Game::default()`
- Initialize `round_state` with `RoundState::default()`
- Reset round state at start of each blind
- Randomize idol/ancient/todo selections at round start

#### Game Logic Updates
- Populate `hand` when dealing cards
- Update `hand` when cards are played/discarded
- Update `hands_played_this_round` after each play
- Track consecutive hands without faces for Ride the Bus

### Affected Jokers

**Hand Access (5 jokers):**
1. Raised Fist - Adds double the rank of lowest card in hand to Mult
2. Shoot the Moon - +13 Mult per Queen in hand
3. Reserved Parking - Queens in hand have 1/3 chance to give $1
4. Blackboard - X3 Mult if all cards in hand are Spades/Clubs
5. Baron - Each King in hand gives X1.5 Mult

**Round State (10 jokers):**
1. The Idol - X2 Mult for each [rank] of [suit] in hand (changes per round)
2. Ancient Joker - Cards with [suit] give X1.5 Mult (suit changes per round)
3. To Do List - $5 if hand is listed type (changes per round)
4. Mail-In Rebate - $3 per discarded [rank] (rank changes per round)
5. Card Sharp - X3 Mult if hand type already played this round
6. Ride the Bus - +1 Mult per consecutive hand without faces (resets on face)
7. Hit the Road - X0.5 Mult per Jack discarded this round (resets each round)
8. Supernova - Adds number of times poker hand played this run to Mult
9. Ice Cream - +100 Chips; -5 per hand played
10. Popcorn - +20 Mult; -4 per round played

---

## Phase 2: Stateful Accumulation (Growing Bonuses)

**Difficulty:** ⭐⭐⭐ Hard
**Estimated Jokers Unlocked:** 15-20
**Timeline:** 3-4 hours

### Requirements

1. **Add state fields to joker structs** - Change from unit structs to structs with data
2. **Update `make_jokers!` macro** - Handle non-Default jokers or provide custom defaults
3. **Add joker update methods** - Mutate joker state when events occur
4. **Handle serialization** - Ensure state persists across saves

### Design Pattern

Use a consistent pattern for stateful jokers:

```rust
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct Canio {
    pub bonus_mult: f32,  // Accumulated X mult (starts at 1.0)
}

impl Joker for Canio {
    // ... standard methods ...

    fn effects(&self, _game: &Game) -> Vec<Effects> {
        let bonus = self.bonus_mult;
        fn apply(g: &mut Game, _hand: MadeHand, mult: f32) {
            g.mult = (g.mult as f32 * mult) as usize;
        }
        let closure = move |g: &mut Game, hand: MadeHand| {
            apply(g, hand, bonus);
        };
        vec![Effects::OnScore(Arc::new(Mutex::new(closure)))]
    }
}

// Add method to update state
impl Canio {
    pub fn on_face_card_destroyed(&mut self) {
        self.bonus_mult += 1.0;
    }
}
```

### Changes Needed

#### Game Event Hooks
Add methods to Game to trigger joker updates:

```rust
impl Game {
    pub fn on_card_destroyed(&mut self, card: &Card) {
        if card.is_face() {
            for joker in &mut self.jokers {
                if let Jokers::Canio(ref mut j) = joker {
                    j.on_face_card_destroyed();
                }
            }
        }
        // Handle Glass Joker, etc.
    }

    pub fn on_card_discarded(&mut self, cards: &[Card]) {
        let count = cards.len();
        for joker in &mut self.jokers {
            if let Jokers::Yorick(ref mut j) = joker {
                j.on_cards_discarded(count);
            }
        }
    }

    pub fn on_hand_played(&mut self) {
        for joker in &mut self.jokers {
            if let Jokers::GreenJoker(ref mut j) = joker {
                j.on_hand_played();
            }
        }
    }

    pub fn on_discard_used(&mut self) {
        for joker in &mut self.jokers {
            if let Jokers::GreenJoker(ref mut j) = joker {
                j.on_discard_used();
            }
        }
    }

    pub fn on_card_added_to_deck(&mut self) {
        for joker in &mut self.jokers {
            if let Jokers::Hologram(ref mut j) = joker {
                j.on_card_added();
            }
        }
    }

    pub fn on_card_sold(&mut self) {
        for joker in &mut self.jokers {
            if let Jokers::Campfire(ref mut j) = joker {
                j.on_card_sold();
            }
        }
    }
}
```

#### Default Implementation Strategy
Use custom Default or builder pattern:

```rust
impl Default for Canio {
    fn default() -> Self {
        Self { bonus_mult: 1.0 }
    }
}
```

### Affected Jokers

**Destroyed Card Tracking (2 jokers):**
1. Canio - Gains X1 Mult per face card destroyed
2. Glass Joker - Gains X0.75 Mult per glass card destroyed

**Discard Tracking (2 jokers):**
1. Yorick - Gains X1 Mult every 23 cards discarded
2. Hit the Road - Gains X0.5 Mult per Jack discarded this round

**Hand/Discard Balance (1 joker):**
1. Green Joker - +1 Mult per hand played, -1 per discard

**Purchase/Sale Tracking (3 jokers):**
1. Red Card - Gains +3 Mult when booster pack skipped
2. Swashbuckler - Adds sell value of jokers + 1 per card sold
3. Campfire - Gains X0.25 Mult per card sold (resets on boss blind)

**Deck Modification Tracking (1 joker):**
1. Hologram - Gains X0.25 Mult when card added to deck

**Consecutive Play Tracking (2 jokers):**
1. Obelisk - Gains X0.2 Mult per consecutive hand without most-played hand
2. Ride the Bus - +1 Mult per consecutive hand without faces (Phase 1 overlap)

**Level/Usage Tracking (3 jokers):**
1. Constellation - Gains X0.1 Mult per Planet card used
2. Fortune Teller - +1 Mult per Tarot card used this run
3. Satellite - $1 at end of round per unique Planet card used

**Special Accumulation (3 jokers):**
1. Loyalty Card - X4 Mult every 6 hands played
2. Egg - Gains $3 sell value at end of round
3. Throwback - X0.25 Mult per blind skipped this run

---

## Phase 3: Game Logic Modifications (Complex)

**Difficulty:** ⭐⭐⭐⭐ Very Hard
**Estimated Jokers Unlocked:** 10-15
**Timeline:** 4-6 hours

### Requirements

1. **Add `GameModifiers` struct** - Flags for rule changes
2. **Modify hand detection logic** - Update poker hand recognition
3. **Modify scoring logic** - Update which cards contribute to score
4. **Dynamic effect copying** - For Blueprint/Brainstorm

### Changes Needed

#### GameModifiers Struct
```rust
pub struct GameModifiers {
    // Hand detection modifiers
    pub four_card_straights: bool,
    pub four_card_flushes: bool,
    pub all_cards_are_faces: bool,
    pub smeared_suits: bool,
    pub gap_straights: bool,

    // Scoring modifiers
    pub all_cards_score: bool,
}
```

#### Hand Detection Updates
Modify `hand.rs` functions:
- `is_straight()` - Check for 4-card and gap straights
- `is_flush()` - Check for 4-card flushes and smeared suits
- Card `is_face()` - Check Pareidolia modifier

#### Dynamic Effect Copying
```rust
impl Blueprint {
    fn effects(&self, game: &Game) -> Vec<Effects> {
        // Find joker to the right
        let my_index = game.jokers.iter().position(|j| matches!(j, Jokers::Blueprint(_)));
        if let Some(idx) = my_index {
            if idx + 1 < game.jokers.len() {
                // Copy effects from joker at idx + 1
                return game.jokers[idx + 1].effects(game);
            }
        }
        vec![]
    }
}
```

### Affected Jokers

**Hand Detection Changes (5 jokers):**
1. Four Fingers - 4-card straights/flushes
2. Pareidolia - All cards are face cards
3. Smeared Joker - Hearts/Diamonds same, Spades/Clubs same
4. Shortcut - Straights with 1-rank gaps
5. Hack - Retrigger 2, 3, 4, 5 (needs retrigger system)

**Scoring Changes (1 joker):**
1. Splash - Every played card counts in scoring

**Dynamic Behavior (4 jokers):**
1. Blueprint - Copy joker to the right
2. Brainstorm - Copy leftmost joker
3. Invisible Joker - Duplicate random joker after 2 rounds
4. Madness - Destroy random joker, create 2 free ones

**Special Mechanics (5 jokers):**
1. Ceremonial Dagger - Destroy joker to right, add sell value to mult
2. Dusk - Retrigger all cards in final hand
3. Seltzer - Retrigger all cards for next 10 hands
4. Sock and Buskin - Retrigger all face cards
5. Hanging Chad - Retrigger first card 2 extra times

---

## Implementation Priority

### Immediate (This Session)
- ✅ Phase 1: Hand Access & Round State
- ✅ Phase 2: Stateful Accumulation

### Near Term (Next Session)
- Phase 3: Game Logic Modifications

### Future Considerations
- End-of-round effects (economy jokers)
- Boss blind interaction jokers
- Retrigger system architecture
- Save/load state persistence testing

---

## Testing Strategy

For each phase:

1. **Unit Tests** - Test individual joker effects in isolation
2. **Integration Tests** - Test joker state updates during gameplay
3. **State Persistence Tests** - Verify serialization/deserialization
4. **Regression Tests** - Ensure existing jokers still work

---

## Notes

- Some jokers span multiple phases (e.g., Hit the Road needs both round state and accumulation)
- Blueprint/Brainstorm are particularly complex and may need special handling
- Retrigger mechanics may need a separate system entirely
- Some economy jokers (end-of-round effects) may need additional hooks

---

## Current Status

- **Total Jokers:** 150
- **Implemented:** 91 (60.7%)
- **Phase 1 Candidates:** 15
- **Phase 2 Candidates:** 15
- **Phase 3 Candidates:** 10
- **Remaining (simple):** 19
