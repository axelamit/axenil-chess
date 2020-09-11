pub mod pieces {
    use super::board;
    use super::units;

    pub fn get_possible_moves(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
    ) -> Vec<(i64, i64)> {
        let mut move_vec: Vec<(i64, i64, bool)> = chess_piece.get_moves();
        let mut moves = Vec::<(i64, i64)>::new();
        while let Some((x, y, mut multiple_steps)) = move_vec.pop() {
            if y + (y_org as i64) < 0
                || x + (x_org as i64) < 0
                || y + (y_org as i64) > 7
                || x + (x_org as i64) > 7
            {
                //Kanske ändra till > chess_board.grid[0].len()
                continue;
            }
            let y_index = (y + (y_org as i64)) as usize;
            let x_index = (x + (x_org as i64)) as usize;
            let next_square = chess_board.get_square(x_index, y_index);

            if !next_square.is_empty() {
                if chess_piece.color.forward() == next_square.piece.color.forward() {
                    continue;
                } else {
                    multiple_steps = false;
                }
            }

            moves.push((x + (x_org as i64), y + (y_org as i64)));
            if multiple_steps {
                let mut next_x = 0;
                let mut next_y = 0;
                if x < 0 {
                    next_x = x - 1;
                }
                if x > 0 {
                    next_x = x + 1;
                }
                if y < 0 {
                    next_y = y - 1;
                }
                if y > 0 {
                    next_y = y + 1;
                }
                move_vec.push((next_x, next_y, multiple_steps))
            }
        }
        moves
    }

    pub fn move_normal(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64)> {
        let chess_piece = chess_board.get_square(x, y).piece;
        let possible_moves = get_possible_moves(x, y, chess_piece, chess_board);
        possible_moves
    }

    /*
    pub fn move_pawn(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }

    pub fn move_bishop(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }

    pub fn move_knight(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }

    pub fn move_rook(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }

    pub fn move_queen(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }

    pub fn move_king(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
        let possible_moves = move_normal(x, y, chess_board);
        possible_moves
    }
    */
}

pub mod units {
    use super::board;
    use super::pieces;

    #[derive(Debug, Copy, Clone)]
    //Kanske ta bort public här
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
                //Några kan bytas ut till bara move_normal
                Variety::Pawn => pieces::move_normal(x, y, chess_board),
                Variety::Bishop => pieces::move_normal(x, y, chess_board),
                Variety::Knight => pieces::move_normal(x, y, chess_board),
                Variety::Rook => pieces::move_normal(x, y, chess_board),
                Variety::Queen => pieces::move_normal(x, y, chess_board),
                Variety::King => pieces::move_normal(x, y, chess_board),
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
}

pub mod board {
    use super::units;
    use std::fs;

    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        pub piece: units::Piece,
    }

    #[allow(warnings)]
    impl Square {
        pub fn is_empty(&self) -> bool {
            match self.piece.variety {
                units::Variety::Empty => true,
                _ => false,
            }
        }
        pub fn get_empty() -> Square {
            Square {
                piece: units::Piece {
                    variety: units::Variety::Empty,
                    color: units::Color::Empty,
                    has_moved: false,
                },
            }
        }
    }

    pub fn get_variety(variety: char) -> units::Variety {
        match variety {
            'p' | 'P' => units::Variety::Pawn,
            'r' | 'R' => units::Variety::Rook,
            'n' | 'N' => units::Variety::Knight,
            'b' | 'B' => units::Variety::Bishop,
            'q' | 'Q' => units::Variety::Queen,
            'k' | 'K' => units::Variety::King,
            _ => units::Variety::Empty,
        }
    }

    pub fn convert_position(pos: &str) -> (usize, usize) {
        let column = pos.chars().nth(0).unwrap();
        let row = pos.chars().nth(1).unwrap();

        let x = (column as usize) - 97;
        let y = 8 - ((row as usize) - 48);
        (x, y)
    }

    pub fn get_color(color: char) -> units::Color {
        let ascii_color = color as u32;
        match ascii_color {
            ascii_color if (ascii_color > 96) => units::Color::White,
            _ => units::Color::Black,
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Board {
        grid: [[Square; 8]; 8],
    }

    impl Board {
        pub fn init() -> Board {
            let empty_square = Square::get_empty();

            Board {
                grid: [[empty_square; 8]; 8],
            }
        }

        pub fn get_square(&self, x: usize, y: usize) -> Square{
            self.grid[y][x]
        }

        pub fn read_board(&self, file_name: &str) -> Vec<char> {
            let contents = fs::read_to_string(file_name).expect("Could not read the file");

            let res: Vec<char> = contents
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            res
        }

        pub fn fill_board(&mut self, file_name: &str) {
            let start_board = self.read_board(file_name);

            for i in 0..8 {
                for j in 0..8 {
                    self.grid[i][j].piece.variety = get_variety(start_board[i * 8 + j]);

                    if !self.grid[i][j].is_empty() {
                        self.grid[i][j].piece.color = get_color(start_board[i * 8 + j]);
                    }
                }
            }
        }

        pub fn get_moves(&self, pos: &str) -> Vec<(i64, i64)> {
            let (x, y) = convert_position(pos);
            let piece = self.grid[y][x].piece;
            //Check if chess piece is the same color as the current player. If not return "non legal move"
            piece.variety.get_moves(x, y, &self)
        }

        pub fn make_move(&mut self, pos: &str) {
            let split = pos.split(" ");
            let vec: Vec<&str> = split.collect();

            let (x0, y0) = convert_position(vec[0]);
            let (x1, y1) = convert_position(vec[1]);
            let moves = self.get_moves(vec[0]);

            for position in moves.iter() {
                let x = position.0 as i64;
                let y = position.1 as i64;
                if x == (x1 as i64) && y == (y1 as i64) {
                    //self.grid
                    //self.grid[y1][x1]
                    self.grid[y1][x1] = self.grid[y0][x0];
                    self.grid[y0][x0] = Square::get_empty();
                    println!("Move is possible")
                }
            }
            self.print_board();
            //println!("Moves: {:?}, X1: {}, Y1: {}", moves, x1, y1);
        }

        pub fn print_board(&self) {
            println!("   a b c d e f g h");
            println!("   ---------------");
            for i in 0..8 {
                print!("{}| ", 8 - i);
                for j in 0..8 {
                    let mut square;
                    match self.grid[i][j].piece.variety {
                        units::Variety::Pawn => square = "p".to_string(),
                        units::Variety::Bishop => square = "b".to_string(),
                        units::Variety::Knight => square = "n".to_string(),
                        units::Variety::Rook => square = "r".to_string(),
                        units::Variety::Queen => square = "q".to_string(),
                        units::Variety::King => square = "k".to_string(),
                        _ => square = "x".to_string(),
                    }
                    match self.grid[i][j].piece.color {
                        units::Color::Black => square = square.to_uppercase(),
                        _ => (),
                    }
                    print!("{} ", square);
                }
                println!("|{}", 8 - i);
            }
            println!("   ---------------");
            println!("   a b c d e f g h");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(2 + 2, 4);
    }
}
