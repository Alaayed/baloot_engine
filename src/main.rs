
pub mod game;
fn main() {
    let deck = game::deck::Deck::new();
    dbg!(deck);
}
