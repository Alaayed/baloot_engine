
pub mod game;
fn main() {
    stress_tester()
}


fn stress_tester() {
    let mut gs = game::game_state::GameState::default();
    use rand::seq::IteratorRandom;
    gs.get_new_hands();
    let mut current_player = 0;
    println!("Running stress tester...");
    for _i in 0..8 {
        // Pick a random valid
        for _ in 0..4 {
            let valid_moves = gs.legal_moves(current_player);
            let choose  = valid_moves
                .iter()
                .enumerate()
                .filter(|(_, v)| **v)
                .map(|(i, _)| i)
                .choose(&mut rand::rng());
            gs = gs.apply(current_player , choose.unwrap()).unwrap();
            current_player +=1;
            current_player %=4;
        }
        current_player = gs.next_trick_player();
    }
    gs.print_tricks();
    println!("YAY")
}