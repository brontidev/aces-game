use crate::types::{
    BruteSide, Coordinate, KINDS, Move, MoveKind, Piece, PieceKind, Player, board::Board,
};

pub struct Game {
    // turn: Player,
    board: Board,
}

pub struct TurnState {
    player: Player,
    possible_moves: Vec<Move>,
}

fn add(lhs: u8, rhs: i8) -> Option<u8> {
    if rhs >= 0 {
        lhs.checked_add(rhs as u8)
    } else {
        lhs.checked_sub(rhs.abs() as u8)
    }
}

fn add_coord(coord: Coordinate, (dx, dy): (i8, i8)) -> Option<Coordinate> {
    match (add(coord.x(), dx), add(coord.y(), dy)) {
        (Some(x), Some(y)) => Some(Coordinate::new(x, y)),
        _ => None,
    }
}

impl Game {
    fn check_coord(&self, coord: Coordinate) -> (bool, bool) {
        let is_valid = Board::is_valid_coord(coord);
        let is_empty = self.board.get_piece(coord).is_none();
        (is_valid, is_empty)
    }

    fn check_candidates(
        &self,
        piece_coord: Coordinate,
        candidates: Vec<(i8, i8)>,
    ) -> (Vec<((i8, i8), Coordinate)>, Vec<((i8, i8), Coordinate)>) {
        candidates
            .iter()
            .filter_map(|dxy| {
                add_coord(piece_coord, *dxy)
                    .map(|coord| (*dxy, coord, self.check_coord(coord)))
                    .and_then(|(dxy, coord, (is_valid, is_empty))| {
                        is_valid.then_some((dxy, coord, is_empty))
                    })
            })
            .fold(
                (Vec::new(), Vec::new()),
                |(mut empty, mut non_empty), (dxy, coord, is_empty)| {
                    if is_empty {
                        empty.push((dxy, coord));
                    } else {
                        non_empty.push((dxy, coord));
                    }
                    (empty, non_empty)
                },
            )
    }

    fn get_possible_moves(&self, plr: Player) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        for (_, kind) in KINDS {
            let piece = Piece {
                plr,
                kind,
                alive: true,
            };
            let piece_coord = self.board.get_coord(&piece);
            if let Some(piece_coord) = piece_coord {
                match kind {
                    PieceKind::Core => {
                        let candidates = vec![(0i8, -1i8), (0, 1), (1, 0), (-1, 0)];
                        let (coords, _) = self.check_candidates(piece_coord, candidates);
                        for (_, coord) in coords {
                            possible_moves.push(Move(piece, MoveKind::Move { to: coord }));
                        }
                    }
                    PieceKind::Monarch => {
                        let candidates = vec![
                            (0i8, -1i8),
                            (0, 1),
                            (1, 0),
                            (-1, 0),
                            (-1, -1),
                            (-1, 1),
                            (1, -1),
                            (1, 1),
                        ];
                        let (coords, _) = self.check_candidates(piece_coord, candidates);
                        for (_, coord) in coords {
                            possible_moves.push(Move(piece, MoveKind::Move { to: coord }));
                        }
                    }
                    PieceKind::Brute(_) => {
                        let candidates = vec![(0i8, -1i8), (0, 1), (1, 0), (-1, 0)];
                        let (move_coords, attack_coords) =
                            self.check_candidates(piece_coord, candidates);
                        for (_, coord) in move_coords {
                            possible_moves.push(Move(piece, MoveKind::Move { to: coord }));
                        }
                        for (_, coord) in attack_coords {
                            possible_moves.push(Move(piece, MoveKind::Attack { target: coord }));
                        }
                    }
                    PieceKind::Tank => {
                        let candidates = vec![(0i8, -1i8), (0, 1), (1, 0), (-1, 0)];
                        let (move_coords, attack_coords) =
                            self.check_candidates(piece_coord, candidates);
                        for (_, coord) in move_coords {
                            possible_moves.push(Move(piece, MoveKind::Move { to: coord }));
                        }

                        for (dxy, coord) in attack_coords {
                            let move_to =
                                add_coord(piece_coord, dxy).and_then(|coord| add_coord(coord, dxy));
                            if let Some(move_to) = move_to {
                                let (is_valid, is_empty) = self.check_coord(move_to);
                                if is_valid && is_empty {
                                    possible_moves.push(Move(
                                        piece,
                                        MoveKind::MoveAndAttack {
                                            to: move_to,
                                            target: coord,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        possible_moves
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
        }
    }
}
