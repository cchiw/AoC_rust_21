use core::cmp::max;
use core::cmp::min;
use fs::File;
use regex::internal::Char;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
type Ty = char;
//type TyP = Vec<char>;

//const INPUT: &str = "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x14.txt";

pub fn take_step(phrase: &mut String, insertions: &mut HashMap<(Ty, Ty), Ty>) -> String {
    let mut out = Vec::new();
    let mut char_2: Ty = '|';

    for char_1 in phrase.chars() {
        if let Some(char_i) = insertions.get(&(char_2, char_1)) {
            out.push(*char_i);
        }
        char_2 = char_1;
        out.push(char_1);
    }
    out.into_iter().collect()
}

pub fn core(step_max: usize) {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let re = Regex::new(r"([A-Z])([A-Z]) -> (\w)").unwrap();
    let mut phrase: String = "".to_string();
    let mut insertions: HashMap<(Ty, Ty), Ty> = HashMap::new();
    let mut index = 0;
    println!("Reading lines");
    for line in reader.lines() {
        let content = line.unwrap();
        if index == 0 {
            phrase = content.to_string();
        } else if index > 1 {
            match re.captures(&content) {
                Some(g) => {
                    let a: Ty = g[1].parse::<Ty>().unwrap();
                    let b: Ty = g[2].parse::<Ty>().unwrap();
                    let c: Ty = g[3].parse::<Ty>().unwrap();
                    insertions.insert((a, b), c);
                }
                None => {}
            };
        }
        index += 1;
    }
    println!("about to take steps");
    for step in 0..step_max {
        println!("step {:?}", step);
        phrase = take_step(&mut phrase, &mut insertions);
    }

    let n_length = phrase.len();
    //print!("\nlength{:?}", &n_length);

    let mut frequency: HashMap<Ty, i64> = HashMap::new();
    for c in phrase.chars() {
        match frequency.get(&c) {
            None => frequency.insert(c.clone(), 0),
            Some(val) => frequency.insert(c.clone(), val + 1),
        };
    }
    let mut min_frequency = i64::MAX;
    let mut max_frequency = 1;
    for key in frequency.keys() {
        let f = frequency.get(key).unwrap();
        min_frequency = min(*f, min_frequency);
        max_frequency = max(*f, max_frequency);
    }
    println!(
        "  min {:?} max {:?} diff {:?}",
        min_frequency,
        max_frequency,
        max_frequency - min_frequency
    );
}

pub fn p1() {
    core(10);
}
pub fn p2() {
    core(40);
}
