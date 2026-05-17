use chess_startpos_rs::chess::{self, Piece};
use chess_startpos_rs::{Constraint, Problem, SquareColor};
use leptos::prelude::*;

pub type ChessConstraint = Constraint<Piece, SquareColor>;
pub type ChessProblem = Problem<Piece, SquareColor>;

pub const BOARD_SQUARES: usize = 8;

pub const ALL_PIECES: [Piece; 5] = [
    Piece::King,
    Piece::Queen,
    Piece::Rook,
    Piece::Bishop,
    Piece::Knight,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct AppState {
    pub alphabet: RwSignal<Vec<Piece>>,
    pub root_constraint: RwSignal<ChessConstraint>,
    pub orientation: RwSignal<Orientation>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            alphabet: RwSignal::new(ALL_PIECES.to_vec()),
            root_constraint: RwSignal::new(Constraint::And(Vec::new())),
            orientation: RwSignal::new(Orientation::White),
        }
    }
}

pub fn fen_for_arrangement(arr: &[Piece]) -> String {
    let rank_white: String = arr.iter().map(|p| piece_letter(*p)).collect();
    let rank_black: String = rank_white.to_ascii_lowercase();
    format!(
        "{}/pppppppp/8/8/8/8/PPPPPPPP/{} w - - 0 1",
        rank_black, rank_white
    )
}

pub fn lichess_editor_url(fen: &str) -> String {
    format!("https://lichess.org/editor/{}", fen.replace(' ', "_"))
}

fn piece_letter(p: Piece) -> char {
    match p {
        Piece::King => 'K',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
        Piece::Bishop => 'B',
        Piece::Knight => 'N',
    }
}

pub fn build_problem(alphabet: Vec<Piece>, constraint: ChessConstraint) -> ChessProblem {
    Problem::builder()
        .squares(BOARD_SQUARES)
        .alternating_colors(SquareColor::Light, SquareColor::Dark)
        .pieces(alphabet)
        .constraint(constraint)
        .build()
}

pub fn is_chess_960(alphabet: &[Piece], root: &ChessConstraint) -> bool {
    let canonical = chess::chess_960().into_problem();
    alphabet == canonical.pieces.as_slice() && root == &canonical.constraint
}
