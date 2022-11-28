pub mod sudoku;
pub mod utils;

use crate::utils::read_file;

fn main() {
    let sdk = read_file("model.txt");
    for vec in sdk {
        println!("{:?}", vec);
    }
}