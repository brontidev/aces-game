use crate::types::{Player, board::Board};

pub struct State {
    turn: Player,
    board: Board,
}

impl State {
    pub fn new() -> Self {
        State {
            turn: Player::A,
            board: Board::new(),
        }
    }
}
