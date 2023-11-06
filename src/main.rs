mod sudoku;

fn main() {
    let mut sudoku = sudoku::sudoku::Board::new();
    sudoku.print();
    println!();
    sudoku.generate_puzzle();
    sudoku.add_candidate(0, 0, 1);
    sudoku.add_candidate(0, 1, 2);
    sudoku.remove_candidate(0, 0, 1);
    sudoku.clear_candidates(0, 1);
    println!("{}", sudoku.is_solved());
    sudoku.print();
}
