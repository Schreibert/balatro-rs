# Implementation Plan for Missing Features

This document outlines the phased implementation plan for adding missing Balatro features to the game engine, prioritized by dependency and impact on reinforcement learning.

---

## Current Implementation Status

### Already Implemented ✓
- Core poker hand identification and scoring (all 13 hand types including secret hands)
- Card structure with `Enhancement`, `Edition`, and `Seal` enums defined
- Stage progression system (PreBlind → Blind → PostBlind → Shop)
- Action generation for plays, discards, card selection, and movement
- Basic shop with joker purchasing
- Joker effect system with callback registry
- Ante progression (1-8)
- Money, interest, and reward calculations

### Partially Implemented ⚠️
- Enhancements/Editions/Seals defined but not integrated into scoring
- Shop only generates jokers, not consumables

---

## Phase 1: Card Enhancement System (Foundation) ✅ COMPLETE
**Priority:** HIGH - Required for most other features
**Complexity:** Medium
**RL Impact:** Moderate - Enriches state space
**Status:** 90% Complete (Wild/Lucky/Stone/Gold deferred to Phase 1.1)
**Completion Date:** 2025-10-01

### 1.1 Integrate Existing Card Modifiers into Scoring ✅

**Files modified:**
- `core/src/card.rs` (+175 lines)
- `core/src/game.rs` (+95 lines)
- `core/src/deck.rs` (+6 lines)

**Completed Tasks:**

1. **Enhancement Scoring** ✅ - Extended `Card` struct methods:
   - ✅ Modified `Card::chips()` to include enhancement bonuses
   - ✅ Added `Card::mult()` method for enhancement multipliers
   - ✅ Added `Card::mult_multiplier()` for Glass/Steel/Polychrome
   - ✅ Added `Card::should_destroy()` for Glass destruction
   - ✅ Added `Card::seal_money_on_play()` for Gold seal
   - ✅ Added `Card::has_retrigger()` for Red seal
   - Implemented enhancements:
     - ✅ **Bonus**: +30 chips
     - ✅ **Mult**: +4 mult
     - ⚠️ **Wild**: Acts as any suit (deferred to Phase 1.1)
     - ✅ **Glass**: ×2 mult, 1/4 chance to destroy after scoring
     - ✅ **Steel**: ×1.5 mult while in hand
     - ⚠️ **Stone**: +50 chips implemented, rank handling deferred
     - ⚠️ **Gold**: +$3 on play implemented, end-of-round deferred
     - ⚠️ **Lucky**: Deferred to Phase 1.1

2. **Edition Effects** ✅ - Integrated into `Game::calc_score()`:
   - ✅ **Foil**: +50 chips
   - ✅ **Holographic**: +10 mult
   - ✅ **Polychrome**: ×1.5 mult
   - ✅ **Negative**: Extra joker slot framework added

3. **Seal Triggers** ✅ - Added seal activation logic:
   - ✅ **Red Seal**: Retrigger card (score it twice)
   - ⏸️ **Blue Seal**: Create Planet card when played (blocked by Phase 2)
   - ⏸️ **Purple Seal**: Create Tarot card when discarded (blocked by Phase 2)
   - ✅ **Gold Seal**: +$3 when played

4. **Card Destruction** ✅ - For Glass enhancement:
   - ✅ Added `destroyed: Vec<Card>` to `Game` state
   - ✅ Implemented `destroy_card()` method
   - ✅ Added `Deck::remove_card()` for permanent removal

**Testing:** ✅ Complete
- ✅ 11 new unit tests for enhancements/editions/seals
- ✅ All 71 tests passing
- ✅ Verified seal triggers fire correctly
- ✅ Tested enhancement/edition combinations

**See:** `PHASE_1_COMPLETION.md` for detailed report

---

## Phase 1.1: Deferred Card Features (Optional)
**Priority:** LOW - Polish for Phase 1
**Complexity:** Medium
**RL Impact:** Low - Minor gameplay completeness
**Estimate:** 6-10 hours

### Tasks:

1. **Wild Card Suit Matching** (3-4 hours)
   - Modify hand detection in `core/src/hand.rs`
   - Update: `is_flush()`, `is_straight_flush()`, `is_flush_house()`, `is_flush_five()`
   - Wild cards can count as any suit for flush detection

2. **Lucky Enhancement** (1-2 hours)
   - Implement probability effects in `Card` methods
   - 1/5 chance for ×1.5 mult when scored
   - 1/15 chance for +$20 when scored
   - Consider seeded RNG for RL determinism

3. **Stone Card Rank Handling** (2-3 hours)
   - Modify hand detection to exclude Stone cards from rank matching
   - Stone cards contribute chips but don't count for pairs/straights/etc.
   - Update: `is_pair()`, `is_three_of_kind()`, `is_straight()`, etc.

4. **Gold Card End-of-Round Money** (1 hour)
   - Add end-of-round hook to check held Gold cards
   - Award +$3 per Gold card in hand
   - Integrate with existing round transition logic

**Note:** Can be implemented any time before full release. Not blocking Phase 2.

---

## Phase 2: Consumables Infrastructure (Core System)
**Priority:** HIGH - Enables Tarots, Planets, Spectrals
**Complexity:** High
**RL Impact:** High - Significantly expands action space

### 2.1 Create Consumable Module

**New files to create:**
- `core/src/consumable.rs` - Base consumable trait and types
- `core/src/tarot.rs` - Tarot card implementations
- `core/src/planet.rs` - Planet card implementations
- `core/src/spectral.rs` - Spectral card implementations

**Tasks:**

1. **Consumable Trait** - Similar to `Joker` trait:
   ```rust
   pub trait Consumable {
       fn name(&self) -> String;
       fn desc(&self) -> String;
       fn effect(&self, game: &mut Game, targets: Option<Vec<Card>>) -> Result<(), GameError>;
       fn requires_target(&self) -> bool;
       fn max_targets(&self) -> usize;
   }
   ```

2. **Consumable Enums** - Using macro pattern:
   - Create `make_consumables!` macro similar to `make_jokers!`
   - Define `Tarots`, `Planets`, `Spectrals` enums
   - Each wraps individual consumable structs

3. **Game State Updates**:
   - Add `consumables: Vec<Consumables>` to `Game` struct
   - Add `consumable_slots: usize` to `Config` (default: 2)
   - Add `last_consumable_used: Option<Consumables>` for The Fool tarot
   - Add `consumable_target_selection: Option<(Consumables, Vec<Card>)>` for multi-step actions

### 2.2 Update Action System

**Files to modify:**
- `core/src/action.rs`
- `core/src/generator.rs`
- `core/src/space.rs`

**Tasks:**

1. **New Action Types**:
   ```rust
   pub enum Action {
       // ... existing actions
       UseConsumable(Consumables),
       SelectCardsForConsumable(Vec<Card>),
       BuyConsumable(Consumables),
       SellConsumable(Consumables),
   }
   ```

2. **Action Space Expansion**:
   - Current vector size: 79
   - Target size after Phase 2: ~150
   - Add indices for:
     - Use consumable (per consumable slot): +2-4 indices
     - Select cards for consumable: +24 indices (one per available card)
     - Buy consumable (shop): +4 indices (typical shop size)

3. **Action Generation**:
   - `gen_actions_use_consumable()` - When consumable can be used
   - `gen_actions_select_for_consumable()` - When consumable needs targets
   - `gen_actions_buy_consumable()` - During shop stage

**Testing:**
- Verify action space remains consistent
- Test consumable purchasing and usage flow
- Validate target selection for targeted consumables

---

## Phase 3A: Tarot Cards (22 Cards)
**Priority:** MEDIUM - High strategic value
**Complexity:** High
**RL Impact:** Very High - Complex strategic decisions

### 3.1 Implement Tarot Effects

**File:** `core/src/tarot.rs`

**Tarot Categories:**

1. **Enhancement Tarots** (6 cards):
   - The Magician (I): 2 cards → Lucky
   - The Empress (III): 2 cards → Mult
   - The Hierophant (V): 2 cards → Bonus
   - The Lovers (VI): 1 card → Wild
   - The Chariot (VII): 1 card → Steel
   - Justice (VIII): 1 card → Glass

2. **Advanced Enhancement Tarots** (2 cards):
   - The Devil (XV): 1 card → Gold
   - The Tower (XVI): 1 card → Stone

3. **Suit Conversion Tarots** (4 cards):
   - The Star (XVII): Up to 3 cards → Diamonds
   - The Moon (XVIII): Up to 3 cards → Clubs
   - The Sun (XIX): Up to 3 cards → Hearts
   - The World (XXI): Up to 3 cards → Spades

4. **Rank Modification Tarots** (2 cards):
   - Strength (XI): Up to 2 cards, raise rank by 1
   - Death (XIII): Convert left card into right card (2 targets)

5. **Generation Tarots** (4 cards):
   - The Fool (0): Copy last Tarot/Planet used
   - The High Priestess (II): Create up to 2 Planet cards
   - The Emperor (IV): Create up to 2 Tarot cards
   - Judgement (XX): Create random Joker

6. **Utility Tarots** (4 cards):
   - The Hermit (IX): Double money (max $20)
   - The Wheel of Fortune (X): 1/4 chance to add edition to random Joker
   - Temperance (XIV): Gain sell value of all Jokers (max $50)
   - The Hanged Man (XII): Destroy up to 2 cards

**Implementation Requirements:**

- Card selection API for targeted tarots
- Deck modification methods:
  - `Deck::add_card(card: Card)`
  - `Deck::remove_card(card: Card)`
  - `Deck::transform_card(from: Card, to: Card)`
- Random generation methods:
  - `Game::generate_planet() -> Planet`
  - `Game::generate_tarot() -> Tarot`
  - `Game::generate_joker(rarity: Option<Rarity>) -> Joker`
- Money manipulation with caps
- Track `last_consumable_used` for The Fool

### 3.2 Shop Integration

**Files to modify:**
- `core/src/shop.rs`

**Tasks:**

1. **Shop State**:
   - Add `consumables: Vec<Consumables>` to `Shop`
   - Add `packs: Vec<Pack>` for booster packs

2. **Consumable Generation**:
   ```rust
   pub struct ConsumableGenerator {
       available_tarots: HashSet<Tarots>,
       available_planets: HashSet<Planets>,
       available_spectrals: HashSet<Spectrals>,
   }
   ```

3. **Spawn Constraints**:
   - Track held consumables
   - Prevent duplicates in shop if already held (unless Showman joker active)
   - When all 22 tarots held, default to Strength

4. **Booster Packs**:
   - Arcana Pack: 2 Tarots ($4)
   - Celestial Pack: 2 Planets ($4)
   - Spectral Pack: 2 Spectrals ($4)

**Testing:**
- Verify all 22 tarots work correctly
- Test spawn constraints
- Validate pack opening mechanics

---

## Phase 3B: Planet Cards (12 Cards)
**Priority:** MEDIUM - Permanent upgrades
**Complexity:** Medium
**RL Impact:** High - Permanent strategic investments

### 3.1 Implement Hand Leveling System

**Files to modify:**
- `core/src/rank.rs`
- `core/src/game.rs`

**Tasks:**

1. **Dynamic Hand Levels**:
   - Change `HandRank::level()` from returning constant to looking up from state
   - Add `hand_levels: HashMap<HandRank, Level>` to `Game` struct
   - Initialize with default Level 1 values
   - Add `upgrade_hand(rank: HandRank)` method

2. **Planet Upgrade Formula**:
   - Level 1→2: +30 chips, +3 mult
   - Level 2→3: +25 chips, +2 mult
   - Level 3+: +20 chips, +2 mult (continues scaling)

### 3.2 Implement 12 Planet Cards

**File:** `core/src/planet.rs`

**Planet Mapping:**

| Planet | Hand Type | Discovery Requirement |
|--------|-----------|----------------------|
| Pluto | High Card | Always available |
| Eris* | Pair | Play Pair once |
| Ceres* | Two Pair | Play Two Pair once |
| (unnamed) | Three of a Kind | Always available |
| Mercury | Straight | Always available |
| Venus | Flush | Always available |
| Earth | Full House | Always available |
| Mars | Four of a Kind | Always available |
| Jupiter* | Five of a Kind | Play Five of a Kind once |
| Saturn | Straight Flush | Always available |
| Uranus* | Flush House | Play Flush House once |
| Neptune | Royal Flush | Always available |

*Secret planets - must be discovered by playing the hand type first

**Implementation:**
- Track `discovered_planets: HashSet<Planets>` in `Game`
- Planet cards upgrade corresponding hand's chips and mult
- Can be used infinitely (not consumed on use in actual Balatro)

### 3.3 Shop Integration

**Tasks:**
- Add planets to consumable shop rotation
- Implement Celestial Pack (2 random planets)
- Only show discovered planets in shop/packs

**Testing:**
- Verify hand levels upgrade correctly
- Test discovery mechanics for secret planets
- Validate persistent level increases

---

## Phase 3C: Spectral Cards (18 Cards)
**Priority:** LOW-MEDIUM - High risk/reward
**Complexity:** High
**RL Impact:** Medium - Risky, situational

### 3.1 Implement Spectral Effects

**File:** `core/src/spectral.rs`

**Spectral Categories:**

1. **Deck Enhancement** (3 cards):
   - Familiar: Destroy 1, add 3 random enhanced face cards
   - Grim: Destroy 1, add 2 random enhanced Aces
   - Incantation: Destroy 1, add 4 random enhanced number cards

2. **Seal Addition** (4 cards):
   - Talisman: Add Gold Seal to 1 card
   - Deja Vu: Add Red Seal to 1 card
   - Trance: Add Blue Seal to 1 card
   - Medium: Add Purple Seal to 1 card

3. **Edition Addition** (1 card):
   - Aura: Add random edition (Foil/Holo/Poly) to 1 card

4. **Deck Transformation** (4 cards):
   - Sigil: Convert all cards to same random suit
   - Ouija: Convert all cards to same rank, -1 hand size
   - Immolate: Destroy 5 random cards, gain $20
   - Cryptid: Create 2 copies of 1 card

5. **Joker Manipulation** (5 cards):
   - Wraith: Create Rare Joker, set money to $0
   - Ankh: Copy 1 Joker, destroy all others
   - Hex: Add Polychrome to 1 Joker, destroy all others
   - Ectoplasm: Add Negative to random Joker, -1 hand size
   - The Soul: Create Legendary Joker

6. **Global Effect** (1 card):
   - Black Hole: Upgrade every poker hand by 1 level

**Implementation Challenges:**

1. **Persistent Hand Size Changes**:
   - Ouija and Ectoplasm permanently reduce hand size
   - Add `hand_size_modifiers: i32` to `Config` or `Game`
   - Apply when dealing cards

2. **Joker Destruction**:
   - Ankh and Hex destroy jokers
   - Need to unregister effects from `EffectRegistry`
   - Clear joker slots properly

3. **Rarity-Specific Generation**:
   - Some spectrals create Rare/Legendary jokers
   - Extend `JokerGenerator` to support rarity constraints

**Testing:**
- Test each spectral effect individually
- Verify hand size reductions persist
- Test joker destruction and effect cleanup

---

## Phase 4: Boss Blind Modifiers
**Priority:** MEDIUM - Increases difficulty
**Complexity:** Medium
**RL Impact:** Very High - Dramatically changes strategy

### 4.1 Implement Boss Modifiers

**New file:** `core/src/boss_modifier.rs`

**Files to modify:**
- `core/src/stage.rs`
- `core/src/game.rs`
- `core/src/generator.rs`

**Boss Types (Examples):**

1. **The Hook**: Discard 2 random cards each hand played
2. **The Ox**: Leftmost scoring card is played face down (no rank/suit)
3. **The House**: First hand drawn with only 1 card
4. **The Wall**: Extra large blind (×2.5 instead of ×2)
5. **The Wheel**: 1/7 chance card is face down
6. **The Arm**: Decrease hand level by 1 for each play
7. **The Club**: All Clubs are debuffed
8. **The Goad**: All Spades are debuffed
9. **The Water**: Start with 0 discards
10. **The Window**: All Diamonds are debuffed
11. **The Manacle**: -1 hand size
12. **The Eye**: No repeated hand types allowed
13. **The Mouth**: Play only 1 hand type this blind
14. **The Plant**: All face cards are debuffed
15. **The Serpent**: Always lose first hand of blind
16. **The Pillar**: Cards played are selected randomly
17. **The Needle**: Play only 1 hand
18. **The Head**: All Hearts are debuffed
19. **The Tooth**: Lose $1 per card played
20. **The Flint**: Halve chips and mult

**Implementation:**

1. **Boss Modifier Enum**:
   ```rust
   pub enum BossModifier {
       TheHook,
       TheOx,
       TheHouse,
       // ... etc
   }
   ```

2. **Modify Stage Enum**:
   ```rust
   pub enum Stage {
       // ...
       Blind(Blind, Option<BossModifier>), // Add modifier to blind
   }
   ```

3. **Apply Modifiers**:
   - In `Game::select_blind()` - Assign random modifier to Boss
   - In `Game::gen_actions()` - Restrict actions based on modifier
   - In `Game::calc_score()` - Apply scoring penalties
   - In `Game::deal()` - Apply hand size changes

**Testing:**
- Test each boss modifier individually
- Verify restrictions are enforced
- Test combinations with jokers

---

## Phase 5: Tags & Skip Blind
**Priority:** LOW - Optional strategic choice
**Complexity:** Low-Medium
**RL Impact:** Medium - Strategic tradeoffs

### 5.1 Implement Tag System

**New file:** `core/src/tag.rs`

**Tag Types:**

1. **Uncommon Tag**: Free uncommon joker
2. **Rare Tag**: Free rare joker
3. **Negative Tag**: Next base edition joker is Negative
4. **Foil Tag**: Next base edition joker is Foil
5. **Holographic Tag**: Next base edition joker is Holographic
6. **Polychrome Tag**: Next base edition joker is Polychrome
7. **Investment Tag**: After cashing out, earn +$5
8. **Voucher Tag**: Next shop has a voucher
9. **Boss Tag**: Reroll boss blind
10. **Standard Tag**: Create free Mega Standard Pack
11. **Charm Tag**: Create free Mega Arcana Pack
12. **Meteor Tag**: Create free Mega Celestial Pack
13. **Buffoon Tag**: Create free Mega Buffoon Pack
14. **Handy Tag**: Gain $1 per hand played in blind
15. **Garbage Tag**: Gain $1 per discard used in blind
16. **Ethereal Tag**: Create free Spectral Pack
17. **Coupon Tag**: Next booster pack is free
18. **Double Tag**: Next X2 tag grants X4
19. **Juggle Tag**: +1 hand size for next blind
20. **D6 Tag**: Start next shop with free reroll
21. **Top-up Tag**: Create up to 5 common jokers
22. **Speed Tag**: Generate X3 tag when skipping this blind
23. **Orbital Tag**: Upgrade poker hand by 3 levels
24. **Economy Tag**: Double your money (max $40)

**Implementation:**

1. **Tag State**:
   - Add `tags: Vec<Tag>` to `Game` struct
   - Tags are consumed when their effect triggers
   - Some tags apply immediately, others on next event

2. **Tag Application**:
   - Modify relevant game methods to check for tags
   - Apply tag effects and remove from list
   - Track tag triggers for proper timing

### 5.2 Add Skip Blind Action

**Files to modify:**
- `core/src/action.rs`
- `core/src/generator.rs`
- `core/src/game.rs`

**Tasks:**

1. **Skip Blind Action**:
   - Uncomment `SkipBlind(Blind)` in `Action` enum
   - Generate during PreBlind stage (for Small/Big only, not Boss)
   - Award random tag instead of playing blind

2. **Stage Progression**:
   - If skipped Small → go to Big
   - If skipped Big → go to Boss
   - No rewards for skipped blinds

**Testing:**
- Verify tags trigger correctly
- Test skip blind progression
- Validate tag consumption

---

## Phase 6: Vouchers
**Priority:** LOW - Permanent upgrades
**Complexity:** Medium
**RL Impact:** Low - Quality of life improvements

### 6.1 Implement Voucher System

**New file:** `core/src/voucher.rs`

**Voucher Categories:**

1. **Shop Vouchers**:
   - Overstock: +1 card slot in shop
   - Clearance Sale: All shop cards are -$1
   - Reroll Surplus: Rerolls cost $2 less
   - Crystal Ball: +1 consumable slot

2. **Resource Vouchers**:
   - Hone: +1 hand per round
   - Glow Up: +1 discard per round
   - Reroll Glut: +1 shop reroll per round
   - Seed Money: +$2 cap on interest earned

3. **Planet Vouchers**:
   - Telescope: Planets are more common in shop
   - Observatory: Planets in consumable slots apply passively
   - Planet Merchant/Tycoon: +1 card slot in Celestial Pack

4. **Tarot Vouchers**:
   - Tarot Merchant/Tycoon: +1 card slot in Arcana Pack

5. **Joker Vouchers**:
   - Grabber: +1 hand size permanently
   - Nacho Tong: +1 joker slot permanently

**Implementation:**

1. **Voucher State**:
   - Add `vouchers: HashSet<Voucher>` to `Game` struct
   - Vouchers are permanent once purchased
   - Some have tiered upgrades (Tier 1 → Tier 2)

2. **Shop Integration**:
   - Add vouchers to shop (1 per shop visit)
   - Cost $10 base, varies by tier
   - Remove from shop after purchase

3. **Apply Effects**:
   - Modify relevant game logic based on owned vouchers
   - Some apply to shop generation
   - Some apply to gameplay (hands, discards, slots)

**Testing:**
- Verify voucher effects persist across rounds
- Test tier upgrades
- Validate shop modifications

---

## Phase 7: Alternative Decks & Stakes
**Priority:** LOW - Variety & difficulty scaling
**Complexity:** Low-Medium
**RL Impact:** High - Generalization across difficulties

### 7.1 Alternative Starting Decks

**Files to modify:**
- `core/src/deck.rs`
- `core/src/config.rs`

**Deck Types:**

1. **Red Deck**: +1 discard per round
2. **Blue Deck**: +1 hand per round
3. **Yellow Deck**: Start with extra $10
4. **Green Deck**: Earn $1 per remaining hand at end of round
5. **Black Deck**: +1 joker slot, -1 hand per round
6. **Magic Deck**: Start with 2 Crystal Ball vouchers and 1 Fool
7. **Nebula Deck**: Start with Meteor voucher, -1 consumable slot
8. **Ghost Deck**: Spectral cards may appear in shop, all cards have "Hex" sticker
9. **Abandoned Deck**: No face cards in deck
10. **Checkered Deck**: Only Spades and Hearts in deck
11. **Zodiac Deck**: Start with 3 Planet cards, Tarot Merchant voucher
12. **Painted Deck**: +1 hand size, -1 joker slot
13. **Anaglyph Deck**: After defeating each boss, gain Double Tag
14. **Plasma Deck**: Balance chips and mult on final hand
15. **Erratic Deck**: All rank and suit values randomized

**Implementation:**

1. **Deck Enum**:
   ```rust
   pub enum DeckType {
       Standard,
       Red,
       Blue,
       // ... etc
   }
   ```

2. **Apply Deck Modifiers**:
   - Modify `Game::new()` to accept deck type
   - Apply starting modifiers (money, cards, vouchers, etc.)
   - Apply ongoing effects (per-round bonuses)

### 7.2 Stakes (Difficulty Levels)

**Files to modify:**
- `core/src/config.rs`
- `core/src/game.rs`

**Stake Levels:**

1. **White Stake**: Base difficulty
2. **Red Stake**: Small Blind gives no reward money
3. **Green Stake**: Required score scales faster (×1.25)
4. **Black Stake**: Shop can contain Eternal (unsellable) stickers
5. **Blue Stake**: -1 discard per round
6. **Purple Stake**: Required score scales faster (×1.5)
7. **Orange Stake**: Shop can contain Perishable (expires in 5 rounds) stickers
8. **Gold Stake**: All previous stakes combined

**Implementation:**

1. **Stake Enum**:
   ```rust
   pub enum Stake {
       White,
       Red,
       Green,
       // ... etc
   }
   ```

2. **Apply Stake Modifiers**:
   - Modify scoring requirements in `Game::required_score()`
   - Reduce discards in `Config`
   - Add stickers to shop items
   - Combine effects for higher stakes

**Testing:**
- Test each deck type's unique mechanics
- Verify stake difficulty scaling
- Test combined stake effects

---

## Implementation Order Summary

### **Tier 1 (Foundation) - START HERE**
1. ✅ Phase 1: Card Enhancement Integration
2. Phase 2: Consumable Infrastructure

### **Tier 2 (High Value)**
3. Phase 3A: Tarot Cards
4. Phase 3B: Planet Cards
5. Phase 4: Boss Blind Modifiers

### **Tier 3 (Medium Value)**
6. Phase 3C: Spectral Cards
7. Phase 5: Tags & Skip Blind

### **Tier 4 (Polish)**
8. Phase 6: Vouchers
9. Phase 7: Alternative Decks & Stakes

---

## Impact on RL Training

### Action Space Growth
- **Current**: 79 discrete actions
- **After Phase 2**: ~150 actions
- **After Phase 3**: ~200+ actions (with consumable targeting)
- **Final**: ~250+ actions

### State Space Growth
- Card enhancements: 8 states per card
- Editions: 5 states per card
- Seals: 4 states per card
- Hand levels: 13×2 dimensions (chips/mult per hand)
- Consumables: 50+ held items
- Boss modifiers: 20+ unique constraints

### Strategic Depth
- Boss modifiers require adaptive strategies per encounter
- Consumables enable long-term planning and deck building
- Multiple build paths (joker-focused, hand-leveling, deck manipulation)
- Risk/reward decisions (skip blinds, spectral cards)

### Training Considerations
- Curriculum learning: Start with Phase 1, progressively add features
- Test generalization after each tier
- May need hierarchical RL for complex multi-step consumable usage
- Boss modifier diversity helps prevent overfitting

---

## Testing Strategy

Each phase should include:

1. **Unit Tests**: Test individual features in isolation
2. **Integration Tests**: Verify features work with existing systems
3. **Game Tests**: Full game runs with random actions (existing test pattern)
4. **RL Tests**: Verify agent can learn with new features enabled

**Test Coverage Goals:**
- 80%+ code coverage for new modules
- All consumable effects tested
- All boss modifiers tested
- Edge cases handled (empty deck, no money, etc.)

---

## Documentation Updates

After each phase:
- Update `CLAUDE.md` with new modules and architecture
- Update `README.md` feature checklist
- Update Python bindings documentation
- Add inline code documentation for complex logic
