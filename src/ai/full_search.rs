use crate::ai::agent::Agent;
use crate::game::game_state::GameState;

pub struct FullSearch;

impl Agent for FullSearch {
    fn choose_action(&self, state: &GameState, player_index: usize) -> usize {
        
        todo!()
    }
}