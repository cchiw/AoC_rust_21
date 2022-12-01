use fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

//const INPUT: &str = "src/data/tmp.txt";
const INPUT: &str = "src/data/input_x8.txt";

pub fn p1() {
    let mut n_2 = 0; //length 2
    let mut n_3 = 0;
    let mut n_4 = 0;
    let mut n_7 = 0;

    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let re = Regex::new(
        r"(\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) \| (\w*) (\w*) (\w*) (\w*)",
    )
    .unwrap();
    for line in reader.lines() {
        let t1 = line.unwrap();
        let k = re.captures(&t1).unwrap();
        let mut index = 11;
        let index_limit = 14;
        while index <= index_limit {
            let output = &k[index];
            let n = output.len();
            if n == 2 {
                n_2 += 1;
            } else if n == 3 {
                n_3 += 1;
            } else if n == 4 {
                n_4 += 1;
            } else if n == 7 {
                n_7 += 1;
            }
            index += 1;
        }
    }
    let n_unique = n_2 + n_3 + n_4 + n_7;
    print!("\n\t {:?}", n_unique);
}

pub fn insert_map(
    a: &str,
    b: &str,
    c: &str,
    mapping: &mut HashMap<char, char>,
    //v_2: &str,
    v_3: &str,
    v_4: &str,
) {
    let mut left = false; // c, e,  a, d, g
                          //let mut middle = false; // c, f,  a, d, g
    let mut right = false; // b, f,  a, d, g
    for x in a.chars() {
        // print!("\n\tx-{:?}", x);
        match mapping.get(&x) {
            Some(_) => {}
            None => {
                if b.contains(x) && c.contains(x) {
                    //print!("SHARED");
                    //a, d, g
                    if v_3.contains(x) {
                        mapping.insert(x, 'a');
                    } else if v_4.contains(x) {
                        mapping.insert(x, 'd');
                    } else {
                        mapping.insert(x, 'g');
                    }
                } else if b.contains(x) || c.contains(x) {
                    //print!("Either");
                    if left {
                        mapping.insert(x, 'c');
                    } else if right {
                        mapping.insert(x, 'f');
                    } else {
                    }
                } else {
                    //print!("ONLY");
                    if v_4.contains(x) {
                        mapping.insert(x, 'b');
                        right = true;
                    } else {
                        mapping.insert(x, 'e');
                        left = true;
                    }
                }
            }
        }
    }

    for x in a.chars() {
        match mapping.get(&x) {
            None => {
                if left {
                    mapping.insert(x, 'c');
                } else if right {
                    mapping.insert(x, 'f');
                }
            }
            Some(_) => {}
        }
    }
}

pub fn redraw(line: &str) -> i32 {
    //let mut v_2 = ""; // {c, f}                  {c, f}
    let mut v_3 = ""; // {a, c, f}               {c, f}
    let mut v_4 = ""; // {b, c, d, f} -          {b, d}
    let mut v_5 = Vec::new(); //{c,e}  .. {c, f,} .. { b, f} All of them {a, d, g}
    let mut v_6 = Vec::new();
    //let mut v_7 = "";
    let mut mapping = HashMap::<char, char>::new();
    let re = Regex::new(
        r"(\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) (\w*) \| (\w*) (\w*) (\w*) (\w*)",
    )
    .unwrap();
    let k = re.captures(&line).unwrap();
    let mut index = 1;
    let index_limit = 10;
    while index <= index_limit {
        let output = &k[index];
        let n: usize = output.len();
        //if n == 2 {
        //    v_2 = output;
        if n == 3 {
            v_3 = output;
        } else if n == 4 {
            v_4 = output;
        } else if n == 5 {
            v_5.push(output);
        } else if n == 6 {
            v_6.push(output);
        } //else if n == 7 {
          //    v_7 = output;
          //}
        index += 1;
    }
    //look for d and g
    let v_5_1 = &v_5[0];
    let v_5_2 = &v_5[1];
    let v_5_3 = &v_5[2];
    insert_map(&v_5_1, &v_5_2, &v_5_3, &mut mapping, &v_3, &v_4);
    insert_map(&v_5_2, &v_5_1, &v_5_3, &mut mapping, &v_3, &v_4);
    insert_map(&v_5_3, &v_5_2, &v_5_1, &mut mapping, &v_3, &v_4);

    let n = mapping.len();
    if n < 7 {
        print!("incomplete");
    }
    let output1 = &k[11];
    let mut total = 1000 * translate_chunk(output1, &mut mapping);
    let output2 = &k[12];
    total += 100 * translate_chunk(output2, &mut mapping);
    let output3 = &k[13];
    total += 10 * translate_chunk(output3, &mut mapping);
    let output4 = &k[14];
    total += translate_chunk(output4, &mut mapping);
    print!("\ntotal {:?}", total);
    return total;
}

pub fn translate_chunk(output: &str, mapping: &mut HashMap<char, char>) -> i32 {
    let n: usize = output.len();
    let mut is_a = false;
    let mut is_b = false;
    let mut is_c = false;
    let mut is_d = false;
    let mut is_e = false;
    let mut is_f = false;
    let mut is_g = false;
    if n == 2 {
        1
    } else if n == 3 {
        7
    } else if n == 4 {
        4
    } else if n == 7 {
        8
    } else {
        for c in output.chars() {
            let t = mapping.get(&c).unwrap();
            //print!(" {:?}-> {:?}", c, t);
            if *t == 'a' {
                is_a = true;
            } else if *t == 'b' {
                is_b = true;
            } else if *t == 'c' {
                is_c = true;
            } else if *t == 'd' {
                is_d = true;
            } else if *t == 'e' {
                is_e = true;
            } else if *t == 'f' {
                is_f = true;
            } else if *t == 'g' {
                is_g = true;
            }
        }

        if n == 5 {
            if is_c && is_e {
                2
            } else if is_b && is_f {
                5
            } else if is_c && is_f {
                3
            } else {
                panic! {"Failure"};
            }
        } else {
            if is_a && is_b && is_c && is_d && is_f && is_g {
                9
            } else if is_a && is_b && is_d && is_e && is_f && is_g {
                6
            } else if is_a && is_b && is_c && is_e && is_f && is_g {
                0
            } else {
                panic! {"Failure"};
            }
        }
    }
}

pub fn p2() {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let mut v = Vec::new();
    let mut total = 0;
    for line in reader.lines() {
        let t1 = line.unwrap();
        v.push(t1);
    }
    for line in v {
        total += redraw(&line);
    }
    print!("total {:?}", total);
}
