use crate::game::deck::{distribute_hands_from_shuffled_deck, Card, Deck,Suit};
use crate::game::trick::card_strength;

// Sun rules
// R1: Must play suit if you have it, otherwise anything else.
// Hokoom
// R1: Must always play above a trump if possible.
pub struct GameState {
    // Card meta data
    pub hands : [Vec<Card> ; 4],
    pub current_trick : Vec<Card>,
    // Game meta data
    pub previous_tricks : Vec<[Card ; 4]>,
    pub trump_suit : Option <Suit>,
    pub in_trick : bool,
    // Trick meta data
    pub current_player : u64,
    pub current_suit : Option <Suit>,
}

impl GameState {
    pub fn new(
        hands : Option <[Vec<Card> ; 4]>,
        trump_suit : Option <Suit>,
        current_player : Option <u64>,
        seed : Option<u64>
    ) -> GameState {
        use crate::game::deck::{Card, };
        match hands { // Start a game
            Some(hands) => {
                GameState {
                    hands : hands,
                    current_trick : Vec::<Card>::new(),
                    previous_tricks : Vec::new(),
                    trump_suit : trump_suit,
                    in_trick : false,
                    current_player : match current_player {Some(c) => c,_=>0},
                    current_suit : None,
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
                let current_player = match current_player {Some(p) => p, None => 0};
                GameState {
                    hands: hands,
                    current_trick : Vec::<Card>::new(),
                    previous_tricks : Vec::new(),
                    trump_suit : trump_suit,
                    in_trick : false,
                    current_player : current_player,
                    current_suit : None,
                }

            }
        }
    }
    pub fn legal_moves (&self, player : usize) -> Vec<bool> {
        let n = self.hands[player].len();
        if !self.in_trick { // First move of the trick
            return vec![true ; n];
        }

        let trick_length  =self.current_trick.len();
        let player_cards = &self.hands[player];
        // all enemy related
        // len == 3, enemy = {2,0}, friend {1}
        // len == 2, enemy = {1}  , friend {0}
        // len == 1, enemy = {0}
        let enemy_team = (trick_length+1) % 2;
        let enemy_played : Vec::<Card> = self.current_trick
            .iter()
            .enumerate() // (index, card)
            .filter (|(idx, _)| idx % 2 == enemy_team)// Keep enemy cards
            .map(|(_, card)| *card) // remove indices
            .collect();
        let enemy_strongest : u64 = enemy_played
            .iter()
            .map(|c| card_strength(c, self.trump_suit))
            .max()
            .unwrap();
        // teammate related
        let friend_card =
            if trick_length > 1
            {Some(self.current_trick[trick_length-2])}
            else {None};
        let friend_won  = friend_card
            .map_or(false, |f| card_strength(&f, self.trump_suit)  > enemy_strongest);
        // All suit related
        let trick_suit= self.current_trick[0].suit;
        let has_suit  = player_cards.iter().any(|c| c.suit == trick_suit);
        let suit_mask      =  player_cards.iter().map(|c| c.suit == trick_suit).collect();
        let mapped_cards:Vec<u64>= player_cards.iter().map(|c| card_strength(c, self.trump_suit)).collect();
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
    pub fn apply(&self, action: Card) -> Result<GameState, &str> { todo!() }
    pub fn is_terminal(&self) -> Option<(u64, u64)> { todo!() }
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
        trick: Vec<Card>,
        trump: Option<Suit>,
        in_trick: bool,
    ) -> GameState {
        GameState {
            hands: [hand, vec![], vec![], vec![]],
            current_trick: trick,
            previous_tricks: vec![],
            trump_suit: trump,
            in_trick,
            current_player: 0,
            current_suit: None,
        }
    }

    // ─── 1. Not-in-trick ────────────────────────────────────────────────────────

    #[test]
    fn not_in_trick_all_legal_sun() {
        // Rule: first player to a trick can play anything
        let s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Hearts, Rank::Seven)],
            vec![],
            None,
            false,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn not_in_trick_all_legal_hokom() {
        let s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Hearts, Rank::Seven)],
            vec![],
            Some(Suit::Spades),
            false,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 2. Sun mode ────────────────────────────────────────────────────────────

    #[test]
    fn sun_has_suit_must_follow() {
        // R1 Sun: must follow suit if able
        // trick led with Spades; player has one Spade and one Heart
        let s = state(
            vec![c(Suit::Spades, Rank::King), c(Suit::Hearts, Rank::Ace)],
            vec![c(Suit::Spades, Rank::Seven), c(Suit::Hearts, Rank::Seven)],
            None,
            true,
        );
        // Only the Spade is legal
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn sun_has_suit_all_suit_cards_legal() {
        // All cards of the led suit are legal (not just highest)
        let s = state(
            vec![
                c(Suit::Spades, Rank::Seven),
                c(Suit::Spades, Rank::Ace),
                c(Suit::Hearts, Rank::Ace),
            ],
            vec![c(Suit::Spades, Rank::King), c(Suit::Hearts, Rank::Seven)],
            None,
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, true, false]);
    }

    #[test]
    fn sun_no_suit_anything_legal() {
        // R1 Sun: no suit in hand → anything goes
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Hearts, Rank::King)],
            vec![c(Suit::Spades, Rank::Seven), c(Suit::Diamonds, Rank::Seven)],
            None,
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 3. Hokom: trump-led trick (trick_suit == trump) ────────────────────────

    #[test]
    fn hokom_trump_trick_must_ascend_if_able() {
        // Friend plays A♠ (strong trump) to force 9♠ . Player has J♠ (beats it) and K♠...
        // Use: enemy plays 9♠, player has [J♠, K♠] one beats 9♠ → ascending_mask=[T,F]
        let s = state(
            vec![c(Suit::Spades, Rank::Jack), c(Suit::Spades, Rank::King)],
            // trick[0] = enemy (len=2, enemy_team=0, idx%2==0 → trick[0])
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Spades, Rank::Nine)],
            Some(Suit::Spades),
            true,
        );
        // Both player cards beat enemy's Seven of trump → must play ascending
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn hokom_trump_trick_cannot_ascend_play_any_trump() {
        // Enemy played Jack of trump (highest possible). Player has 9♠ and 7♠.
        // Neither beats Jack → can't ascend → suit_mask = all trumps = [T, T]
        let s = state(
            vec![c(Suit::Spades, Rank::Nine), c(Suit::Spades, Rank::Seven)],
            vec![c(Suit::Spades, Rank::Jack), c(Suit::Spades, Rank::Eight)],
            Some(Suit::Spades),
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_trump_trick_friend_winning_play_any_trump() {
        // trick_length == 3: trick[0]=enemy, trick[1]=friend, trick[2]=enemy
        // Friend played Jack (strongest), enemies played 7s → friend_won = true
        // Even though player can ascend, friend already winning → suit_mask
        let s = state(
            vec![c(Suit::Spades, Rank::Ace), c(Suit::Spades, Rank::Nine)],
            vec![
                c(Suit::Spades, Rank::Seven),  // enemy (idx 0, 0%2==1? len=3 → enemy_team=1)
                c(Suit::Spades, Rank::Jack),   // friend (idx 1)
                c(Suit::Spades, Rank::Eight),  // enemy (idx 2)
            ],
            Some(Suit::Spades),
            true,
        );
        // friend_won=true → no need to ascend → suit_mask = all trumps
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 4. Hokom: non-trump-led trick ──────────────────────────────────────────

    #[test]
    fn hokom_suit_led_has_suit_must_follow() {
        // Hearts led, trump=Spades. Player has one Heart and one Spade.
        // Must follow Hearts regardless of trump.
        let s = state(
            vec![c(Suit::Hearts, Rank::Seven), c(Suit::Spades, Rank::Ace)],
            vec![c(Suit::Hearts, Rank::King), c(Suit::Hearts, Rank::Eight)],
            Some(Suit::Spades),
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, false]);
    }

    #[test]
    fn hokom_no_suit_enemy_trumped_can_ascend_must_ascend() {
        // Hearts led. Player has no Hearts. Enemy played a trump (low trump).
        // Player has a higher trump → must ascend (over-trump).
        //
        // trick[1] is enemy (len=2, enemy_team=0 → idx%2==0 → trick[0])
        // Actually len=2 → enemy_team=0, enemy is trick[0].
        // trick[0] = Hearts (led by enemy), trick[1] = 7♠ (enemy trumped? No—trick[0] led)
        //
        // Restructure: trick[0]=Hearts (led), trick[1]=7♠ (enemy over-trumped)
        // enemy_team = 2%2 = 0 → enemy = idx%2==0 → trick[0] and trick[2]
        // But trick[0] is Hearts, not trump. We want enemy_trumped=true.
        // Use trick length 3: enemy_team=1, enemy=trick[1]
        // trick[0]=Hearts(led), trick[1]=7♠(enemy trumped), trick[2]=3Hearts(friend no suit)
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Ace)], // no Hearts, has high trump
            vec![
                c(Suit::Hearts, Rank::King),   // idx 0: enemy led with a king
                c(Suit::Hearts, Rank::Ace),  // idx 1: friend played up
                c(Suit::Spades, Rank::Seven),  // idx 2: enemy trumped with low trump
            ],
            Some(Suit::Spades),
            true,
        );
        // enemy_trumped=true, player A♠ > enemy 7♠, friend_won=false
        // → must play cards that beat enemy's 7♠
        // A♠ beats 7♠, A♣ does not (not trump)
        // ascending_mask: [false, true] (A♣ doesn't beat, A♠ beats)
        assert_eq!(s.legal_moves(0), vec![false, true]);
    }

    #[test]
    fn hokom_no_suit_enemy_trumped_cannot_ascend_anything_legal() {
        // Enemy trumped with Jack (highest). Player has no suit, and only low trump.
        // Can't ascend → anything legal.
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            vec![
                c(Suit::Hearts, Rank::King),
                c(Suit::Spades, Rank::Jack), // enemy trumped with Jack
                c(Suit::Hearts, Rank::Eight),
            ],
            Some(Suit::Spades),
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_no_suit_has_trump_enemy_not_trumped_must_play_trump() {
        // Hearts led (non-trump). Player has no Hearts. Enemy didn't trump.
        // Player has trump → must play trump (force opponent to ruff).
        // trick length 2: enemy_team=0, enemy=trick[0]
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            vec![
                c(Suit::Hearts, Rank::Eight),  // trick[1]: enemy, no trump
                c(Suit::Hearts, Rank::King),   // trick[0]: friend
            ],
            Some(Suit::Spades),
            true,
        );
        // enemy_trumped=false, has_trump=true, friend_won=false → trump_mask
        // [A♣=false, 7♠=true]
        assert_eq!(s.legal_moves(0), vec![false, true]);
    }

    #[test]
    fn hokom_no_suit_no_trump_anything_legal() {
        // Player void in led suit AND has no trump at all → play anything
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Diamonds, Rank::King)],
            vec![
                c(Suit::Hearts, Rank::King),
                c(Suit::Hearts, Rank::Eight),
            ],
            Some(Suit::Spades),
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    #[test]
    fn hokom_no_suit_friend_winning_no_need_to_trump() {
        // trick_length=3: friend is trick[1], friend winning.
        // Player has no suit. Even with trump available, friend already winning.
        // → anything legal (falls through to vec![true; n])
        let s = state(
            vec![c(Suit::Clubs, Rank::Ace), c(Suit::Spades, Rank::Seven)],
            vec![
                c(Suit::Hearts, Rank::King),   // idx 0: enemy (enemy_team=1 → idx%2==1 → trick[1])
                c(Suit::Hearts, Rank::Ace),    // idx 1: friend (highest non-trump)
                c(Suit::Hearts, Rank::Seven),  // idx 2: enemy
            ],
            Some(Suit::Spades),
            true,
        );
        // friend played A♥ (strongest non-trump), friend_won=true
        // no suit, friend won → anything legal
        assert_eq!(s.legal_moves(0), vec![true, true]);
    }

    // ─── 5. Bug: panic on trick_length == 1 in hokom ───────────────────────────

    #[test]
    fn bug_trick_length_1_panics_on_unwrap() {
        // When trick_length==1 (2nd player playing), enemy_team=1,
        // filter idx%2==1 on [0] → empty enemy_played → .max().unwrap() panics.
        // This test documents the bug. Fix: handle empty enemy_played gracefully.
        let s = state(
            vec![c(Suit::Hearts, Rank::Ace)],
            vec![c(Suit::Hearts, Rank::Seven)], // only 1 card in trick
            Some(Suit::Spades),
            true,
        );
        assert_eq!(s.legal_moves(0), vec![true]);
    }

    // ─── 6. Edge cases ──────────────────────────────────────────────────────────

    #[test]
    fn single_card_hand_always_legal() {
        // Player has exactly one card — always must play it
        let s = state(
            vec![c(Suit::Clubs, Rank::Seven)],
            vec![c(Suit::Hearts, Rank::King), c(Suit::Hearts, Rank::Eight)],
            Some(Suit::Spades),
            true,
        );
        // No suit, no trump → anything → [true]
        assert_eq!(s.legal_moves(0), vec![true]);
    }

    #[test]
    fn hokom_all_cards_are_trump_trump_trick_ascend() {
        // All player cards are trump, trump trick.
        // Enemy played low trump → all player's cards can ascend
        let s = state(
            vec![
                c(Suit::Spades, Rank::Ace),
                c(Suit::Spades, Rank::King),
                c(Suit::Spades, Rank::Nine),
            ],
            vec![c(Suit::Spades, Rank::Seven), c(Suit::Spades, Rank::Eight)],
            Some(Suit::Spades),
            true,
        );
        // enemy played 7♠, all player cards beat it
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
        let s = state(hand, vec![], None, false);
        assert_eq!(s.legal_moves(0), vec![true; 8]);
    }
}