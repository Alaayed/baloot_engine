use crate::ai::agent::Agent;
use crate::game::game_state::GameState;
use rand::seq::IteratorRandom;
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn choose_action (&self, state : &GameState, player_index : usize) -> usize {
        let legal_moves  = state.legal_moves(player_index);
        //dbg!(&legal_moves);
        //dbg!(state);
        legal_moves
            .iter()
            .enumerate()
            .filter(|(_,b)| **b)
            .map(|(index, _)| index)
            .choose(&mut rand::rng())
            .unwrap()
    }
}