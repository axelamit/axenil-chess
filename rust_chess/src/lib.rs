pub mod pieces {
    use super::board;
    use super::units;

    pub fn get_possible_moves(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
        capture: bool,
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

            if !next_square.is_empty(){
                if (chess_piece.color.forward() == next_square.piece.color.forward()) || !capture {
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

    pub fn get_pawn_capture_moves(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
    ) -> Vec<(i64, i64)> {
        let y = ((y_org as i64)+chess_piece.color.forward()) as usize; 
        let mut moves = Vec::<(i64, i64)>::new();

        if x_org+1 < 8{
            let next_square = chess_board.get_square(x_org+1, y);  
            if (next_square.piece.color.forward() != chess_piece.color.forward()) && !next_square.is_empty(){
                moves.push(((x_org+1) as i64, y as i64)); 
            }
        }

        if (x_org as i64)-1 >= 0 {
            let next_square = chess_board.get_square(x_org-1, y);  
            if (next_square.piece.color.forward() != chess_piece.color.forward()) && !next_square.is_empty(){
                moves.push(((x_org-1) as i64, y as i64)); 
            }
        }

        moves
    }

    pub fn pawn_two_steps(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
    ) -> Vec<(i64, i64)> {
        let mut moves = Vec::<(i64, i64)>::new();
        if !chess_piece.has_moved{
            let y1 = ((y_org as i64)+chess_piece.color.forward()) as usize; 
            let y2 = ((y_org as i64)+chess_piece.color.forward()*2) as usize; 
            let one_forward = chess_board.get_square(x_org, y1); 
            let two_forward = chess_board.get_square(x_org, y2); 
            if one_forward.is_empty() && two_forward.is_empty(){
                moves.push((x_org as i64, y2 as i64)); 
            }
        }
        moves
    }
    
    pub fn move_pawn(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64)> {
        let mut moves = Vec::<(i64, i64)>::new();
        let chess_piece = chess_board.get_square(x, y).piece;

        let normal_moves = move_normal(x, y, chess_board, false);
        let capture_moves = get_pawn_capture_moves(x, y, chess_piece, chess_board); 
        let two_steps = pawn_two_steps(x, y, chess_piece, chess_board); 

        for normal_move in normal_moves.iter(){
            moves.push(normal_move.to_owned()); 
        }

        for capture_move in capture_moves.iter(){
            moves.push(capture_move.to_owned()); 
        }

        for two_step_move in two_steps.iter(){
            moves.push(two_step_move.to_owned());
        }

        moves
    }

    pub fn move_normal(x: usize, y: usize, chess_board: &board::Board, capture: bool) -> Vec<(i64, i64)> {
        let chess_piece = chess_board.get_square(x, y).piece;
        let possible_moves = get_possible_moves(x, y, chess_piece, chess_board, capture);
        possible_moves
    }
    /*
    pub fn move_rook(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64, bool)> {
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
                Variety::Pawn => pieces::move_pawn(x, y, chess_board),
                Variety::Bishop => pieces::move_normal(x, y, chess_board, true),
                Variety::Knight => pieces::move_normal(x, y, chess_board, true),
                Variety::Rook => pieces::move_normal(x, y, chess_board, true),
                Variety::Queen => pieces::move_normal(x, y, chess_board, true),
                Variety::King => pieces::move_normal(x, y, chess_board, true),
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
        current_player: units::Color,
    }

    impl Board {
        pub fn init() -> Board {
            let empty_square = Square::get_empty();

            Board {
                grid: [[empty_square; 8]; 8],
                current_player: units::Color::White,
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
            let (valid, message) = self.check_if_possible_move(pos, true);
            println!("Valid: {}, Message: {}", valid, message);  
            self.print_board();
            //println!("Moves: {:?}, X1: {}, Y1: {}", moves, x1, y1);
        }

        pub fn check_if_possible_move(&mut self, pos: &str, move_piece: bool) -> (bool, String){
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
                    let old_square = self.grid[y1][x1]; 
                    self.grid[y1][x1] = self.grid[y0][x0];
                    self.grid[y0][x0] = Square::get_empty();

                    if self.chess(){
                        self.grid[y0][x0] = self.grid[y1][x1]; 
                        self.grid[y1][x1] = old_square; 
                        return (false, "Chess!".to_string())
                    }
                    else{
                        if !move_piece{
                            self.grid[y0][x0] = self.grid[y1][x1]; 
                            self.grid[y1][x1] = old_square; 
                        }
                        else{
                            self.grid[y1][x1].piece.has_moved = true; 
                        }
                        return (true, "".to_string())
                    }
                }
            }
            (false, "Invalid move".to_string())
        }

        pub fn chess(&self) -> bool{
            let mut king_x = 0; 
            let mut king_y = 0; 
            for i in 0..8{
                for j in 0..8{
                    if self.grid[i][j].piece.color.forward() == self.current_player.forward() {
                        match self.grid[i][j].piece.variety{
                            units::Variety::King => {
                                king_x = j; 
                                king_y = i; 
                            }, 
                            _ => (),
                        }
                    }
                }
            }

            for i in 0..8{
                for j in 0..8{
                    if !self.grid[i][j].is_empty() && (self.grid[i][j].piece.color.forward() != self.current_player.forward()) {
                        let moves = self.grid[i][j].piece.variety.get_moves(j, i, &self); 
                        for position in moves.iter(){
                            if position.0 == (king_x as i64) && position.1 == (king_y as i64) {
                                return true
                            } 
                        }
                    }
                }
            }

            false
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
