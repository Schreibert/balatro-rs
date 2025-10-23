# Phase 3C Implementation Plan: Spectral Cards

## Overview
Implement all 18 Spectral card effects. These are high-impact consumables with strong or risky effects.

## Implementation Strategy

### 1. Categorize by Complexity

**Category A: Seal Addition (4 cards) - START HERE**
Simple effects that add seals to cards:
- Talisman: Add Gold Seal to 1 card
- Deja Vu: Add Red Seal to 1 card
- Trance: Add Blue Seal to 1 card
- Medium: Add Purple Seal to 1 card

**Category B: Card Creation/Destruction (4 cards)**
Destroy cards and add enhanced replacements:
- Familiar: Destroy 1 random, add 3 enhanced face cards
- Grim: Destroy 1 random, add 2 enhanced Aces
- Incantation: Destroy 1 random, add 4 enhanced numbers
- Immolate: Destroy 5 random cards, gain $20

**Category C: Edition/Enhancement (2 cards)**
Add editions to cards/jokers:
- Aura: Add random edition (Foil/Holo/Poly) to 1 card
- Cryptid: Create 2 copies of a card

**Category D: Joker Manipulation (4 cards)**
Create or modify jokers:
- Wraith: Create Rare Joker, set money to $0
- The Soul: Create Legendary Joker
- Ankh: Copy 1 Joker, destroy others
- Hex: Add Polychrome to 1 Joker, destroy others

**Category E: Bulk Deck Operations (2 cards)**
Modify all cards at once:
- Sigil: Convert all cards to same random suit
- Ouija: Convert all cards to same rank, -1 hand size

**Category F: Hand Size Modifiers (1 card)**
Permanent game state changes:
- Ectoplasm: Add Negative to random Joker, -1 hand size

**Category G: Universal Upgrade (1 card)**
Upgrade everything:
- Black Hole: Upgrade all poker hands

### 2. Required Infrastructure

**Already Exists:**
```rust
// Card modification
impl Card {
    pub fn set_seal(&mut self, seal: Seal);
    pub fn set_edition(&mut self, edition: Edition);
    pub fn set_enhancement(&mut self, enhancement: Enhancement);
}

// Deck operations
impl Game {
    pub fn add_card_to_deck(&mut self, card: Card);
    pub fn destroy_card(&mut self, card: Card);
    pub fn modify_card_in_deck<F>(&mut self, card_id: usize, f: F);
}

// Joker operations
impl Game {
    pub fn generate_random_joker(&self) -> Jokers;
}
```

**Needs to be Added:**
```rust
impl Game {
    // Random card selection
    pub fn get_random_card_from_deck(&self) -> Option<Card>;
    pub fn get_random_cards(&self, count: usize) -> Vec<Card>;

    // Enhanced card creation
    pub fn create_enhanced_face_card(&self) -> Card;  // Random J/Q/K with enhancement
    pub fn create_enhanced_ace(&self) -> Card;         // Random Ace with enhancement
    pub fn create_enhanced_number(&self) -> Card;      // Random 2-10 with enhancement

    // Joker operations
    pub fn generate_rare_joker(&self) -> Jokers;
    pub fn generate_legendary_joker(&self) -> Jokers;
    pub fn copy_joker(&self, joker: &Jokers) -> Jokers;
    pub fn destroy_all_jokers_except(&mut self, keep_idx: usize);

    // Bulk operations
    pub fn convert_all_cards_to_suit(&mut self, suit: Suit);
    pub fn convert_all_cards_to_rank(&mut self, rank: Value);

    // Hand size modification
    pub fn modify_hand_size(&mut self, delta: i32);
}

impl Jokers {
    pub fn by_rarity(rarity: Rarity) -> Vec<Self>;  // Already exists but needs pub
    pub fn all_rare() -> Vec<Self>;
    pub fn all_legendary() -> Vec<Self>;
}
```

**Hand Size Tracking:**
```rust
// In Config or Game struct
pub hand_size: usize,  // Default 8, modified by Ouija/Ectoplasm
```

### 3. Implementation Order

**Phase 3C.1: Infrastructure (Day 1)**
- [ ] Random card selection methods
- [ ] Enhanced card creation methods
- [ ] Rare/Legendary joker generation
- [ ] Bulk deck operation methods
- [ ] Hand size tracking field
- [ ] Joker copy/destroy methods
- **Estimated:** 150 lines + 100 test lines

**Phase 3C.2: Category A - Seal Addition (Day 1)**
- [ ] Talisman: Gold Seal
- [ ] Deja Vu: Red Seal
- [ ] Trance: Blue Seal
- [ ] Medium: Purple Seal
- **Estimated:** 40 lines + 80 test lines

**Phase 3C.3: Category B - Card Creation/Destruction (Day 2)**
- [ ] Familiar: Destroy 1, add 3 face cards
- [ ] Grim: Destroy 1, add 2 Aces
- [ ] Incantation: Destroy 1, add 4 numbers
- [ ] Immolate: Destroy 5, gain $20
- **Estimated:** 80 lines + 120 test lines

**Phase 3C.4: Category C - Edition/Enhancement (Day 2)**
- [ ] Aura: Random edition to card
- [ ] Cryptid: Copy card twice
- **Estimated:** 40 lines + 60 test lines

**Phase 3C.5: Category D - Joker Manipulation (Day 3)**
- [ ] Wraith: Rare joker, $0
- [ ] The Soul: Legendary joker
- [ ] Ankh: Copy joker, destroy others
- [ ] Hex: Polychrome joker, destroy others
- **Estimated:** 80 lines + 100 test lines

**Phase 3C.6: Category E - Bulk Operations (Day 3)**
- [ ] Sigil: All cards → same suit
- [ ] Ouija: All cards → same rank, -1 hand size
- **Estimated:** 40 lines + 60 test lines

**Phase 3C.7: Category F & G - Special Effects (Day 4)**
- [ ] Ectoplasm: Negative joker, -1 hand size
- [ ] Black Hole: Upgrade all hands
- **Estimated:** 40 lines + 60 test lines

**Total Estimated: ~470 production lines + ~580 test lines**

## Test Strategy

### Test Template for Each Spectral

```rust
#[test]
fn test_spectral_<name>_basic_effect() {
    // Setup: Create game with necessary state
    // Execute: Use spectral with targets (if needed)
    // Verify: Check expected state changes
}

#[test]
fn test_spectral_<name>_deck_size() {
    // Test: Verify deck size changes correctly
}

#[test]
fn test_spectral_<name>_edge_cases() {
    // Test: Empty deck, no jokers, etc.
}
```

### Integration Tests

```rust
#[test]
fn test_hand_size_modifications() {
    // Test cumulative hand size changes from Ouija + Ectoplasm
}

#[test]
fn test_random_card_selection() {
    // Verify random selection works correctly
}

#[test]
fn test_enhanced_card_generation() {
    // Verify face/ace/number cards created with enhancements
}
```

## Card Targeting Approach

**Simple Approach (Phase 3C):**
- Use existing `Option<Vec<Card>>` parameter in `use_effect()`
- For spectrals that need targets, pass selected card(s)
- For random selection (Familiar, Grim, etc.), select in effect

**Example:**
```rust
// Talisman - user selects 1 card
g.use_consumable(Consumables::Spectral(Spectrals::Talisman), Some(vec![card])).unwrap();

// Familiar - random selection, no user input
g.use_consumable(Consumables::Spectral(Spectrals::Familiar), None).unwrap();
```

## Deferred Items

**Random vs Selected:**
- Some spectrals say "1 random card" in game but we'll allow targeting
- This makes testing easier and gives more control

**Joker Edition Field:**
- Need to add edition field to individual Joker structs OR
- Store editions separately in Game (HashMap<JokerIdx, Edition>)
- Decision: Add `edition: Option<Edition>` to each Joker struct

**Hand Size Persistence:**
- Add `hand_size: usize` to Config or Game
- Default to 8, modify with Ouija/Ectoplasm
- Affects how many cards are drawn

## Success Criteria

### Phase 3C Complete When:
- [ ] All 18 spectral `use_effect()` methods implemented
- [ ] All helper methods working (random selection, enhanced creation, etc.)
- [ ] At least 36 tests covering spectral effects (2 per spectral minimum)
- [ ] Hand size modification system working
- [ ] Joker rarity generation working
- [ ] All tests passing
- [ ] Documentation updated (PHASE_3C_COMPLETION.md)

### Quality Metrics:
- [ ] 80%+ code coverage for spectral.rs
- [ ] All edge cases tested (empty deck, no jokers, etc.)
- [ ] Integration tests verify complex interactions
- [ ] No regressions in existing 137 tests

## Timeline Estimate

**Aggressive (3 days):**
- Day 1: Infrastructure + Categories A & B
- Day 2: Categories C, D, E
- Day 3: Categories F, G + comprehensive testing

**Realistic (5 days):**
- Days 1-2: Infrastructure + Categories A & B
- Days 3-4: Categories C, D, E
- Day 5: Categories F, G + testing + documentation

**Conservative (7 days):**
- Days 1-2: Infrastructure
- Days 3-4: Categories A, B, C
- Days 5-6: Categories D, E, F, G
- Day 7: Testing + documentation

## Dependencies

**External:**
- None - all infrastructure exists or is simple to add

**Internal:**
- Card struct (exists)
- Enhancement/Edition/Seal enums (exist)
- Joker generation (exists for common, need rare/legendary)
- Consumable trait (exists)
- Game state (exists)

**Blocked By:**
- Nothing - can start immediately

**Blocks:**
- Shop spectral generation (needs spectrals to generate)
- Spectral Pack implementation (needs spectrals in pool)

## Risk Assessment

**Low Risk:**
- Category A (seal addition) - straightforward
- Category C (editions) - similar to tarot effects
- Black Hole - just loop through all hands

**Medium Risk:**
- Random card selection - need to handle empty deck
- Enhanced card creation - need to pick random enhancements
- Hand size modifications - need persistence

**High Risk:**
- Joker manipulation (Ankh, Hex) - complex destruction logic
- Bulk operations (Sigil, Ouija) - modify all cards at once
- Ectoplasm - Negative edition needs to be tracked on jokers

**Mitigation:**
- Start with low-risk items
- Test extensively before moving to high-risk
- Add comprehensive edge case tests for risky spectrals

## Notes

- This plan focuses on **effect implementation only**
- Card targeting in action space is **deferred**
- Shop generation is **separate phase**
- All 18 spectrals will be **manually testable** by end of Phase 3C
- Hand size changes are **permanent** and cumulative
