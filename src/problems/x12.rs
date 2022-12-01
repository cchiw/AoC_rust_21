use fs::File;
use regex::Regex;
use std::any::type_name;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

//const INPUT: &str = "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x12.txt";
pub fn mk_edges(edges: &mut HashMap<String, HashSet<String>>) {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\w*)-(\w*)").unwrap();
    //2,2 -> 2,1
    for line in reader.lines() {
        let content = line.unwrap();
        let group = re.captures(&content).unwrap();
        edges
            .entry(group[2].to_string())
            .or_insert_with(HashSet::new)
            .insert(group[1].to_string());
        edges
            .entry(group[1].to_string())
            .or_insert_with(HashSet::new)
            .insert(group[2].to_string());
    }
}

pub fn paths(limit_visit: bool) {
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    //or_insert_with: Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry.
    mk_edges(&mut edges);
    let mut all_paths: Vec<(Vec<String>, bool)> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    let mut start: String = "start".to_string();
    let end: String = "end".to_string();
    path.push(start);
    all_paths.push((path, limit_visit));
    let mut total = 0;
    let mut done: HashSet<Vec<String>> = HashSet::new();
    while !all_paths.is_empty() {
        let mut tmp_all_paths: Vec<(Vec<String>, bool)> = Vec::new();
        for (current_paths, current_used_special) in &all_paths {
            if done.contains(current_paths) {
                continue;
            } else {
                done.insert((current_paths).to_vec());
            }
            let tail = current_paths.last().unwrap();
            match edges.get(tail) {
                None => continue,
                Some(connections) => {
                    for next in connections {
                        let first_letter = next.chars().nth(0).unwrap();
                        let mut new_used_special: bool = *current_used_special;
                        if next == &"start".to_string() {
                            continue;
                        } else if next == &end {
                            total += 1;
                        } else if first_letter.is_lowercase()
                            && current_paths.contains(next)
                            && *current_used_special == true
                        {
                            continue;
                        } else {
                            if first_letter.is_lowercase() && current_paths.contains(next) {
                                new_used_special = true;
                            }
                            let mut new_path = current_paths.clone();
                            new_path.push(next.to_string());
                            tmp_all_paths.push((new_path, new_used_special));
                        }
                    }
                }
            } // end match
        } //end for loop
        all_paths = tmp_all_paths;
    } //end while
    println!("total {:?}", total);
} //end p

pub fn p1() {
    paths(true);
}
pub fn p2() {
    paths(false);
}
