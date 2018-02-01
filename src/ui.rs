use board::{self, Board, Error};

use cursive::Printer;
use cursive::direction::Direction;
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
        format!("{}", self.board)
            .lines()
            .enumerate()
            .for_each(|(i,l)| printer.print((0,i), l));
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
        let (x,y) = self.board.size_hint();
        Vec2 { x, y }
    }
}
