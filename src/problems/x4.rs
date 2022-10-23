use fs::File;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

//const INPUT:&str= "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x4.txt";

#[derive(Debug)]
pub struct Board {
    pub data: Vec<String>,
    pub timing: usize,
}
impl Board {
    pub fn when_win(
        positions: &Vec<usize>,
        new_data: &Vec<String>,
        calls: &HashMap<String, usize>,
    ) -> usize {
        let mut most = usize::MIN;
        for p1 in positions {
            let key = &new_data[*p1];
            let call = calls.get(key);
            //println!("p1 {:?} key {:?} call{:?}", p1, key, call);
            match call {
                None => {
                    return usize::MAX;
                }
                Some(timing) => {
                    most = cmp::max(most, *timing);
                }
            }
        }
        return most;
    }
    pub fn compute(&self, calls: &HashMap<String, usize>) -> usize {
        let mut sum = 0;
        let mut last_val = 0;
        for d1 in &self.data {
            let call = calls.get(d1);
            let d2 = d1.parse::<usize>().unwrap();
            //println!("\n-- board piece: {:?}",d2);
            match call {
                None => {
                    sum += d2;
                }
                Some(timing) => {
                    //print!("\t has timing {:?}", timing);
                    if timing > &self.timing {
                        sum += d2;
                    } else if timing == &self.timing {
                        last_val = d2;
                    }
                }
            }
        }
        //	println!("last val {:?} sum {:?}", last_val, sum);
        return last_val * sum;
    }
    pub fn process(data: &Vec<String>, calls: &HashMap<String, usize>) -> Self {
        //load each vector into a vector .
        let mut new_data = Vec::new();
        for t0 in data {
            let mut iter = t0.split(' ');
            let mut item = iter.next();
            while item != None {
                let tmp = item.unwrap().to_string();
                if tmp != "" {
                    new_data.push(tmp)
                };
                item = iter.next();
            }
        }
        //println!("new_data{:?} ", &new_data);
        //row 0
        let mut best_timing = usize::MAX;
        let mut ix = 0;
        while ix < 24 {
            let positions = vec![ix, ix + 1, ix + 2, ix + 3, ix + 4];
            let timing = Board::when_win(&positions, &new_data, &calls);
            if timing < best_timing {
                best_timing = timing;
            }
            ix = ix + 5;
            //println!(" timing {:?}", &timing);
        }
        // println!("best timing {:?}", &best_timing);
        let mut ix = 0;
        while ix < 5 {
            let positions = vec![ix, ix + 5, ix + 10, ix + 15, ix + 20];
            let timing = Board::when_win(&positions, &new_data, &calls);
            if timing < best_timing {
                best_timing = timing;
            }
            ix = ix + 1;
        }
        Board {
            data: new_data,
            timing: best_timing,
        }
    } //process end here
}

pub fn p1p2() {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut calls: HashMap<String, usize> = HashMap::new();
    let mut tmp = Vec::new();
    let mut boards = Vec::new();

    for line in reader.lines() {
        let t0 = line.unwrap();
        if index == 0 {
            let mut iter = t0.split(',');
            let mut item = iter.next();
            let mut place = 0;
            while item != None {
                calls.insert(item.unwrap().to_string(), place);
                place += 1;
                item = iter.next();
            }
        } else {
            let mval = (index - 1) % 6;
            if mval == 0 {
                if !tmp.is_empty() {
                    let t2 = Board::process(&tmp, &calls);
                    boards.push(t2);
                }
                tmp = Vec::new();
            } else {
                tmp.push(t0);
            }
            //print!("\t mod {:?}", mval);
        } //end if
          //print!("\t tmp {:?}", tmp);
        index = index + 1;
    } //end for loop
    println!("\nCalls {:?}", &calls);
    //println!("\n Boards {:?}", &boards);
    println!("\n Part 1 ");
    let mut best_board = &boards[0];
    let mut best_time = best_board.timing;
    for b in &boards {
        if b.timing < best_time {
            best_time = b.timing;
            best_board = b;
        }
    }
    println!("best_time {:?} best_board {:?}", best_time, best_board);
    let val = best_board.compute(&calls);
    println!("val {:?}", val);

    println!("\n Part 2 ");
    let mut worst_board = &boards[0];
    let mut worst_time = worst_board.timing;
    for b in &boards {
        if b.timing > worst_time {
            worst_time = b.timing;
            worst_board = b;
        }
    }
    println!("worst_time {:?} worst_board {:?}", worst_time, worst_board);
    let val = worst_board.compute(&calls);
    println!("val {:?}", val);
}
