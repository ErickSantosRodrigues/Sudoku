use rand::Rng;

#[derive(Debug)]
pub struct Sudoku {
    map: Vec<Vec<i32>>
}

impl Sudoku {
    pub fn new() -> Sudoku { 
        let mut sudoku = Sudoku {map: Vec::new()};
        for _a in 0..9 as usize{
            let mut new: Vec<i32> = Vec::new();
            for _b in 0..9 as usize{
                new.push(10);
            }
        sudoku.map.push(new);
        }
        sudoku.print_game();
        // for i in 0..9 {
        //     for j in 0..9 {
        //         sudoku.map[i][j] = (i * 3 + i / 3 + j) as i32 % 9 + 1;
        //     }
        //     sudoku.print_game();
        // }
        sudoku.valid_game();
        sudoku
    } 
 
    fn valid_game(&mut self) {
        let mut rng = rand::thread_rng();
        for a in 0..9 as usize{
            for b in 0..9 as usize{
                loop {
                    let new_nun = rng.gen_range(1..=9); //the error is in the generation order, once there is no options for the last
                                    //number it will never be able to finish the process since it can`t put any other used number in that location
                    let mut cont = 0;
                    cont += 1;
                    self.map[a][b] = new_nun;
                    if !self.check_for_retitive_nunbers(a , b) || cont > 10 { break; }
                }
                    println!("line: {}, column: {}, n: {}", a, b, self.map[a][b]); 
            }
            self.print_game();
            println!("finished line");
        }
    }

    fn check_for_retitive_nunbers(&self, line: usize, column: usize) -> bool {
        let number = self.map[line][column];
        for a in 0..9 {
            if a != line && self.map[a][column] == number { return true; }
        }
        for b in 0..9 {
            if b != column && self.map[line][b] == number { return true; }
        }
        let s_line = (line / 3) * 3;
        let s_column = (column / 3) * 3;
        for i in s_line..s_line + 3 {
            for j in s_column..s_column + 3 {
                if i != line && j != column && self.map[i][j] == self.map[line][column] {
                return true;
                }
            }
        }
        false

    }
    pub fn print_game(&self) {
        self.map.iter()
            .map(| x| x.into_iter()
                       .map(|y| y)
                       .for_each(|t| print!("| {:?} |", t)))
            .for_each(|()| println!(""));
        println!("");
    }


}
