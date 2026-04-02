use crate::game::deck::{Card, Rank, Suit};
use std::fmt;
#[derive(Clone, Debug)]
pub struct Trick {
    pub(crate) cards: [Option<Card> ; 4],
    start_player_index: usize,
    current_player_index: usize,
    suit : Option<Suit>,
    trump : Option<Suit>,
    winner: i64,
}
// Maintains a trick based on absolute played indices
// i.e., if player 1 starts a trick and player 2 starts another later
// player 1's card played should be in position 0 in both tricks
// (Assuming starting position is correct)
impl fmt::Display for Trick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Trick [start: P{}] ", self.start_player_index)?;
        for (i, card) in self.cards.iter().enumerate() {
            let player =i ;
            match card {
                Some(c) => write!(f, "P{}: {} ", player, c)?,
                None    => write!(f, "P{}: -- ", player)?,
            }
        }
        if self.winner >= 0 {
            write!(f, "| winner: P{}", self.winner)?;
        }
        Ok(())
    }
}
impl Trick {
    // allocates a new trick, reserves 4 cards for the trick
    pub fn new(starting_player : usize) -> Trick {
        Trick {
            cards : [None, None, None, None],
            start_player_index: starting_player,
            current_player_index: starting_player,
            winner : -1,
            suit: None,
            trump: None,
        }
    }
    pub fn set_player(&mut self, player : usize) {
        self.current_player_index = player;
        self.start_player_index = player;
    }
    pub fn set_suit(&mut self, suit : Suit) {
        self.suit = Some(suit);
    }
    pub fn set_trump(&mut self, trump : Suit) { self.trump = Some(trump); }
    pub fn push(&mut self, card: Card) {
        if self.cards[self.current_player_index].is_some() {
            panic!("Trick already full");
        }
        // Create a circular buffer
        self.cards[self.current_player_index] = Some(card);
        self.current_player_index += 1;
        self.current_player_index %= 4;

    }
    pub fn convert_to_vec(&self) -> Vec::<Card> {
        self.cards
            .map(|c| c.expect("Trick converted before all cards evalutated"))
            .to_vec()
    }
    // Returns the winner of the trick
    pub fn get_winner(&self) -> Option<u64> {
        if self.winner != -1 { Some(self.winner as u64) }
        else {
            None
        }
    }
    pub fn compute_winner(&mut self, tsuit: Option<Suit>) -> Result<(), &'static str> {
        if self.cards.iter().any(|card| card.is_none()) {
            return Err("Trick in progress, winner has not been decided.");
        }

        let mut cur_max = 0;
        let mut cur_idx: u64 = 0;
        for i in 0..4 {
            let cur_score = card_strength(&self.cards[i].as_ref().unwrap(), 
                                          tsuit, 
                                          self.suit.unwrap());
            if cur_score > cur_max {
                cur_max = cur_score;
                cur_idx = i as u64;
            }
        }

        self.winner = (cur_idx) as i64;
        Ok(())
    }
    pub fn len(&self) -> usize
    { self.cards.iter().map(|c| if c.is_some() {1} else {0}).sum()}
    pub fn get_enemy_cards(&self, player : usize) -> Vec::<Card> {
        let enemy_team = (player+1) % 2;
        self.cards
            .iter()
            .enumerate() // (index, card)
            .filter (|(idx,_ )| idx % 2 == enemy_team)// Keep enemy cards
            .filter_map(|(_, card)| *card) // remove indices
            .collect()
    }
    pub fn get_friend_card(&self, player : usize) -> Option<Card> {
        self.cards[(player+2) % 4]
    }
}

pub fn card_strength(card: &Card, trump_suit: Option<Suit>, current_suit : Suit) -> u64 {

    match trump_suit {
        Some(trump) if card.suit == trump => {
            match card.rank {
                Rank::Jack  => 16,
                Rank::Nine  => 15,
                Rank::Ace   => 14,
                Rank::Ten   => 13,
                Rank::King  => 12,
                Rank::Queen => 11,
                Rank::Eight => 10,
                Rank::Seven => 9,
            }
        }
        _ => { // Sun
            if card.suit != current_suit {return 0;}
            match card.rank {
                Rank::Ace   => 8,
                Rank::Ten   => 7,
                Rank::King  => 6,
                Rank::Queen => 5,
                Rank::Jack  => 4,
                Rank::Nine  => 3,
                Rank::Eight => 2,
                Rank::Seven => 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    // --- card_trick_score ---

    #[test]
    fn trump_jack_is_highest() {
        let score = card_strength(&card(Suit::Hearts, Rank::Jack), Some(Suit::Hearts), Suit::Hearts);
        assert_eq!(score, 16);
    }

    #[test]
    fn trump_nine_beats_trump_ace() {
        let nine  = card_strength(&card(Suit::Hearts, Rank::Nine), Some(Suit::Hearts), Suit::Hearts);
        let ace   = card_strength(&card(Suit::Hearts, Rank::Ace), Some(Suit::Hearts), Suit::Hearts);
        assert!(nine > ace);
    }

    #[test]
    fn trump_order() {
        let trump = Some(Suit::Spades);
        let s = |r| card_strength(&card(Suit::Spades, r), trump, Suit::Hearts);
        assert!(s(Rank::Jack) > s(Rank::Nine));
        assert!(s(Rank::Nine) > s(Rank::Ace));
        assert!(s(Rank::Ace)  > s(Rank::Ten));
        assert!(s(Rank::Ten)  > s(Rank::King));
        assert!(s(Rank::King) > s(Rank::Queen));
        assert!(s(Rank::Queen)> s(Rank::Eight));
        assert!(s(Rank::Eight)> s(Rank::Seven));
    }

    #[test]
    fn non_trump_order() {
        let trump = Some(Suit::Clubs);
        let s = |r| card_strength(&card(Suit::Hearts, r), trump, Suit::Hearts);
        assert!(s(Rank::Ace)  > s(Rank::Ten));
        assert!(s(Rank::Ten)  > s(Rank::King));
        assert!(s(Rank::King) > s(Rank::Queen));
        assert!(s(Rank::Queen)> s(Rank::Jack));
        assert!(s(Rank::Jack) > s(Rank::Nine));
        assert!(s(Rank::Nine) > s(Rank::Eight));
        assert!(s(Rank::Eight)> s(Rank::Seven));
    }

    #[test]
    fn trump_beats_non_trump_ace() {
        let trump = Some(Suit::Diamonds);
        let trump_seven = card_strength(&card(Suit::Diamonds, Rank::Seven), trump, Suit::Hearts);
        let off_ace     = card_strength(&card(Suit::Hearts, Rank::Ace), trump, Suit::Hearts);
        assert!(trump_seven > off_ace);
    }

    #[test]
    fn no_trump_all_suits_use_sun_ranking() {
        let ace_hearts   = card_strength(&card(Suit::Hearts, Rank::Ace), None, Suit::Hearts);
        let ace_spades   = card_strength(&card(Suit::Spades, Rank::Ace), None, Suit::Hearts);
        let seven_hearts = card_strength(&card(Suit::Hearts, Rank::Seven), None, Suit::Hearts);
        assert_ne!(ace_hearts, ace_spades);
        assert!(ace_hearts > seven_hearts);
        assert!(seven_hearts > ace_spades);
    }

    // --- Trick::new ---

    #[test]
    fn new_trick_starts_at_given_player() {
        let trick = Trick::new(2);
        assert_eq!(trick.start_player_index, 2);
        assert_eq!(trick.current_player_index, 2);
        assert!(trick.cards.iter().all(|c| c.is_none()));
    }

    // --- Trick::push ---

    #[test]
    fn push_fills_slots_in_circular_order() {
        let mut trick = Trick::new(2);
        trick.push(card(Suit::Hearts, Rank::Seven));   // slot 2
        trick.push(card(Suit::Hearts, Rank::Eight));   // slot 3
        trick.push(card(Suit::Hearts, Rank::Nine));    // slot 0
        trick.push(card(Suit::Hearts, Rank::Ten));     // slot 1

        assert!(trick.cards[2].is_some());
        assert!(trick.cards[3].is_some());
        assert!(trick.cards[0].is_some());
        assert!(trick.cards[1].is_some());
    }

    #[test]
    #[should_panic(expected = "Trick already full")]
    fn push_panics_when_full() {
        let mut trick = Trick::new(0);
        for _ in 0..5 {
            trick.push(card(Suit::Hearts, Rank::Seven));
        }
    }

    // --- Trick::winner ---

    #[test]
    #[should_panic(expected = "Trick in progress, winner has not been decided.")]
    fn winner_returns_none_when_incomplete() {
        let mut trick = Trick::new(0);
        trick.push(card(Suit::Hearts, Rank::Ace));
        trick.push(card(Suit::Hearts, Rank::King));
        trick.compute_winner(None).expect("FUCK");
        assert!(trick.get_winner().is_none());
    }

    #[test]
    fn winner_no_trump_highest_ace_wins() {
        // player 0: Ace, 1: King, 2: Queen, 3: Jack
        let mut trick = Trick::new(0);
        trick.push(card(Suit::Hearts,   Rank::Ace));
        trick.push(card(Suit::Diamonds, Rank::King));
        trick.push(card(Suit::Clubs,    Rank::Queen));
        trick.push(card(Suit::Spades,   Rank::Jack));
        trick.compute_winner(None).expect("TODO: panic message");
        assert_eq!(trick.get_winner(), Some(0));
    }

    #[test]
    fn winner_trump_jack_beats_all() {
        // player 0: trump Jack, 1: off-suit Ace, 2: off-suit Ace, 3: trump Nine
        let mut trick = Trick::new(0);
        trick.push(card(Suit::Spades, Rank::Jack));   // slot 0 — trump Jack (score 16)
        trick.push(card(Suit::Hearts, Rank::Ace));    // slot 1 — off-suit Ace (score 8)
        trick.push(card(Suit::Clubs,  Rank::Ace));    // slot 2 — off-suit Ace (score 8)
        trick.push(card(Suit::Spades, Rank::Nine));   // slot 3 — trump Nine (score 15)
        trick.compute_winner(Some(Suit::Spades)).expect("ah, why");
        assert_eq!(trick.get_winner(), Some(0));
    }

    #[test]
    fn winner_trump_nine_beats_non_trump_ace() {
        // player 0: off-suit Ace, 1: trump Nine, 2: off-suit King, 3: off-suit Queen
        let mut trick = Trick::new(0);
        trick.push(card(Suit::Hearts,   Rank::Ace));
        trick.push(card(Suit::Diamonds, Rank::Nine));
        trick.push(card(Suit::Clubs,    Rank::King));
        trick.push(card(Suit::Hearts,   Rank::Queen));
        trick.compute_winner(Some(Suit::Diamonds)).unwrap();
        assert_eq!(trick.get_winner(), Some(1));
    }

    #[test]
    fn winner_with_non_zero_start_player() {
        // starting player 3, push order: slot3, slot0, slot1, slot2
        let mut trick = Trick::new(3);
        trick.push(card(Suit::Hearts, Rank::Seven));  // slot 3
        trick.push(card(Suit::Hearts, Rank::Ace));    // slot 0 — highest
        trick.push(card(Suit::Hearts, Rank::King));   // slot 1
        trick.push(card(Suit::Hearts, Rank::Queen));  // slot 2
        trick.compute_winner(None).expect("TODO: panic message");
        assert_eq!(trick.get_winner(), Some(0));
    }
}