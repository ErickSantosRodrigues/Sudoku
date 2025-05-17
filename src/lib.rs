use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Sudoku {
    map: Vec<Vec<i32>>,
    solution: Vec<Vec<i32>>,
}
pub struct Solver;

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku {
            map: vec![vec![0; 9]; 9],
            solution: vec![vec![0; 9]; 9],
        };
        sudoku.generate_valid_game();
        sudoku
    }

    fn generate_valid_game(&mut self) {
        self.fill_diagonal_boxes();

        self.solve_remaining(0, 3);
        self.solution = self.map.clone();
        self.remove_k_cells(40); // Cria o puzzle
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

    fn remove_k_cells(&mut self, k: usize) {
        let mut rng = rand::thread_rng();
        let mut positions: Vec<(usize, usize)> = (0..9)
            .flat_map(|i| (0..9).map(move |j| (i, j)))
            .collect();
        positions.shuffle(&mut rng);
        for &(i, j) in positions.iter().take(k) {
            self.map[i][j] = 0;
        }
    }

    pub fn guess(&self, row: usize, col: usize, value: i32) -> bool {
        if self.map[row][col] != 0 {
            return false; 
        }
        self.solution[row][col] == value
    }

}



impl Solver {
    pub fn new() -> Solver {
        Solver {}
    }
    fn solutioner( sudoku: Sudoku, solved: Arc<Mutex<bool>>) -> () {
        let mut rng = rand::thread_rng();
        let mut l_sudoku = sudoku.clone();
        let mut empty: Vec<(usize, usize)> = vec![];

        let mut attempts: usize = 0;

        // acha todas as posições vazias
        for i in 0..9 {
            for j in 0..9 {
                if l_sudoku.map[i][j] == 0 {
                    empty.push((i, j));
                }
            }
        }
    
        loop {
            if *solved.lock().unwrap() {
                return ;
            }
    
            // Embaralhar a ordem de preenchimento
            empty.shuffle(&mut rng);

            attempts += 1;
            if !empty.is_empty() {
                let (row, col) = empty[empty.len() - 1];
                let value = rng.gen_range(1..=9);


                if l_sudoku.guess(row, col, value) {
                    empty.pop(); // Remove a posição preenchida
                    l_sudoku.map[row][col] = value;

                    if empty.is_empty() {
                        let mut solved = solved.lock().unwrap();
                            *solved = true;
                            println!("Solução encontrada em {} tentativas!", attempts);
                            l_sudoku.print_game();
                            return;
                        }
                    }
                }
                // Checa se outra thread encontrou a solução
                if *solved.lock().unwrap() {
                    println!("Solução não encontrada!");
                    l_sudoku.print_game();
                    return ;
                }
                //thread::sleep(Duration::from_millis(1)); // Simulate work
            }
            
        }
    
    
    pub fn solve_with_threads(&self, sudoku: Sudoku) {
        let sudoku = sudoku.clone();
        let solved = Arc::new(Mutex::new(false));
        let mut handles = vec![];
    
        for _ in 0..4 {
            let sudoku_clone = sudoku.clone();
            let solved_clone = solved.clone();
    
            let handle = thread::spawn(move || {
                Solver::solutioner(sudoku_clone, solved_clone);
            });
            handles.push(handle);
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
    }
}



