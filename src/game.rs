use crate::deck::Deck;
use crate::hand::Hand;
use std::io;

pub struct Game {
    deck: Deck,
    hand: Hand,
}

impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(),
            hand: Hand::new(),
        }
    }

    pub fn play(&mut self) {
        self.deck.shuffle();

        for _ in 0..5 {
            if let Some(card) = self.deck.draw() {
                self.hand.add_card(card);
            }
        }

        self.hand.display();
        self.handle_card_exchange();

        let hand_rank = self.hand.evaluate();
        println!("\n{}!", hand_rank.to_string_ja());
    }

    fn handle_card_exchange(&mut self) {
        println!("\n入れ替えたいカードの番号を入力してください（例: 1 2 3）");
        println!("何も入力せずに Enter を押すと入れ替えを終了します");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if !input.trim().is_empty() {
            let numbers: Vec<usize> = input
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            for number in numbers {
                if number > 0 && number <= 5 {
                    if let Some(new_card) = self.deck.draw() {
                        self.hand.cards[number - 1] = new_card;
                    }
                }
            }

            self.hand.sort();
            self.hand.display();
        }
    }
}
