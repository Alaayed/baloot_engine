# baloot_engine

A Rust implementation of the Baloot card game engine. Baloot is a 4-player trick-taking card game popular in the Middle East.

## Structure

- `deck.rs` — 32-card deck with seeded Fisher-Yates shuffle and hand distribution
- `trick.rs` — Trick evaluation and card strength ranking
- `scorer.rs` — Point scoring for Sun and Hokom contracts
- `game_state.rs` — Game state and legal move validation
- `ai/alpha_beta.rs` — Parallelized Alpha-Beta pruning agent
- `ai/random.rs` — Random agent for benchmarking and testing

## Game Modes

- **Sun (No Trump):** 120 total points, simple suit-following rules
- **Hokom (Trump):** 152 total points, complex ascending trump requirements

## AI

The current AI uses **Alpha-Beta pruning** with a search depth of 16 moves, parallelized at the root level using **Rayon**. This allows a full game traversal in approximately 5.5 seconds.

## Status

**Done:** Deck management, trick evaluation, scoring, legal move validation, parallelized Alpha-Beta agent, 40+ unit tests

**In Progress:** Performance tuning and search optimization, such as using iterative deepening

**Future Direction:**
- Reduce full-game traversal time through move ordering, transposition tables, and other pruning heuristics
- Explore ML-based approaches such as ISMCTS or reinforcement learning to handle the imperfect information inherent to Baloot

## Dependencies

- `rand = "0.10.0"` — Seeded shuffling for reproducible games
- `rayon = "1.11.0"` — Data parallelism for root-level Alpha-Beta search
