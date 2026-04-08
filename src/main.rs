
pub mod game;
mod ai;

use std::time::Duration;
use ai::agent::Agent;
use ai::random::RandomAgent;
use ai::alpha_beta::AlphaBeta;
use game::{game_state::GameState, deck::{Suit}};
fn main() {
    let mut times = Vec::<Duration>::new();
    for _ in 0..1 {
        let start = std::time::Instant::now();
        dbg!(run_game (Some(100),&AlphaBeta, None, None ));
        times.push(start.elapsed());
    }
    let avg = times.iter().sum::<Duration>() / (times.len()) as u32 ;
    println!("{:?}", avg);
}
fn run_game(seed: Option<u64>, agent : &dyn Agent, trump: Option<Suit>, cp : Option<usize>) -> (u64,u64) {
    let mut gs = GameState::new(None, trump, cp , seed );
    let mut i = 1;
    while gs.is_terminal().is_none() {
        let cp = gs.get_current_player();
        let action = agent.choose_action( &gs , cp);
        //println!("Action Chosen: {i}");
        i+=1;
        gs = gs.apply(cp, action).expect("Crashed through legal action");
    }
    gs.print_tricks();
    gs.is_terminal().expect("Game should be finished")
}
fn stress_tester() {
    for _i in 0..1000 {
        run_game (None, &RandomAgent, None, None);
    }
}