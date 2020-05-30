use rand;
pub const CARD_VALUE: [&str; 13] = [
    "Ace",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Jack",
    "Queen",
    "King",
];

pub const CARD_SUIT: [&str; 4] = [
    "Hearts",
    "Diamonds",
    "Spades",
    "Clubs"
];

#[derive(Debug)]
pub struct Card {
    value: String,
    suit: String,
}

#[derive(Debug)]
pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();
        for suit in CARD_SUIT.to_vec() {
            for value in CARD_VALUE.to_vec() {
                deck.push( Card { value: value.into(), suit: suit.into()});
            }
        }
        Deck { deck }
    }

    pub fn extend(&mut self, new_deck: Vec<Card>) {
        self.deck.extend(new_deck);
    }

    pub fn shuffle(&mut self) {
        for _ in 0..1000 {
            let index_a = rand::random::<usize>() % 52;
            let index_b = rand::random::<usize>() % 52;
            self.deck.swap(index_a, index_b);
        }
    }
}