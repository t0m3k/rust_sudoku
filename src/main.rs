mod sudoku;

fn main() {
    let mut sudoku = sudoku::sudoku::Board::new();
    sudoku.print();
    println!();
    sudoku.generate_puzzle();
    sudoku.print();
}
