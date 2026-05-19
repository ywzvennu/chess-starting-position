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
        .constraint(editor_simplify(constraint))
        .build()
}

/// Treat editor-time "in-progress placeholder" empties as no-ops, then run
/// the upstream strict [`Constraint::simplify`].
///
/// Mid-edit users frequently leave an `Or([])` or `Not(empty)` in the tree
/// they haven't filled in yet. Strict logical reading collapses the whole
/// problem to unsatisfiable (count → 0), which is mathematically right but
/// awful UX. This wrapper first rewrites every such empty branch to the
/// vacuously-true `And([])`, then hands the result to `Constraint::simplify`
/// which folds the resulting redundant structure.
fn editor_simplify(constraint: ChessConstraint) -> ChessConstraint {
    map_editor_noops(constraint).simplify()
}

fn map_editor_noops(c: ChessConstraint) -> ChessConstraint {
    match c {
        // Empty Or — editor placeholder, treat as vacuously true.
        Constraint::Or(v) if v.is_empty() => Constraint::And(Vec::new()),
        Constraint::And(v) => Constraint::And(v.into_iter().map(map_editor_noops).collect()),
        Constraint::Or(v) => Constraint::Or(v.into_iter().map(map_editor_noops).collect()),
        Constraint::Not(inner) => {
            let inner = map_editor_noops(*inner);
            // After mapping the inner subtree, if it reduces to "no-op
            // skeleton" (an And([])), the Not wrapper is also a no-op
            // placeholder in editor terms.
            if matches!(&inner, Constraint::And(v) if v.is_empty()) {
                Constraint::And(Vec::new())
            } else {
                Constraint::Not(Box::new(inner))
            }
        }
        leaf => leaf,
    }
}

pub fn is_chess_960(alphabet: &[Piece], root: &ChessConstraint) -> bool {
    let canonical = chess::chess_960().into_problem();
    alphabet == canonical.pieces.as_slice() && root == &canonical.constraint
}

const URL_HASH_PREFIX: &str = "#c=";

/// Read the active (alphabet, root constraint) state from the location hash.
/// Returns `None` if no hash payload is present or it fails to parse.
pub fn read_url_state() -> Option<(Vec<Piece>, ChessConstraint)> {
    let win = web_sys::window()?;
    let hash = win.location().hash().ok()?;
    let payload = hash.strip_prefix(URL_HASH_PREFIX)?;
    let decoded = js_sys::decode_uri_component(payload).ok()?.as_string()?;
    serde_json::from_str::<(Vec<Piece>, ChessConstraint)>(&decoded).ok()
}

/// Mirror the current state into the URL hash via `history.replaceState`,
/// so the URL always reflects the live problem without polluting back-button
/// history. Silently no-ops on browsers/environments where window/history is
/// unavailable.
pub fn write_url_state(alphabet: &[Piece], root: &ChessConstraint) {
    let Some(win) = web_sys::window() else {
        return;
    };
    let payload: (Vec<Piece>, ChessConstraint) = (alphabet.to_vec(), root.clone());
    let Ok(json) = serde_json::to_string(&payload) else {
        return;
    };
    let encoded = js_sys::encode_uri_component(&json)
        .as_string()
        .unwrap_or_default();
    let hash = format!("{}{}", URL_HASH_PREFIX, encoded);
    if let Ok(history) = win.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::null(), "", Some(&hash));
    }
}

/// True when the active state matches the default (full alphabet + empty
/// root constraint) — the state a fresh page would otherwise serialise into
/// a verbose `#c=…` hash for no information gain.
pub fn is_default_state(alphabet: &[Piece], root: &ChessConstraint) -> bool {
    if alphabet != ALL_PIECES.as_slice() {
        return false;
    }
    matches!(root, Constraint::And(v) if v.is_empty())
}

/// Remove any URL hash via `history.replaceState`, leaving only the
/// `pathname` (+ `search`). Used when the active state matches the default
/// so the URL stays clean on a fresh load.
pub fn clear_url_state() {
    let Some(win) = web_sys::window() else {
        return;
    };
    let Ok(history) = win.history() else {
        return;
    };
    let location = win.location();
    let pathname = location.pathname().unwrap_or_default();
    let search = location.search().unwrap_or_default();
    let url = format!("{pathname}{search}");
    let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::null(), "", Some(&url));
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess_startpos_rs::chess;

    #[test]
    fn fen_for_standard_fide_position() {
        let arr = chess::chess_960()
            .sp_id(518)
            .expect("sp_id 518 is the standard FIDE position");
        assert_eq!(
            fen_for_arrangement(&arr),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1"
        );
    }

    #[test]
    fn fen_for_all_kings_arrangement() {
        let arr = vec![Piece::King; BOARD_SQUARES];
        assert_eq!(
            fen_for_arrangement(&arr),
            "kkkkkkkk/pppppppp/8/8/8/8/PPPPPPPP/KKKKKKKK w - - 0 1"
        );
    }

    #[test]
    fn lichess_url_replaces_spaces_with_underscores() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
        assert_eq!(
            lichess_editor_url(fen),
            "https://lichess.org/editor/rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR_w_-_-_0_1"
        );
    }

    #[test]
    fn is_chess_960_recognises_canonical_problem() {
        let canonical = chess::chess_960().into_problem();
        assert!(is_chess_960(&canonical.pieces, &canonical.constraint));
    }

    #[test]
    fn is_chess_960_rejects_shuffle_preset() {
        let shuffle = chess::shuffle();
        assert!(!is_chess_960(&shuffle.pieces, &shuffle.constraint));
    }

    #[test]
    fn is_chess_960_rejects_empty_constraint() {
        let canonical = chess::chess_960().into_problem();
        let empty_root = Constraint::And(Vec::new());
        assert!(!is_chess_960(&canonical.pieces, &empty_root));
    }

    #[test]
    fn editor_simplify_collapses_empty_or_to_noop() {
        // Or([]) alone simplifies to no-op (And([])).
        let c: ChessConstraint = Constraint::Or(Vec::new());
        let simplified = editor_simplify(c);
        assert_eq!(simplified, Constraint::And(Vec::new()));
    }

    #[test]
    fn editor_simplify_drops_empty_or_inside_and() {
        // Without the wrapper this would collapse the whole And to Or([])
        // (strict false propagation). With editor semantics it should keep
        // the meaningful leaf.
        let leaf = Constraint::Count {
            piece: chess::Piece::King,
            op: chess_startpos_rs::CountOp::Eq,
            value: 1,
        };
        let c = Constraint::And(vec![leaf.clone(), Constraint::Or(Vec::new())]);
        assert_eq!(editor_simplify(c), leaf);
    }

    #[test]
    fn editor_simplify_collapses_not_of_placeholder() {
        // Not(Or([])) — Not over an editor placeholder — should be a no-op,
        // not the strict reading of "negation of false = true".
        // After map_editor_noops, the inner Or([]) becomes And([]), and
        // Not(And([])) collapses to no-op in the wrapper.
        let c: ChessConstraint = Constraint::Not(Box::new(Constraint::Or(Vec::new())));
        assert_eq!(editor_simplify(c), Constraint::And(Vec::new()));
    }

    #[test]
    fn editor_simplify_preserves_real_constraint() {
        // A real, non-placeholder constraint goes through unchanged
        // (modulo upstream simplification of redundant wrappers).
        let leaf = Constraint::At {
            piece: chess::Piece::Queen,
            square: 3,
        };
        let wrapped = Constraint::And(vec![leaf.clone()]);
        assert_eq!(editor_simplify(wrapped), leaf);
    }
}
