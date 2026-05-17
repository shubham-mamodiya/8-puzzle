use crate::board::Board;

mod board;
fn main() {
    let mut b = match Board::new(3) {
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

    b.print_it();
    println!("{}", b.humming());
    let mut c = match Board::new(3) {
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
    println!("{}", b.is_equal(&b));
    println!("{}", b.is_equal(&c));

    println!("{}", b.manhattan());
    b.shuffle();

    b.print_it();
    println!("{}", b.manhattan());
}
