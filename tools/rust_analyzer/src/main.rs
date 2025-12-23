use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use std::time::Instant;

const ROWS: usize = 7;
const COLS: usize = 11;
const BOARD_SIZE: usize = ROWS * COLS;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PieceKind {
    Core,
    Monarch,
    BruteL,
    BruteR,
    Tank,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Piece {
    owner: char, // 'A' or 'B'
    kind: PieceKind,
}

#[derive(Clone, Debug)]
struct Move {
    actor: usize,
    kind: &'static str, // "move", "brute_capture", "dash"
    dest: Option<usize>,
    captured: Option<usize>,
}

#[derive(Clone, Debug)]
struct State {
    board: [Option<Piece>; BOARD_SIZE],
    turn: char,
    last_core_moved_by: Option<char>,
}

impl Default for State {
    fn default() -> Self {
        let mut board: [Option<Piece>; BOARD_SIZE] = array_init::array_init(|_| None);
        // Player A top row (row 0)
        let row_a = 0;
        board[idx(row_a, 0)] = Some(Piece { owner: 'A', kind: PieceKind::BruteL });
        board[idx(row_a, 4)] = Some(Piece { owner: 'A', kind: PieceKind::Monarch });
        board[idx(row_a, 5)] = Some(Piece { owner: 'A', kind: PieceKind::Core });
        board[idx(row_a, 6)] = Some(Piece { owner: 'A', kind: PieceKind::Tank });
        board[idx(row_a, 10)] = Some(Piece { owner: 'A', kind: PieceKind::BruteR });
        // Player B bottom row (row 6)
        let row_b = ROWS - 1;
        board[idx(row_b, 0)] = Some(Piece { owner: 'B', kind: PieceKind::BruteL });
        board[idx(row_b, 4)] = Some(Piece { owner: 'B', kind: PieceKind::Monarch });
        board[idx(row_b, 5)] = Some(Piece { owner: 'B', kind: PieceKind::Core });
        board[idx(row_b, 6)] = Some(Piece { owner: 'B', kind: PieceKind::Tank });
        board[idx(row_b, 10)] = Some(Piece { owner: 'B', kind: PieceKind::BruteR });
        State { board, turn: 'A', last_core_moved_by: None }
    }
}

fn idx(r: usize, c: usize) -> usize {
    r * COLS + c
}

fn rc(i: usize) -> (usize, usize) {
    (i / COLS, i % COLS)
}

impl State {
    fn on_board_pos(r: isize, c: isize) -> bool {
        r >= 0 && r < ROWS as isize && c >= 0 && c < COLS as isize
    }

    fn are_adjacent_pos(a: usize, b: usize) -> bool {
        let (ar, ac) = rc(a);
        let (br, bc) = rc(b);
        let dr = (ar as isize - br as isize).abs();
        let dc = (ac as isize - bc as isize).abs();
        (dr <= 1) && (dc <= 1) && !(dr == 0 && dc == 0)
    }

    fn find_piece(&self, owner: char, kind: PieceKind) -> Option<usize> {
        for i in 0..BOARD_SIZE {
            if let Some(p) = self.board[i] {
                if p.owner == owner && p.kind == kind {
                    return Some(i);
                }
            }
        }
        None
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for i in 0..BOARD_SIZE {
            if let Some(piece) = self.board[i] {
                if piece.owner != self.turn { continue }
                let (r, c) = rc(i);
                match piece.kind {
                    PieceKind::Monarch => {
                        let deltas = [ (0isize,-1), (0,1), (-1,-1), (-1,1), (1,-1), (1,1) ];
                        for (dr,dc) in deltas {
                            let nr = r as isize + dr; let nc = c as isize + dc;
                            if !State::on_board_pos(nr,nc) { continue }
                            let ni = idx(nr as usize, nc as usize);
                            if self.board[ni].is_none() || self.board[ni].unwrap().owner != piece.owner {
                                moves.push(Move{actor:i, kind:"move", dest:Some(ni), captured: if self.board[ni].is_some() {Some(ni)} else {None}});
                            }
                        }
                    }
                    PieceKind::Core => {
                        if let Some(mon_pos) = self.find_piece(piece.owner, PieceKind::Monarch) {
                            if !State::are_adjacent_pos(i, mon_pos) { continue }
                        } else { continue }
                        for dr in -1..=1 {
                            for dc in -1..=1 {
                                if dr==0 && dc==0 { continue }
                                for step in 1..=2 {
                                    let nr = r as isize + dr*step; let nc = c as isize + dc*step;
                                    if !State::on_board_pos(nr,nc) { continue }
                                    let ni = idx(nr as usize, nc as usize);
                                    if step==2 {
                                        // can hop over anything
                                        if self.board[ni].is_none() || self.board[ni].unwrap().owner != piece.owner {
                                            moves.push(Move{actor:i, kind:"move", dest:Some(ni), captured: if self.board[ni].is_some() {Some(ni)} else {None}});
                                        }
                                    } else {
                                        if self.board[ni].is_none() || self.board[ni].unwrap().owner != piece.owner {
                                            moves.push(Move{actor:i, kind:"move", dest:Some(ni), captured: if self.board[ni].is_some() {Some(ni)} else {None}});
                                        }
                                    }
                                }
                            }
                        }
                    }
                    PieceKind::BruteL | PieceKind::BruteR => {
                        let orth = [(-1,0),(1,0),(0,-1),(0,1)];
                        for (dr,dc) in orth {
                            let nr = r as isize + dr; let nc = c as isize + dc;
                            if !State::on_board_pos(nr,nc) { continue }
                            let ni = idx(nr as usize, nc as usize);
                            if self.board[ni].is_none() { moves.push(Move{actor:i, kind:"move", dest:Some(ni), captured:None}) }
                        }
                        let (fdr, fdc) = if piece.owner == 'A' { (1, 0) } else { (-1, 0) };
                        let fr = r as isize + fdr; let fc = c as isize + fdc;
                        if State::on_board_pos(fr, fc) {
                            let fi = idx(fr as usize, fc as usize);
                            if let Some(t) = self.board[fi] {
                                if t.owner != piece.owner {
                                    moves.push(Move{actor:i, kind:"brute_capture", dest:None, captured:Some(fi)});
                                }
                            }
                        }
                    }
                    PieceKind::Tank => {
                        let orth = [(-1,0),(1,0),(0,-1),(0,1)];
                        for (dr,dc) in orth {
                            let nr = r as isize + dr; let nc = c as isize + dc;
                            if !State::on_board_pos(nr,nc) { continue }
                            let ni = idx(nr as usize, nc as usize);
                            if self.board[ni].is_none() { moves.push(Move{actor:i, kind:"move", dest:Some(ni), captured:None}) }
                        }
                        let (fdr, fdc) = if piece.owner == 'A' { (1, 0) } else { (-1, 0) };
                        let fr = r as isize + fdr; let fc = c as isize + fdc;
                        let lr = r as isize + 2*fdr; let lc = c as isize + 2*fdc;
                        if State::on_board_pos(fr, fc) && State::on_board_pos(lr, lc) {
                            let fi = idx(fr as usize, fc as usize);
                            let li = idx(lr as usize, lc as usize);
                            if let Some(mid) = self.board[fi] {
                                if mid.kind != PieceKind::Core && self.board[li].is_none() {
                                    moves.push(Move{actor:i, kind:"dash", dest:Some(li), captured:Some(fi)});
                                }
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn apply_move(&self, m: &Move) -> State {
        let mut s = self.clone();
        let piece = s.board[m.actor].expect("actor must exist");
        match m.kind {
            "move" => {
                if let Some(dest) = m.dest {
                    if let Some(target) = s.board[dest] { if target.owner != piece.owner { s.board[dest] = None; } }
                    s.board[m.actor] = None;
                    s.board[dest] = Some(piece);
                    if piece.kind == PieceKind::Core { s.last_core_moved_by = Some(piece.owner); }
                }
            }
            "brute_capture" => {
                if let Some(cap) = m.captured { s.board[cap] = None; }
            }
            "dash" => {
                if let Some(cap) = m.captured { s.board[cap] = None; }
                if let Some(dest) = m.dest { s.board[m.actor] = None; s.board[dest] = Some(piece); }
            }
            _ => {}
        }
        s.turn = if s.turn == 'A' { 'B' } else { 'A' };
        s
    }

    fn is_terminal(&self) -> Option<char> {
        if let (Some(a_pos), Some(b_pos)) = (self.find_piece('A', PieceKind::Core), self.find_piece('B', PieceKind::Core)) {
            if State::are_adjacent_pos(a_pos, b_pos) {
                return self.last_core_moved_by;
            }
        }
        None
    }
}

// ---------------- MCTS ----------------

struct MCTSNode {
    state: State,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: u32,
    wins: f64,
    untried: Vec<Move>,
    move_from_parent: Option<Move>,
}

impl MCTSNode {
    fn new(state: State, move_from_parent: Option<Move>, parent: Option<usize>) -> Self {
        let untried = state.legal_moves();
        MCTSNode { state, parent, children: Vec::new(), visits: 0, wins: 0.0, untried, move_from_parent }
    }
}

fn chebyshev_distance(a: usize, b: usize) -> usize {
    let (ar, ac) = rc(a);
    let (br, bc) = rc(b);
    let dr = if ar > br { ar - br } else { br - ar };
    let dc = if ac > bc { ac - bc } else { bc - ac };
    std::cmp::max(dr, dc)
}

fn random_playout(mut s: State, mut rng: &mut impl Rng, max_moves: usize, biased: bool) -> Option<char> {
    for _ in 0..max_moves {
        if let Some(w) = s.is_terminal() { return Some(w); }
        let moves = s.legal_moves();
        if moves.is_empty() { return None; }
        let chosen = if !biased {
            moves.choose(&mut rng).unwrap().clone()
        } else {
            // bias random playout towards moves that reduce distance between cores for the current player
            // compute weights based on resulting distance (lower distance -> higher weight)
            let mut weights: Vec<f64> = Vec::with_capacity(moves.len());
            for m in &moves {
                let s2 = s.apply_move(&m);
                // if terminal immediate, prefer it
                if s2.is_terminal().is_some() { weights.push(100.0); continue; }
                let a_pos = s2.find_piece('A', PieceKind::Core);
                let b_pos = s2.find_piece('B', PieceKind::Core);
                if a_pos.is_none() || b_pos.is_none() { weights.push(1.0); continue; }
                let dist = chebyshev_distance(a_pos.unwrap(), b_pos.unwrap()) as f64;
                // for player who moved next (s2.turn), we want to favor smaller distance for that player
                // if current player is 'A', favor moves that decrease distance; otherwise similar
                // choose weight = exp(-dist) but scaled
                let w = (-0.5 * dist).exp();
                weights.push(w);
            }
            // sample according to weights
            let sum: f64 = weights.iter().sum();
            let mut pick = rng.gen::<f64>() * sum;
            let mut idx = 0usize;
            while idx + 1 < weights.len() && pick > weights[idx] {
                pick -= weights[idx];
                idx += 1;
            }
            moves[idx].clone()
        };
        s = s.apply_move(&chosen);
    }
    None
}

fn uct_score(parent_visits: u32, child: &MCTSNode, c: f64) -> f64 {
    if child.visits == 0 { return f64::INFINITY; }
    (child.wins / child.visits as f64) + c * ((parent_visits as f64).ln() / child.visits as f64).sqrt()
}

fn mcts_action(root_state: &State, iterations: usize, c: f64, rng: &mut impl Rng, biased_playout: bool, playout_max: usize) -> Option<Move> {
    let mut nodes: Vec<MCTSNode> = Vec::new();
    nodes.push(MCTSNode::new(root_state.clone(), None, None));

    for _ in 0..iterations {
        // selection
        let mut node_idx = 0usize;
        loop {
            if !nodes[node_idx].untried.is_empty() || nodes[node_idx].children.is_empty() { break }
            // pick child with max UCT
            let mut best = None; let mut best_score = -1f64;
            for &child_idx in &nodes[node_idx].children {
                let score = uct_score(nodes[node_idx].visits, &nodes[child_idx], c);
                if score.is_infinite() || score > best_score { best_score = score; best = Some(child_idx); }
            }
            if let Some(b) = best { node_idx = b } else { break }
        }
        // expansion
        if !nodes[node_idx].untried.is_empty() {
            let midx = rng.gen_range(0..nodes[node_idx].untried.len());
            let mv = nodes[node_idx].untried.remove(midx);
            let child_state = nodes[node_idx].state.apply_move(&mv);
            nodes.push(MCTSNode::new(child_state, Some(mv.clone()), Some(node_idx)));
            let new_idx = nodes.len() - 1;
            nodes[node_idx].children.push(new_idx);
            node_idx = new_idx;
        }
        // simulation
        let winner = random_playout(nodes[node_idx].state.clone(), rng, playout_max, biased_playout);
        // backprop
        let mut cur = Some(node_idx);
        while let Some(ci) = cur {
            nodes[ci].visits += 1;
            if winner == Some(root_state.turn) { nodes[ci].wins += 1.0; }
            else if winner.is_none() { nodes[ci].wins += 0.5; }
            cur = nodes[ci].parent;
        }
    }

    // choose best child by visits
    if nodes[0].children.is_empty() { return None }
    let mut best_visits = 0u32; let mut best_move: Option<Move> = None;
    for &ci in &nodes[0].children {
        if nodes[ci].visits > best_visits { best_visits = nodes[ci].visits; best_move = nodes[ci].move_from_parent.clone(); }
    }
    best_move
}

// ------------ CLI ------------

#[derive(Parser, Debug)]
#[command(author, version, about = "Core Battle Rust Analyzer CLI")]
struct Args {
    /// number of MCTS iterations per move
    #[arg(short, long, default_value_t = 1000)]
    iters: usize,

    /// random seed
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// enable biased playouts (heuristic-weighted)
    #[arg(long, default_value_t = false)]
    biased_playout: bool,

    /// playout max moves
    #[arg(long, default_value_t = 100)]
    playout_max: usize,

    /// number of self-play games to run (1 = single probe; >1 runs statistics)
    #[arg(long, default_value_t = 1)]
    games: usize,

    /// maximum turns per game before declaring draw
    #[arg(long, default_value_t = 1000)]
    max_turns: usize,
}

fn play_game(mut st: State, iters: usize, c: f64, biased_playout: bool, playout_max: usize, max_turns: usize, rng: &mut impl Rng) -> Option<char> {
    let mut turns = 0usize;
    loop {
        if let Some(w) = st.is_terminal() { return Some(w) }
        if turns >= max_turns { return None }
        let mv = mcts_action(&st, iters, c, rng, biased_playout, playout_max);
        if mv.is_none() { return None }
        st = st.apply_move(&mv.unwrap());
        turns += 1;
    }
}

fn main() {
    let args = Args::parse();
    let mut global_rng = rand::rngs::StdRng::seed_from_u64(args.seed);

    if args.games <= 1 {
        let st = State::default();
        let t0 = Instant::now();
        let mv = mcts_action(&st, args.iters, 1.4, &mut global_rng, args.biased_playout, args.playout_max);
        let dur = t0.elapsed();
        if let Some(m) = mv {
            println!("Best move after {} iterations: actor={}, kind={}, dest={:?}, captured={:?}", args.iters, m.actor, m.kind, m.dest, m.captured);
        } else {
            println!("No move found");
        }
        println!("Elapsed: {:?}", dur);
        return;
    }

    let mut results = vec!['X'; args.games]; // 'A','B', or 'D' for draw
    for g in 0..args.games {
        // seed each game differently for variance
        let seed = args.seed.wrapping_add(g as u64);
        let mut game_rng = rand::rngs::StdRng::seed_from_u64(seed);
        let st = State::default();
        let winner = play_game(st, args.iters, 1.4, args.biased_playout, args.playout_max, args.max_turns, &mut game_rng);
        match winner {
            Some('A') => { results[g] = 'A'; println!("Game {}/{}: winner=A", g+1, args.games); }
            Some('B') => { results[g] = 'B'; println!("Game {}/{}: winner=B", g+1, args.games); }
            None => { results[g] = 'D'; println!("Game {}/{}: draw", g+1, args.games); }
            _ => { results[g] = 'D'; println!("Game {}/{}: unknown result", g+1, args.games); }
        }
    }

    let a_wins = results.iter().filter(|&&r| r == 'A').count();
    let b_wins = results.iter().filter(|&&r| r == 'B').count();
    let draws = results.iter().filter(|&&r| r == 'D').count();
    println!("--- Summary ({} games) ---", args.games);
    println!("A wins: {}\nB wins: {}\nDraws: {}", a_wins, b_wins, draws);
    if a_wins + b_wins > 0 {
        let rate = (a_wins as f64) / ((a_wins + b_wins) as f64);
        println!("First-player win rate (A / decisive games): {:.2}", rate);
    }
}
