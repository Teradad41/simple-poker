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

    // TODO: Implement this method
    pub fn evaluate(&self) -> HandRank {
        let rank_count = self.count_rank();

        if rank_count == 2 {
            return HandRank::OnePair;
        }

        HandRank::HighCard
    }

    fn count_rank(&self) -> usize {
        2
    }

    pub fn display(&self) {
        println!("===== Hand =====");
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}: {}", i + 1, card);
        }
    }
}
