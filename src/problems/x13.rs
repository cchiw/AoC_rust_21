use core::cmp::max;
use fs::File;
use regex::Regex;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
type Ty = usize;
const INPUT: &str = "src/data/tmp.txt";
//const INPUT: &str = "src/data/input_x13.txt";

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: Ty,
    pub y: Ty,
}
impl Point {
    pub fn new(x: Ty, y: Ty) -> Self {
        Point { x: x, y: y }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Fold {
    X(Ty),
    Y(Ty),
}

pub fn read_dots(input: &str, dots: &mut Vec<Point>, folds: &mut Vec<Fold>) -> (usize, usize) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    let re_d = Regex::new(r"(\d*),(\d*)").unwrap();
    let re_x = Regex::new(r"fold along x=(\d*)").unwrap();
    let re_y = Regex::new(r"fold along y=(\d*)").unwrap();
    let mut maxX = 0;
    let mut maxY = 0;

    for line in reader.lines() {
        let content = line.unwrap();
        match (
            re_d.captures(&content),
            re_x.captures(&content),
            re_y.captures(&content),
        ) {
            (Some(group), None, None) => {
                let x = group[1].parse::<Ty>().unwrap();
                let y = group[2].parse::<Ty>().unwrap();
                maxX = max(maxX, x);
                maxY = max(maxY, y);
                dots.push(Point::new(x, y));
            }
            (None, Some(group), None) => {
                let x = group[1].parse::<Ty>().unwrap();
                folds.push(Fold::X(x));
            }
            (None, None, Some(group)) => {
                let y = group[1].parse::<Ty>().unwrap();
                folds.push(Fold::Y(y));
            }
            (_, _, _) => println!("finished points"),
        }
    }
    maxX += 1;
    maxY += 1;
    return (maxX.into(), maxY.into());
}

pub fn fold_along(fold: Fold, grid1: &mut Vec<Vec<bool>>) {
    match fold {
        Fold::Y(val) => {
            let mut grid2 = Vec::new();
            for n in 0..val {
                grid2.push(vec![false; grid1[0].len()]);
            }
            for x in 0..grid1[0].len() {
                for y in 0..val {
                    grid2[val - y - 1][x] = grid1[val - y - 1][x] || grid1[val + y + 1][x];
                }
            }
            *grid1 = grid2.clone();
        } //match case
        Fold::X(val) => {
            let mut grid2 = Vec::new();
            for n in 0..grid1.len() {
                grid2.push(vec![false; val]);
            }
            for y in 0..grid1.len() {
                for x in 0..val {
                    grid2[y][val - x - 1] = grid1[y][val - x - 1] || grid1[y][val + x + 1];
                }
            }
            *grid1 = grid2.clone();
        } //match case
    } //match
} // fn
pub fn count_stars(grid: &Vec<Vec<bool>>) -> Ty {
    let mut count = 0;
    for row in grid {
        count += row.iter().filter(|x| **x).count();
    }
    return count;
}
pub fn print_stars(grid: &Vec<Vec<bool>>) {
    for row in grid {
        println!("");
        for i in row {
            if *i {
                print!("#");
            } else {
                print!(" ");
            }
        }
    }
}

pub fn p1p2() {
    let mut dots = Vec::new();
    let mut folds = Vec::new();
    let (nx, ny) = read_dots(INPUT, &mut dots, &mut folds);
    let mut grid = Vec::new();
    for n in 0..ny {
        grid.push(vec![false; nx]);
    }
    for point in &dots {
        grid[point.y][point.x] = true;
    }

    print_stars(&grid);
    for f in folds {
        println!("\n\n*************** Folding ******************");
        fold_along(f, &mut grid);
        println!("\nCount {:?}", count_stars(&grid));
        print_stars(&grid);
    }
}
