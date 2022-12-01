use crate::shared::readings::read_grid;
use std::collections::HashSet;
type Tyv = i32;
type TypeGrid = Vec<Vec<Tyv>>;
type TyCoord = (usize, usize);
const INPUT: &str = "src/data/input_x11.txt";
const FLASHING: Tyv = i32::MAX;

pub fn printgrid(grid: &mut TypeGrid) {
    for row in grid {
        print!("\nrow: [");
        for item in row {
            if *item == FLASHING {
                print!(" *,");
            } else {
                print!(" {:?},", &item);
            }
        }
        print!("]");
    }
}
//bump up by one if in bounds
pub fn bump(grid: &mut TypeGrid, i: usize, j: usize, new_sparkles: &mut HashSet<TyCoord>) {
    if i >= 0 && i < grid.len() && j >= 0 && j < grid[0].len() {
        let item = grid[i][j];
        if item == 9 {
            //new sparkle

            new_sparkles.insert((i, j));
            grid[i][j] = FLASHING;
        } else if item == FLASHING { //already sparkling
        } else {
            grid[i][j] += 1;
        }
    }
}
pub fn spread(grid: &mut TypeGrid, i: usize, j: usize, new_sparkles: &mut HashSet<TyCoord>) {
    if i > 0 {
        if j > 0 {
            bump(grid, i - 1, j - 1, new_sparkles);
        }
        bump(grid, i - 1, j, new_sparkles);
        bump(grid, i - 1, j + 1, new_sparkles);
    }
    if j > 0 {
        bump(grid, i, j - 1, new_sparkles);
        bump(grid, i + 1, j - 1, new_sparkles);
    }

    bump(grid, i, j + 1, new_sparkles);
    bump(grid, i + 1, j, new_sparkles);
    bump(grid, i + 1, j + 1, new_sparkles);
}
pub fn take_step(grid: &mut TypeGrid) -> Tyv {
    let mut sparkle = HashSet::new();
    //increment everything +1
    let mut i = 0;
    while i < grid.len() {
        let mut j = 0;
        while j < grid[0].len() {
            if grid[i][j] == 9 {
                grid[i][j] = FLASHING;
                sparkle.insert((i, j));
            } else {
                grid[i][j] += 1;
            }
            j += 1
        }
        i += 1
    }

    print!("\n\n *****************increment everything \n");
    printgrid(grid);

    while !sparkle.is_empty() {
        let mut new_sparkles = HashSet::new();
        for spark in &sparkle {
            //infect neighbors
            let (i, j) = &spark;

            spread(grid, *i, *j, &mut new_sparkles);
            print!("\n\n ***************** Spread spark  {:?}\n", &spark);
            printgrid(grid);
        }
        sparkle = new_sparkles;
        //print!("\nwhat is in sparkle {:?}", sparkle);
    }

    // set all highlighted pieces to zero
    let mut i = 0;
    let mut flashes = 0;
    while i < grid.len() {
        let mut j = 0;
        while j < grid[0].len() {
            if grid[i][j] == FLASHING {
                grid[i][j] = 0;
                flashes += 1;
            }
            j += 1
        }
        i += 1
    }
    print!("\n\n ***************** set FLASHING to 0\n");
    printgrid(grid);
    print!("flashes {:?}", flashes);
    return flashes;
}

pub fn p1() {
    let mut grid: TypeGrid = Vec::new();
    read_grid(INPUT, &mut grid);
    printgrid(&mut grid);
    let mut step = 0;
    let step_limit = 100;
    let mut flashes = 0;
    while step < step_limit {
        flashes += take_step(&mut grid);
        step += 1;
        print!("\n Step {:?}", step);
    }
    print!("Flashes {:?}", flashes);
}

pub fn p2() {
    let mut grid: TypeGrid = Vec::new();
    read_grid(INPUT, &mut grid);
    printgrid(&mut grid);
    let mut step = 0;
    let step_limit = 800;
    let mut flashes = 0;
    let num = grid.len() * grid[0].len();
    while step < step_limit {
        print!("\n Step {:?}", step);
        let current_flashes = take_step(&mut grid);
        step += 1;
        print!("\n Step {:?}", step);
        if current_flashes == num.try_into().unwrap() {
            print!("\nFound Moment");
            break;
        }
        flashes += current_flashes;
    }
    print!("Flashes {:?}", flashes);
}
