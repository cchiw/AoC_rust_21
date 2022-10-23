use crate::shared::helpers::vec_sum;
use crate::shared::readings::read_frequency;
const INPUT: &str = "src/data/input_x6.txt";

pub fn timepass(frequency: &mut Vec<i64>) {
    let newbies = frequency[0];
    frequency.rotate_left(1);
    frequency[6] += newbies;
}

pub fn p1() {
    let mut frequency = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    read_frequency(INPUT, &mut frequency);
    let mut timestep = 0;
    let timelimit = 256;
    while timestep < timelimit {
        timepass(&mut frequency);
        timestep += 1;
    }
    let total = vec_sum(&frequency);
    println!("total {:?}", total);
}
