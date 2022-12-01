use crate::shared::readings::read_grid;
use core::cmp::max;
use core::cmp::min;
use fs::File;
use regex::internal::Char;
use regex::Regex;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

//const INPUT: &str = "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x15.txt";

type ty = i32;
type ty_position = (usize, usize);

pub fn p1() {
    let mut vals: Vec<Vec<ty>> = Vec::new(); //vec![vec![1,20,11], vec![3, 54,40]];
    read_grid(INPUT, &mut vals);
    let start = (0, 0);
    let nx = vals.len();
    let ny = vals[0].len();
    let end = (nx - 1, ny - 1);

    let mut min_risk: HashMap<ty_position, ty> = HashMap::new(); // risk from position_start to position.
    min_risk.insert(start, 0);

    let mut visited: HashSet<ty_position> = HashSet::new(); // everything that has been visited already

    let mut queue = BinaryHeap::new();
    queue.push((0, start));

    while !queue.is_empty() {
        let (_, position) = queue.pop().unwrap();
        visited.insert(position);
        //let min_val_for_position = min_risk.get(&position).unwrap();

        let x = position.0;
        let y = position.1;
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x + 1 < nx {
            neighbors.push((x + 1, y));
        }
        if y + 1 < ny {
            neighbors.push((x, y + 1));
        }
        println!("current position {:?}, neighbors {:?}", position, neighbors);
        //neighbors = neighbors.into_iter().filter(|x| !visited.contains(&x)).collect();

        for neighbor in neighbors {
            let new_risk = min_risk.get(&position).unwrap() + vals[neighbor.0][neighbor.1];
            if !min_risk.contains_key(&neighbor) || min_risk.get(&neighbor).unwrap() > &new_risk {
                min_risk.insert(neighbor, new_risk);
                queue.push((-new_risk, neighbor));
            }
        }
    }
    println!("min_risk {:?}", min_risk);
    println!("min_risk {:?}", min_risk.get(&(0, 0)).unwrap());
    println!("min_risk {:?}", min_risk.get(&end).unwrap());
}
