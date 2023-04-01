use rand::Rng;

#[derive(Debug)]
pub struct Sudoku {
    map: Vec<Vec<i32>>
}

impl Sudoku {
    pub fn new() -> Sudoku { 
        let mut sudoku = Sudoku {map: Vec::new()};
        let mut rng = rand::thread_rng();
        for _a in 0..9{
            let mut new: Vec<i32> = Vec::new();
            for _b in 0..9{
            new.push(rng.gen_range(1..=9));
            }
        sudoku.map.push(new);
        }
        sudoku
    }
    pub fn print_game(&self) {
        self.map.iter()
            .map(| x| x.into_iter()
                       .map(|y| y)
                       .for_each(|t| print!("| {:?} |", t)))
            .for_each(|()| println!(""));
    }

}
