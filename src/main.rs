#![feature(conservative_impl_trait)]

extern crate rand;
extern crate cursive;

mod board;
mod ui;

use cursive::Cursive;
use cursive::views::{Button, Dialog, LinearLayout};

use ui::BoardView;

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(
        Dialog::new()
            .title("b2048")
            .padding((2, 2, 1, 1))
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New game   ", new_game))
                    .child(Button::new_raw(" Best scores ", |s| {
                        s.add_layer(Dialog::info("Not yet!").title("Scores"))
                    }))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn new_game(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("b2048")
            .padding((2, 2, 1, 1))
            .content(
                LinearLayout::horizontal()
                    .child(BoardView::new(4)),
            )
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );
}