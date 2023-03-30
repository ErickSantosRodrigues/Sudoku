use rand::Rng;
// sudoku game 
// for each line, collunm and group of nine is need to generate some non repetitive number
// range from 1 to 9
// struct map {
//     vec<i32>
// }

fn print_game(map: Vec<Vec<i32>>) {
    
}

fn main() {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut rng = rand::thread_rng();
    for _a in 0..9{
        let mut new: Vec<i32> = Vec::new();
        for _b in 0..9{
            new.push(rng.gen_range(1..=9));
        }
        map.push(new);
    }

    println!("{:?}", map);

}
