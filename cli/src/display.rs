use kno3_chess_engine::Chessboard;

trait DisplayBoard {
    fn display(&self) -> String;
}

impl DisplayBoard for Chessboard {
    fn display(&self) -> String {
        todo!()
    }
}
