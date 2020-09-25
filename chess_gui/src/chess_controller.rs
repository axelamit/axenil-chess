use piston::input::{Button, GenericEvent, MouseButton};
use rust_chess::*;

pub struct ChessController {
    pub chess_board: rust_chess::board::Board,
    pub board_string: [[String; 8]; 8],
    pub selected_space: Option<(u8, u8)>,
    pub highlighted_spaces: Option<Vec<(i64, i64)>>,
    pub mouse_pos: [f64; 2],
}

impl ChessController {
    pub fn new(chess_board: rust_chess::board::Board) -> ChessController {
        let mut controller = ChessController {
            chess_board,
            board_string: Default::default(),
            selected_space: None,
            highlighted_spaces: None,
            mouse_pos: [0.0; 2],
        };
        controller.update_board();
        controller
    }

    fn update_board(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                let mut piece_str = match self.chess_board.get_square(x, y).piece.variety {
                    units::Variety::Empty => " ",
                    units::Variety::Pawn => "pawn",
                    units::Variety::Bishop => "bishop",
                    units::Variety::Knight => "knight",
                    units::Variety::Rook => "rook",
                    units::Variety::Queen => "queen",
                    units::Variety::King => "king",
                };
                if self.chess_board.get_square(x, y).piece.color.forward() == -1 {
                    let piece_str: String = "white_".to_string() + piece_str;
                    self.board_string[y][x] = piece_str;
                } else {
                    let piece_str: String = "black_".to_string() + piece_str;
                    self.board_string[y][x] = piece_str;
                }
            }
        }
    }

    pub fn event<E: GenericEvent>(&mut self, view_pos: [f64; 2], view_size: f64, event: &E) {
        if let Some(r) = event.render_args() {}

        if let Some(pos) = event.mouse_cursor_args() {
            self.mouse_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let (x, y) = (self.mouse_pos[0] - view_pos[0], self.mouse_pos[1]);
            if x > 0.0 && x < view_size && y > 0.0 && y < view_size {
                let (x, y) = ((x / view_size * 8.0) as u8, 7 - (y / view_size * 8.0) as u8);
                if let Some(highlighted_spaces) = &self.highlighted_spaces {
                    for spaces in highlighted_spaces {
                        if x == spaces.0 as u8 && y == 7 - spaces.1 as u8 {
                            let mut input = rust_chess::board::position_to_string(
                                self.selected_space.expect("no inital piece selection").0,
                                self.selected_space.expect("no inital piece selection").1,
                            ) + " ";
                            input.push_str(rust_chess::board::position_to_string(x, y).as_str());
                            self.chess_board.make_move(input.as_str());
                            self.update_board();
                            break;
                        }
                    }
                    self.selected_space = None;
                    self.highlighted_spaces = None;
                } else {
                    self.selected_space = Some((x, y));
                    let pos = rust_chess::board::position_to_string(x, y);
                    let possible_moves = self.chess_board.get_moves(pos.as_str());
                    self.highlighted_spaces = Some(possible_moves);
                }
            } else {
                self.selected_space = None;
                self.highlighted_spaces = None;
            }
        }
    }
}
