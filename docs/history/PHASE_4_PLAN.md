# Phase 4 Implementation Plan: Boss Blind Modifiers

## Overview
Implement 20 boss blind modifiers that add constraints and challenges when facing boss blinds. These modifiers dramatically change gameplay strategy and are critical for RL training diversity.

## Implementation Strategy

### 1. Categorize by Complexity

**Category A: Simple Constraints (Start Here - 6 modifiers)**
Modifiers that affect game parameters but don't require complex logic:
- **The Wall**: ×2.5 score requirement instead of ×2
- **The Manacle**: -1 hand size for this blind
- **The Water**: Start with 0 discards
- **The Needle**: Can only play 1 hand this blind
- **The Arm**: Decrease level of played hand by 1 after each play
- **The Tooth**: Lose $1 per card played

**Category B: Card Debuffing (6 modifiers)**
Modifiers that debuff (disable) certain cards:
- **The Club**: All Clubs are debuffed
- **The Goad**: All Spades are debuffed
- **The Window**: All Diamonds are debuffed
- **The Head**: All Hearts are debuffed
- **The Plant**: All face cards (J/Q/K) are debuffed
- **The Flint**: Chips and mult are halved

**Category C: Hand/Card Restrictions (4 modifiers)**
Modifiers that restrict which hands/cards can be played:
- **The Eye**: No hand type can be repeated
- **The Mouth**: Only 1 specific hand type can be played
- **The Serpent**: First hand played always scores 0
- **The Hook**: Discard 2 random cards after each hand played

**Category D: Complex Mechanics (4 modifiers)**
Modifiers requiring special game state tracking:
- **The Ox**: Leftmost card is played face-down (no rank/suit)
- **The House**: First hand is dealt with only 1 card
- **The Wheel**: Each card has 1/7 chance to be face-down
- **The Pillar**: Cards are selected randomly for play

## Implementation Steps

### Step 1: Create BossModifier Module

**New file:** `core/src/boss_modifier.rs`

```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BossModifier {
    // Category A: Simple Constraints
    TheWall,
    TheManacle,
    TheWater,
    TheNeedle,
    TheArm,
    TheTooth,

    // Category B: Card Debuffing
    TheClub,
    TheGoad,
    TheWindow,
    TheHead,
    ThePlant,
    TheFlint,

    // Category C: Hand/Card Restrictions
    TheEye,
    TheMouth,
    TheSerpent,
    TheHook,

    // Category D: Complex Mechanics
    TheOx,
    TheHouse,
    TheWheel,
    ThePillar,
}

impl BossModifier {
    pub fn name(&self) -> &str { ... }
    pub fn description(&self) -> &str { ... }

    /// Returns the score multiplier (default 2.0 for boss blinds)
    pub fn score_multiplier(&self) -> f64 { ... }

    /// Returns hand size modifier
    pub fn hand_size_modifier(&self) -> i32 { ... }

    /// Returns starting discards modifier
    pub fn discard_modifier(&self) -> i32 { ... }

    /// Check if a card is debuffed by this modifier
    pub fn is_card_debuffed(&self, card: &Card) -> bool { ... }

    /// Get all boss modifiers
    pub fn all() -> Vec<Self> { ... }

    /// Get a random boss modifier
    pub fn random() -> Self { ... }
}
```

### Step 2: Update Stage System

**File:** `core/src/stage.rs`

Modify `Stage` enum to include boss modifier:

```rust
pub enum Stage {
    PreBlind(),
    Blind(Blind, Option<BossModifier>), // Add modifier
    PostBlind(),
    Shop(),
    End(End),
}
```

### Step 3: Update Game State

**File:** `core/src/game.rs`

Add fields to track boss modifier state:

```rust
pub struct Game {
    // ... existing fields

    // Boss modifier tracking
    pub current_boss_modifier: Option<BossModifier>,
    pub hands_played_this_blind: Vec<HandRank>,  // For The Eye
    pub first_hand_of_blind: bool,                // For The Serpent
    pub hands_remaining: Option<usize>,           // For The Needle
}
```

### Step 4: Implement Modifier Logic

#### A. Score Calculation (`calc_score`)
- Apply The Flint's halving
- Check for debuffed cards
- Track hand types for The Eye

#### B. Action Generation (`gen_actions`)
- Restrict hand types for The Mouth/The Eye
- Limit hands for The Needle

#### C. Game State Updates
- Apply The Arm's level decrease
- Deduct money for The Tooth
- Handle The Hook's discard
- Track first hand for The Serpent

#### D. Deal/Draw Logic
- Apply The Manacle's hand size reduction
- Implement The House's single-card deal
- Handle The Water's 0 discards

### Step 5: Test Strategy

**Test Template:**
```rust
#[test]
fn test_boss_<name>_basic_effect() {
    // Setup: Create game in Boss blind with modifier
    // Execute: Play a hand/perform action
    // Verify: Check modifier constraint is enforced
}

#[test]
fn test_boss_<name>_scoring() {
    // Test: Score calculation with modifier active
}

#[test]
fn test_boss_<name>_edge_cases() {
    // Test: Edge cases specific to modifier
}
```

## Implementation Order

### Phase 4.1: Infrastructure (Day 1)
- [x] Create boss_modifier.rs module
- [x] Update Stage enum
- [x] Add game state tracking fields
- [x] Implement BossModifier methods (name, desc, etc.)
- **Estimated:** 150 lines + basic tests

### Phase 4.2: Category A - Simple Constraints (Day 1-2)
- [ ] The Wall (score multiplier)
- [ ] The Manacle (hand size)
- [ ] The Water (discards)
- [ ] The Needle (hand limit)
- [ ] The Arm (level decrease)
- [ ] The Tooth (money loss)
- **Estimated:** 120 lines + 12 tests (2 per modifier)

### Phase 4.3: Category B - Card Debuffing (Day 2)
- [ ] The Club/Goad/Window/Head (suit debuffs)
- [ ] The Plant (face card debuff)
- [ ] The Flint (score halving)
- **Estimated:** 80 lines + 12 tests

### Phase 4.4: Category C - Hand Restrictions (Day 3)
- [ ] The Eye (no repeats)
- [ ] The Mouth (single hand type)
- [ ] The Serpent (first hand zero)
- [ ] The Hook (discard after play)
- **Estimated:** 100 lines + 8 tests

### Phase 4.5: Category D - Complex Mechanics (Day 3-4)
- [ ] The Ox (face-down card)
- [ ] The House (single card deal)
- [ ] The Wheel (random face-down)
- [ ] The Pillar (random selection)
- **Estimated:** 120 lines + 8 tests

**Total Estimated: ~570 production lines + ~40 tests**

## Boss Modifier Details

### Category A: Simple Constraints

1. **The Wall**
   - Effect: Boss blind requires ×2.5 score instead of ×2
   - Implementation: Modify `required_score()` calculation
   - Test: Verify score threshold is 2.5x base

2. **The Manacle**
   - Effect: -1 hand size for this blind
   - Implementation: Modify `deal()` to draw fewer cards
   - Test: Verify hand size is reduced by 1

3. **The Water**
   - Effect: Start with 0 discards
   - Implementation: Set discards to 0 in blind setup
   - Test: Verify no discards available

4. **The Needle**
   - Effect: Only 1 hand can be played
   - Implementation: Track hands_remaining, restrict when 0
   - Test: Verify only 1 hand action generated

5. **The Arm**
   - Effect: Played hand's level decreases by 1
   - Implementation: Downgrade hand level after each play
   - Test: Verify level decreases, stops at 1

6. **The Tooth**
   - Effect: Lose $1 per card played
   - Implementation: Deduct money in handle_action
   - Test: Verify money loss matches cards played

### Category B: Card Debuffing

7-10. **The Club/Goad/Window/Head**
    - Effect: All cards of specific suit are debuffed
    - Implementation: Check card suit, skip if debuffed
    - Test: Verify debuffed cards don't score

11. **The Plant**
    - Effect: All face cards are debuffed
    - Implementation: Check if card is J/Q/K
    - Test: Verify face cards don't score

12. **The Flint**
    - Effect: Chips and mult are halved
    - Implementation: Divide final score by 2
    - Test: Verify score is exactly half

### Category C: Hand Restrictions

13. **The Eye**
    - Effect: No hand type can be repeated
    - Implementation: Track played hands, restrict repeats
    - Test: Verify same hand type blocked after first play

14. **The Mouth**
    - Effect: Only 1 specific hand type allowed
    - Implementation: Randomly choose hand type, restrict others
    - Test: Verify only chosen hand type can be played

15. **The Serpent**
    - Effect: First hand always scores 0
    - Implementation: Track first_hand flag, return 0 score
    - Test: Verify first hand scores 0, second scores normally

16. **The Hook**
    - Effect: Discard 2 random cards after each hand
    - Implementation: Remove 2 random cards from hand after play
    - Test: Verify 2 cards removed after play

### Category D: Complex Mechanics

17. **The Ox**
    - Effect: Leftmost scoring card is face-down
    - Implementation: Mask leftmost selected card's rank/suit
    - Test: Verify leftmost card doesn't contribute rank/suit

18. **The House**
    - Effect: First hand dealt with only 1 card
    - Implementation: Override deal() for first hand
    - Test: Verify first hand has 1 card, rest normal

19. **The Wheel**
    - Effect: Each card has 1/7 chance to be face-down
    - Implementation: Randomly mask cards in deal()
    - Test: Verify probabilistic face-down (run multiple times)

20. **The Pillar**
    - Effect: Cards selected randomly for play
    - Implementation: Override card selection in play action
    - Test: Verify player can't choose specific cards

## Debuffed Card Mechanics

**Debuffed cards:**
- Don't contribute chips
- Don't contribute mult
- Don't contribute to hand rank detection
- Don't trigger abilities
- Essentially "invisible" for scoring

**Implementation:**
```rust
pub struct Card {
    // ... existing fields
    pub debuffed: bool,  // Set by boss modifier
}

impl Card {
    pub fn chips(&self) -> usize {
        if self.debuffed { return 0; }
        // ... normal logic
    }

    pub fn is_scoring(&self) -> bool {
        !self.debuffed
    }
}
```

## Integration Points

### 1. Blind Selection (PreBlind → Blind)
```rust
fn select_blind(&mut self, blind: Blind) {
    let modifier = if blind == Blind::Boss {
        Some(BossModifier::random())
    } else {
        None
    };
    self.stage = Stage::Blind(blind, modifier);
    self.current_boss_modifier = modifier;
    self.apply_boss_modifier_setup();
}
```

### 2. Action Generation
```rust
fn gen_actions(&self) -> impl Iterator<Item = Action> {
    if let Some(modifier) = self.current_boss_modifier {
        // Apply modifier restrictions
        match modifier {
            BossModifier::TheNeedle => {
                // Only generate actions if hands remaining
            }
            BossModifier::TheEye => {
                // Filter out already-played hand types
            }
            // ... etc
        }
    }
    // ... normal generation
}
```

### 3. Score Calculation
```rust
fn calc_score(&mut self) -> usize {
    // Mark debuffed cards
    if let Some(modifier) = self.current_boss_modifier {
        for card in &mut self.selected {
            if modifier.is_card_debuffed(card) {
                card.debuffed = true;
            }
        }
    }

    // Calculate score
    let mut score = // ... normal calculation

    // Apply modifier adjustments
    if let Some(modifier) = self.current_boss_modifier {
        if modifier == BossModifier::TheFlint {
            score /= 2;
        }
    }

    score
}
```

## Testing Strategy

### Unit Tests (40 tests minimum)
- 2 tests per modifier (basic + edge case)
- Test each in isolation

### Integration Tests
- Test combinations with jokers
- Test with consumables
- Test with card enhancements

### Edge Cases
- No cards in hand (The Hook)
- Last hand (The Needle)
- All cards debuffed
- Level 1 hand (The Arm can't decrease further)
- No money (The Tooth can't take more)

## Success Criteria

### Phase 4 Complete When:
- [ ] All 20 boss modifiers implemented
- [ ] Stage system updated with modifier support
- [ ] At least 40 tests passing (2 per modifier)
- [ ] Boss modifiers randomly assigned to Boss blinds
- [ ] All modifier effects working correctly
- [ ] No regressions in existing 183 tests
- [ ] Documentation updated (PHASE_4_COMPLETION.md)

### Quality Metrics:
- [ ] 80%+ code coverage for boss_modifier.rs
- [ ] All edge cases tested
- [ ] Integration tests verify interactions
- [ ] Modifier assignment is random but deterministic with seed

## Timeline Estimate

**Aggressive (3-4 days):**
- Day 1: Infrastructure + Category A
- Day 2: Category B + start Category C
- Day 3: Finish Category C + Category D
- Day 4: Testing + documentation

**Realistic (5-6 days):**
- Days 1-2: Infrastructure + Category A
- Day 3: Category B
- Day 4: Category C
- Day 5: Category D
- Day 6: Comprehensive testing + documentation

## Risk Assessment

**Low Risk:**
- Category A (simple constraints) - straightforward parameter changes
- Category B (debuffing) - similar to existing card state

**Medium Risk:**
- Category C (restrictions) - requires state tracking
- The Arm (level manipulation) - need to ensure level 1 minimum

**High Risk:**
- The Pillar (random selection) - may conflict with action generation
- The Wheel (probabilistic) - need deterministic randomness for RL
- The Ox/House (special dealing) - complex deal logic changes

**Mitigation:**
- Start with low-risk categories
- Test extensively before moving to high-risk
- Use seeded RNG for determinism

## Notes

- Boss modifiers only apply to Boss blinds (not Small/Big)
- Modifiers are randomly assigned each ante
- Future: Could add "fixed" bosses that always have same modifier
- Future: Could add difficulty scaling (harder modifiers at higher antes)
- All modifiers must work with existing joker/consumable systems
