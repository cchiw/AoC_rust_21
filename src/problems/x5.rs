use crate::shared::point::Point;
use crate::shared::readings::read_points;
const INPUT: &str = "src/data/input_x5.txt";

use std::collections::HashMap;

pub fn implment(is_diag: bool) {
    let mut coords: Vec<Vec<Point>> = Vec::new();
    read_points(INPUT, &mut coords);

    let mut v = Vec::new();
    for line in coords {
        Point::makeline(&line[0], &line[1], &mut v, is_diag);
    }

    let mut seen = HashMap::new();
    let mut counter = 0; //seen twice
    for p in &v {
        let inc = match seen.get(p) {
            Some(1) => {
                counter += 1;
                2
            }
            Some(n) => n + 1,
            None => 1,
        };
        seen.insert(p, inc);
    } // points end
    println!("counter {:?}", counter);
} // p1 end

pub fn p1() {
    implment(false);
}

pub fn p2() {
    implment(true);
}
