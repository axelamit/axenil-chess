use crate::board;
use crate::moves;

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub variety: Variety,
    pub color: Color,
    pub has_moved: bool,
}

impl Piece {
    pub fn get_moves(&self) -> Vec<(i64, i64, bool)> {
        match self.variety {
            Variety::Pawn => vec![(0, self.color.forward(), false)],
            Variety::Bishop => vec![(1, 1, true), (1, -1, true), (-1, 1, true), (-1, -1, true)],
            Variety::Knight => vec![
                (2, 1, false),
                (2, -1, false),
                (-2, 1, false),
                (-2, -1, false),
                (1, 2, false),
                (1, -2, false),
                (-1, 2, false),
                (-1, -2, false),
            ],
            Variety::Rook => vec![(1, 0, true), (-1, 0, true), (0, 1, true), (0, -1, true)],
            Variety::Queen => vec![
                (1, 1, true),
                (1, -1, true),
                (-1, 1, true),
                (-1, -1, true),
                (1, 0, true),
                (-1, 0, true),
                (0, 1, true),
                (0, -1, true),
            ],
            Variety::King => vec![
                (1, 1, false),
                (1, -1, false),
                (-1, 1, false),
                (-1, -1, false),
                (1, 0, false),
                (-1, 0, false),
                (0, 1, false),
                (0, -1, false),
            ],
            _ => Vec::<(i64, i64, bool)>::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Variety {
    Empty,
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl Variety {
    pub fn get_moves(&self, x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64)> {
        match self {
            Variety::Pawn => moves::move_pawn(x, y, chess_board),
            Variety::Bishop => moves::move_normal(x, y, chess_board, true),
            Variety::Knight => moves::move_normal(x, y, chess_board, true),
            Variety::Rook => moves::move_normal(x, y, chess_board, true),
            Variety::Queen => moves::move_normal(x, y, chess_board, true),
            Variety::King => moves::move_normal(x, y, chess_board, true),
            _ => Vec::<(i64, i64)>::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Empty,
    Black,
    White,
}

#[allow(warnings)]
impl Color {
    pub fn inverse(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            _ => Color::Empty,
        }
    }
    pub fn forward(&self) -> i64 {
        match self {
            Color::Black => 1,
            Color::White => -1,
            _ => 0,
        }
    }
}
