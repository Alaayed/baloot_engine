use game::deck::Deck;

mod game;
fn main() {
    println!("Hello, world!");
    let mut deck : Deck= Deck::new();
    (&mut deck).inplace_shuffle(41);
    dbg!(&deck);
}
