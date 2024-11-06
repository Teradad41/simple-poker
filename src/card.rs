use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: i32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self.rank {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        write!(f, "{} {}", self.suit, rank_str)
    }
}
