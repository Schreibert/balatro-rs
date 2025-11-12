
# Balatro — Basic Rules

This document summarizes the **core gameplay rules of Balatro**. It is designed as a quick reference for implementing or simulating the game.

---

## 1. Objective

- Progress through **Antes 1–8**, each containing three blinds: **Small Blind**, **Big Blind**, and **Boss Blind**.  
- To clear a blind, you must score **chips** greater than or equal to the blind’s **target score** before running out of available plays.  
- Winning the **Boss Blind** advances you to the next Ante.  
- Clearing Ante 8 (base stake) counts as a run victory. Endless mode can continue beyond.

---

## 2. Game Flow

1. **Start of Ante**  
   - Begin with a deck of standard playing cards (varies if using alternative decks).  
   - You are assigned three blinds in sequence: Small → Big → Boss.  

2. **Blinds**
   - **Small/Big Blind:** Optional to play; can be skipped for a **Tag** (see section 8).
   - **Boss Blind:** Mandatory to clear in order to advance; **cannot be skipped**.
   - Each blind has a **target score** which scales with Ante number.  

3. **Turns (within a Blind)**  
   - You have a limited number of **hands (plays)** and **discards** per blind (base: 4 plays, 3 discards).  
   - On your turn:  
     - Select 1–5 cards to **play** as a poker hand.  
     - Score = (Hand Chips × Hand Multiplier), modified by Jokers, enhancements, and other effects.  
     - If target score is met, the blind is cleared immediately.  
     - If you run out of plays without reaching the target, you lose the run.  
   - You may instead **discard** a subset of cards (up to hand size) to draw new ones.  

4. **After a Blind**  
   - Rewards: money, sometimes packs or bonuses.  
   - Move to next blind or, after Boss, advance to next Ante.  

5. **Shop Phase (Post-Boss or Post-Blind)**  
   - Spend money on:  
     - **Jokers** (passive scoring modifiers)  
     - **Consumables** (Tarot, Planet, Spectral cards)  
     - **Packs** (Arcana, Celestial, Spectral)  
     - **Rerolls** (refresh shop offers)  
   - Money carries over between rounds; excess money can grant **interest** at the end of each round (e.g., +$1 per $5 saved, capped).  

---

## 3. Scoring System

- Every played hand is evaluated as a **poker hand**.  
- Each hand type has a **base chip value** and **base multiplier**.  
  - Example (typical values):  
    - Pair: 10 chips × 2 mult  
    - Full House: 40 chips × 4 mult  
    - Straight Flush: 100 chips × 8 mult  
- Final score = (base chips × base mult) × (modifiers from Jokers, editions, seals, tags, etc.).  
- **Secret hands** exist (e.g., Five of a Kind, Flush House, Flush Five) with even higher values.  

---

## 4. Resources

- **Plays (hands):** Number of poker hands you can attempt each blind.  
- **Discards:** Number of times you can redraw part of your hand.  
- **Money ($):** Used in shop; generates interest if saved.  
- **Consumable Slots:** Hold Tarots, Planets, Spectrals for later use.  
- **Joker Slots:** Hold Jokers that modify scoring or gameplay.  

---

## 5. Cards & Modifiers

- **Standard Cards:** 52-card deck, hand size 8 by default.  
- **Enhancements:** Cards can become Foil, Holographic, Polychrome, Glass, Stone, etc.  
- **Seals:** Red, Blue, Purple, Gold seals that add special effects when played.  
- **Jokers:** Passive modifiers purchased in the shop; can be common, uncommon, rare, or legendary.  
- **Consumables:** Tarots (modify cards, generate), Planets (upgrade poker hands), Spectrals (high-impact swings).  

---

## 6. Win & Loss Conditions

- **Win:** Clear the Boss Blind of Ante 8 (base stake).  
- **Loss:** Fail to reach blind target before exhausting plays.  
- **Endless Mode:** Continue beyond Ante 8 with scaling blinds until defeat.  

---

## 7. Strategy Highlights

- Balance short-term survival (clearing current blind) with long-term growth (saving money, acquiring powerful Jokers/consumables).
- Skipping blinds for tags introduces branching strategies with significant opportunity costs.
- Deck manipulation via Tarots/Planets/Spectrals is critical for scaling.

---

## 8. Skip Blind & Tag System

### 8.1 Skip Blind Mechanics

**Which Blinds Can Be Skipped:**
- **Small Blind:** Can be skipped ✅
- **Big Blind:** Can be skipped ✅
- **Boss Blind:** Cannot be skipped ❌

**What Happens When Skipping:**
1. The blind's scoring phase is completely bypassed (no hands played)
2. The shop phase after that blind is also skipped
3. You receive exactly **one Tag** (shown before skipping)
4. Game progresses directly to the next blind

**Opportunity Costs of Skipping:**
- Loss of blind reward money (Small: $3, Big: $4)
- Loss of interest accumulation
- Loss of shop access (cannot buy jokers, consumables, or reroll)
- Loss of joker scaling opportunities (no hands played = no triggers)
- Loss of hand leveling (no hands played = no leveling from experience)

**Strategic Considerations:**
- Generally ill-advised except for high-value tags (Investment in Ante 1, Economy with low money)
- Double Tag can amplify value of next skip
- Cumulative tags (Speed, Handy, Garbage) benefit from multiple skips throughout run

### 8.2 Tag Acquisition

**Primary Method:** Skip Small Blind or Big Blind
- One tag per skip
- Random selection from eligible tags

**Alternative Methods:**
1. **Diet Cola Joker:** Selling this Uncommon Joker creates a Double Tag
2. **Anaglyph Deck:** Defeating Boss Blind grants a Double Tag
3. **Blueprint/Brainstorm + Diet Cola:** Copying Diet Cola's sell effect

### 8.3 Tag Selection Rules

**Eligibility Filters:**

1. **Ante Filter:**
   - Ante 1: Only 15 tags available (see list below)
   - Ante 2+: All 24 tags available

2. **Unlock Filter (edition tags require discovery):**
   - **Rare Tag:** Requires discovering Blueprint joker
   - **Foil Tag:** Requires discovering Foil edition
   - **Holographic Tag:** Requires discovering Holographic edition
   - **Polychrome Tag:** Requires discovering Polychrome edition
   - **Negative Tag:** Requires discovering Negative edition

**Selection:** Equal probability among all eligible tags

### 8.4 Tag Mechanics

**Tag Queue:**
- Tags persist across rounds, blinds, and antes until consumed
- Multiple tags can be held simultaneously
- Trigger order: **FIFO (First-In-First-Out)**
  - Oldest tag triggers first
  - Newest tag triggers last

**Consumption:**
- Most tags are one-time use (consumed when effect triggers)
- Cumulative tags (Speed, Handy, Garbage) track continuously and never consumed

### 8.5 Tag Categories by Trigger Timing

**Immediate Triggers (11 tags):**
- Activate instantly upon obtaining the tag
- Tags: Charm, Buffoon, Meteor, Ethereal, Standard, Economy, Speed, Handy, Garbage, Orbital, Top-up

**Shop Triggers (9 tags):**
- Activate when entering the next shop
- Tags: Uncommon, Rare, Foil, Holographic, Polychrome, Negative, Voucher, Coupon, D6

**Round Trigger (1 tag):**
- Activate at start of next blind
- Tags: Juggle

**Boss Triggers (2 tags):**
- Investment: Activates after defeating next Boss Blind
- Boss: Activates before encountering next Boss Blind

**Tag-Obtained Trigger (1 tag):**
- Double: Activates when the next tag is obtained

### 8.6 Complete Tag Reference

#### Ante 1 Tags (15 total)

| Tag | Effect | Trigger | Value |
|-----|--------|---------|-------|
| **Uncommon Tag** | Shop has a free Uncommon Joker | Next shop | High (early) |
| **Rare Tag** | Shop has a free Rare Joker | Next shop | High (early) |
| **Foil Tag** | Next base edition shop Joker becomes Foil (+50 Chips) and free | Next shop | Medium |
| **Holographic Tag** | Next base edition shop Joker becomes Holographic (+10 Mult) and free | Next shop | Medium |
| **Polychrome Tag** | Next base edition shop Joker becomes Polychrome (×1.5 Mult) and free | Next shop | High |
| **Investment Tag** | Gain $25 after defeating next Boss Blind | After boss defeat | Very High (Ante 1) |
| **Voucher Tag** | Adds a Voucher to next shop | Next shop | Medium-High |
| **Boss Tag** | Re-rolls the next Boss Blind | Before boss | Medium |
| **Charm Tag** | Open free Mega Arcana Pack (choose 2 of 5 Tarot cards) | Immediate | Medium |
| **Coupon Tag** | Initial jokers, consumables, packs are $0 in next shop | Next shop | Medium |
| **Double Tag** | Gives a copy of the next Tag selected (excluding Double) | Next tag | Very High (situational) |
| **Juggle Tag** | +3 Hand Size for next round only | Next round | Low |
| **D6 Tag** | Rerolls in next shop start at $0 (then +$1 each) | Next shop | Low |
| **Economy Tag** | Doubles money (max +$40) | Immediate | High (low money) |
| **Speed Tag** | $5 per skipped Blind this run | Immediate | Low (cumulative) |

#### Ante 2+ Tags (9 additional)

| Tag | Effect | Trigger | Value |
|-----|--------|---------|-------|
| **Negative Tag** | Next base edition shop Joker becomes Negative (+1 joker slot) and free | Next shop | Very High |
| **Standard Tag** | Open free Mega Standard Pack (choose 2 of 5 Playing cards) | Immediate | Situational |
| **Meteor Tag** | Open free Mega Celestial Pack (choose 2 of 5 Planet cards) | Immediate | Medium-High |
| **Buffoon Tag** | Open free Mega Buffoon Pack (choose 2 of 4 Jokers) | Immediate | Medium |
| **Handy Tag** | $1 per played hand this run | Immediate | Low (cumulative) |
| **Garbage Tag** | $1 per unused discard this run | Immediate | Low (cumulative) |
| **Ethereal Tag** | Open free Spectral Pack (choose 1 of 2 Spectral cards) | Immediate | Medium |
| **Top-up Tag** | Creates up to 2 Common Jokers | Immediate | Low-Medium |
| **Orbital Tag** | Upgrade random poker hand by 3 levels | Immediate | Medium |

### 8.7 Special Tag Mechanics

#### Double Tag
- **Effect:** Copies the very next non-Double tag obtained
- **Stacking:** Multiple Double Tags stack additively
  - 1 Double Tag = 2 copies of next tag (original + 1 copy)
  - 2 Double Tags = 3 copies of next tag (original + 2 copies)
  - N Double Tags = (N+1) copies of next tag
- **Important:** Double Tag does NOT copy itself
- **Processing:** All Double Tags convert simultaneously to the next tag

#### Edition Tags (Foil, Holographic, Polychrome, Negative)
- **Target:** "Next base edition shop Joker"
  - Only affects jokers without existing editions
  - Skips jokers that already have editions
- **Effect:** Makes joker FREE ($0) AND applies the edition
- **Guarantee:** 100% effect (changed from probabilistic in v1.0.1f)

#### Cumulative Tags (Speed, Handy, Garbage)
- **Tracking:** Accumulate value throughout entire run
- **Never Consumed:** Persist until run ends
- **Continuous:** Continue accumulating even after obtained

**Speed Tag:**
- Pays $5 per blind skipped this run
- Includes the skip that granted this tag
- Counter increments each time any blind is skipped

**Handy Tag:**
- Pays $1 per hand played this run
- Includes all hands played before and after obtaining tag
- Counts every single hand played

**Garbage Tag:**
- Pays $1 per discard NOT used this run
- Formula: (Total available discards) - (Discards used) = Payout
- Example: 6 rounds × 3 discards = 18 total; used 6 → pay $12
- **Stake Penalty:** Blue Stake+ reduces payout by $1 per round

#### Investment Tag
- **Payout:** $25 (changed from $15 in v1.0.1f)
- **Timing:** One-time payout after next Boss Blind defeated
- **Stacking:** Multiple Investment Tags pay out independently

#### Boss Tag
- **Effect:** Rerolls the next Boss Blind encounter
- **Bug/Interaction:** Consumes Director's Cut voucher reroll if used
  - Recommendation: Use Director's Cut before Boss Tag

#### Coupon Tag
- **Free Items:** Initial shop display only (jokers, consumables, packs)
- **NOT Free:** Vouchers, items after reroll
- **Inflation Interaction:** Free items from Coupon don't increase prices

#### Voucher Tag
- **Effect:** Adds additional voucher to shop
- **Stacking:** Multiple Voucher Tags add multiple vouchers
- **Upgrade Interaction:** If base voucher purchased before tag triggers, can generate tier 2 version

#### Pack Tags (Charm, Buffoon, Meteor, Standard, Ethereal)
- **Mega Packs (Charm, Buffoon, Meteor, Standard):**
  - Contain more cards than normal packs
  - Choose 2 items instead of 1
- **Regular Pack (Ethereal):**
  - Standard Spectral Pack (choose 1 of 2)
- **Skipping Packs:** Can skip selection (minimal benefit)

### 8.8 Version History

**Version 1.0.1f Changes:**
- Investment Tag: $15 → $25
- Uncommon Tag: Now makes joker free (100%)
- Rare Tag: 1/3 chance free → guaranteed free
- Foil Tag: 50% chance → guaranteed effect
- Holographic Tag: 1/3 chance → guaranteed effect
- Polychrome Tag: Now makes joker free

**Version 0.9.0m-DEMO:**
- 9 tags restricted to Ante 2+ (Negative, Standard, Meteor, Buffoon, Handy, Garbage, Ethereal, Top-up, Orbital)

---
