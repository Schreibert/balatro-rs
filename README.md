# balatro-rs

Game engine and move generator for a simplified version of [balatro](https://www.playbalatro.com/), written in rust with python bindings

## Overview

This library implements a subset of balatro's rules allowing execution of games or simulations. It provides an exhaustive list of actions a user could take at any given stage, as well as an engine to execute those actions and progress the game.

The goal of providing all valid actions is to serve as a move generator. This would be the first step to apply reinforcement learning for balatro.

## Example

```rust
use balatro_rs::{action::Action, game::Game, stage::Stage};
use rand::Rng;

fn main() {
    let mut g = Game::default();
    g.start();
    while !g.is_over() {
        // Get all available moves
        let actions: Vec<Action> = g.gen_moves().collect();
        if actions.len() == 0 {
            break;
        }

        // Pick a random move and execute it
        let i = rand::thread_rng().gen_range(0..actions.len());
        let action = actions[i].clone();
        g.handle_action(action);
    }
    let result = g.result();
}
```

## Features

This library does not yet implement all aspects of Balatro.

The following features are implemented (including move generation):
- [x] identification and scoring of poker hands (all 13 hand ranks)
- [x] playing/discarding/reordering of cards
- [x] blind pass/fail and game win/lose conditions
- [x] money/interest generation
- [x] ante progression (up to ante 8)
- [x] blind progression (small, big, boss)
- [x] stage transition (pre-blind, blind, post-blind, shop)
- [x] boss blind modifiers (all 20 modifiers)
- [x] buying/selling/using jokers (~120 functional jokers)
- [x] buying/selling/using tarots (all 22 tarot cards)
- [x] buying/selling/using planets (all 12 planet cards)
- [x] buying/selling/using spectrals (all 18 spectral cards)
- [x] card enhancements (6/8 - bonus, mult, stone, glass, steel, gold)
- [x] card editions (foil, holographic, polychrome, negative)
- [x] card seals (red, gold, blue, purple)
- [x] voucher system (all 24 vouchers)
- [x] booster packs (arcana, celestial, spectral, buffoon)
- [x] skip blind/tags (all 24 tag types)
- [x] alternative decks (14/15 standard decks)

The following features are missing and may or may not be added:
- [ ] wild and lucky card enhancements
- [ ] stakes (difficulty levels)
- [ ] remaining ~30 jokers (some require new systems)
- [ ] plasma deck special scoring
- [ ] some complex joker mechanics (retriggers, effect copying)

See [PROJECT_STATUS.md](PROJECT_STATUS.md) for detailed feature breakdown and [docs/reference/MISSING_FEATURES_DETAILED.md](docs/reference/MISSING_FEATURES_DETAILED.md) for comprehensive missing feature list.


## Python bindings

This library uses [pyo3](https://pyo3.rs) to provide python bindings. For more details on the python work and attempts at applying reinforcement learning, check the work in the directory [/pylatro](https://github.com/evanofslack/balatro-rs/tree/main/pylatro).

## Documentation

- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Comprehensive project status and feature completion
- **[CLAUDE.md](CLAUDE.md)** - AI assistant instructions for working with this codebase
- **[docs/](docs/)** - All additional documentation organized by category:
  - **[docs/reference/](docs/reference/)** - Game rules and feature references
  - **[docs/history/](docs/history/)** - Phase completion documents
  - **[docs/sessions/](docs/sessions/)** - Development session summaries
  - **[docs/design/](docs/design/)** - Implementation plans and design documents

See [docs/INDEX.md](docs/INDEX.md) for a complete guide to the documentation structure.
