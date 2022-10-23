use crate::shared::readings::read_dict;
use std::collections::HashMap;

const INPUT: &str = "src/data/input_x7.txt";
//const INPUT: &str = "src/data/tmp.txt";

type Ty = i64;

pub fn compute_p1(seen: &HashMap<Ty, Ty>, mark: Ty) -> Ty {
    let mut total = 0;
    for (val, key) in seen {
        let diff = key * (val - mark);
        if diff > 0 {
            total += diff;
        } else {
            total += diff * -1;
        }
    }
    return total;
}

pub fn compute_p2(seen: &HashMap<Ty, Ty>, mark: Ty) -> Ty {
    let mut total = 0;
    for (val, key) in seen {
        let mut diff = val - mark;
        if diff < 0 {
            diff = diff * -1;
        }
        let cost = diff * (diff + 1) / 2;
        total += key * cost;
    }
    return total;
}

pub fn crabs(part: Ty) {
    let mut seen = HashMap::new();
    read_dict(INPUT, &mut seen);
    println!("frequency {:?}", seen);

    let mut mark: Ty = 0;
    let limit: Ty = seen.len().try_into().unwrap();
    let mut lowest = Ty::MAX;
    while mark < limit {
        let current = {
            if part == 1 {
                compute_p1(&seen, mark)
            } else {
                compute_p2(&seen, mark)
            }
        };
        if current < lowest {
            lowest = current;
            mark += 1;
        } else {
            print!("Lowest {:?}", lowest);
            return;
        }
    }
}

pub fn p1() {
    crabs(1);
}
pub fn p2() {
    crabs(2);
}
