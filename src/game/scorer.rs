use super::deck::Suit;
use super::deck::Card;
use super::deck::Rank;
// Score mapping

// Scores Tricks, doesn't score projects.
// Returns (team_tricks, other_team_tricks)
pub fn score_tricks_points(team_tricks : Vec<Card>,
                           trump_suit : Option<Suit>,
                           has_bought : bool) -> (u64, u64) {
    let func = |c: & Card| score_card(c, &trump_suit);
    let trick_sum : u64 = team_tricks.iter().map(func).sum();
    let sun : u64= 132;
    let hokom: u64= 163;
    match &trump_suit {
        Some(_) => { // Hokom Branch
            // You bought and lost the tricks
            if has_bought && trick_sum < 82 {(0, hokom)}
            // They bought and you won the trick
            else if !has_bought && trick_sum >= 82 {(hokom, 0)}
            // You split the points
            else {(trick_sum , hokom-trick_sum)}
        }
        _ => { // Sun Branch
            // Strictly below half (66) to lose contract--exact half is a split
            if has_bought && trick_sum < 66 {(0, sun)}
            else if !has_bought && trick_sum > 66 {(sun, 0)}
            else {(trick_sum , sun-trick_sum)}
        }
    }
}
#[rustfmt::skip]
pub fn score_card (card : &Card, trump_suit : &Option<Suit>) -> u64 {
    match trump_suit {
        Some(trump) if card.suit == *trump => {
            match card.rank {
                Rank::Jack  => 20,
                Rank::Nine  => 14,
                Rank::Ace   => 11,
                Rank::Ten   => 10,
                Rank::King  => 5,
                Rank::Queen => 4,
                _           => 0
            }
        }
        _ => { // Sun
            match card.rank {
                Rank::Ace   => 11,
                Rank::Ten   => 10,
                Rank::King  => 5,
                Rank::Queen => 4,
                Rank::Jack  => 3,
                _           => 0
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Card, Suit, Rank};

    #[test]
    fn test_score_card_trump_jack() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Jack };
        assert_eq!(score_card(&card, &Some(Suit::Hearts)), 20);
    }
    #[test]
    fn test_score_card_trump_nine() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Nine };
        assert_eq!(score_card(&card, &Some(Suit::Hearts)), 14);
    }
    #[test]
    fn test_score_card_non_trump_jack() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Jack };
        assert_eq!(score_card(&card, &Some(Suit::Spades)), 3);
    }

    #[test]
    fn test_score_card_no_trump() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Ace };
        assert_eq!(score_card(&card, &None), 11);
    }

    #[test]
    fn test_bought_and_lost_hokom() {
        let tricks = vec![
            Card { suit: Suit::Hearts, rank: Rank::Seven }, // 0 points
        ];
        let result = score_tricks_points(tricks, Some(Suit::Hearts), true);
        assert_eq!(result, (0, 163));
    }

    #[test]
    fn test_sun_exact_split() {
        // trick_sum == 66 should be a split, not a contract loss
        let tricks = vec![
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
            Card { suit: Suit::Hearts, rank: Rank::Jack },// 3
            Card { suit: Suit::Hearts, rank: Rank::Jack },// 3
            // 66
        ];
        // build a hand that sums to exactly 66...
        let result = score_tricks_points(tricks, None, true);
        assert_eq!(result, (66,66))
    }
}