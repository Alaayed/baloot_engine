# baloot_engine

A Rust implementation of the Baloot card game engine. Baloot is a 4-player trick-taking card game popular in the Middle East.

## Structure

- `deck.rs` — 32-card deck with Fisher-Yates shuffle and hand distribution
- `trick.rs` — Trick evaluation and card strength ranking
- `scorer.rs` — Point scoring for Sun and Hokom contracts
- `game_state.rs` — Game state and legal move validation

## Game Modes

- **Sun (No Trump):** 132 total points, simple suit-following rules
- **Hokom (Trump):** 163 total points, complex ascending trump requirements

## Status

**Done:** Deck management, trick evaluation, scoring, legal move validation, 40+ unit tests

**TODO:** `GameState::apply()`, `GameState::is_terminal()`, AI bot (Alpha-Beta pruning)

## Dependencies

- `rand = "0.10.0"` — Seeded shuffling for reproducible games
