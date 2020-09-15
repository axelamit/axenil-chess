use rust_chess::board;

fn main() {
    let mut chess_board = board::Board::init();
    //chess_board.fill_board("./data/board.txt");
    chess_board.fill_board("./data/castling_test.txt"); 
    //chess_board.get_moves("d4");
    //chess_board.make_move("f2 f4");
    //chess_board.make_move("b2 b4");
    chess_board.make_move("O-O"); 
    println!("Chess? : {}", chess_board.chess()); 
}
