extern crate rand;
extern crate nalgebra;

mod game;
use game::{Game, Direction, Error};

use std::io::{self, Write};

fn main() {
    let mut game = Game::new(4);

    while !game.is_over() {
        let mut input = String::new();

        print!("\nScore: {}\n\n{}\n> ", game.score(), game);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let res = match input.trim() {
            "u" | "U" => game.strafe(Direction::Up),
            "d" | "D" => game.strafe(Direction::Down),
            "l" | "L" => game.strafe(Direction::Left),
            "r" | "R" => game.strafe(Direction::Right),
            _         => { eprintln!("\nInvalid input!"); Ok(()) },
        };

        match res {
            Ok(_) | Err(Error::NoMoveOccurred) => continue,
            Err(_) => break,
        };
    }

    println!("\n{}\nGame over! Your final score is: {}\n", game, game.score());
}
