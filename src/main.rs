extern crate rand;
extern crate cursive;

mod game;
use game::{Game, Direction, Error};

use std::io::{self, Write};

fn main() {
    let mut game = Game::new(4);

    while !game.over() {
        let mut input = String::new();

        print!("\nScore: {}\n\n{}\n> ", game.score(), game);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let res = match input.trim() {
            "u" | "U" => game.swipe(Direction::Up),
            "d" | "D" => game.swipe(Direction::Down),
            "l" | "L" => game.swipe(Direction::Left),
            "r" | "R" => game.swipe(Direction::Right),
            _         => { eprintln!("\nInvalid input!"); Ok(()) },
        };

        match res {
            Ok(_) | Err(Error::InvalidMove) => continue,
            Err(_) => break,
        };
    }

    println!("\n{}\nGame over! Your final score is: {}\n", game, game.score());
}
