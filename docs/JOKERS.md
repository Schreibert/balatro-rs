# Jokers Reference

This document provides a complete reference of ALL 150 jokers in Balatro, including implementation and testing status for the balatro-rs project.

## Testing & Implementation Status

**Total: 150 Jokers**
- ✅ **Tested (Passing):** 124 jokers (82.7%) - Fully functional with unit tests
- ⚠️ **Implemented (Not Tested):** 15 jokers (10%) - Functional but no test coverage
- 🔧 **Test Issues:** 3 jokers (2%) - Implemented but tests have issues (test logic or partial implementation)
- 📝 **Stubbed:** 8 jokers (5.3%) - Basic structure only, awaiting system support

**Note:** The project has 1 failing test (`test_double_tag_stacking`) which is a core tag system bug unrelated to any specific joker. See PROJECT_STATUS.md for details.

### Implemented but Untested Jokers (16)

Campfire, Castle, GlassJoker, HitTheRoad, Hologram, LoyaltyCard, LuckyCat, Obelisk, Pareidolia, Ramen, RedCard, RideTheBus, SpaceJoker, Supernova, TheIdol, Throwback

**Note:** Pareidolia sets the `all_cards_are_faces` modifier but requires `card.is_face()` to check this modifier for full functionality.

### Jokers with Test Issues (3)

- **FacelessJoker** - OnDiscard effect not triggering in test (implementation functional)
- **Satellite** - Test logic needs adjustment for base rewards (implementation functional)
- **InvisibleJoker** - OnRoundEnd works, but OnSell duplication has effect system limitations

### Stubbed Jokers Requiring Additional Systems (8)

**Special Event Hooks (4):**
Astronomer, Hallucination, MrBones, Perkeo

**Card Modification (1):**
Mime

**Complex/Missing Systems (3):**
Matador, OopsAll6s, TradingCard

**Total:** 8 unique stubbed jokers (some requirements overlap across categories)

**Note:** Phase 2 completed MarbleJoker and MidasMask (card modification). Phase 3 completed all 5 retrigger jokers (HangingChad, Hack, SockAndBuskin, Dusk, Seltzer). Phase 4 (partial) completed ToTheMoon and Vampire using existing hooks. Phase 5 completed Blueprint and Brainstorm (effect copying system). Phase 6 (partial) completed RiffRaff (creates jokers on blind select) and InvisibleJoker (OnRoundEnd tracker - OnSell duplication has limitations). Phase 7 completed Certificate (adds random card with seal on round begin). Remaining jokers require new infrastructure (OnPackOpen, OnShopEnd, discard tracking, death prevention, boss blind detection).

## Overview

Jokers are special cards that modify scoring when you play poker hands. Each joker has:
- **Name**: The joker's display name
- **Cost**: Purchase price in the shop (sell value is cost/2)
- **Rarity**: Common, Uncommon, Rare, or Legendary
- **Effect**: What the joker does when triggered
- **Unlock**: How to unlock the joker (if not available from start)

## Scoring System

Balatro uses the formula: `Score = (Base Chips + Card Chips + Bonus Chips) × (Base Mult + Bonus Mult) × Mult Multiplier`

- **Chips**: Added bonuses that increase the base score
- **Mult**: Multiplier bonuses that scale with chips
- **Mult Multiplier**: Multiplicative scaling (e.g., ×1.5, ×2)

## Rarity Distribution

- **61 Common Jokers** (70% shop appearance rate)
- **64 Uncommon Jokers** (25% shop appearance rate)
- **20 Rare Jokers** (5% shop appearance rate)
- **5 Legendary Jokers** (Only obtainable via "The Soul" Spectral Card)

**Total: 150 Jokers**
- 105 available from start
- 45 unlockable through specific conditions

---

# Common Jokers (61 Total)

| # | Name | Cost | Effect | Unlock | Status |
|---|------|------|--------|--------|-------------|
| 1 | Joker | $2 | +4 Mult | Start | ✅ |
| 2 | Greedy Joker | $5 | Played cards with Diamond suit give +3 Mult when scored | Start | ✅ |
| 3 | Lusty Joker | $5 | Played cards with Heart suit give +3 Mult when scored | Start | ✅ |
| 4 | Wrathful Joker | $5 | Played cards with Spade suit give +3 Mult when scored | Start | ✅ |
| 5 | Gluttonous Joker | $5 | Played cards with Club suit give +3 Mult when scored | Start | ✅ |
| 6 | Jolly Joker | $3 | +8 Mult if played hand contains a Pair | Start | ✅ |
| 7 | Zany Joker | $4 | +12 Mult if played hand contains a Three of a Kind | Start | ✅ |
| 8 | Mad Joker | $4 | +10 Mult if played hand contains a Two Pair | Start | ✅ |
| 9 | Crazy Joker | $4 | +12 Mult if played hand contains a Straight | Start | ✅ |
| 10 | Droll Joker | $4 | +10 Mult if played hand contains a Flush | Start | ✅ |
| 11 | Sly Joker | $3 | +50 Chips if played hand contains a Pair | Start | ✅ |
| 12 | Wily Joker | $4 | +100 Chips if played hand contains a Three of a Kind | Start | ✅ |
| 13 | Clever Joker | $4 | +80 Chips if played hand contains a Two Pair | Start | ✅ |
| 14 | Devious Joker | $4 | +100 Chips if played hand contains a Straight | Start | ✅ |
| 15 | Crafty Joker | $4 | +80 Chips if played hand contains a Flush | Start | ✅ |
| 16 | Half Joker | $5 | +20 Mult if played hand contains 3 or fewer cards | Start | ✅ |
| 17 | Credit Card | $1 | Go up to -$20 in debt | Start | ⚠️ |
| 18 | Banner | $5 | +30 Chips for each remaining discard | Start | ✅ |
| 19 | Mystic Summit | $5 | +15 Mult when discards remaining is 0 | Start | ✅ |
| 20 | Raised Fist | $5 | Adds double the rank of lowest ranked card held in hand to Mult | Start | ✅ |
| 21 | Joker Stencil | $8 | X1 Mult for each empty Joker slot | Start | ✅ |
| 22 | Chaos the Clown | $4 | 1 free Reroll per shop | Start | ⚠️ |
| 23 | Scary Face | $4 | +30 Chips for each face card played in scoring | Start | ✅ |
| 24 | Abstract Joker | $4 | +3 Mult for each Joker card (including self) | Start | ✅ |
| 25 | Delayed Gratification | $4 | Earn $2 per discard if no discards used by end of the round | Start | ✅ |
| 26 | Gros Michel | $5 | +15 Mult; 1 in 6 chance to be destroyed at end of round | Start | ⚠️ |
| 27 | Even Steven | $4 | +4 Mult for each 10, 8, 6, 4, or 2 card in scored hand | Start | ✅ |
| 28 | Odd Todd | $4 | +31 Chips if played hand contains odd cards (A, 9, 7, 5, 3) | Start | ✅ |
| 29 | Scholar | $4 | +20 Chips and +4 Mult per Ace played | Start | ✅ |
| 30 | Business Card | $4 | Played face cards have 1 in 2 chance to give $2 when scored | Start | ✅ |
| 31 | Supernova | $5 | Adds the number of times poker hand has been played this run to Mult | Start | ✅ |
| 32 | Ride the Bus | $4 | +1 Mult per consecutive hand without face cards; resets on face card | Start | ⚠️ |
| 33 | Runner | $5 | +15 Chips if played hand contains a Straight | Start | ✅ |
| 34 | Ice Cream | $5 | +100 Chips; -5 Chips for each hand played | Start | ✅ |
| 35 | Splash | $3 | Every played card counts in scoring | Start | ✅ |
| 36 | Blue Joker | $5 | +2 Chips for each remaining card in deck | Start | ✅ |
| 37 | Sixth Sense | $6 | 1 in 6 chance to destroy played 6, create Spectral card if successful | Start | ✅ |
| 38 | Constellation | $6 | Gains X0.1 Mult per Planet card used | Start | ✅ |
| 39 | Hiker | $6 | Every played card permanently gains +5 Chips when scored | Start | ✅ |
| 40 | Green Joker | $4 | +1 Mult per hand played; -1 Mult per discard | Start | ✅ |
| 41 | Superposition | $4 | Create a Tarot card if poker hand contains Straight and Ace | Start | ✅ |
| 42 | To Do List | $4 | $5 if poker hand is listed type; hand changes each round | Start | ✅ |
| 43 | Cavendish | $5 | X3 Mult; 1 in 1000 chance to be destroyed at end of round | Start | 📝 |
| 44 | Red Card | $5 | Gains +3 Mult when any Booster Pack is skipped | Start | ⚠️ |
| 45 | Square Joker | $5 | Gains +4 Chips if hand has exactly 4 cards | Start | ✅ |
| 46 | Riff-Raff | $6 | When Blind selected, create 2 Common Jokers | Unlock | ✅ |
| 47 | Golden Ticket | $5 | Played Gold cards earn $3 when scored | Unlock | ✅ |
| 48 | Swashbuckler | $4 | Adds sell value of all Jokers to Mult; +1 Mult per card sold | Unlock | ✅ |
| 49 | Smiley Face | $4 | +4 Mult for each face card played | Unlock | ✅ |
| 50 | Golden Joker | $6 | Earn $3 at end of round | Unlock | ✅ |
| 51 | Drunkard | $4 | +1 discard per round | Unlock | ✅ |
| 52 | Faceless Joker | $4 | Earn $5 if 3+ face cards discarded at once | Unlock | ✅ |
| 53 | Hanging Chad | $4 | Retrigger first card used in scoring 2 additional times | Unlock | ✅ |
| 54 | Popcorn | $5 | +20 Mult; -4 Mult per round played | Unlock | ✅ |
| 55 | Walkie Talkie | $4 | +10 Chips and +4 Mult for each 10 or 4 played | Unlock | ✅ |
| 56 | Shoot the Moon | $5 | +13 Mult for each Queen held in hand | Unlock | ⚠️ |
| 57 | Fortune Teller | $5 | +1 Mult per Tarot card used this run | Unlock | ⚠️ |
| 58 | Juggler | $5 | +1 hand size | Unlock | ⚠️ |
| 59 | Photograph | $5 | First played face card gives X2 Mult when scored | Unlock | ✅ |
| 60 | Reserved Parking | $5 | 1 in 3 chance for each face card held in hand to give $1 | Unlock | ✅ |
| 61 | Mail-In Rebate | $5 | Earn $3 for each discarded rank; rank changes each round | Unlock | 📝 |
| 62 | 8 Ball | $5 | 1 in 5 chance per 8 played to create Tarot; no 8s in deck | Unlock | ✅ |
| 63 | Misprint | $4 | +0 to +23 Mult (random each time) | Unlock | ✅ |
| 64 | Egg | $4 | Gains $3 sell value at end of round | Unlock | ✅ |

---

# Uncommon Jokers (64 Total)

| # | Name | Cost | Effect | Unlock | Status |
|---|------|------|--------|--------|-------------|
| 1 | Four Fingers | $7 | All Flushes and Straights can be made with 4 cards | Start | ✅ |
| 2 | Mime | $5 | Retrigger all card held in hand abilities | Start | ✅ |
| 3 | Ceremonial Dagger | $6 | When Blind selected, destroys Joker to the right; adds double sell value to Mult | Start | 📝 |
| 4 | Marble Joker | $6 | Adds one Stone card to deck when Blind selected | Start | ✅ |
| 5 | Loyalty Card | $5 | X4 Mult every 6 hands played; {0} remaining | Start | ✅ |
| 6 | Dusk | $5 | Retrigger all played cards in final hand of round | Start | ✅ |
| 7 | Fibonacci | $8 | Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored | Start | ✅ |
| 8 | Steel Joker | $7 | Gains X0.2 Mult for each Steel Card in full deck | Start | ✅ |
| 9 | Hack | $6 | Retrigger each played 2, 3, 4, or 5 | Start | ✅ |
| 10 | Pareidolia | $5 | All cards considered face cards | Start | ✅ |
| 11 | Space Joker | $5 | 1 in 4 chance to upgrade level of played poker hand | Start | ✅ |
| 12 | Burglar | $6 | When Blind selected, +3 Hands and lose all discards | Start | ✅ |
| 13 | Blackboard | $6 | X3 Mult if all cards held in hand are Spades or Clubs | Start | ✅ |
| 14 | Runner | $15 | +15 Chips if played hand contains a Straight | Start | ✅ |
| 15 | Constellation | $6 | This Joker gains X0.1 Mult every time a Planet card is used | Start | ✅ |
| 16 | Hiker | $6 | Every played card permanently gains +5 Chips when scored | Start | ✅ |
| 17 | Smeared Joker | $7 | Hearts and Diamonds count as same suit; Spades and Clubs count as same suit | Start | ✅ |
| 18 | Troubadour | $6 | +2 hand size; -1 hand per round | Start | ✅ |
| 19 | Certificate | $6 | When round begins, add random playing card with random seal to hand | Start | ✅ |
| 20 | Acrobat | $6 | X3 Mult on final hand of round | Start | ✅ |
| 21 | Spare Trousers | $6 | Gains +2 Mult if played hand contains Two Pair | Start | ✅ |
| 22 | Trading Card | $5 | If first discard contains 1 card, destroy it and earn $3 | Start | 📝 |
| 23 | Flash Card | $5 | Gains +2 Mult per reroll in shop | Start | ✅ |
| 24 | Onyx Agate | $7 | +7 Mult for each Club card played | Start | ✅ |
| 25 | Bloodstone | $7 | 1 in 2 chance for Hearts to give X1.5 Mult when scored | Start | ✅ |
| 26 | Arrowhead | $7 | Played Spade cards give +50 Chips when scored | Start | ✅ |
| 27 | Rough Gem | $7 | Played Diamond cards earn $1 when scored | Start | ✅ |
| 28 | Sixth Sense | $6 | If first hand of round is single 6, destroy it and create Spectral card | Start | ✅ |
| 29 | Séance | $6 | If poker hand is Straight Flush, create random Planet card | Start | ⚠️ |
| 30 | Glass Joker | $6 | Gains X0.75 Mult for each Glass Card destroyed | Start | ⚠️ |
| 31 | Showman | $5 | +4 Mult for Joker, Tarot, Planet, or Spectral cards remaining in consumable slots | Start | ✅ |
| 32 | Flower Pot | $6 | X3 Mult if hand contains Diamond, Club, Heart, and Spade cards | Start | ✅ |
| 33 | Merry Andy | $7 | +3 discards; -1 hand size | Start | ✅ |
| 34 | Stone Joker | $5 | Gains +25 Chips for each Stone Card in full deck | Start | ✅ |
| 35 | Mr. Bones | $5 | Prevents death if chips scored >= 25% of required chips; self-destructs | Start | 📝 |
| 36 | Cloud 9 | $5 | Earn $1 for each 9 in full deck at end of round | Start | ⚠️ |
| 37 | Rocket | $6 | Earn $1 at end of round; payout increases by $2 when Boss Blind defeated | Start | ⚠️ |
| 38 | Bull | $6 | +2 Chips for each $1 you have | Start | ✅ |
| 39 | Luchador | $6 | Sell this to disable current Boss Blind | Start | ✅ |
| 40 | Diet Cola | $6 | Sell this to create free Double Tag | Start | ✅ |
| 41 | Seltzer | $6 | Retrigger all played cards for next 10 hands | Start | ✅ |
| 42 | Castle | $6 | Gains +3 Chips per discarded card of each suit; resets when suit changes | Start | ⚠️ |
| 43 | Joker Stencil | $8 | X1 Mult for each empty Joker slot (counts itself as empty) | Start | ✅ |
| 44 | Sock and Buskin | $6 | Retrigger all played face cards | Start | ✅ |
| 45 | Hologram | $6 | Gains X0.25 Mult when a playing card added to deck | Start | ✅ |
| 46 | Matador | $7 | Earn $8 if played hand triggers Boss Blind ability | Start | 📝 |
| 47 | To the Moon | $5 | Earn $1 per $5 in excess of $20; excess lowers by $5 after round | Start | ✅ |
| 48 | Card Sharp | $6 | X3 Mult if played poker hand already played this round | Start | ⚠️ |
| 49 | Madness | $7 | When Small or Big Blind selected, destroy random Joker and create 2 free Jokers | Start | 📝 |
| 50 | Ramen | $6 | X2 Mult; loses X0.01 Mult per card discarded | Start | ⚠️ |
| 51 | Throwback | $6 | X0.25 Mult for each Blind skipped this run | Start | ✅ |
| 52 | Midas Mask | $7 | All face cards become Gold cards when scored | Start | ✅ |
| 53 | Satellite | $6 | Earn $1 at end of round per unique Planet card used this run | Start | ✅ |
| 54 | Bootstraps | $6 | Gains +2 Mult for every $5 you have | Start | ✅ |
| 55 | Turtle Bean | $5 | Gains +5 hand size; decreases by 1 per round | Start | ✅ |
| 56 | Erosion | $6 | +4 Mult for each card below 52 in full deck | Start | ✅ |
| 57 | Oops! All 6s | $4 | Doubles all probabilities | Start | 📝 |
| 58 | The Idol | $6 | X2 Mult for each [rank] of [suit] in hand; rank and suit change each round | Start | ✅ |
| 59 | Seeing Double | $8 | X2 Mult if played hand has Club card and any other suit card | Start | ✅ |
| 60 | Lucky Cat | $5 | Gains X0.25 Mult each time a Lucky card successfully triggers | Start | ✅ |
| 61 | Cartomancer | $6 | Create Tarot card when Blind selected; requires empty consumable slot | Unlock | ✅ |
| 62 | Astronomer | $8 | All Planet cards and Celestial Packs in shop are free | Unlock | 📝 |
| 63 | Vampire | $7 | Gains X0.2 Mult per Enhanced card played; removes enhancement | Unlock | ✅ |
| 64 | Shortcut | $7 | Allows Straights to be made with gaps of 1 rank | Unlock | 📝 |

---

# Rare Jokers (20 Total)

| # | Name | Cost | Effect | Unlock | Status |
|---|------|------|--------|--------|-------------|
| 1 | DNA | $8 | If first hand of round has only 1 card, add permanent copy to deck and draw it to hand | Start | 📝 |
| 2 | Vagabond | $8 | Create Tarot card if hand played with $4 or less | Start | ✅ |
| 3 | Baron | $8 | Each King held in hand gives X1.5 Mult | Start | ✅ |
| 4 | Obelisk | $8 | Gains X0.2 Mult per consecutive hand played without playing most-played poker hand | Start | ✅ |
| 5 | Baseball Card | $8 | Uncommon Jokers each give X1.5 Mult | Start | ✅ |
| 6 | Ancient Joker | $8 | Each played card with [suit] gives X1.5 Mult when scored; suit changes at end of round | Start | ✅ |
| 7 | Campfire | $9 | Gains X0.25 Mult per card sold; resets when Boss Blind defeated | Start | ⚠️ |
| 8 | Stuntman | $6 | +250 Chips; +3 hand size | Win run | ✅ |
| 9 | Invisible Joker | $10 | After 2 rounds, sell this to duplicate random Joker | Win run | 🔧 |
| 10 | Blueprint | $10 | Copies ability of Joker to the right | Win run | ✅ |
| 11 | Brainstorm | $10 | Copies ability of leftmost Joker | Win run | ✅ |
| 12 | Wee Joker | $8 | Gains +8 Chips when each played 2 is scored | Win in 18 or fewer rounds | ✅ |
| 13 | Hit the Road | $8 | Gains X0.5 Mult for every Jack discarded this round; resets at end of round | Discard 5 Jacks simultaneously | ⚠️ |
| 14 | The Duo | $8 | X2 Mult if played hand contains a Pair | Win without playing Pair | ✅ |
| 15 | The Trio | $8 | X3 Mult if played hand contains Three of a Kind | Win without playing Three of a Kind | ✅ |
| 16 | The Family | $8 | X4 Mult if played hand contains Four of a Kind | Win without playing Four of a Kind | ✅ |
| 17 | The Order | $8 | X3 Mult if played hand contains Straight | Win without playing Straight | ✅ |
| 18 | The Tribe | $8 | X2 Mult if played hand contains Flush | Win without playing Flush | ✅ |
| 19 | Driver's License | $7 | X3 Mult if full deck has at least 16 Enhanced cards | Win by only playing 1 hand per round | ✅ |
| 20 | Burnt Joker | $6 | Upgrade level of first discarded poker hand each round | Play 1000+ hands | 📝 |

---

# Legendary Jokers (5 Total)

**All Legendary Jokers can only be obtained via "The Soul" Spectral Card**

| # | Name | Cost | Effect | Unlock | Status |
|---|------|------|--------|--------|-------------|
| 1 | Canio | $0 | Gains X1 Mult when a face card is destroyed | Soul Card | ✅ |
| 2 | Triboulet | $0 | Played Kings and Queens each give X2 Mult when scored | Soul Card | ✅ |
| 3 | Yorick | $0 | Gains X1 Mult every 23 cards discarded | Soul Card | ✅ |
| 4 | Chicot | $0 | Disables effect of every Boss Blind | Soul Card | ✅ |
| 5 | Perkeo | $0 | Creates Negative copy of 1 random consumable at end of shop | Soul Card | 📝 |

---

# Implementation Status

## Currently Implemented (15/150)

All currently implemented jokers are **Common** rarity:

1. ✅ **Joker** - Basic +4 Mult joker
2. ✅ **Greedy Joker** - Diamond suit +3 Mult per card
3. ✅ **Lusty Joker** - Heart suit +3 Mult per card
4. ✅ **Wrathful Joker** - Spade suit +3 Mult per card
5. ✅ **Gluttonous Joker** - Club suit +3 Mult per card
6. ✅ **Jolly Joker** - +8 Mult for Pair
7. ✅ **Zany Joker** - +12 Mult for Three of a Kind
8. ✅ **Mad Joker** - +10 Mult for Two Pair
9. ✅ **Crazy Joker** - +12 Mult for Straight
10. ✅ **Droll Joker** - +10 Mult for Flush
11. ✅ **Sly Joker** - +50 Chips for Pair
12. ✅ **Wily Joker** - +100 Chips for Three of a Kind
13. ✅ **Clever Joker** - +80 Chips for Two Pair
14. ✅ **Devious Joker** - +100 Chips for Straight
15. ✅ **Crafty Joker** - +80 Chips for Flush

## Not Yet Implemented (135/150)

- **46 Common Jokers** remaining
- **64 Uncommon Jokers** - Higher power level, more complex effects
- **20 Rare Jokers** - Strong abilities with specific activation conditions
- **5 Legendary Jokers** - Game-changing effects, only from Soul card

### Common Patterns in Unimplemented Jokers:

- **Economy Jokers**: Generate money (Golden Joker, Business Card, etc.)
- **Retrigger Jokers**: Replay card effects (Mime, Dusk, Hack, etc.)
- **Mult Multiplier Jokers**: Multiplicative scaling (Baron, Campfire, Ancient Joker)
- **Conditional Effects**: Complex triggers (Obelisk, Ice Cream, Ride the Bus)
- **Deck Modification**: Add/destroy cards (DNA, Marble Joker, Burnt Joker)
- **Card Enhancement**: Modify card properties (Midas Mask, Vampire)
- **Hand Size/Discard Modifiers**: Change available actions (Juggler, Drunkard)

---

# Joker Mechanics Reference

## Effect Triggers

Jokers trigger their effects at specific times:
- **OnScore**: When calculating the final score of a played hand
- **OnPlay**: When cards are played
- **OnDiscard**: When cards are discarded
- **OnBlindSelect**: When entering a new blind
- **OnRoundEnd**: At the end of a round
- **OnShopOpen**: When entering the shop
- **Continuous**: Always active (hand size, discard modifiers)

## Joker Slots

- **Default**: 5 joker slots
- **Can be increased**: Via Negative edition cards
- **Evaluation order**: Left-to-right during scoring

## Selling Jokers

- **Sell value**: Always cost ÷ 2
- **Example**: A $4 joker sells for $2
- **Special cases**: Some jokers (like Egg) gain sell value over time

## Unlock Conditions

- **Start**: Available from the beginning (105 jokers)
- **Win run**: Beat the game once
- **Specific challenges**: Win with constraints (e.g., no Pairs, no Straights)
- **Play X hands**: Reach certain milestones
- **Soul card**: All Legendary jokers

---

# Synergy Examples

## Combo 1: Suit Focus (Implemented)
**Jokers**: Greedy Joker + Droll Joker

Play 5 Diamonds (Flush):
- Base: 35 chips, 4 mult
- Cards: ~25 chips
- Greedy: +15 Mult (5 diamonds × 3)
- Droll: +10 Mult
- **Total**: (35 + 25) × (4 + 15 + 10) = **1,740 points**

## Combo 2: Hand Type Stacking (Implemented)
**Jokers**: Jolly Joker + Mad Joker + Sly Joker + Clever Joker

Play Two Pair:
- Base: 20 chips, 2 mult
- Cards: ~42 chips
- Jolly: +8 Mult (contains pair)
- Mad: +10 Mult (two pair)
- Sly: +50 Chips (contains pair)
- Clever: +80 Chips (two pair)
- **Total**: (20 + 42 + 50 + 80) × (2 + 8 + 10) = **3,840 points**

## Combo 3: Multiplicative Scaling (Not Implemented)
**Jokers**: Baron + Blueprint

Hold 3 Kings:
- Base hand score: 100
- Baron: X1.5 × X1.5 × X1.5 = X3.375 Mult
- Blueprint: Copies Baron = X3.375 Mult again
- **Total**: 100 × 3.375 × 3.375 = **1,139 points** (just from mult multipliers!)

## Combo 4: Retrigger Effects (Not Implemented)
**Jokers**: Hack + Fibonacci

Play 2, 3, 5 (all retriggered by Hack):
- Each 2, 3, 5 triggers twice
- Fibonacci: +8 Mult per trigger
- 6 triggers total = +48 Mult

---

# Code Reference

Jokers are implemented in `core/src/joker.rs`:

```rust
pub trait Joker {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn cost(&self) -> usize;
    fn rarity(&self) -> Rarity;
    fn categories(&self) -> Vec<Categories>;
    fn effects(&self, game: &Game) -> Vec<Effects>;
    fn sell_value(&self) -> usize; // Defaults to cost/2
}
```

## Rarity Enum

```rust
pub enum Rarity {
    Common,    // 70% shop spawn rate
    Uncommon,  // 25% shop spawn rate
    Rare,      // 5% shop spawn rate
    Legendary, // Only from Soul card
}
```

## Effect Categories

```rust
pub enum Categories {
    MultPlus,      // Adds to Mult
    Chips,         // Adds to Chips
    MultMult,      // Multiplies Mult
    Economy,       // Generates money
    Retrigger,     // Replays effects
    Effect,        // Special mechanics
}
```

---

# Testing

Each implemented joker has comprehensive unit tests:

```bash
# Run all joker tests
cargo test -p balatro-rs --lib joker

# Run specific joker test
cargo test -p balatro-rs test_greedy_joker
```

Test structure:
1. Calculate score without joker (baseline)
2. Calculate score with joker
3. Verify effect triggered correctly
4. Verify score increased by expected amount

---

# References

This document compiled from official Balatro sources:
- [Balatro Wiki (Fandom)](https://balatrogame.fandom.com/wiki/Jokers)
- [Balatro Wiki (Community)](https://balatrowiki.org/w/Jokers)
- [Steam Community Guide](https://steamcommunity.com/sharedfiles/filedetails/?id=3164787574)
- Balatro version 1.0.1o-FULL

For implementation details:
- `core/src/joker.rs` - Source code
- `MISSING_FEATURES_DETAILED.md` - Unimplemented features
- `PROJECT_STATUS.md` - Overall project status
- `BALATRO_BASIC_RULES.md` - Complete game rules
