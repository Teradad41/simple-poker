use rand::seq::SliceRandom;

use crate::card::{Card, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

const SUITS: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
const DECK_SIZE: usize = 52;

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(DECK_SIZE);
        for suit in SUITS {
            for rank in 1..=13 {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
