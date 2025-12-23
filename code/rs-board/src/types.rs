use std::slice::SliceIndex;

pub const ROWS: usize = 7;
pub const COLS: usize = 11;

pub const PLRS: [Player; 2] = [Player::A, Player::B];
pub const KINDS: [(u8, PieceKind); 5] = [
    (0, PieceKind::Brute(BruteSide::Left)),
    // 1
    // 2
    // 3
    (4, PieceKind::Core),
    (5, PieceKind::Monarch),
    (6, PieceKind::Tank),
    // 7
    // 8
    // 9
    (10, PieceKind::Brute(BruteSide::Right)),
];

#[derive(Clone, Copy, Debug)]
pub enum BruteSide {
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
    Brute(BruteSide),
    Tank,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub plr: Player,
    pub kind: PieceKind,
    pub alive: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct Coordinate(usize);

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Self(Self::idx(x, y))
    }

    pub fn x(&self) -> u8 {
        (self.0 % COLS) as u8
    }

    pub fn y(&self) -> u8 {
        (self.0 / COLS) as u8
    }

    pub fn xy(&self) -> (u8, u8) {
        (self.x(), self.y())
    }

    #[inline]
    pub const fn idx(x: u8, y: u8) -> usize {
        // row-major indexing: row * COLS + column
        (y as usize) * COLS + (x as usize)
    }
}

impl Into<usize> for Coordinate {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MoveKind {
    Move { to: Coordinate },
    Attack { target: Coordinate },
    MoveAndAttack { to: Coordinate, target: Coordinate },
}

#[derive(Clone, Copy, Debug)]
pub struct Move(Piece, MoveKind);

pub mod board {
    use std::ops::Index;

    use super::*;
    type RawBoard = ([Option<u8>; ROWS * COLS], [Piece; 10]);

    const fn default_board() -> RawBoard {
        let mut board: RawBoard = (
            [None; ROWS * COLS],
            [Piece {
                alive: false,
                kind: PieceKind::Core,
                plr: Player::A,
            }; 10],
        );
        let mut i = 0u8;

        while i < 10 {
            let plr_i = i / 5;
            let plr = PLRS[plr_i as usize];
            let (x, kind) = KINDS[i as usize % 5];
            let y = plr_i * (ROWS as u8 - 1);
            board.0[Coordinate::idx(x, y)] = Some(i as u8);
            board.1[i as usize] = Piece {
                alive: true,
                kind,
                plr,
            };

            i += 1;
        }

        board
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Board {
        positions: [Option<u8>; ROWS * COLS],
        pieces: [Piece; 10],
    }

    impl Board {
        fn piece_id(
            Piece {
                plr,
                kind,
                alive: _,
            }: &Piece,
        ) -> usize {
            let plr_i = match plr {
                Player::A => 0usize,
                Player::B => 1usize,
            };
            let kind_i = match kind {
                PieceKind::Brute(BruteSide::Left) => 0usize,
                PieceKind::Core => 1usize,
                PieceKind::Monarch => 2usize,
                PieceKind::Tank => 3usize,
                PieceKind::Brute(BruteSide::Right) => 4usize,
            };
            plr_i * 5 + kind_i
        }
    }

    impl Board {
        pub fn new() -> Self {
            let (positions, pieces) = default_board();
            Self { positions, pieces }
        }

        pub fn get_piece(&self, Coordinate(idx): Coordinate) -> Option<&Piece> {
            self.positions[idx].map(|piece| &self.pieces[piece as usize])
        }

        pub fn get_coord(&self, piece: Piece) -> Option<Coordinate> {
            let piece_id = Self::piece_id(&piece);
            self.positions.iter().enumerate().find_map(|(idx, &pos)| {
                if pos == Some(piece_id as u8) {
                    Some(Coordinate(idx))
                } else {
                    None
                }
            })
        }

        pub fn set(&mut self, Coordinate(idx): Coordinate, piece: Piece) {
            let piece_id = Self::piece_id(&piece);
            self.positions[idx] = Some(piece_id as u8);
            self.pieces[piece_id as usize] = piece;
        }
    }
}
