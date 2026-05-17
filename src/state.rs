use chess_startpos_rs::chess::Piece;
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

#[derive(Copy, Clone)]
pub struct AppState {
    pub alphabet: RwSignal<Vec<Piece>>,
    pub root_constraint: RwSignal<ChessConstraint>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            alphabet: RwSignal::new(ALL_PIECES.to_vec()),
            root_constraint: RwSignal::new(Constraint::And(Vec::new())),
        }
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
