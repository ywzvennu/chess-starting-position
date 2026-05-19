//! Inline SVG chess pieces (Cburnett set, CC BY-SA 3.0).
//!
//! See `assets/pieces/README.md` for attribution.

use crate::state::Orientation;
use chess_startpos_rs::chess::Piece;

const WK: &str = include_str!("../assets/pieces/wK.svg");
const WQ: &str = include_str!("../assets/pieces/wQ.svg");
const WR: &str = include_str!("../assets/pieces/wR.svg");
const WB: &str = include_str!("../assets/pieces/wB.svg");
const WN: &str = include_str!("../assets/pieces/wN.svg");
const BK: &str = include_str!("../assets/pieces/bK.svg");
const BQ: &str = include_str!("../assets/pieces/bQ.svg");
const BR: &str = include_str!("../assets/pieces/bR.svg");
const BB: &str = include_str!("../assets/pieces/bB.svg");
const BN: &str = include_str!("../assets/pieces/bN.svg");

pub fn piece_svg(p: Piece, orient: Orientation) -> &'static str {
    match (orient, p) {
        (Orientation::White, Piece::King) => WK,
        (Orientation::White, Piece::Queen) => WQ,
        (Orientation::White, Piece::Rook) => WR,
        (Orientation::White, Piece::Bishop) => WB,
        (Orientation::White, Piece::Knight) => WN,
        (Orientation::Black, Piece::King) => BK,
        (Orientation::Black, Piece::Queen) => BQ,
        (Orientation::Black, Piece::Rook) => BR,
        (Orientation::Black, Piece::Bishop) => BB,
        (Orientation::Black, Piece::Knight) => BN,
    }
}
