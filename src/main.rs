mod card;
mod deck;
mod game;
mod hand;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.play();
}
