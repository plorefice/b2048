use board::{self, Board, Error};

use cursive::Printer;
use cursive::direction::Direction;
use cursive::theme::{Color, ColorStyle};
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::views::Dialog;
use cursive::event::{Event, EventResult, Key};

pub struct BoardView {
    board: Board
}

impl BoardView {
    pub fn new(size: usize) -> Self {
        BoardView {
            board: Board::new(size),
        }
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        for (n, tile) in self.board.get_tiles().enumerate() {
            let i = (n % self.board.size()) * 7;
            let j = (n / self.board.size()) * 4;

            let (front, back) = match *tile {
                2    => (Color::Rgb(0x00, 0x00, 0x00), Color::Rgb(0xee, 0xe4, 0xda)),
                4    => (Color::Rgb(0x00, 0x00, 0x00), Color::Rgb(0xed, 0xe0, 0xc8)),
                8    => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xf2, 0xb1, 0x79)),
                16   => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xf5, 0x95, 0x63)),
                32   => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xf6, 0x7c, 0x5f)),
                64   => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xf6, 0x5e, 0x3b)),
                128  => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xed, 0xcf, 0x72)),
                256  => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xed, 0xcc, 0x61)),
                512  => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xed, 0xc8, 0x50)),
                1024 => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xed, 0xc5, 0x3f)),
                2048 => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0xed, 0xc2, 0x2e)),
                _    => (Color::Rgb(0xf9, 0xf6, 0xf2), Color::Rgb(0x3c, 0x3a, 0x32)),
            };

            if *tile != 0 {
                printer.with_color(ColorStyle::Custom{ front, back }, |printer| {
                    printer.print((i+1, j+1), "      ");
                    printer.print((i+1, j+2), &format!(" {:^4} ", *tile));
                    printer.print((i+1, j+3), "      ");
                });
            }
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let dir = match event {
            Event::Key(Key::Up)    => board::Direction::Up,
            Event::Key(Key::Down)  => board::Direction::Down,
            Event::Key(Key::Left)  => board::Direction::Left,
            Event::Key(Key::Right) => board::Direction::Right,
            _                      => return EventResult::Ignored,
        };

        match self.board.swipe(dir) {
            Err(Error::BoardFull) => EventResult::with_cb(|s| {
                s.add_layer(Dialog::text("Game over!")
                    .button("Ok", |s| s.pop_layer()));
            }),
            _ => EventResult::Consumed(None),
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2 { x: 7 * self.board.size() + 1, y: 4 * self.board.size() + 1 }
    }
}
