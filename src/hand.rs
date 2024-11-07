use crate::card::Card;

pub enum HandRank {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandRank {
    pub fn to_string_ja(&self) -> &str {
        match self {
            HandRank::RoyalFlush => "ロイヤルフラッシュ",
            HandRank::StraightFlush => "ストレートフラッシュ",
            HandRank::FourOfAKind => "フォーカード",
            HandRank::FullHouse => "フルハウス",
            HandRank::Flush => "フラッシュ",
            HandRank::Straight => "ストレート",
            HandRank::ThreeOfAKind => "スリーカード",
            HandRank::TwoPair => "ツーペア",
            HandRank::OnePair => "ワンペア",
            HandRank::HighCard => "役なし...",
        }
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

const HAND_SIZE: usize = 5;

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::with_capacity(HAND_SIZE),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.sort();
    }

    pub fn sort(&mut self) {
        self.cards.sort_by(|a, b| a.rank.cmp(&b.rank))
    }

    pub fn display(&self) {
        println!("===== Hand =====");
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}: {}", i + 1, card);
        }
    }

    pub fn evaluate(&self) -> HandRank {
        let is_flush = self.is_flush();
        let is_straight = self.is_straight();

        if is_flush && is_straight && self.is_royal() {
            return HandRank::RoyalFlush;
        }
        if is_flush && is_straight {
            return HandRank::StraightFlush;
        }

        let rank_counts = self.count_ranks();
        if rank_counts.contains(&4) {
            return HandRank::FourOfAKind;
        }
        if rank_counts.contains(&3) && rank_counts.contains(&2) {
            return HandRank::FullHouse;
        }
        if is_flush {
            return HandRank::Flush;
        }
        if is_straight {
            return HandRank::Straight;
        }
        if rank_counts.contains(&3) {
            return HandRank::ThreeOfAKind;
        }
        if rank_counts.iter().filter(|&&count| count == 2).count() == 2 {
            return HandRank::TwoPair;
        }
        if rank_counts.contains(&2) {
            return HandRank::OnePair;
        }
        HandRank::HighCard
    }

    fn is_flush(&self) -> bool {
        let first_suit = self.cards[0].suit;
        self.cards.iter().all(|card| card.suit == first_suit)
    }

    fn is_straight(&self) -> bool {
        for i in 0..self.cards.len() - 1 {
            if self.cards[i + 1].rank - self.cards[i].rank != 1 {
                return false;
            }
        }
        true
    }

    fn is_royal(&self) -> bool {
        self.cards.first().unwrap().rank == 1 && self.cards.last().unwrap().rank == 13
    }

    fn count_ranks(&self) -> Vec<i32> {
        let mut counts = vec![0; 14];
        for card in &self.cards {
            counts[card.rank as usize] += 1;
        }
        counts
    }
}
