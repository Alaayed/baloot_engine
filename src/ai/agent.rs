use crate::game::game_state::GameState;

pub trait Agent {
    fn choose_action (&self, state : &GameState, player_index : usize) -> usize;
}

