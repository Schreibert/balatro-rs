# Phase 4 Remaining: Unimplemented Boss Modifiers

**Status**: 📋 Documentation Complete - Ready for Implementation
**Remaining**: 8/20 boss modifiers (Categories C & D)
**Estimated Effort**: 16-22 hours total

## Overview

This document details the 8 unimplemented boss modifiers from Phase 4, their requirements, and implementation plans. Categories C and D were deferred from the initial Phase 4 implementation due to their need for additional state tracking and architectural changes.

---

## Category C: Hand/Card Restrictions (4 modifiers)

These modifiers require state tracking within each blind but have relatively straightforward implementations.

### 1. The Eye - "No hand type can be repeated"

**Description**: Once you play a hand type (e.g., Pair), you cannot play that same hand type again during this blind.

**Requirements**:
- Track all played hand ranks for the current blind
- Validate hand rank before allowing play
- Reset tracking when blind ends

**State Changes Needed**:
```rust
pub struct Game {
    // ... existing fields ...
    pub played_hand_ranks: HashSet<HandRank>,  // NEW: Track played hands this blind
}
```

**Logic Changes**:

1. **In `select_blind()`**: Reset tracking when entering new blind
```rust
if let Some(modifier) = boss_modifier {
    if modifier.prevents_repeats() {
        self.played_hand_ranks.clear();
    }
}
```

2. **In `play_selected()`**: Validate and track before playing
```rust
pub(crate) fn play_selected(&mut self) -> Result<(), GameError> {
    if self.plays <= 0 {
        return Err(GameError::NoRemainingPlays);
    }

    let selected = SelectHand::new(self.available.selected());
    let best = selected.best_hand()?;

    // NEW: Check if hand type is allowed
    if let Some(modifier) = self.stage.boss_modifier() {
        if modifier.prevents_repeats() && self.played_hand_ranks.contains(&best.rank) {
            return Err(GameError::InvalidAction); // Hand type already played
        }
    }

    self.plays -= 1;
    let score = self.calc_score(best.clone());

    // NEW: Track this hand rank
    if let Some(modifier) = self.stage.boss_modifier() {
        if modifier.prevents_repeats() {
            self.played_hand_ranks.insert(best.rank);
        }
    }

    // ... rest of play_selected
}
```

3. **In `clear_blind()`**: Reset tracking
```rust
fn clear_blind(&mut self) {
    self.score = self.config.base_score;
    self.plays = self.config.plays;
    self.discards = self.config.discards;
    self.played_hand_ranks.clear(); // NEW
    self.deal();
}
```

**Tests Required**:
- `test_the_eye_prevents_repeat_hands()` - Cannot play same hand twice
- `test_the_eye_allows_different_hands()` - Can play different hands
- `test_the_eye_resets_on_new_blind()` - Tracking resets between blinds

**Complexity**: Low-Medium (2-3 hours)

---

### 2. The Mouth - "Only 1 specific hand type allowed"

**Description**: At the start of the blind, a random hand type is chosen. You can only play that hand type during this blind.

**Requirements**:
- Randomly select one hand rank when blind starts
- Validate all plays against allowed hand rank
- Display allowed hand to player

**State Changes Needed**:
```rust
pub struct Game {
    // ... existing fields ...
    pub allowed_hand_rank: Option<HandRank>,  // NEW: The only hand rank allowed (for The Mouth)
}
```

**Logic Changes**:

1. **In `select_blind()`**: Choose random hand rank
```rust
if let Some(modifier) = boss_modifier {
    if matches!(modifier, BossModifier::TheMouth) {
        use rand::seq::SliceRandom;
        let all_ranks = vec![
            HandRank::HighCard, HandRank::OnePair, HandRank::TwoPair,
            HandRank::ThreeOfAKind, HandRank::Straight, HandRank::Flush,
            HandRank::FullHouse, HandRank::FourOfAKind, HandRank::StraightFlush,
            HandRank::RoyalFlush, HandRank::FiveOfAKind, HandRank::FlushHouse,
            HandRank::FlushFive,
        ];
        self.allowed_hand_rank = Some(*all_ranks.choose(&mut rand::thread_rng()).unwrap());
    }
}
```

2. **In `play_selected()`**: Validate hand rank
```rust
let best = selected.best_hand()?;

// NEW: Check if hand type is allowed
if let Some(allowed_rank) = self.allowed_hand_rank {
    if best.rank != allowed_rank {
        return Err(GameError::InvalidAction); // Wrong hand type
    }
}
```

3. **In `clear_blind()`**: Reset allowed rank
```rust
fn clear_blind(&mut self) {
    self.score = self.config.base_score;
    self.plays = self.config.plays;
    self.discards = self.config.discards;
    self.played_hand_ranks.clear();
    self.allowed_hand_rank = None; // NEW
    self.deal();
}
```

**Helper Method**:
```rust
impl BossModifier {
    /// Returns true if this modifier restricts to one hand type
    pub fn restricts_hand_type(&self) -> bool {
        matches!(self, Self::TheMouth)
    }
}
```

**Tests Required**:
- `test_the_mouth_only_allows_specific_hand()` - Only allowed hand can be played
- `test_the_mouth_rejects_other_hands()` - Other hands rejected with error
- `test_the_mouth_random_selection()` - Different hands selected with different seeds

**Complexity**: Medium (3-4 hours)

---

### 3. The Serpent - "First hand always scores 0"

**Description**: The first hand played in this blind always scores 0, regardless of what you play.

**Requirements**:
- Track number of hands played this blind
- Return 0 score for first hand only

**State Changes Needed**:
```rust
pub struct Game {
    // ... existing fields ...
    pub hands_played_this_blind: usize,  // NEW: Count hands played in current blind
}
```

**Logic Changes**:

1. **In `select_blind()`**: Reset counter
```rust
if let Some(modifier) = boss_modifier {
    self.hands_played_this_blind = 0; // NEW: Reset counter
}
```

2. **In `calc_score()`**: Return 0 for first hand
```rust
pub(crate) fn calc_score(&mut self, hand: MadeHand) -> usize {
    let boss_modifier = self.stage.boss_modifier();

    // NEW: Check if first hand scores zero
    if boss_modifier.map(|m| m.first_hand_scores_zero()).unwrap_or(false) {
        if self.hands_played_this_blind == 0 {
            self.hands_played_this_blind += 1;
            return 0;
        }
    }

    self.hands_played_this_blind += 1; // NEW: Increment counter

    // ... rest of calc_score
}
```

3. **In `clear_blind()`**: Reset counter
```rust
fn clear_blind(&mut self) {
    self.score = self.config.base_score;
    self.plays = self.config.plays;
    self.discards = self.config.discards;
    self.played_hand_ranks.clear();
    self.allowed_hand_rank = None;
    self.hands_played_this_blind = 0; // NEW
    self.deal();
}
```

**Tests Required**:
- `test_the_serpent_first_hand_zero()` - First hand scores 0
- `test_the_serpent_second_hand_normal()` - Second hand scores normally
- `test_the_serpent_resets_on_new_blind()` - Counter resets between blinds

**Complexity**: Low (1-2 hours) - Easiest to implement

---

### 4. The Hook - "Discard 2 random cards after each hand"

**Description**: After playing each hand, 2 random cards from your current hand are discarded.

**Requirements**:
- Select 2 random cards from available cards
- Remove them after play (don't redraw)
- Handle edge cases (fewer than 2 cards available)

**State Changes Needed**:
None - uses existing structures

**Logic Changes**:

1. **In `play_selected()`**: Discard random cards after scoring
```rust
pub(crate) fn play_selected(&mut self) -> Result<(), GameError> {
    if self.plays <= 0 {
        return Err(GameError::NoRemainingPlays);
    }
    self.plays -= 1;
    let selected = SelectHand::new(self.available.selected());
    let best = selected.best_hand()?;
    let score = self.calc_score(best);
    let clear_blind = self.handle_score(score)?;

    self.discarded.extend(self.available.selected());
    let removed = self.available.remove_selected();
    self.draw(removed);

    // NEW: The Hook - discard random cards after play
    if let Some(modifier) = self.stage.boss_modifier() {
        let discard_count = modifier.cards_to_discard_after_play();
        if discard_count > 0 {
            use rand::seq::SliceRandom;
            let available_cards = self.available.cards();
            let to_discard_count = discard_count.min(available_cards.len());

            if to_discard_count > 0 {
                let to_discard: Vec<Card> = available_cards
                    .choose_multiple(&mut rand::thread_rng(), to_discard_count)
                    .copied()
                    .collect();

                for card in to_discard {
                    self.discarded.push(card);
                    self.available.remove_card(card);
                }
            }
        }
    }

    if clear_blind {
        self.clear_blind();
    }
    return Ok(());
}
```

2. **Helper method in Available**:
```rust
// In core/src/available.rs
impl Available {
    /// Remove a specific card (not just selected cards)
    pub fn remove_card(&mut self, card: Card) {
        self.cards.retain(|c| c != &card);
    }
}
```

**Tests Required**:
- `test_the_hook_discards_two_cards()` - 2 cards discarded after play
- `test_the_hook_handles_few_cards()` - Works with <2 cards available
- `test_the_hook_multiple_plays()` - Discards on each play

**Complexity**: Medium (2-3 hours)

---

## Category D: Complex Mechanics (4 modifiers)

These modifiers require significant architectural changes to support card visibility states and override core game mechanics.

### 5. The Ox - "Leftmost card is face-down"

**Description**: The leftmost card in your hand is dealt face-down (you can't see its rank or suit). It can still be played but acts as a wildcard with minimal scoring value.

**Requirements**:
- Add face-down state to cards
- Render face-down cards differently
- Modify scoring for face-down cards
- Update card at leftmost position when hand changes

**State Changes Needed**:
```rust
// In core/src/card.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub id: usize,
    pub value: Value,
    pub suit: Suit,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Seal,
    pub face_down: bool,  // NEW: Is this card face-down?
}

impl Card {
    pub fn set_face_down(&mut self, face_down: bool) {
        self.face_down = face_down;
    }

    pub fn is_face_down(&self) -> bool {
        self.face_down
    }
}
```

**Logic Changes**:

1. **In `deal()`**: Mark leftmost card as face-down
```rust
pub(crate) fn deal(&mut self) {
    self.deck.append(&mut self.discarded);
    self.deck.extend(self.available.cards());
    self.available.empty();
    self.deck.shuffle();
    self.draw(self.config.available);

    // NEW: The Ox - make leftmost card face-down
    if let Some(modifier) = self.stage.boss_modifier() {
        if matches!(modifier, BossModifier::TheOx) {
            if let Some(leftmost) = self.available.cards_mut().first_mut() {
                leftmost.set_face_down(true);
            }
        }
    }
}
```

2. **In `Available`**: Update face-down status when cards move
```rust
// In core/src/available.rs
impl Available {
    pub fn move_card(&mut self, direction: MoveDirection, card: Card) -> Result<(), GameError> {
        // ... existing move logic ...

        // NEW: Update face-down status for The Ox
        if let Some(game) = self.game_ref {
            if let Some(modifier) = game.stage.boss_modifier() {
                if matches!(modifier, BossModifier::TheOx) {
                    // Clear all face-down
                    for c in &mut self.cards {
                        c.set_face_down(false);
                    }
                    // Set leftmost face-down
                    if let Some(leftmost) = self.cards.first_mut() {
                        leftmost.set_face_down(true);
                    }
                }
            }
        }

        Ok(())
    }
}
```

3. **In `calc_score()`**: Handle face-down cards
```rust
for card in hand.hand.cards().iter() {
    let is_debuffed = boss_modifier
        .map(|m| m.is_card_debuffed(card))
        .unwrap_or(false);

    // NEW: Face-down cards score minimally (1 chip, 0 mult)
    if card.is_face_down() {
        self.chips += 1;
        continue;
    }

    if !is_debuffed {
        // ... normal scoring
    }
}
```

**Tests Required**:
- `test_the_ox_leftmost_face_down()` - Leftmost card is face-down
- `test_the_ox_face_down_updates_on_move()` - Face-down updates when cards move
- `test_the_ox_face_down_minimal_score()` - Face-down cards score minimally
- `test_the_ox_face_down_after_play()` - New leftmost becomes face-down after play

**Complexity**: High (6-8 hours) - Requires card state changes, needs Available refactoring

---

### 6. The House - "First hand dealt with only 1 card"

**Description**: The first time cards are dealt in this blind, you only get 1 card instead of the normal hand size.

**Requirements**:
- Track deal count per blind
- Override deal amount for first deal only

**State Changes Needed**:
```rust
pub struct Game {
    // ... existing fields ...
    pub deals_this_blind: usize,  // NEW: Count deals in current blind
}
```

**Logic Changes**:

1. **In `select_blind()`**: Reset deal counter
```rust
if let Some(modifier) = boss_modifier {
    self.deals_this_blind = 0; // NEW
}
```

2. **In `deal()`**: Modify deal amount for first deal
```rust
pub(crate) fn deal(&mut self) {
    self.deck.append(&mut self.discarded);
    self.deck.extend(self.available.cards());
    self.available.empty();
    self.deck.shuffle();

    // NEW: The House - first deal only 1 card
    let draw_amount = if let Some(modifier) = self.stage.boss_modifier() {
        if matches!(modifier, BossModifier::TheHouse) && self.deals_this_blind == 0 {
            self.deals_this_blind += 1;
            1
        } else {
            self.deals_this_blind += 1;
            self.config.available
        }
    } else {
        self.config.available
    };

    self.draw(draw_amount);
}
```

3. **In `clear_blind()`**: Reset counter
```rust
fn clear_blind(&mut self) {
    self.score = self.config.base_score;
    self.plays = self.config.plays;
    self.discards = self.config.discards;
    self.played_hand_ranks.clear();
    self.allowed_hand_rank = None;
    self.hands_played_this_blind = 0;
    self.deals_this_blind = 0; // NEW
    self.deal();
}
```

**Tests Required**:
- `test_the_house_first_deal_one_card()` - First deal only 1 card
- `test_the_house_subsequent_deals_normal()` - Later deals normal amount
- `test_the_house_resets_on_new_blind()` - Counter resets between blinds

**Complexity**: Medium (2-3 hours)

---

### 7. The Wheel - "1/7 chance for cards to be face-down"

**Description**: Each card has a 1/7 (14.3%) chance to be dealt face-down. Multiple cards can be face-down simultaneously.

**Requirements**:
- Probabilistic face-down assignment per card
- Reroll on each deal
- Same face-down mechanics as The Ox

**State Changes Needed**:
Same as The Ox (face_down field on Card)

**Logic Changes**:

1. **In `deal()`**: Randomly mark cards face-down
```rust
pub(crate) fn deal(&mut self) {
    self.deck.append(&mut self.discarded);
    self.deck.extend(self.available.cards());
    self.available.empty();
    self.deck.shuffle();
    self.draw(self.config.available);

    // NEW: The Wheel - 1/7 chance per card
    if let Some(modifier) = self.stage.boss_modifier() {
        if matches!(modifier, BossModifier::TheWheel) {
            use rand::Rng;
            for card in self.available.cards_mut() {
                let roll = rand::thread_rng().gen_range(1..=7);
                card.set_face_down(roll == 1); // 1/7 chance
            }
        }
    }
}
```

2. **Scoring**: Same as The Ox (handle face-down in calc_score)

**Tests Required**:
- `test_the_wheel_probabilistic_face_down()` - Some cards face-down with seeded RNG
- `test_the_wheel_reruns_on_deal()` - New roll on each deal
- `test_the_wheel_face_down_scoring()` - Face-down cards score minimally

**Complexity**: High (4-5 hours) - Depends on The Ox implementation

---

### 8. The Pillar - "Cards selected randomly for play"

**Description**: You cannot manually select which cards to play. Instead, the game randomly selects cards for you when you press Play.

**Requirements**:
- Override card selection mechanism
- Random selection when Play is pressed
- Maintain normal hand size limits

**State Changes Needed**:
None directly, but requires action generation changes

**Logic Changes**:

1. **In `play_selected()` or new method**: Override selection
```rust
pub(crate) fn play_selected(&mut self) -> Result<(), GameError> {
    if self.plays <= 0 {
        return Err(GameError::NoRemainingPlays);
    }

    // NEW: The Pillar - random card selection
    if let Some(modifier) = self.stage.boss_modifier() {
        if matches!(modifier, BossModifier::ThePillar) {
            return self.play_random_cards();
        }
    }

    // Normal play logic
    // ...
}

fn play_random_cards(&mut self) -> Result<(), GameError> {
    use rand::seq::SliceRandom;

    // Clear any manual selections
    self.available.clear_selection();

    // Randomly select 1-5 cards
    let available = self.available.cards();
    let select_count = rand::thread_rng().gen_range(1..=5.min(available.len()));
    let to_select = available
        .choose_multiple(&mut rand::thread_rng(), select_count)
        .copied()
        .collect::<Vec<_>>();

    for card in to_select {
        self.available.select_card(card)?;
    }

    // Play normally
    self.plays -= 1;
    let selected = SelectHand::new(self.available.selected());
    let best = selected.best_hand()?;
    let score = self.calc_score(best);
    let clear_blind = self.handle_score(score)?;
    self.discarded.extend(self.available.selected());
    let removed = self.available.remove_selected();
    self.draw(removed);

    if clear_blind {
        self.clear_blind();
    }

    Ok(())
}
```

2. **In `Available`**: Add clear_selection helper
```rust
impl Available {
    pub fn clear_selection(&mut self) {
        for card in &mut self.cards {
            card.selected = false;
        }
    }
}
```

**Tests Required**:
- `test_the_pillar_random_selection()` - Cards selected randomly
- `test_the_pillar_ignores_manual_selection()` - Manual selection ignored
- `test_the_pillar_valid_hand_sizes()` - Selects 1-5 cards
- `test_the_pillar_different_seeds()` - Different selections with different seeds

**Complexity**: Very High (5-6 hours) - Requires action generation overrides, complex testing

---

## Implementation Plan

### Phase 4B: Category C (Estimated 8-12 hours)

**Order of Implementation**:
1. The Serpent (1-2 hours) - Simplest, good warmup
2. The Eye (2-3 hours) - Basic state tracking
3. The Hook (2-3 hours) - Card manipulation
4. The Mouth (3-4 hours) - Most complex validation

**Shared Infrastructure**:
- Add new fields to Game struct
- Update Game::new() initialization
- Update clear_blind() to reset all new state
- Add helper methods to BossModifier

### Phase 4C: Category D (Estimated 8-10 hours)

**Prerequisites**: Must implement The Ox first (provides face-down foundation)

**Order of Implementation**:
1. The Ox (6-8 hours) - Foundation for face-down mechanics
2. The House (2-3 hours) - Simple deal override
3. The Wheel (4-5 hours) - Builds on The Ox
4. The Pillar (5-6 hours) - Most complex, requires action system changes

**Shared Infrastructure**:
- Add face_down field to Card
- Update Card display methods
- Modify Available to track face-down state
- Update calc_score for face-down handling

### Testing Strategy

For each modifier:
1. **Unit Tests**: Test helper methods in boss_modifier.rs
2. **Integration Tests**: Test full gameplay in game.rs
3. **Edge Cases**: Test boundary conditions
4. **Reset Tests**: Verify state resets between blinds

### Total Effort Estimate

- **Category C**: 8-12 hours (4 modifiers)
- **Category D**: 8-10 hours (4 modifiers, assuming Ox is foundation)
- **Total**: 16-22 hours

## Benefits of Completing Categories C & D

1. **Full Boss Modifier Coverage**: 100% of Balatro's boss modifiers
2. **Enhanced RL Training**: More diverse constraints for agents
3. **Complete Feature Parity**: Closer to actual Balatro gameplay
4. **Architectural Improvements**: Face-down system enables future features
5. **Testing Coverage**: Comprehensive edge case coverage

## Alternative: Partial Implementation

If time is limited, consider:
- **Implement Category C only** (8-12 hours): Gets to 16/20 (80%) with moderate effort
- **Defer Category D indefinitely**: High complexity, diminishing returns
- **Cherry-pick**: Implement only The Serpent and The Eye (3-5 hours) for quick wins

Current implementation (12/20, 60%) already provides substantial value for RL training.

---

## References

- PHASE_4_PLAN.md - Original planning document
- PHASE_4_COMPLETION.md - Categories A & B implementation details
- core/src/boss_modifier.rs - Current implementation (lines 1-318)
- Balatro Wiki - Boss Blind reference: https://balatrogame.fandom.com/wiki/Blinds
