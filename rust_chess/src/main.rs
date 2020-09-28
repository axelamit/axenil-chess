use rust_chess::board;
fn main() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");
}
