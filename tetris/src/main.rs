//! # Tetris
//! Entry point of the program

use engine::Engine;
use game::Game;
use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let game = Game::new();
    let mut engine = Engine::new(game);
    
    engine.run()
}
