#[derive(Debug, Copy, Clone)]
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
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
pub struct Deck {
    pub cards: Vec<Card>
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
}