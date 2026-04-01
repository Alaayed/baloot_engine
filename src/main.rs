
pub mod game;
mod ai;
use ai::agent::Agent;
use ai::random::RandomAgent;
use game::{game_state::GameState, deck::{Suit}};
fn main() {
    stress_tester()
}
fn run_game(seed: Option<u64>, agent : &dyn Agent, trump: Option<Suit>, cp : Option<usize>) -> (u64,u64) {
    let mut gs = GameState::new(None, trump, cp , seed );
    while gs.is_terminal().is_none() {
        let cp = gs.get_current_player();
        let action = agent.choose_action( &gs , cp);
        gs = gs.apply(cp, action).expect("Crashed through legal action");
    }
    gs.is_terminal().expect("Game should be finished")
}
fn stress_tester() {
    for _i in 0..1000 {
        run_game (None, &RandomAgent, None, None);
    }
}