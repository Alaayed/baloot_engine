use crate::ai::agent::Agent;
use crate::game::game_state::GameState;
use std::cmp::{max, min};
pub struct AlphaBeta;

impl Agent for AlphaBeta{
    fn choose_action(&self, state: &GameState, player_index: usize) -> usize {

        todo!()
    }
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