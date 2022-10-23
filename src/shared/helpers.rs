pub fn vec_sum(frequency: &Vec<i64>) -> i64 {
    let mut total = 0;
    for v in frequency {
        total += v;
    }
    return total;
}
