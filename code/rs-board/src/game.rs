use crate::types::{BruteSide, KINDS, Move, Piece, PieceKind, Player, board::Board};

pub struct Game {
    // turn: Player,
    board: Board,
}

pub struct TurnState {
    player: Player,
    possible_moves: Vec<Move>,
}

impl Game {
    fn get_possible_moves(&self, plr: Player) {
        for (_, kind) in KINDS {
            let piece = Piece { plr, kind, alive: true };
            match kind {
                PieceKind::Core => todo!(),
                PieceKind::Monarch => todo!(),
                PieceKind::Brute(BruteSide::Left) => todo!(),
                PieceKind::Brute(BruteSide::Right) => todo!(),
                PieceKind::Tank => todo!(),
            }
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }
}
