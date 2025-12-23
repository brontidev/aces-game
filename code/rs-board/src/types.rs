pub const ROWS: usize = 7;
pub const COLS: usize = 11;

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Player {
    A,
    B,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceKind {
    Core,
    Monarch,
    Brute(Side),
    Tank,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece(Player, PieceKind);
#[derive(Clone, Copy, Debug)]
pub struct Coordinate(usize, usize);

pub enum MoveKind {
    Move { to: Coordinate },
    Attack { target: Coordinate },
    MoveAndAttack { to: Coordinate, target: Coordinate },
}

pub struct Move(Piece, MoveKind);

pub mod board {
    use super::*;
    type RawBoard = [Option<Piece>; ROWS * COLS];

    #[inline]
    const fn idx(x: usize, y: usize) -> usize {
        y * COLS + x
    }

    const fn default_board() -> RawBoard {
        const PLRS: [Player; 2] = [Player::A, Player::B];
        const KINDS: [(usize, PieceKind); 5] = [
            (0, PieceKind::Brute(Side::Left)),
            // 1
            // 2
            // 3
            (4, PieceKind::Core),
            (5, PieceKind::Monarch),
            (6, PieceKind::Tank),
            // 7
            // 8
            // 9
            (10, PieceKind::Brute(Side::Right)),
        ];
        let mut board: RawBoard = [None; ROWS * COLS];
        let mut i = 0usize;

        while i < 10 {
            let plr_i = i / 5;
            let plr = PLRS[plr_i];
            let (x, kind) = KINDS[i % 5];
            let board_index = idx(x, plr_i * (ROWS - 1));
            board[board_index] = Some(Piece(plr, kind));
            i += 1;
        }

        board
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Board {
        positions: RawBoard,
        core_positions: (Coordinate, Coordinate),
    }

    impl Board {
        pub fn new() -> Self {
            let positions = default_board();
            Self {
                core_positions: (Coordinate(5, 0), Coordinate(5, ROWS - 1usize)),
                positions,
            }
        }
    }
}
