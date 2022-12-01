use crate::shared::readings::read_i64;
use fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
const INPUT: &str = "src/data/input_x1.txt";

pub fn p1() {
    let file = File::open(INPUT).expect("File not found");
    let reader = BufReader::new(file);
    let data: Vec<i64> = reader
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut prev = data[0];
    let mut inc = 0;
    for current in data {
        if current > prev {
            inc += 1;
        }
        prev = current;
    }
    print!("inc:{:?}", inc);
}
pub fn p2() {
    let mut v: Vec<i64> = Vec::new();
    read_i64(&mut v, &INPUT);
    let mut inc = 0;
    let mut i = 0;
    while i + 3 < v.len() {
        let a = v[i];
        let b = v[i + 3];
        if a < b {
            inc += 1;
        }
        i += 1;
    }
    println!("inc:{}", inc);
}
