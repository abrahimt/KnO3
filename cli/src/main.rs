use kno3_chess_engine::Chessboard;
mod display;

    pub trait DisplayBoard {
        fn display(&self) -> String;
    }

    impl DisplayBoard for Chessboard {
        fn display(&self) -> String {
            todo!()
        }
    }

fn main() {
    let cb = Chessboard::new();
    cb.display();
    println!("Hello, world!");
}
