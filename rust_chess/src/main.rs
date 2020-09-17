use rust_chess::board;

fn main() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/passant_test.txt");
    chess_board.make_move("c4 c5"); 
    chess_board.make_move("b7 b5"); 
    chess_board.make_move("c5 b6");
    /*
    chess_board.make_move("b2 b3"); 
    chess_board.make_move("b7 b6");
    chess_board.make_move("b1 c3");
    chess_board.make_move("c8 a6"); */
    //chess_board.make_move("c8 a6");  
    
    //chess_board.fill_board("./data/castling_test.txt"); 
    //chess_board.get_moves("d4");
    //chess_board.make_move("f2 f4");
    //chess_board.make_move("b2 b4");
    //chess_board.make_move("O-O-O"); 

    //chess_board.make_move("h3 h2=R"); 
    //let (gamestate, invalid_move, message) = chess_board.make_move("h3 h2"); 
    //println!("Gamestate: {}, Invalid_move: {}, Message: {}", gamestate, invalid_move, message); 
    //chess_board.make_move("e4 e5")
    
    //println!("Chess? : {}", chess_board.chess()); 
}
