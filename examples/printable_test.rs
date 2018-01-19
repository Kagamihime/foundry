extern crate rlife;

use std::process;

use rlife::Grid;

fn main() {
    let mut grid = match Grid::from_file(&String::from("test.life")) {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        },
        Ok(grid) => grid,
    };

    println!("{}", grid);
    for _ in 0..5 {
        grid = grid.next_gen();
        println!("{}", grid);
    }
}
