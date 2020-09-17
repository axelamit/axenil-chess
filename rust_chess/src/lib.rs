pub mod moves {
    use super::board;
    use super::units;
    use std::cmp;

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
        let mut moves = Vec::<(i64, i64)>::new();
        if y_org > 6 || y_org < 1{
            return moves;
        }
        let y = ((y_org as i64)+chess_piece.color.forward()) as usize; 

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

    //Kanske kolla att den hamnar i en ruta (är kanske onödigt då det bara gäller när has_moved är false och den befinner sig på näst sista rutan)
    pub fn pawn_two_steps(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
    ) -> Vec<(i64, i64)> {
        let mut moves = Vec::<(i64, i64)>::new();
        let y1 = (y_org as i64)+chess_piece.color.forward(); 
        let y2 = (y_org as i64)+chess_piece.color.forward()*2; 
        if !chess_piece.has_moved && y2 <= 7 && y2 >= 0{
            let one_forward = chess_board.get_square(x_org, y1 as usize); 
            let two_forward = chess_board.get_square(x_org, y2 as usize); 
            if one_forward.is_empty() && two_forward.is_empty(){
                moves.push((x_org as i64, y2)); 
            }
        }
        moves
    }

    pub fn pawn_passant(
        x_org: usize,
        y_org: usize,
        chess_piece: units::Piece,
        chess_board: &board::Board,
    ) -> Vec<(i64, i64)> {
        let mut moves = Vec::<(i64, i64)>::new(); 
        let passant = chess_board.get_passant(); 
        if passant.1 == (y_org as i64) && ((passant.0 as i64)-(x_org as i64)).abs() == 1{
            let y = (y_org as i64)+chess_piece.color.forward(); 
            let x = passant.0 as i64; 
            if y >= 0 && y <= 7 && x >= 0 && x <= 7{
                if chess_board.get_square(x as usize, y as usize).is_empty(){
                    moves.push((x, y)); 
                }
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
        let passant_moves = pawn_passant(x, y, chess_piece, chess_board); 

        for normal_move in normal_moves.iter(){
            moves.push(normal_move.to_owned()); 
        }

        for capture_move in capture_moves.iter(){
            moves.push(capture_move.to_owned()); 
        }

        for two_step_move in two_steps.iter(){
            moves.push(two_step_move.to_owned());
        }

        for passant_move in passant_moves.iter(){
            moves.push(passant_move.to_owned()); 
        }

        moves
    }

    pub fn move_normal(x: usize, y: usize, chess_board: &board::Board, capture: bool) -> Vec<(i64, i64)> {
        let chess_piece = chess_board.get_square(x, y).piece;
        let possible_moves = get_possible_moves(x, y, chess_piece, chess_board, capture);
        possible_moves
    }

    pub fn castling(
        chess_board: &mut board::Board, 
        king_org_x: usize,
        king_new_x: usize,
        rook_org_x: usize,
        rook_new_x: usize,
        y: usize,
    ) -> (bool, usize, usize, usize, usize, usize){
        let king = chess_board.get_square(king_org_x, y).piece;
        let rook = chess_board.get_square(rook_org_x, y).piece; 
        if let units::Variety::King = king.variety{
            if let units::Variety::Rook = rook.variety{
                if !king.has_moved && !rook.has_moved{
                    let mn_row = cmp::min(king_org_x, rook_org_x); 
                    let mx_row = cmp::max(king_org_x, rook_org_x); 

                    for x_pos in (mn_row+1)..(mx_row){
                        if !chess_board.get_square(x_pos, y).is_empty(){
                            return (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y)
                        }
                    }

                    let mn = cmp::min(king_org_x, king_new_x); 
                    let mx = cmp::max(king_org_x, king_new_x); 

                    for x_pos in mn..(mx+1){
                        if chess_board.move_check(king_org_x, y, x_pos, y) {
                            return (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y)
                        }
                    }

                    return (true, king_org_x, king_new_x, rook_org_x, rook_new_x, y)
                }
            }
        }
        (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y)
    }
    
    pub fn kingside_castling(chess_board: &mut board::Board) -> (bool, usize, usize, usize, usize, usize){
        let mut y = 0; 
        if let units::Color::White = chess_board.get_current_player(){
            y = 7; 
        }

        castling(chess_board, 4, 6, 7, 5, y)
    }

    pub fn queenside_castling(chess_board: &mut board::Board) -> (bool, usize, usize, usize, usize, usize){
        let mut y = 0; 
        if let units::Color::White = chess_board.get_current_player(){
            y = 7; 
        }
        castling(chess_board, 4, 2, 0, 3, y)
    }
}

pub mod units {
    use super::board;
    use super::moves;

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
}

pub mod board {
    use super::units; 
    use super::moves; 

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

    pub fn string_to_position(pos: &str) -> (usize, usize) {
        let column = pos.chars().nth(0).unwrap();
        let row = pos.chars().nth(1).unwrap();

        let x = (column as usize) - 97;
        let y = 8 - ((row as usize) - 48);
        (x, y)
    }

    pub fn position_to_string(r: u8, c: u8) -> String {
        let row = (r as char).to_string();
        let col = (c as char).to_string(); 
        let pos = [col, row].join(""); 
        pos
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
        passant: (i64, i64),
    }

    impl Board {
        pub fn init() -> Board {
            let empty_square = Square::get_empty();

            Board {
                grid: [[empty_square; 8]; 8],
                current_player: units::Color::White,
                passant: (-1, -1), 
            }
        }

        pub fn get_square(&self, x: usize, y: usize) -> Square{
            self.grid[y][x]
        }

        pub fn get_current_player(&self) -> units::Color{
            self.current_player
        }

        pub fn get_passant(&self) -> (i64, i64){
            self.passant
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
            let (x, y) = string_to_position(pos);
            let piece = self.grid[y][x].piece;
            if piece.color.forward() == self.current_player.forward(){
                return piece.variety.get_moves(x, y, &self)
            }
            Vec::<(i64, i64)>::new()
        }

        //Kolla om spelaren inte är i schack och inte kan göra några drag, isåfall lika 
        pub fn make_move(&mut self, input: &str) -> (bool, bool, String){
            let tokens: Vec<&str>= input.split("=").collect();
            let pos = tokens[0]; 
            let mut promotion = "".to_string(); 
            if tokens.len() > 1{
                promotion.push_str(tokens[1]); 
            }

            if let "O-O-O" = pos {
                let (valid, king_org_x, king_new_x, rook_org_x, rook_new_x, y) = moves::queenside_castling(self); 
                if valid{
                    self.grid[y][king_new_x] = self.grid[y][king_org_x]; 
                    self.grid[y][king_org_x] = Square::get_empty();

                    self.grid[y][rook_new_x] = self.grid[y][rook_org_x]; 
                    self.grid[y][rook_org_x] = Square::get_empty();
                }
                else{
                    return (false, true, "Queenside castling not possible".to_string()); 
                }
            }
            else if let "O-O" = pos{
                let (valid, king_org_x, king_new_x, rook_org_x, rook_new_x, y) = moves::kingside_castling(self);
                if valid{
                    self.grid[y][king_new_x] = self.grid[y][king_org_x]; 
                    self.grid[y][king_org_x] = Square::get_empty();

                    self.grid[y][rook_new_x] = self.grid[y][rook_org_x]; 
                    self.grid[y][rook_org_x] = Square::get_empty();
                }
                else{
                    return (false, true, "Kingside castling not possible".to_string()); 
                }
            }
            else{
                let (valid, message) = self.check_if_legal_move(pos, true);
                if !valid{
                    return (false, true, message); 
                }
                if !promotion.is_empty(){
                    self.promotion(get_variety(promotion.chars().next().unwrap()));
                }
            }
            self.current_player = self.current_player.inverse();   
            self.print_board();

            let state = self.get_state(); 
            if !state.0{
                return (true, false, state.1); 
            }
            (false, false, state.1)
        }

        pub fn promotion(&mut self, variety: units::Variety){
            for i in 0..8{
                if let units::Variety::Pawn = self.grid[0][i].piece.variety{
                    self.grid[0][i].piece.variety = variety; 
                } 
                if let units::Variety::Pawn = self.grid[7][i].piece.variety{
                    self.grid[7][i].piece.variety = variety; 
                }
            }
        } 

        pub fn get_state(&mut self) -> (bool, String){
            if self.checkmate(){
                if self.check(){
                    return (false, "Checkmate".to_string());  
                }
                else{
                    return (false, "Stalemate".to_string()); 
                }
            }
            else{
                if self.check(){
                    return (true, "Check".to_string()); 
                }
                else{
                    return (true, "".to_string()); 
                }
            }
        }

        pub fn check_if_legal_move(&mut self, pos: &str, move_piece: bool) -> (bool, String){
            let split = pos.split(" ");
            let vec: Vec<&str> = split.collect();

            let (x0, y0) = string_to_position(vec[0]);
            let (x1, y1) = string_to_position(vec[1]);
            let moves = self.get_moves(vec[0]);

            for position in moves.iter() {
                let x = position.0 as i64;
                let y = position.1 as i64;
                if x == (x1 as i64) && y == (y1 as i64) {
                    let old_square = self.grid[y1][x1]; 
                    self.grid[y1][x1] = self.grid[y0][x0];
                    self.grid[y0][x0] = Square::get_empty();

                    if self.check(){
                        self.grid[y0][x0] = self.grid[y1][x1]; 
                        self.grid[y1][x1] = old_square; 
                        return (false, "Check!".to_string())
                    }
                    else{
                        if !move_piece{
                            self.grid[y0][x0] = self.grid[y1][x1]; 
                            self.grid[y1][x1] = old_square; 
                        }
                        else{
                            let mut set_passant = false; 
                            if let units::Variety::Pawn = self.grid[y1][x1].piece.variety{
                                if ((y1 as i64)-(y0 as i64)).abs() > 1{
                                    set_passant = true; 
                                }
                                if old_square.is_empty() && ((y1 as i64)-(y0 as i64)).abs() == 1 && ((x1 as i64)-(x0 as i64)).abs() == 1{
                                    self.grid[y0][x1] = Square::get_empty(); 
                                }
                            }

                            if set_passant{
                                self.passant = (x1 as i64, y1 as i64); 
                            }
                            else {
                                self.passant = (-1, -1); 
                            }
                            self.grid[y1][x1].piece.has_moved = true; 
                        }
                        return (true, "".to_string())
                    }
                }
            }
            (false, "Invalid move".to_string())
        }

        pub fn get_all_valid_moves(&mut self, positions: Vec<String>) -> Vec<String>{
            let mut valid_moves: Vec<String> = Vec::new(); 
            for pos1 in positions.iter(){
                for pos2 in positions.iter(){
                    let pos = pos1.to_owned()+&" ".to_string()+&pos2.to_owned(); 
                    let (valid, _) = self.check_if_legal_move(&pos, false);
                    if valid{
                        valid_moves.push(pos);
                    }
                }
            }
            valid_moves
        }

        pub fn checkmate(&mut self) -> bool{
            let mut positions: Vec<String> = Vec::new(); 
            for i in 49..57{
                for j in 97..105{
                    let pos = position_to_string(i as u8, j as u8); 
                    positions.push(pos.to_owned());
                }
            }

            let valid_moves = self.get_all_valid_moves(positions); 

            if valid_moves.len() > 0{
                return false
            }

            true
        }

        pub fn move_check(&mut self, x_org: usize, y_org: usize, x_new: usize, y_new: usize) -> bool{
            let check; 
            
            if x_org != x_new || y_org != y_new{
                let old_square = self.grid[y_new][x_new]; 
                self.grid[y_new][x_new] = self.grid[y_org][x_org]; 
                self.grid[y_org][x_org] = Square::get_empty(); 

                check = self.check(); 

                self.grid[y_org][x_org] = self.grid[y_new][x_new]; 
                self.grid[y_new][x_new] = old_square;
            } 
            else{
                check = self.check(); 
            }

            check
        }

        pub fn check(&self) -> bool{
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
