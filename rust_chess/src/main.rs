use rust_chess::board;

fn main() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/checkmate_test.txt");
    //chess_board.fill_board("./data/castling_test.txt"); 
    //chess_board.get_moves("d4");
    //chess_board.make_move("f2 f4");
    //chess_board.make_move("b2 b4");
    //chess_board.make_move("O-O-O"); 

    let (gamestate, invalid_move, message) = chess_board.make_move("h3 h2"); 
    println!("Gamestate: {}, Invalid_move: {}, Message: {}", gamestate, invalid_move, message); 
    //chess_board.make_move("e4 e5")
    
    //println!("Chess? : {}", chess_board.chess()); 
}
