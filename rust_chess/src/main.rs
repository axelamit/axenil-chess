use rust_chess::board;

fn main() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    chess_board.make_move("e2 e4");
    chess_board.make_move("e7 e5");

    chess_board.make_move("g1 f3");
    chess_board.make_move("f7 f6");

    chess_board.make_move("f3 e5");
    chess_board.make_move("f6 e5");

    chess_board.make_move("d1 h5");
    chess_board.check();
    chess_board.make_move("e8 e7");

    chess_board.make_move("h5 e5");
    chess_board.check();
    chess_board.make_move("e7 f7");

    chess_board.make_move("f1 c4");
    chess_board.check();
    chess_board.make_move("d7 d5");

    chess_board.make_move("c4 d5");
    chess_board.check();
    chess_board.make_move("f7 g6");

    chess_board.make_move("h2 h4");
    chess_board.make_move("h7 h5");

    chess_board.make_move("d5 b7");
    chess_board.make_move("c8 b7");

    chess_board.make_move("e5 f5");
    chess_board.check();
    chess_board.make_move("g6 h6");

    chess_board.make_move("d2 d4");
    chess_board.check();
    chess_board.make_move("g7 g5");

    chess_board.make_move("f5 f7");
    chess_board.make_move("d8 e7");

    chess_board.make_move("h4 g5");
    chess_board.make_move("e7 g5");

    chess_board.make_move("h1 h5");
}
