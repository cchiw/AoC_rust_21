use fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

const INPUT: &str = "src/data/input_x2.txt";
const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

//1692075
//1749524700
pub fn p1() {
    let mut forward = 0;
    let mut down = 0;
    let file = File::open(INPUT).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let x = line.unwrap();
        let (direction, magnitude) = x.split_once(" ").unwrap();
        let m = &magnitude.parse::<i32>().unwrap();
        match direction {
            FORWARD => forward += m,
            UP => down -= m,
            DOWN => down += m,
            _ => panic!("error"),
        };
    }
    print!("\n{:?}\n", forward * down);
}

pub fn p2() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let file = File::open(INPUT).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let x = line.unwrap();
        let (direction, magnitude) = x.split_once(" ").unwrap();
        let m = &magnitude.parse::<i32>().unwrap();
        match direction {
            FORWARD => {
                horizontal += m;
                depth += aim * m;
            }
            UP => aim -= m,
            DOWN => aim += m,
            _ => panic!("error"),
        };
    }
    print!("\n{:?}\n", horizontal * depth);
}
