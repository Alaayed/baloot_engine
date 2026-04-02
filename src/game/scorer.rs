use super::deck::Suit;
use super::deck::Card;
use super::deck::Rank;
use super::trick::{Trick};
// Score mapping

// Scores Tricks, doesn't score projects.
// Returns (team_tricks, other_team_tricks)
pub fn score_tricks_points(all_tricks : &Vec<Trick>,
                           trump_suit : Option<Suit>,
                           bidding_team: u64,
                           bidding_team_projects : u64,
                           other_team_projects  : u64) -> (u64, u64) {
    let err_msg = "Incomplete trick passed to score_tricks_points(), major error.";
    let bidding_trick_score: u64 = all_tricks
        .iter()
        .filter(| t| (t.get_winner().expect(err_msg)) % 2 == bidding_team)
        .map(|t| score_trick(t , trump_suit))
        .sum();
    let other_trick_score: u64 = all_tricks
        .iter()
        .filter(| t| ((t.get_winner().expect(err_msg)) % 2) == ((bidding_team +1)%2))
        .map(|t| score_trick(t , trump_suit))
        .sum();
    // Sun / Hokom + all projects
    let sun : u64= 120 + other_team_projects + bidding_team_projects;
    let hokom: u64= 152 + other_team_projects + bidding_team_projects;
    // Score of each team, tricks + projects
    let mut other_team_score = other_team_projects + other_trick_score;
    let mut bidding_team_score = bidding_team_projects + bidding_trick_score;
    match trump_suit {
        Some(_) => {
            if other_team_score > hokom / 2 { // Won over half points
                other_team_score = hokom;
                bidding_team_score = 0;
            }
        } _ => {
            if other_team_score > sun / 2 { // Won over half points
                other_team_score = sun;
                bidding_team_score = 0;
            }
        }
    };
    if bidding_team == 0 { (bidding_team_score, other_team_score) }
    else {(other_team_score, bidding_team_score)}
}
pub fn score_trick (t : &Trick, trump_suit : Option<Suit>) -> u64 {
    t.cards
        .iter()
        .filter_map( |c| *c)
        .map(|c| card_score(&c, &trump_suit))
        .sum()
}
#[rustfmt::skip]
pub fn card_score (card : &Card, trump_suit : &Option<Suit>) -> u64 {
    match trump_suit {
        Some(trump) if card.suit == *trump => {
            match card.rank {
                Rank::Jack  => 20,
                Rank::Nine  => 14,
                Rank::Ace   => 11,
                Rank::Ten   => 10,
                Rank::King  => 4,
                Rank::Queen => 3,
                _           => 0
            }
        }
        _ => { // Sun
            match card.rank {
                Rank::Ace   => 11,
                Rank::Ten   => 10,
                Rank::King  => 4,
                Rank::Queen => 3,
                Rank::Jack  => 2,
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
        assert_eq!(card_score(&card, &Some(Suit::Hearts)), 20);
    }
    #[test]
    fn test_score_card_trump_nine() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Nine };
        assert_eq!(card_score(&card, &Some(Suit::Hearts)), 14);
    }
    #[test]
    fn test_score_card_non_trump_jack() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Jack };
        assert_eq!(card_score(&card, &Some(Suit::Spades)), 2);
    }

    #[test]
    fn test_score_card_no_trump() {
        let card = Card { suit: Suit::Hearts, rank: Rank::Ace };
        assert_eq!(card_score(&card, &None), 11);
    }
    //TODO: Re impl these tests after

    // #[test]
    // fn test_bought_and_lost_hokom() {
    //     let tricks = vec![
    //         Card { suit: Suit::Hearts, rank: Rank::Seven }, // 0 points
    //     ];
    //     let result = score_tricks_points(tricks, Some(Suit::Hearts), true);
    //     assert_eq!(result, (0, 163));
    // }
    //
    // #[test]
    // fn test_sun_exact_split() {
    //     // trick_sum == 66 should be a split, not a contract loss
    //     let tricks = vec![
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Ten }, // 10
    //         Card { suit: Suit::Hearts, rank: Rank::Jack },// 3
    //         Card { suit: Suit::Hearts, rank: Rank::Jack },// 3
    //         // 66
    //     ];
    //     // build a hand that sums to exactly 66...
    //     let result = score_tricks_points(tricks, None, true);
    //     assert_eq!(result, (66,66))
    // }
}