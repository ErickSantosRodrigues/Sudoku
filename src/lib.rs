use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Sudoku {
    map: Vec<Vec<i32>>,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku {
            map: vec![vec![0; 9]; 9],
        };
        sudoku.generate_valid_game();
        sudoku
    }

    fn generate_valid_game(&mut self) {
        self.fill_diagonal_boxes();

        self.solve_remaining(0, 3);
    }

    fn fill_diagonal_boxes(&mut self) {
        let mut rng = rand::thread_rng();
        for box_idx in 0..3 {
            let mut numbers: Vec<i32> = (1..=9).collect();
            numbers.shuffle(&mut rng);

            let start_row = box_idx * 3;
            let start_col = box_idx * 3;

            let mut index = 0;
            for i in 0..3 {
                for j in 0..3 {
                    self.map[start_row + i][start_col + j] = numbers[index];
                    index += 1;
                }
            }
        }
    }

    fn solve_remaining(&mut self, i: usize, j: usize) -> bool {
        let mut i = i;
        let mut j = j;

        if j >= 9 {
            j = 0;
            i += 1;
            if i >= 9 {
                return true;
            }
        }

        // Skip already filled cells (diagonal boxes)
        if i < 3 && j < 3 {
            return self.solve_remaining(i, j + 1);
        } else if i >= 3 && i < 6 && j >= 3 && j < 6 {
            return self.solve_remaining(i, j + 1);
        } else if i >= 6 && j >= 6 {
            return self.solve_remaining(i, j + 1);
        }

        let mut numbers: Vec<i32> = (1..=9).collect();
        let mut rng = rand::thread_rng();
        numbers.shuffle(&mut rng);

        for &num in &numbers {
            if self.is_safe(i, j, num) {
                self.map[i][j] = num;

                if self.solve_remaining(i, j + 1) {
                    return true;
                }

                self.map[i][j] = 0;
            }
        }

        false
    }

    fn is_safe(&self, row: usize, col: usize, num: i32) -> bool {
        for j in 0..9 {
            if self.map[row][j] == num {
                return false;
            }
        }

        for i in 0..9 {
            if self.map[i][col] == num {
                return false;
            }
        }
        let box_row = row - row % 3;
        let box_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.map[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn print_game(&self) {
        for row in &self.map {
            for &num in row {
                print!("| {:2} |", num);
            }
            println!();
        }
        println!();
    }
}
