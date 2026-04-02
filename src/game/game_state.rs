use crate::game::deck::{distribute_hands_from_shuffled_deck, Card, Deck,Suit};
use crate::game::trick::{card_strength,Trick};
use super::scorer;
// Sun rules
// R1: Must play suit if you have it, otherwise anything else.
// Hokoom
// R1: Must always play above a trump if possible.
#[derive(Debug, Clone)]
pub struct GameState {
    // Game meta data
    pub previous_tricks : Vec<Trick>,
    pub seed : u64,
    // Bid meta data
    pub bidding_team: usize,
    pub trump_suit : Option<Suit>,
    // Trick meta data
    pub current_player : usize,
    pub in_trick : bool,
    pub current_suit : Option <Suit>,
    pub hands : [Vec<Card> ; 4],
    pub current_trick : Trick,
    pub bidding_team_projects : u64,
    pub other_team_projects : u64,
    pub score : (u64,u64)
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            previous_tricks: Vec::new(),
            seed: 42,
            bidding_team: 0,
            trump_suit: None,
            current_player: 0,
            in_trick: false,
            current_suit: None,
            hands: [vec![], vec![], vec![], vec![]],
            current_trick: Trick::new(0),
            bidding_team_projects: 0,
            other_team_projects: 0,
            score : (0,0)
        }
    }
}
impl GameState {
    pub fn new(
        hands : Option <[Vec<Card> ; 4]>,
        trump_suit : Option <Suit>,
        current_player : Option <usize>,
        seed : Option<u64>
    ) -> GameState {
        match hands { // Start a game
            Some(hands) => {
                GameState {
                    hands,
                    trump_suit,
                    current_player : match current_player {Some(c) => c,_=>0},
                    ..GameState::default()
                }
            }
            _ => { // Manufacture a game
                let mut deck : Deck = Deck::new();
                match seed {
                    Some(seed) => deck.inplace_shuffle(seed),
                    None => deck.inplace_shuffle(42)
                }
                // Distribute hands
                let hands = distribute_hands_from_shuffled_deck(deck);
                let current_player = current_player.unwrap_or_else(|| 0);
                GameState {
                    hands,
                    trump_suit,
                    current_player,
                    seed : seed.unwrap_or_else(|| 42),
                    ..GameState::default()
                }

            }
        }
    }
    pub fn get_new_hands(&mut self) {
        let seed = self.seed;
        self.seed +=1;
        let mut deck = Deck::new();
        deck.inplace_shuffle(seed);
        self.hands = distribute_hands_from_shuffled_deck(deck)
    }
    pub fn get_player_hand(&self, player : usize) -> &Vec<Card> {
        &self.hands[player]
    }
    pub fn legal_moves (&self, player : usize) -> Vec<bool> {
        let n = self.hands[player].len();
        if !self.in_trick { // First move of the trick
            return vec![true ; n];
        }
        // TODO: Store enemy played and enemy strongest, friend etc since state persists
        let player_cards = &self.hands[player];
        // all enemy related
        // len == 3, enemy = {2,0}, friend {1}
        // len == 2, enemy = {1}  , friend {0}
        // len == 1, enemy = {0}
        let enemy_played : Vec::<Card> = self.current_trick.get_enemy_cards(player);
        let enemy_strongest : u64 = enemy_played
            .iter()
            .map(|c| card_strength(c, self.trump_suit, self.current_suit.unwrap()))
            .max()
            .unwrap();
        // teammate related
        let friend_card = self.current_trick.get_friend_card(player);
        let friend_won  = friend_card
            .map_or(false, |f| card_strength(&f, self.trump_suit, self.current_suit.unwrap())  > enemy_strongest);
        // All suit related
        let trick_suit= self.current_suit.expect("In trick, but no trick suit? \
        Check apply function for proper init of trick");
        let has_suit  = player_cards.iter().any(|c| c.suit == trick_suit);
        let suit_mask      =  player_cards.iter().map(|c| c.suit == trick_suit).collect();
        let mapped_cards:Vec<u64>= player_cards.iter().map(|c| card_strength(c, self.trump_suit, self.current_suit.unwrap())).collect();
        // Overriding rule, you must ALWAYS play the suit
        match &self.trump_suit {
            Some(trump) => {

                let has_trump = player_cards.iter().any(|c| c.suit == *trump );
                let enemy_trumped = enemy_played.iter().any(|c| c.suit == *trump );
                let can_ascend_trump = mapped_cards.iter().any(|&c| c > enemy_strongest);
                let ascending_mask = mapped_cards.iter().map(|&c| c > enemy_strongest).collect();
                let trump_mask = player_cards.iter().map(|c| c.suit == *trump).collect();
                // Forces Hokom if you MUST beat
                // Note: does not enforce that hokom was played
                // Case 1: you have the suit
                if has_suit {
                    // You must ALWAYS play the suit
                    if trick_suit == *trump{
                        // Trump trick, check ascending play condition
                        if  can_ascend_trump && !friend_won {
                            return ascending_mask;
                        }
                        // Revert to normal suit play
                    }
                    return suit_mask;
                }
                else {
                    // You don't have the suit
                    if enemy_trumped && can_ascend_trump && !friend_won {
                        // Enemy trumped, must ascend
                        return ascending_mask;
                    }
                    if has_trump && !friend_won && !enemy_trumped{
                        // You have trump & friend has not won, and enemy has not trumped
                        return trump_mask;
                    }
                }
                // No suit, play anything
                vec![true ; n]
            } _ => { // Sun (after all that...)
                if has_suit {suit_mask} else {vec![true; n]}
            }
        }
    }
    // Applies a card played by a player and return the resulting, new, game_state.
    // Will not enforce legal moves or prevent a player from playing twice.
    pub fn apply(&self, player :usize , card_idx: usize) -> Result<GameState, &str> {
        
        let mut new_state : GameState = (*self).clone();
        if !new_state.in_trick { // Start Trick
            // Player just started a new trick
            // Don't clean up prev state, other codes problem, assume clean state
            new_state.in_trick = true;
            // 1. Remove the card and add it to NEW trick
            let removed_card = new_state.hands[player].swap_remove(card_idx);
            new_state.current_trick.set_player(player);
            new_state.current_trick.push(removed_card);
            // 2. Specify current suit
            new_state.current_suit = Some(removed_card.suit);
            new_state.current_trick.set_suit(removed_card.suit);
        } else { // In the middle of current trick
            // 1. Remove the card and add it to trick
            let removed_card = new_state.hands[player].swap_remove(card_idx);
            new_state.current_trick.push(removed_card);
        }
        // Now, inc current player
        new_state.current_player += 1;
        new_state.current_player %= 4;
        // Check if trick is done
        if new_state.current_trick.len() == 4 {
            // Move trick to prev_tricks, giving up ownership
            new_state.current_trick.compute_winner(new_state.trump_suit).expect("wtf");
            // Get trick winner
            let trick_winner = new_state.current_trick.get_winner().unwrap() as usize;
            let trick_score = scorer::score_trick(&new_state.current_trick, new_state.trump_suit);
            // Maintain running trick score
            if trick_winner % 2 == 0 {
                new_state.score.0 += trick_score;
            } else {
                new_state.score.1 += trick_score;
            }
            new_state.previous_tricks.push(new_state.current_trick);
            new_state.current_trick = Trick::new(trick_winner);
            new_state.current_player = trick_winner;
            new_state.in_trick = false;
        }
        Ok(new_state)
    }
    pub fn get_current_scores(&self) -> (u64,u64) {
        self.score
    }
    pub fn is_terminal(&self) -> Option<(u64, u64)> {
        let current = self.score;

        let sun = 120;
        let hokom = 152;
        let projects = self.other_team_projects + self.bidding_team_projects;
        let finished = (current.0 + current.1) == (sun+projects)
            || (current.0 + current.1) == (hokom+projects);
        if finished {
            Some(current)
        } else {
            None
        }
    }
    pub fn get_current_player(&self) -> usize {
        self.current_player
    }
    pub fn print_tricks(&self) {
        for trick in self.previous_tricks.clone() {
            println!("{trick}")
        }
    }
    // TODO: implement early termination if sufficient points collected.
}

// Claude testing code
// ------------------------------
// Assumes standard Baloot card_strength ordering:
//   Trump:     J > 9 > A > 10 > K > Q > 8 > 7
//   Non-trump: A > 10 > K > Q > J > 9 > 8 > 7

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Card, Rank, Suit};

    // ─── Helpers ────────────────────────────────────────────────────────────────

    fn c(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    /// Construct a minimal GameState for testing legal_moves.
    /// `trick` is ordered by play order (trick[0] led, etc.)
    fn state(
        hand: Vec<Card>,
        trump: Option<Suit>,
        in_trick: bool,
    ) -> GameState {
        GameState {
            hands: [hand, vec![], vec![], vec![]],
            trump_suit: trump,
            in_trick,
            ..GameState::default()
        }
    }

    // ─── 1. Not-in-trick ────────────────────────────────────────────────────────

    #[test]
    fn not_in_trick_all_legal_sun() {
        // Rule: first player to a trick can play anything
        let s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Hearts, Rank::Seven)],
            None,
            false,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn not_in_trick_all_legal_hokom() {
        let s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Hearts, Rank::Seven)],
            Some(Suit::Spades),
            false,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 2. Sun mode ────────────────────────────────────────────────────────────
    // For player 0: enemies occupy absolute slots 1 & 3, friend occupies slot 2.
    // len=1: enemy started (slot 1).
    // len=2: friend started (slot 2), enemy followed (slot 3).
    // len=3: enemy started (slot 1), friend (slot 2), enemy (slot 3).

    #[test]
    fn sun_has_suit_must_follow() {
        // R1 Sun: must follow suit if able.
        // Friend led Spades (slot 2), enemy played Hearts (slot 3).
        // Player has one Spade and one Heart → only Spade is legal.
        let mut s = state(
            vec![c(Suit::Spades, Rank::King), c(Suit::Hearts, Rank::Ace)],
            None,
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Spades, Rank::Seven)); // slot 2: friend
        s.current_trick.push(c(Suit::Hearts, Rank::Seven)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn sun_has_suit_all_suit_cards_legal() {
        // All cards of the led suit are legal (not just highest).
        // Friend led Spades (slot 2), enemy played Hearts (slot 3).
        let mut s = state(
            vec![
                c(Suit::Spades, Rank::Seven),
                c(Suit::Spades, Rank::Ace),
                c(Suit::Hearts, Rank::Ace),
            ],
            None,
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Spades, Rank::King));  // slot 2: friend
        s.current_trick.push(c(Suit::Hearts, Rank::Seven)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, true, false]);
    }

    #[test]
    fn sun_no_suit_anything_legal() {
        // R1 Sun: no suit in hand → anything goes.
        // Enemy led Spades (slot 1); player has no Spades.
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Hearts, Rank::King)],
            None,
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Spades, Rank::Seven)); // slot 1: enemy
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 3. Hokom: trump-led trick (trick_suit == trump) ────────────────────────

    #[test]
    fn hokom_trump_trick_must_ascend_if_able() {
        // Friend led A♠ (slot 2), enemy followed with 9♠ (slot 3, strength 15).
        // Player has [J♠, K♠]: J♠ (16) beats 9♠, K♠ (12) does not → [true, false].
        let mut s = state(
            vec![c(Suit::Spades, Rank::Jack), c(Suit::Spades, Rank::King)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Spades, Rank::Ace));  // slot 2: friend
        s.current_trick.push(c(Suit::Spades, Rank::Nine)); // slot 3: enemy (strength 15)
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn hokom_trump_trick_cannot_ascend_play_any_trump() {
        // Friend led 8♠ (slot 2), enemy played J♠ (slot 3, strength 16, highest).
        // Player has [9♠, 7♠]; neither beats J♠ → suit_mask = all trumps = [true, true].
        let mut s = state(
            vec![c(Suit::Spades, Rank::Nine), c(Suit::Spades, Rank::Seven)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Spades, Rank::Eight)); // slot 2: friend
        s.current_trick.push(c(Suit::Spades, Rank::Jack));  // slot 3: enemy (strength 16)
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_trump_trick_friend_winning_play_any_trump() {
        // Enemy led 7♠ (slot 1), friend played J♠ (slot 2, strongest), enemy played 8♠ (slot 3).
        // friend_won=true → no need to ascend → suit_mask = all trumps.
        let mut s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Spades, Rank::Nine)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Spades, Rank::Seven)); // slot 1: enemy
        s.current_trick.push(c(Suit::Spades, Rank::Jack));  // slot 2: friend (winning)
        s.current_trick.push(c(Suit::Spades, Rank::Eight)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 4. Hokom: non-trump-led trick ──────────────────────────────────────────

    #[test]
    fn hokom_suit_led_has_suit_must_follow() {
        // Hearts led, trump=Spades. Friend led K♥ (slot 2), enemy played 8♥ (slot 3).
        // Player has one Heart and one Spade → must follow Hearts.
        let mut s = state(
            vec![c(Suit::Hearts, Rank::Seven), c(Suit::Spades, Rank::Ace)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Hearts, Rank::King));  // slot 2: friend
        s.current_trick.push(c(Suit::Hearts, Rank::Eight)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn hokom_no_suit_enemy_trumped_can_ascend_must_ascend() {
        // Hearts led. Enemy led K♥ (slot 1), friend played A♥ (slot 2),
        // enemy trumped with 7♠ (slot 3). Player has no Hearts, holds A♠ which beats 7♠.
        // → ascending_mask: [A♣=false, A♠=true].
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Ace)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Hearts, Rank::King));   // slot 1: enemy
        s.current_trick.push(c(Suit::Hearts, Rank::Ace));    // slot 2: friend
        s.current_trick.push(c(Suit::Spades, Rank::Seven));  // slot 3: enemy trumped
        assert_eq!(s.legal_moves(0), vec![false, true]);
    }

    #[test]
    fn hokom_no_suit_enemy_trumped_cannot_ascend_anything_legal() {
        // Hearts led. Enemy led K♥ (slot 1), friend played A♥ (slot 2),
        // enemy trumped with J♠ (slot 3, highest). Player has no Hearts, only 7♠.
        // Can't ascend over J♠ → anything legal.
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Hearts, Rank::King));  // slot 1: enemy
        s.current_trick.push(c(Suit::Hearts, Rank::Ace));   // slot 2: friend
        s.current_trick.push(c(Suit::Spades, Rank::Jack));  // slot 3: enemy trumped with Jack
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_no_suit_has_trump_enemy_not_trumped_must_play_trump() {
        // Hearts led (non-trump). Enemy led K♥ (slot 1), friend played 8♥ (slot 2).
        // Player has no Hearts, enemy didn't trump → must play trump.
        // [A♣=false, 7♠=true]
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Hearts, Rank::King));  // slot 1: enemy
        s.current_trick.push(c(Suit::Hearts, Rank::Eight)); // slot 2: friend
        assert_eq!(s.legal_moves(0), vec![false, true]);
    }

    #[test]
    fn hokom_no_suit_no_trump_anything_legal() {
        // Player void in led suit AND has no trump at all → play anything.
        // Friend led K♥ (slot 2), enemy played 8♥ (slot 3).
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Diamonds, Rank::King)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Hearts, Rank::King));  // slot 2: friend
        s.current_trick.push(c(Suit::Hearts, Rank::Eight)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_no_suit_friend_winning_no_need_to_trump() {
        // Enemy led K♥ (slot 1), friend played A♥ (slot 2, winning), enemy played 7♥ (slot 3).
        // Player has no suit; friend already winning → anything legal.
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Hearts, Rank::King));   // slot 1: enemy
        s.current_trick.push(c(Suit::Hearts, Rank::Ace));    // slot 2: friend (winning)
        s.current_trick.push(c(Suit::Hearts, Rank::Seven));  // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 5. Edge case: trick_length == 1 ────────────────────────────────────────

    #[test]
    fn bug_trick_length_1_panics_on_unwrap() {
        // Enemy led 7♥ (slot 1); player is 2nd to play.
        // With absolute-slot Trick, enemy_played is non-empty → no panic.
        let mut s = state(
            vec![c(Suit::Hearts, Rank::Ace)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(1);
        s.current_trick.push(c(Suit::Hearts, Rank::Seven)); // slot 1: enemy
        assert_eq!(s.legal_moves(0), vec![true]);
    }

    // ─── 6. Edge cases ──────────────────────────────────────────────────────────

    #[test]
    fn single_card_hand_always_legal() {
        // Player has exactly one card — always must play it.
        // Friend led K♥ (slot 2), enemy played 8♥ (slot 3). No suit, no trump → [true].
        let mut s = state(
            vec![c(Suit::Clubs, Rank::Seven)],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Hearts);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Hearts, Rank::King));  // slot 2: friend
        s.current_trick.push(c(Suit::Hearts, Rank::Eight)); // slot 3: enemy
        assert_eq!(s.legal_moves(0), vec![true]);
    }

    #[test]
    fn hokom_all_cards_are_trump_trump_trick_ascend() {
        // All player cards are trump, trump trick.
        // Friend led 7♠ (slot 2), enemy played 8♠ (slot 3, strength 10).
        // All player cards beat 8♠ → ascending_mask = [true, true, true].
        let mut s = state(
            vec![
                c(Suit::Spades, Rank::Ace),
                c(Suit::Spades, Rank::King),
                c(Suit::Spades, Rank::Nine),
            ],
            Some(Suit::Spades),
            true,
        );
        s.current_suit = Some(Suit::Spades);
        s.current_trick.set_player(2);
        s.current_trick.push(c(Suit::Spades, Rank::Seven)); // slot 2: friend
        s.current_trick.push(c(Suit::Spades, Rank::Eight)); // slot 3: enemy (strength 10)
        assert_eq!(s.legal_moves(0), vec![true, true, true]);
    }

    #[test]
    fn sun_no_trick_all_legal_full_hand() {
        // Full 8-card hand, sun game, not in trick
        let hand = vec![
            c(Suit::Spades, Rank::Ace),   c(Suit::Spades, Rank::King),
            c(Suit::Hearts, Rank::Ace),   c(Suit::Hearts, Rank::King),
            c(Suit::Clubs, Rank::Ace),    c(Suit::Clubs, Rank::King),
            c(Suit::Diamonds, Rank::Ace), c(Suit::Diamonds, Rank::King),
        ];
        let s = state(hand, None, false);
        assert_eq!(s.legal_moves(0), vec![true; 8]);
    }
}