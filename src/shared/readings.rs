use crate::shared::point::Point;
use fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
type Tyv = i32;

pub fn read_i64(v: &mut Vec<i64>, input: &str) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    *v = reader.lines().map(|x| x.unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
}

pub fn read_u32s(m: &mut Vec<Vec<u32>>, input: &str) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    const RADIX: u32 = 10;
    for line in reader.lines() {
        let t0 = line.unwrap();
        let t1 = t0.chars();
        let mut current: Vec<u32> = Vec::new();
        for t2 in t1 {
            let t3 = t2.to_digit(RADIX).unwrap();
            current.push(t3);
        }
        m.push(current);
    }
}

pub fn read_points(input: &str, coords: &mut Vec<Vec<Point>>) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    //2,2 -> 2,1
    for line in reader.lines() {
        let content = line.unwrap();
        let group = re.captures(&content).unwrap();
        let x1 = group[1].parse::<i64>().unwrap();
        let y1 = group[2].parse::<i64>().unwrap();
        let p1 = Point::new(x1, y1);
        let x2 = group[3].parse::<i64>().unwrap();
        let y2 = group[4].parse::<i64>().unwrap();
        let p2 = Point::new(x2, y2);
        let mut v: Vec<Point> = Vec::new();
        v.push(p1);
        v.push(p2);
        coords.push(v);
    }
}

pub fn read_frequency(input: &str, frequency: &mut Vec<i64>) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let t0 = line.unwrap();
        let mut iter = t0.split(",");
        let mut item = iter.next();
        while item != None {
            let t1 = item.unwrap().parse::<usize>().unwrap();
            frequency[t1] += 1;
            item = iter.next();
        }
    }
}

pub fn read_dict(input: &str, seen: &mut HashMap<i64, i64>) {
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let t0 = line.unwrap();
        let mut iter = t0.split(",");
        let mut item = iter.next();
        while item != None {
            let t1 = item.unwrap().parse::<i64>().unwrap();
            let t2 = match seen.get(&t1) {
                Some(n) => *n + 1,
                None => 1,
            };
            seen.insert(t1, t2);
            item = iter.next();
        }
    }
}

pub fn read_grid(input:&str, grid: &mut Vec<Vec<Tyv>>){
    let file = File::open(input).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut row: Vec<Tyv> = Vec::new();
        for c in line.unwrap().chars() {
            let t = c as Tyv - 0x30;
            row.push(t);
        }
        grid.push(row);
    }
}
