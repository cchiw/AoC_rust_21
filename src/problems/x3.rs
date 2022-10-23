use crate::shared::readings::read_u32s;
use crate::shared::tree::BST;
use fs::File;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

//const INPUT:&str= "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x3.txt";

pub fn p1() {
    let mut m: Vec<Vec<u32>> = Vec::new();
    read_u32s(&mut m, INPUT);
    let mut hm = HashMap::new();
    for vec in m {
        let mut index: u32 = 0;
        for element in vec {
            let key = (index, element);
            let value = match hm.get(&key) {
                None => 1,
                Some(v) => v + 1,
            };
            hm.insert(key, value);
            index += 1;
        }
    }
    let mut index = 0;
    let mut most: Vec<u32> = Vec::new();
    while index < u32::MAX {
        let key_0 = (index, 0);
        let key_1 = (index, 1);
        match (hm.get(&key_0), hm.get(&key_1)) {
            (None, None) => index = u32::MAX,
            (Some(_), None) => {
                most.push(0);
                index += 1;
            }
            (None, Some(_)) => {
                most.push(1);
                index += 1;
            }
            (Some(a), Some(b)) => {
                if a > b {
                    most.push(0);
                    index += 1;
                } else {
                    most.push(1);
                    index += 1;
                }
            } //end some case
        } //end match
    } //end index while loop

    let mut index = 0;
    let mut least: Vec<u32> = Vec::new();
    while index < u32::MAX {
        let key_0 = (index, 0);
        let key_1 = (index, 1);
        match (hm.get(&key_0), hm.get(&key_1)) {
            (None, None) => index = u32::MAX,
            (Some(_), None) => {
                least.push(0);
                index += 1;
            }
            (None, Some(_)) => {
                least.push(1);
                index += 1;
            }
            (Some(a), Some(b)) => {
                if a < b {
                    least.push(0);
                    index += 1;
                } else {
                    least.push(1);
                    index += 1;
                }
            } //end some case
        } //end match
    } //end index while loop
    let t2 = most
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");
    let gamma = u32::from_str_radix(&t2, 2).unwrap();
    let t3 = least
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");
    let epsilon = u32::from_str_radix(&t3, 2).unwrap();
    let power_consumption = gamma * epsilon;
    println!(
        "The Decimal equivalent is most{:?}, least {:?} power consump {:?}",
        gamma, epsilon, power_consumption
    );
}

pub fn p2() {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let mut tree = BST::new();

    for line in reader.lines() {
        let t0 = line.unwrap();
        let t1 = t0.chars();
        tree.insert(t1);
    }

    let oxygen_rating = tree.get_most_bin();
    let co2_rating = tree.get_least_bin();
    let lifesupport_rating = oxygen_rating * co2_rating;
    println!(
        "The Decimal equivalent is most{:?}, least {:?} lifesupport_rating {:?}",
        oxygen_rating, co2_rating, lifesupport_rating
    );
}
