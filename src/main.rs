pub mod sudoku;
pub mod utils;

use crate::utils::{read_file, get_index_as};
use crate::sudoku::Grid;

fn main() {
    let sdk = read_file("model.txt");
    let sdk = Grid { content: sdk };
    let fec = sdk.first_empty_cell();
    if let Ok(c) = fec {
        let a = c;
        let b = get_index_as(3, 5, "square".to_string());
        println!("{:?}", b);
    }
}