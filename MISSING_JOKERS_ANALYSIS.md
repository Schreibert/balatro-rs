# Missing Jokers Analysis

## Summary

**Total Jokers in Balatro:** 150
**Currently Implemented:** 120
**Missing:** 30 (28 identified below, 2 need verification)

---

## Missing Jokers by Complexity

### SIMPLE (8 jokers)
These jokers can be implemented quickly as they only require basic existing systems (chips/mult bonuses, simple conditionals, no new state tracking needed).

**None identified** - All remaining missing jokers require at least medium complexity features.

---

### MEDIUM (12 jokers)
These jokers use existing game systems but require some state tracking, probabilistic effects, or conditional logic that's already partially supported. These should be relatively straightforward to implement.

#### 1. **Shortcut** - $7 (Uncommon)
**Effect:** Allows Straights to be made with gaps of 1 rank
**Complexity:** Requires modifying the hand detection logic in `hand.rs` to allow rank gaps of 1 in straights.
**Systems needed:** Hand detection modification

#### 2. **Driver's License** - $7 (Rare)
**Effect:** X3 Mult if full deck has at least 16 Enhanced cards
**Complexity:** Needs to count enhanced cards in deck (enhancement system partially exists).
**Systems needed:** Enhanced card tracking

#### 3. **Matador** - $7 (Uncommon)
**Effect:** Earn $8 if played hand triggers Boss Blind ability
**Complexity:** Needs to detect Boss Blind trigger events.
**Systems needed:** Boss blind event detection

#### 4. **Mr. Bones** - $5 (Uncommon)
**Effect:** Prevents death if chips scored >= 25% of required chips; self-destructs
**Complexity:** Requires death prevention hook and self-destruction.
**Systems needed:** Death prevention system, self-destruct on use

#### 5. **Troubadour** - $6 (Uncommon)
**Effect:** +2 hand size; -1 hand per round
**Complexity:** Dynamic stat modification that changes over time.
**Systems needed:** Per-round stat decay

#### 6. **Turtle Bean** - $5 (Uncommon)
**Effect:** Gains +5 hand size; decreases by 1 per round
**Complexity:** Similar to Troubadour but with different values.
**Systems needed:** Per-round stat decay

#### 7. **Vagabond** - $8 (Rare)
**Effect:** Create Tarot card if hand played with $4 or less
**Complexity:** Checks money condition and creates consumable.
**Systems needed:** Tarot card creation, money check

#### 8. **Cartomancer** - $6 (Uncommon)
**Effect:** Create Tarot card when Blind selected; requires empty consumable slot
**Complexity:** OnBlindSelect trigger + consumable creation.
**Systems needed:** Tarot card creation, consumable slot check

#### 9. **Séance** - $6 (Uncommon)
**Effect:** If poker hand is Straight Flush, create random Planet card
**Complexity:** Check specific hand type and create consumable.
**Systems needed:** Planet card creation

#### 10. **Astronomer** - $8 (Uncommon)
**Effect:** All Planet cards and Celestial Packs in shop are free
**Complexity:** Modifies shop pricing logic for specific item types.
**Systems needed:** Shop price override system

#### 11. **Certificate** - $6 (Uncommon)
**Effect:** When round begins, add random playing card with random seal to hand
**Complexity:** Requires seal system implementation + card generation.
**Systems needed:** Seal system, OnRoundBegin trigger

#### 12. **Trading Card** - $5 (Uncommon)
**Effect:** If first discard contains 1 card, destroy it and earn $3
**Complexity:** Track discard number, check count, destroy card, earn money.
**Systems needed:** First discard tracking, card destruction

---

### COMPLEX (17 jokers)
These jokers require new systems, advanced state management, or significant modifications to game architecture.

#### 1. **Hack** - $6 (Uncommon)
**Effect:** Retrigger each played 2, 3, 4, or 5
**Complexity:** Requires retrigger system for specific card values.
**Systems needed:** **Retrigger system** (major feature)

#### 2. **Dusk** - $5 (Uncommon)
**Effect:** Retrigger all played cards in final hand of round
**Complexity:** Needs retrigger system + final hand detection.
**Systems needed:** **Retrigger system**, final hand tracking

#### 3. **Sock and Buskin** - $6 (Uncommon)
**Effect:** Retrigger all played face cards
**Complexity:** Retrigger system for face cards specifically.
**Systems needed:** **Retrigger system**

#### 4. **Seltzer** - $6 (Uncommon)
**Effect:** Retrigger all played cards for next 10 hands
**Complexity:** Retrigger system + multi-hand duration tracking.
**Systems needed:** **Retrigger system**, duration counter

#### 5. **Brainstorm** - $10 (Rare)
**Effect:** Copies ability of leftmost Joker
**Complexity:** Dynamic effect copying from other jokers (Blueprint is implemented, so this could follow similar pattern).
**Systems needed:** **Effect copying system** (Blueprint exists as reference)

#### 6. **Ceremonial Dagger** - $6 (Uncommon)
**Effect:** When Blind selected, destroys Joker to the right; adds double sell value to Mult
**Complexity:** Joker destruction + value extraction + permanent stat gain.
**Systems needed:** Joker destruction, permanent stat modification

#### 7. **DNA** - $8 (Rare)
**Effect:** If first hand of round has only 1 card, add permanent copy to deck and draw it to hand
**Complexity:** First hand detection, card duplication, deck modification, drawing.
**Systems needed:** Deck modification, card duplication, draw system

#### 8. **Midas Mask** - $7 (Uncommon)
**Effect:** All face cards become Gold cards when scored
**Complexity:** Requires Gold card enhancement system.
**Systems needed:** **Gold card enhancement** (foil/enhancement system)

#### 9. **Vampire** - $7 (Uncommon)
**Effect:** Gains X0.2 Mult per Enhanced card played; removes enhancement
**Complexity:** Enhancement removal + permanent stat growth.
**Systems needed:** Enhancement removal, permanent joker stat modification

#### 10. **Burnt Joker** - $6 (Rare)
**Effect:** Upgrade level of first discarded poker hand each round
**Complexity:** Track first discard per round, upgrade hand levels.
**Systems needed:** Hand level upgrade system, first discard tracking

#### 11. **Invisible Joker** - $10 (Rare)
**Effect:** After 2 rounds, sell this to duplicate random Joker
**Complexity:** Round counter, self-sell trigger, joker duplication.
**Systems needed:** Joker duplication system, round counter on joker

#### 12. **Madness** - $7 (Uncommon)
**Effect:** When Small or Big Blind selected, destroy random Joker and create 2 free Jokers
**Complexity:** Random joker destruction + free joker creation.
**Systems needed:** Random joker selection, joker destruction, free joker creation

#### 13. **Luchador** - $6 (Uncommon)
**Effect:** Sell this to disable current Boss Blind
**Complexity:** Special sell effect + Boss Blind disabling.
**Systems needed:** Boss blind disable system, sell trigger

#### 14. **Diet Cola** - $6 (Uncommon)
**Effect:** Sell this to create free Double Tag
**Complexity:** Requires Tag system implementation.
**Systems needed:** **Tag system** (not implemented)

#### 15. **Perkeo** - $0 (Legendary)
**Effect:** Creates Negative copy of 1 random consumable at end of shop
**Complexity:** Negative edition system + consumable duplication.
**Systems needed:** **Negative edition system**, consumable duplication

#### 16. **To the Moon** - $5 (Uncommon)
**Effect:** Earn $1 per $5 in excess of $20; excess lowers by $5 after round
**Complexity:** Complex money tracking with threshold and decay.
**Systems needed:** Persistent money threshold tracking

---

## SIMPLE Jokers for Quick Implementation (0 found)

Unfortunately, all remaining unimplemented jokers require at least medium complexity features. The 120 already implemented jokers covered all the "simple" cases (basic chips/mult bonuses with straightforward conditions).

---

## Recommended Implementation Order

### Phase 1: Medium Complexity (Can implement now with existing systems)
1. **Shortcut** - Modify hand.rs to allow gap-1 straights
2. **Troubadour** / **Turtle Bean** - Add per-round stat decay
3. **Matador** - Add Boss Blind trigger detection
4. **Mr. Bones** - Add death prevention hook
5. **Trading Card** - Track first discard per round

### Phase 2: Consumable Creators (Need Tarot/Planet card system)
6. **Vagabond** - Create Tarot when low on money
7. **Cartomancer** - Create Tarot on blind select
8. **Séance** - Create Planet on Straight Flush

### Phase 3: Enhancement System Required
9. **Driver's License** - Count enhanced cards
10. **Certificate** - Requires seal system
11. **Midas Mask** - Requires Gold card enhancement
12. **Vampire** - Remove enhancements

### Phase 4: Major New Systems
13. **Hack** / **Dusk** / **Sock and Buskin** / **Seltzer** - Requires **RETRIGGER SYSTEM**
14. **Brainstorm** - Requires **EFFECT COPYING SYSTEM** (can reference Blueprint implementation)
15. **DNA** / **Ceremonial Dagger** / **Burnt Joker** - Requires **DECK MODIFICATION SYSTEM**
16. **Diet Cola** / **Perkeo** - Requires **TAG SYSTEM** and **NEGATIVE EDITION**

Note: Blueprint is already implemented, so Brainstorm can follow the same pattern.

---

## Key Missing Systems

Based on this analysis, the following major systems are blocking multiple jokers:

1. **Retrigger System** (blocks 4 jokers: Hack, Dusk, Sock and Buskin, Seltzer)
2. **Effect Copying System** (blocks 1 joker: Brainstorm) - Blueprint already implemented
3. **Enhancement/Foil System** (blocks 4 jokers: Midas Mask, Vampire, Driver's License, Certificate)
4. **Tag System** (blocks 1 joker: Diet Cola)
5. **Negative Edition** (blocks 1 joker: Perkeo)
6. **Deck Modification** (blocks 3 jokers: DNA, Ceremonial Dagger, Burnt Joker)

---

## Notes on "Simple" Categorization

The reason no jokers fell into the "Simple" category is because the codebase has already implemented all straightforward jokers:
- Basic stat bonuses (Joker, Half Joker, etc.) ✅
- Suit-based bonuses (Greedy, Lusty, Wrathful, Gluttonous) ✅
- Hand-type conditionals (Jolly, Zany, Mad, Crazy, Droll, etc.) ✅
- Card counting in hand/deck (Blue Joker, Square Joker, etc.) ✅
- State tracking (Green Joker, Ice Cream, etc.) ✅

All remaining jokers involve either:
- Creating consumables (Tarot/Planet cards)
- Modifying cards permanently (enhancements, foils)
- Advanced mechanics (retriggers, copying effects)
- Multi-step conditional logic
- New systems (tags, seals, negative edition)

This is actually a good sign - it means the implementation has already covered the foundational jokers and is now at the point of needing more complex game systems.
