use rust_chess::board;

fn main() {
    let chess_board = board::Board::init();
    chess_board.print_board(); 
}
