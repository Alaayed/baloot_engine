use rand::SeedableRng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}
pub fn distribute_hands_from_shuffled_deck(mut deck : Deck) -> [Vec<Card> ; 4]{
    let c = & mut deck.cards;
    let p1 : Vec<Card> =c.drain(..8).collect();
    let p2 : Vec<Card> =c.drain(..8).collect();
    let p3 : Vec<Card> =c.drain(..8).collect();
    let p4 : Vec<Card> =c.drain(..8).collect();
    [p1,p2,p3,p4]
}
impl Deck {
    pub fn new() -> Deck {
        // Create a deck of cards
        let mut new_deck = Deck {
            cards: Vec::<Card>::with_capacity(32)
        };
        // All possible suits
        let suits= Vec::<Suit>::from(
            [Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs,
            Suit::Spades]
        );
        let ranks = Vec::<Rank>::from(
            [Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace]
        );
        for suit in suits.iter() {
            for rank in ranks.iter() {
                new_deck.cards.push(
                    Card { 
                        suit: suit.clone(),
                        rank: rank.clone() }
                );
            }
        }
        return new_deck
}
    pub fn inplace_shuffle(&mut self, seed: u64) {
        // impl: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
        // Ensures each permutation is equally likely.
        use rand::rngs::StdRng;
        use rand::RngExt;
        let mut rng : StdRng= StdRng::seed_from_u64(seed);
        for i in (1..self.cards.len()).rev() {
            // uniform dist: [0, i]
            let idx : usize  = RngExt::random_range(&mut rng,0..=i);
            (&mut self.cards).swap(idx, i);
        }
    }
}