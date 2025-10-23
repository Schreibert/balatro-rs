# Phase 3A Implementation Plan: Tarot Cards

## Overview
Implement all 22 Tarot card effects, focusing on a test-driven approach with incremental complexity.

## Implementation Strategy

### 1. Categorize by Complexity

**Category A: No Targets (5 cards) - START HERE**
Simple effects that don't require card selection:
- The Hermit (IX): Double money (max $20)
- Temperance (XIV): Gain sell value of Jokers (max $50)
- The High Priestess (II): Create 2 Planet cards
- The Emperor (IV): Create 2 Tarot cards
- Judgement (XX): Create random Joker

**Category B: Enhancement Tarots (8 cards)**
Modify card enhancement, requires 1-2 card targets:
- The Hierophant (V): 2 cards → Bonus
- The Empress (III): 2 cards → Mult
- The Magician (I): 2 cards → Lucky
- The Lovers (VI): 1 card → Wild
- The Chariot (VII): 1 card → Steel
- Justice (VIII): 1 card → Glass
- The Devil (XV): 1 card → Gold
- The Tower (XVI): 1 card → Stone

**Category C: Suit Conversion Tarots (4 cards)**
Change card suit, requires 1-3 card targets:
- The Star (XVII): 3 cards → Diamonds
- The Moon (XVIII): 3 cards → Clubs
- The Sun (XIX): 3 cards → Hearts
- The World (XXI): 3 cards → Spades

**Category D: Special Effect Tarots (5 cards)**
Complex or unique mechanics:
- The Fool (0): Copy last Tarot/Planet
- The Wheel of Fortune (X): 1/4 chance add edition to Joker
- Strength (XI): Raise rank of 2 cards by 1
- The Hanged Man (XII): Destroy up to 2 cards
- Death (XIII): Convert left card into right card

### 2. Required Infrastructure

**Card Modification Helpers:**
```rust
impl Card {
    pub fn set_enhancement(&mut self, enhancement: Enhancement);
    pub fn set_suit(&mut self, suit: Suit);
    pub fn set_rank(&mut self, rank: Value);
}

impl Game {
    pub fn modify_card_in_deck(&mut self, card_id: CardId, f: impl FnOnce(&mut Card));
    pub fn add_card_to_deck(&mut self, card: Card);
}
```

**Random Generation:**
```rust
impl Game {
    pub fn generate_random_planet(&self) -> Consumables;
    pub fn generate_random_tarot(&self) -> Consumables;
    pub fn generate_random_joker(&self) -> Jokers;
}
```

**Money Helpers:**
```rust
impl Game {
    pub fn add_money_capped(&mut self, amount: usize, cap: usize);
}
```

**Joker Helpers:**
```rust
impl Game {
    pub fn get_joker_sell_value(&self) -> usize;
    pub fn add_edition_to_random_joker(&mut self, edition: Edition) -> bool;
}
```

**Rank Manipulation:**
```rust
impl Value {
    pub fn raise_rank(&self) -> Option<Value>;  // 2→3, K→A, A→None
}
```

### 3. Implementation Order

**Phase 3A.1: Infrastructure (Week 1)**
- [ ] Card modification helpers (50 lines + 100 test lines)
- [ ] Random generation methods (30 lines + 60 test lines)
- [ ] Money/Joker helpers (30 lines + 60 test lines)
- [ ] Rank manipulation (20 lines + 40 test lines)

**Phase 3A.2: Category A - No Target Tarots (Week 1)**
- [ ] The Hermit: Double money
- [ ] Temperance: Joker sell value
- [ ] The High Priestess: Create planets
- [ ] The Emperor: Create tarots
- [ ] Judgement: Create joker
- **Estimated:** 50 lines + 100 test lines

**Phase 3A.3: Category B - Enhancement Tarots (Week 2)**
- [ ] 8 enhancement tarots (one effect per card)
- [ ] Test each with different target counts
- **Estimated:** 80 lines + 160 test lines

**Phase 3A.4: Category C - Suit Conversion Tarots (Week 2)**
- [ ] 4 suit conversion tarots
- [ ] Verify deck persistence
- **Estimated:** 40 lines + 80 test lines

**Phase 3A.5: Category D - Special Tarots (Week 3)**
- [ ] The Fool: Copy mechanism
- [ ] The Wheel of Fortune: Probability + edition
- [ ] Strength: Rank raising
- [ ] The Hanged Man: Card destruction
- [ ] Death: Card transformation
- **Estimated:** 100 lines + 200 test lines

**Total Estimated: ~400 production lines + ~800 test lines**

## Test Strategy

### Test Template for Each Tarot

```rust
#[test]
fn test_tarot_<name>_basic_effect() {
    // Setup: Create game with necessary state
    // Execute: Use tarot with targets (if needed)
    // Verify: Check expected state changes
}

#[test]
fn test_tarot_<name>_constraints() {
    // Test: Insufficient funds, wrong targets, etc.
}

#[test]
fn test_tarot_<name>_edge_cases() {
    // Test: Maximum values, empty deck, etc.
}
```

### Integration Tests

```rust
#[test]
fn test_all_enhancement_tarots() {
    // Verify all 8 enhancement tarots work
}

#[test]
fn test_tarot_targeting_validation() {
    // Verify min/max target requirements enforced
}

#[test]
fn test_tarot_via_action_space() {
    // End-to-end: Buy → Use → Verify effect
}
```

## Card Targeting System

### Approach 1: Simple (For Phase 3A)
Use existing `Option<Vec<Card>>` parameter in `use_effect()`:
- Caller manually selects cards from deck/hand
- Pass as targets to `use_consumable()`
- Validate count in tarot `use_effect()`

**Pros:** No action space changes, works immediately
**Cons:** Requires manual card selection in tests/agents

### Approach 2: Action Space Expansion (Future)
Add `SelectCardForConsumable(Card)` action:
- Multi-step: Select cards, then use consumable
- Track partial state in Game
- Generate actions based on selected cards

**Pros:** Full RL support
**Cons:** Complex, expand action space significantly

**Decision:** Start with Approach 1, defer Approach 2 to Phase 3A.6

## Deferred Items

**Card Discovery/Unlocking:**
- Not implementing collection/unlock system
- All tarots available immediately

**Showman Joker Interaction:**
- Not implementing duplicate prevention logic
- Assume all tarots can appear

**Strength Default Behavior:**
- Not implementing "default to Strength when all 22 held"
- Edge case, low priority

**Purple Seal Spawning:**
- Requires seal effect implementation
- Blocked until Purple Seal triggers work

## Success Criteria

### Phase 3A Complete When:
- [ ] All 22 tarot `use_effect()` methods implemented
- [ ] All card modification helpers working
- [ ] At least 40 tests covering tarot effects (2 per tarot minimum)
- [ ] All tests passing
- [ ] Documentation updated (PHASE_3A_COMPLETION.md)

### Quality Metrics:
- [ ] 80%+ code coverage for tarot.rs
- [ ] All edge cases tested (caps, invalid targets, etc.)
- [ ] Integration tests verify end-to-end flow
- [ ] No regressions in existing 103 tests

## Timeline Estimate

**Aggressive (1 week):**
- Day 1: Infrastructure + Category A
- Day 2-3: Category B (Enhancement tarots)
- Day 4: Category C (Suit conversion)
- Day 5-6: Category D (Special effects)
- Day 7: Testing, documentation, polish

**Realistic (2 weeks):**
- Week 1: Infrastructure + Categories A & B
- Week 2: Categories C & D + comprehensive testing

**Conservative (3 weeks):**
- Week 1: Infrastructure + Category A
- Week 2: Categories B & C
- Week 3: Category D + testing + documentation

## Dependencies

**External:**
- None - all infrastructure exists

**Internal:**
- Card struct (exists)
- Enhancement/Edition/Seal enums (exist)
- Consumable trait (exists)
- Game state (exists)

**Blocked By:**
- Nothing - can start immediately

**Blocks:**
- Shop consumable generation (needs tarots to generate)
- Purple Seal effects (needs tarot generation)
- Booster pack implementation (needs tarots in pool)

## Risk Assessment

**Low Risk:**
- Category A & B tarots (straightforward effects)
- Card modification helpers (simple setters)

**Medium Risk:**
- Category C tarots (need to handle deck vs hand cards)
- The Fool (requires last_consumable_used tracking - already exists)
- The Wheel of Fortune (probability + joker edition)

**High Risk:**
- Death (complex card transformation)
- Strength (rank raising with wraparound logic)
- The Hanged Man (permanent card destruction)

**Mitigation:**
- Start with low-risk items
- Test extensively before moving to high-risk
- Iterate on complex effects with multiple test cases

## Notes

- This plan focuses on **effect implementation only**
- Card targeting in action space is **deferred**
- Shop generation is **separate phase**
- All 22 tarots will be **manually testable** by end of Phase 3A
