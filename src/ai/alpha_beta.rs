use crate::ai::agent::Agent;
use crate::game::game_state::GameState;
use std::cmp::{max, min};
use rayon::prelude::*;
pub struct AlphaBeta;
const DEPTH : usize= 20;
impl Agent for AlphaBeta{
    fn choose_action(&self, state: &GameState, player_index: usize) -> usize {
        return parallel_choose_action(state, player_index);
        let mut best_val : i64= 0;
        if player_index % 2 == 0 {
            best_val = -1000
        } else {
            best_val = 1000
        }
        let mut best_idx : usize = 0;
        // depth is the number of cards to explore
        let mut depth :usize = state.hands.iter().map(|c| c.len()).sum();
        depth = min(depth, 16);
        // Iterate through all legal moves
        for (idx, is_legal) in state.legal_moves(player_index)
            .iter()
            .enumerate(){
            if !is_legal {
                continue;
            }
            let ns = state.apply(player_index, idx).unwrap();
            let ab_val = alpha_beta(&ns, -1000, 1000, depth as i64);
            if player_index % 2 == 0 {
                if best_val < ab_val {
                    best_val = ab_val;
                    best_idx = idx;
                }
            } else {
                if best_val > ab_val {
                    best_val = ab_val;
                    best_idx = idx;
                }
            }
        }
        best_idx
    }
}
fn parallel_choose_action(state: &GameState, player_index: usize) -> usize {
    let depth = min(state.hands.iter().map(|c| c.len()).sum::<usize>(), DEPTH);
    let maximize = player_index % 2 == 0;

    let results: Vec<(usize, i64)> = state.legal_moves(player_index)
        .into_par_iter()          // parallel iterator
        .enumerate()
        .filter(|(_, is_legal)| *is_legal)
        .map(|(idx, _)| {
            let ns = state.apply(player_index, idx).unwrap();
            let val = alpha_beta(&ns, -1000, 1000, depth as i64);
            (idx, val)
        })
        .collect();

    results.into_iter()
        .reduce(|best, cur| {
            if maximize { if cur.1 > best.1 { cur } else { best } }
            else        { if cur.1 < best.1 { cur } else { best } }
        })
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}
// As seen in https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning#Pseudocode
fn alpha_beta(state: &GameState,
              mut alpha: i64,
              mut beta : i64,
              depth : i64) -> i64 {
    if state.is_terminal().is_some() || depth == 0{
        let score = state.get_current_scores();
        return (score.0 as i64 - score.1 as i64)
    }
    let player_index = state.get_current_player();
    let mut value;
    if player_index % 2 == 0 {
        value = -180;
        for (idx, is_legal) in state.legal_moves(player_index).iter().enumerate() {
            if !is_legal {continue;}
            let new_state = state.apply(player_index, idx).unwrap();
            value = max(value, alpha_beta(&new_state, alpha, beta, depth-1));
            // Break condition
            if value >= beta {break;}
            alpha = max(alpha, value);
        };
    } else {
            value = 180;
        for (idx, is_legal) in state.legal_moves(player_index).iter().enumerate() {
            if !is_legal {continue;}
            let new_state = state.apply(player_index, idx).unwrap();
            value = min(value, alpha_beta(&new_state, alpha, beta, depth-1));
            // Break condition
            if value >= beta {break;}
            beta = min(beta, value);
        };
    }
    value
}