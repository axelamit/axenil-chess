use rust_chess::board;

fn main() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");
    //chess_board.print_board();
    //chess_board.get_moves("d4");
    chess_board.make_move("b1 c3");
}
