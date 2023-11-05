extern crate rand;  // Add this line at the top of your file
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
        for i in 0..9 {
            if self.get(row, i).value == num {
                return false;
            }
        }
        // Check the column
        for i in 0..9 {
            if self.get(i, col).value == num {
                return false;
            }
        }
        // Check the 3x3 sub-grid
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
        for row in 0..9 {
            for col in 0..9 {
                let value = self.get(row, col).value;
                if value == 0 {
                    continue;
                }
                for i in 0..9 {
                    if i != col && self.get(row, i).value == value {
                        return false;
                    }
                    if i != row && self.get(i, col).value == value {
                        return false;
                    }
                }
                let block_row = row / 3;
                let block_col = col / 3;
                for i in 0..3 {
                    for j in 0..3 {
                        let row = block_row * 3 + i;
                        let col = block_col * 3 + j;
                        if row != row && col != col && self.get(row, col).value == value {
                            return false;
                        }
                    }
                }
            }
        }
        true
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
}

#[derive(Clone)]
pub struct Cell {
    value: u8,
    candidates: Vec<u8>,
}
