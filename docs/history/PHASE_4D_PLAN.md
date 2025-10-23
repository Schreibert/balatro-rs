# Phase 4D Plan: Category D Boss Modifiers

**Status**: Planning
**Date**: 2025-10-16
**Goal**: Implement the final 4 boss modifiers (Category D) to achieve 100% coverage (20/20)

## Overview

Category D modifiers are the most complex boss modifiers, requiring significant architectural changes to support face-down cards and modified game logic. These modifiers fundamentally change how cards are dealt, displayed, and selected.

## Current State

- **Implemented**: 16/20 boss modifiers (80%)
  - Category A (6/6): Simple constraints
  - Category B (6/6): Card debuffing
  - Category C (4/4): Hand/card restrictions with state tracking
- **Remaining**: 4/20 boss modifiers (20%)
  - Category D (0/4): Complex mechanics requiring architectural changes
- **Tests Passing**: 218/218

## Category D: The 4 Remaining Modifiers

### 1. The Ox
**Effect**: Leftmost card in hand is face-down (no rank/suit visible)
**Complexity**: High
**Requirements**:
- Add `is_face_down: bool` field to Card struct
- Cards that are face-down cannot contribute to hand detection
- Face-down cards are "invisible" for hand ranking purposes
- UI must indicate face-down state (but that's not our concern in core)
- When played, face-down cards flip up and score normally

**Implementation Strategy**:
1. Add `is_face_down` field to Card struct
2. Modify `deal()` to mark leftmost card as face-down when The Ox is active
3. Update hand detection in `hand.rs` to skip face-down cards
4. Ensure face-down cards flip up during scoring

### 2. The House
**Effect**: First hand is dealt with only 1 card
**Complexity**: Medium
**Requirements**:
- Track whether this is the first deal of the blind
- Modify `deal()` to draw only 1 card on first deal
- Subsequent deals are normal (8 cards)

**Implementation Strategy**:
1. Add `first_deal_this_blind: bool` to Game struct
2. Check this flag in `deal()` when The House is active
3. Draw 1 card if first deal, otherwise normal amount
4. Reset flag in `clear_blind()`

### 3. The Wheel
**Effect**: Each card has 1/7 chance to be face-down when dealt
**Complexity**: High
**Requirements**:
- Similar to The Ox, requires `is_face_down` field
- Apply probabilistic face-down state during `deal()`
- Each card independently has 1/7 chance

**Implementation Strategy**:
1. Reuse `is_face_down` field from The Ox implementation
2. Modify `deal()` to randomly mark cards as face-down (1/7 chance each)
3. Hand detection already handles face-down from The Ox

### 4. The Pillar
**Effect**: Cards are selected randomly for play instead of player choice
**Complexity**: Very High
**Requirements**:
- Override card selection logic
- When player attempts to play, randomly select cards instead
- Maintain same number of cards as player attempted to select
- This is a UI/input override rather than game state change

**Implementation Strategy**:
1. Add logic in `play_selected()` to randomize selection when The Pillar is active
2. Clear current selection, randomly select same number of cards
3. Proceed with normal play logic

## Architectural Changes Required

### Card Struct Changes

```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub id: usize,
    pub edition: Edition,
    pub enhancement: Option<Enhancement>,
    pub seal: Option<Seal>,
    pub is_face_down: bool,  // NEW FIELD
}
```

**Impact**:
- All `Card::new()` calls must initialize `is_face_down: false`
- Serialization/deserialization includes new field
- Python bindings include new field

### Game Struct Changes

```rust
pub struct Game {
    // ... existing fields ...

    // Phase 4D: Category D Boss Modifier State
    pub first_deal_this_blind: bool,  // For The House
}
```

### Hand Detection Changes

In `hand.rs`, modify `SelectHand` to filter out face-down cards before analysis:

```rust
impl SelectHand {
    pub fn new(cards: Vec<Card>) -> Self {
        // Filter out face-down cards - they don't participate in hand detection
        let visible_cards: Vec<Card> = cards.into_iter()
            .filter(|c| !c.is_face_down)
            .collect();

        Self { cards: visible_cards }
    }
}
```

### Boss Modifier Query Methods

Add to `boss_modifier.rs`:

```rust
impl BossModifier {
    /// Returns true if leftmost card should be face-down
    pub fn leftmost_face_down(&self) -> bool {
        matches!(self, Self::TheOx)
    }

    /// Returns true if first hand should be dealt with 1 card
    pub fn first_hand_one_card(&self) -> bool {
        matches!(self, Self::TheHouse)
    }

    /// Returns probability (0.0-1.0) that each card is face-down
    pub fn face_down_probability(&self) -> f64 {
        match self {
            Self::TheWheel => 1.0 / 7.0,
            _ => 0.0,
        }
    }

    /// Returns true if cards should be randomly selected for play
    pub fn random_card_selection(&self) -> bool {
        matches!(self, Self::ThePillar)
    }
}
```

## Implementation Order

Follow TDD approach - write tests first:

1. **Step 1: Card Infrastructure**
   - Add `is_face_down` field to Card
   - Update all Card::new() calls
   - Add Card::set_face_down() and Card::is_visible() methods
   - Write tests for face-down state

2. **Step 2: Hand Detection Updates**
   - Modify SelectHand::new() to filter face-down cards
   - Write tests for hand detection with face-down cards
   - Verify existing hand tests still pass

3. **Step 3: The Ox**
   - Add query method to BossModifier
   - Modify deal() to mark leftmost card face-down
   - Write 3 tests:
     - Leftmost card is face-down when dealt
     - Hand detection ignores face-down card
     - Face-down card flips up when played
   - Verify all tests pass

4. **Step 4: The House**
   - Add `first_deal_this_blind` field to Game
   - Add query method to BossModifier
   - Modify deal() to handle first deal special case
   - Reset flag in clear_blind()
   - Write 3 tests:
     - First deal has 1 card
     - Second deal has normal amount
     - Flag resets on new blind
   - Verify all tests pass

5. **Step 5: The Wheel**
   - Add query method to BossModifier
   - Modify deal() to probabilistically mark cards face-down
   - Write 3 tests:
     - Cards can be face-down (use seeded RNG)
     - Multiple cards can be face-down
     - Hand detection works with multiple face-down
   - Verify all tests pass

6. **Step 6: The Pillar**
   - Add query method to BossModifier
   - Modify play_selected() to randomize selection
   - Write 3 tests:
     - Selection is randomized (verify different cards selected)
     - Maintains same number of cards
     - Play proceeds normally after randomization
   - Verify all tests pass

## Test Strategy

### Unit Tests (boss_modifier.rs)
- Test query methods return correct values
- Test all 20 modifiers still work

### Integration Tests (game.rs)
- 3 tests per modifier (12 total new tests)
- Test face-down mechanics
- Test first deal mechanics
- Test probabilistic mechanics
- Test randomization mechanics

### Existing Tests
- All 218 existing tests must continue to pass
- Card tests will need updates for new field
- Hand tests should be unaffected (face_down defaults to false)

## Expected Test Count

- Starting: 218 tests
- Card infrastructure tests: +3 tests
- Hand detection tests: +2 tests
- The Ox tests: +3 tests
- The House tests: +3 tests
- The Wheel tests: +3 tests
- The Pillar tests: +3 tests
- **Final: 235 tests**

## Risks and Mitigations

### Risk 1: Breaking Existing Functionality
- **Mitigation**: Add `is_face_down: false` default to all existing Card::new() calls
- **Mitigation**: Run full test suite after each change
- **Mitigation**: Ensure backward compatibility by making face-down opt-in

### Risk 2: Hand Detection Edge Cases
- **Mitigation**: Comprehensive tests for face-down + visible card combinations
- **Mitigation**: Test all hand types with face-down cards

### Risk 3: Serialization Issues
- **Mitigation**: serde should handle new field automatically with default
- **Mitigation**: Test Python bindings after Card struct changes

### Risk 4: The Pillar Complexity
- **Mitigation**: Implement last after other 3 are working
- **Mitigation**: May be able to simplify by clearing and re-selecting in available

## Success Criteria

1. ✅ All 20 boss modifiers implemented
2. ✅ All 235 tests passing
3. ✅ No regression in existing functionality
4. ✅ Face-down cards work correctly in all scenarios
5. ✅ Documentation complete (PHASE_4D_COMPLETION.md)

## Timeline Estimate

- Card infrastructure: 2 hours
- Hand detection updates: 1 hour
- The Ox: 2 hours
- The House: 2 hours
- The Wheel: 2 hours
- The Pillar: 3 hours
- Testing and debugging: 2 hours
- Documentation: 1 hour
- **Total: ~15 hours**

## Next Steps

1. Add `is_face_down` field to Card struct
2. Update all Card::new() calls to include `is_face_down: false`
3. Write tests for card face-down state
4. Modify hand detection to filter face-down cards
5. Proceed with modifier implementations in order

---

**Note**: This is the most complex phase of boss modifier implementation. Take time to ensure each step is correct before proceeding to the next.
