use crate::board::Board;

mod board;
fn main() {
    let b = match Board::new(0) {
        Ok(board) => board,
        Err(_e) => {
            println!("Error creating board with size 1");
            match Board::new(1) {
                Ok(board) => board,
                Err(_e) => {
                    return;
                }
            }
        }
    };

    println!(
        "board: {:?}\n size: {}\n dimension:{}",
        b.board(),
        b.size(),
        b.dimension()
    );
}
