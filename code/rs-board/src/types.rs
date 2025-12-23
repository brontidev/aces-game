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

        pub fn where_core(&self, player: Player) -> &Coordinate {
            match player {
                Player::A => &self.core_positions.0,
                Player::B => &self.core_positions.1,
            }
        }

        pub fn get(&self, Coordinate(x, y): Coordinate) -> &Option<Piece> {
            &self.positions[idx(x, y)]
        }

        pub fn set(&mut self, Coordinate(x, y): Coordinate, piece: Option<Piece>) {
            if let Some(Piece(player, PieceKind::Core)) = piece {
                match player {
                    Player::A => self.core_positions.0 = Coordinate(x, y),
                    Player::B => self.core_positions.1 = Coordinate(x, y),
                }
            };
            self.positions[idx(x, y)] = piece;
        }
    }

    impl Index<Coordinate> for Board {
        type Output = Option<Piece>;

        fn index(&self, coordinate: Coordinate) -> &Self::Output {
            self.get(coordinate)
        }
    }

    impl Index<Player> for Board {
        type Output = Coordinate;

        fn index(&self, player: Player) -> &Self::Output {
            &self.where_core(player)
        }
    }
}
