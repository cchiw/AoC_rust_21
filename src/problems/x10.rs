use crate::shared::stack::Stack;
use fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
//const INPUT:&str = "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x10.txt";

pub fn p1p2() {
    let get_open = HashMap::from([(')', '('), ('}', '{'), ('>', '<'), (']', '[')]);
    let get_close = HashMap::from([('(', ')'), ('{', '}'), ('<', '>'), ('[', ']')]);
    let is_open = HashSet::from(['(', '{', '<', '[']);
    let p1_points = HashMap::from([(')', 3), ('}', 1197), ('>', 25137), (']', 57)]);
    let p2_points = HashMap::from([(')', 1), ('}', 3), ('>', 4), (']', 2)]);

    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);

    let mut part1_total = 0;
    let mut part2_scores = Vec::new();

    for line in reader.lines() {
        let mut stack1 = Stack::new();
        let mut current_total = 0;
        for item in line.unwrap().chars() {
            if is_open.contains(&item) {
                stack1.push(item);
            } else {
                match stack1.peek() {
                    None => {
                        current_total = *p1_points.get(&item).unwrap();
                        break;
                    }
                    Some(last_item) => {
                        if &last_item == get_open.get(&item).unwrap() {
                            stack1.pop();
                        } else {
                            current_total = *p1_points.get(&item).unwrap();
                            break;
                        }
                    }
                };
            }
        }
        if current_total > 0 {
            part1_total += current_total;
        } else {
            let mut points: i64 = 0;
            let vs: Vec<char> = stack1.stack;
            for item in vs.iter().rev() {
                let item_close = get_close.get(&item).unwrap();
                let val = p2_points.get(&item_close).unwrap();
                points = points * 5 + val;
            }
            part2_scores.push(points);
        }
    }
    part2_scores.sort();
    let n = part2_scores.len();
    print!("middle score {:?}", part2_scores[n / 2]);
}
