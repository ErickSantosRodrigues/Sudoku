use sudoku::{Sudoku, Solver};

fn main() {
    let sudoku = Sudoku::new();
    println!("Puzzle inicial:");
    sudoku.print_game();
    let solver = Solver::new();
    solver.solve_with_threads(sudoku);
}
