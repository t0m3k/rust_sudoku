use rand;  // Add this line at the top of your file
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![
                Cell {
                    value: 0,
                    candidates: Vec::new(),
                };
                81
            ],
        }
    }

    pub fn solve(&mut self) -> bool {
        // Find the first empty cell
        if let Some((row, col)) = self.find_empty() {
            // Try placing digits 1 to 9 in the cell, in a random order
            let mut digits: Vec<u8> = (1..=9).collect();
            digits.shuffle(&mut thread_rng());
            for &num in &digits {
                if self.is_safe(row, col, num) {
                    self.set(row, col, num);
                    if self.solve() {
                        // If solve() returns true, the solution is found
                        return true;
                    }
                    // Otherwise, backtrack by setting the cell to 0
                    self.set(row, col, 0);
                }
            }
        } else {
            // If no empty cell is found, the puzzle is solved
            return true;
        }
        false
    }

    pub fn generate_puzzle(&mut self) {
        // Step 1: Solve an empty board to generate a solved Sudoku grid
        self.solve();
        // Step 2: Remove some numbers to create a puzzle
        let mut rng = thread_rng();
        for _ in 0..40 {  // Remove 40 numbers as an example; adjust as needed
            let row = rng.gen_range(0..9);
            let col = rng.gen_range(0..9);
            self.set(row, col, 0);
        }
    }

    pub fn find_empty(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.get(row, col).value == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    pub fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        // Check the row
        if !self.row_is_safe(row, num) {
            return false;
        }
        // Check the column
        if !self.column_is_safe(col, num) {
            return false;
        }
        // Check the 3x3 sub-grid
        if !self.block_is_safe(row, col, num) {
            return false;
        }
        true
    }

    pub fn row_is_safe(&self, row: usize, num: u8) -> bool {
        // Check if `num` is in the row
        for i in 0..9 {
            if self.get(row, i).value == num {
                return false;
            }
        }
        true
    }

    pub fn column_is_safe(&self, col: usize, num: u8) -> bool {
        // Check if `num` is in the column
        for i in 0..9 {
            if self.get(i, col).value == num {
                return false;
            }
        }
        true
    }

    pub fn block_is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if `num` is in the 3x3 sub-grid
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.get(i + start_row, j + start_col).value == num {
                    return false;
                }
            }
        }
        true
    }


    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        // Set cell at `row`, `col` to `value`
        self.cells[row * 9 + col].value = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &Cell {
        // Get cell at `row`, `col`
        &self.cells[row * 9 + col]
    }

    pub fn is_solved(&self) -> bool {
        // Check if all cells are filled
        for row in 0..9 {
            for col in 0..9 {
                if self.get(row, col).value == 0 {
                    return false;
                }
            }
        }
        self.is_valid() & true
    }

    pub fn is_valid(&self) -> bool {
        // Check rows and columns
        for i in 0..9 {
            let mut row_digits: Vec<u8> = Vec::new();
            let mut col_digits: Vec<u8> = Vec::new();
            for j in 0..9 {
                row_digits.push(self.get(i, j).value);
                col_digits.push(self.get(j, i).value);
            }
            if !Board::is_valid_digits(row_digits) || !Board::is_valid_digits(col_digits) {
                return false;
            }
        }
        // Check 3x3 sub-grids
        for i in 0..3 {
            for j in 0..3 {
                let mut digits: Vec<u8> = Vec::new();
                for k in 0..3 {
                    for l in 0..3 {
                        digits.push(self.get(3 * i + k, 3 * j + l).value);
                    }
                }
                if !Board::is_valid_digits(digits) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_valid_digits(digits: Vec<u8>) -> bool {
        // Check if `digits` contains all digits from 1 to 9
        let mut digits = digits;
        digits.sort();
        digits.dedup();
        digits.len() == 9 && digits[0] == 1 && digits[8] == 9
    }

    pub fn print(&self) {
        for row in 0..9 {
            for col in 0..9 {
                if self.get(row, col).value == 0 {
                    print!(". ");
                } else {
                    print!("{} ", self.get(row, col).value);
                }
                if col % 3 == 2 {
                    print!(" ");
                }
            }
            if row % 3 == 2 {
                println!();
            }
            println!();
        }
    }

    pub fn add_candidate(&mut self, row: usize, col: usize, value: u8) {
        self.cells[row * 9 + col].candidates.push(value);
    }

    pub fn remove_candidate(&mut self, row: usize, col: usize, value: u8) {
        let index = self.cells[row * 9 + col]
            .candidates
            .iter()
            .position(|&x| x == value)
            .unwrap();
        self.cells[row * 9 + col].candidates.remove(index);
    }

    pub fn clear_candidates(&mut self, row: usize, col: usize) {
        self.cells[row * 9 + col].candidates.clear();
    }

}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    value: u8,
    candidates: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert_eq!(board.cells.len(), 81);
        for cell in board.cells.iter() {
            assert_eq!(cell.value, 0);
        }
    }

    #[test]
    fn test_row_is_safe() {
        let mut board = Board::new();
        board.set(0, 0, 5);
        assert!(!board.row_is_safe(0, 5));
        assert!(board.row_is_safe(0, 6));
    }

    #[test]
    fn test_column_is_safe() {
        let mut board = Board::new();
        board.set(0, 0, 5);
        assert!(!board.column_is_safe(0, 5));
        assert!(board.column_is_safe(0, 6));
    }

    #[test]
    fn test_block_is_safe() {
        let mut board = Board::new();
        board.set(0, 0, 5);
        assert!(!board.block_is_safe(1, 1, 5));
        assert!(board.block_is_safe(1, 1, 6));
    }

    #[test]
    fn test_is_safe() {
        let mut board = Board::new();
        board.set(0, 0, 5);
        assert!(!board.is_safe(0, 1, 5)); // Same row
        assert!(!board.is_safe(1, 0, 5)); // Same column
        assert!(!board.is_safe(1, 1, 5)); // Same block
        assert!(board.is_safe(0, 1, 6));  // Different number
    }

    #[test]
    fn test_solve() {
        let mut board: Board = Board::new();
        assert!(board.solve()); // Should solve an empty board
        assert!(board.is_solved());
        board.set(0, 0, 0);
        assert!(board.solve()); // Should still solve the board
        assert!(board.is_solved());
    }

    #[test]
    fn test_generate_puzzle() {
        let mut board = Board::new();
        board.generate_puzzle();
        let filled_cells = board.cells.iter().filter(|c| c.value != 0).count();
        assert_ne!(filled_cells, 81); // Make sure it's not fully filled
        assert!(filled_cells > 0);    // And not empty
    }
}
