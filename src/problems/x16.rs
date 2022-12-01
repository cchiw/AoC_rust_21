use core::str::Chars;
use fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

const INPUT: &str = "src/data/tmp.txt";
//const INPUT: &str = "src/data/input_x16.txt";

pub fn mk_translation(hm: &mut HashMap<char, String>) {
    hm.insert('0', "0000".to_string());
    hm.insert('1', "0001".to_string());
    hm.insert('2', "0010".to_string());
    hm.insert('3', "0011".to_string());
    hm.insert('4', "0100".to_string());
    hm.insert('5', "0101".to_string());
    hm.insert('6', "0110".to_string());
    hm.insert('7', "0111".to_string());
    hm.insert('8', "1000".to_string());
    hm.insert('9', "1001".to_string());
    hm.insert('A', "1010".to_string());
    hm.insert('B', "1011".to_string());
    hm.insert('C', "1100".to_string());
    hm.insert('D', "1101".to_string());
    hm.insert('E', "1110".to_string());
    hm.insert('F', "1111".to_string());
}

pub fn convert(val: &str) -> u32 {
    println!(" val :{:?}", val);
    let mut ttl = 0;
    let mut p = 0;
    for x in val.chars().rev() {
        ttl += x.to_digit(10).unwrap() * (2_u32.pow(p));
        p += 1;
    }
    let x = ttl.try_into().unwrap();
    println!("x {:?}", x);
    return x;
}

pub fn process_literal(item: &mut String, index_start: usize, index_chunk: usize) -> usize {
    //process chunks of 5
    let mut running = true;
    let mut index = index_start;
    let mut literal: String = "".to_string();
    while running {
        let new_literal = &item[index + 1..index + 5];
        literal.push_str(new_literal);
        if &item[index..index + 1] == "0" {
            running = false;
            let index_end = index + 5;
            let done = index_end - index_chunk;
            let buffer = 1;
            println!("LITERAL : {:?}\n", literal);
            return index_end + buffer;
        } else {
            index += 5;
        }
    }

    return index;
}

pub fn process_mode0(item: &mut String, index_start: usize, index_chunk: usize) -> usize {
    let index = index_start + 15;
    let total_length_of_bits = &item[index_start..index];
    println!("total_length of bits {:?}\n\n", total_length_of_bits);
    let index = process_literal(item, index, index_chunk);
    let index = process_literal(item, index, index_chunk);
    index
}

pub fn process_mode1(item: &mut String, index_start: usize, index_chunk: usize) -> usize {
    let mut index = index_start + 11;
    let num_of_sub_packets = convert(&item[index_start..index]);
    let mut count = 0;
    while count < num_of_sub_packets {
        index = process_literal(item, index, index_chunk);
        count += 1;
    }
    index
}

pub fn process_operator(item: &mut String, index: usize, index_chunk: usize) -> usize {
    if &item[index..index + 1] == "0" {
        println!(" mode 0 ");
        process_mode0(item, index + 1, index_chunk)
    } else {
        println!("mode 1");
        process_mode1(item, index + 1, index_chunk)
    }
}

pub fn new_chunk(item: &mut String, index_chunk: usize) -> usize {
    let index = index_chunk;
    let packet_version = &item[index..index + 3];
    println!("packet_version {:?}", packet_version);
    let type_id = &item[index + 3..index + 6];
    println!("type_id {:?}", type_id);
    let index = index + 6;
    if type_id == "100" {
        return process_literal(item, index, index_chunk);
    } else {
        return process_operator(item, index, index_chunk);
    }
}

pub fn p1() {
    print!("p16");
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(file);
    let mut item: String = "".to_string();
    let mut hm: HashMap<char, String> = HashMap::new();
    mk_translation(&mut hm);

    for line in reader.lines() {
        let content = line.unwrap();
        for c in content.chars() {
            item.push_str(hm.get(&c).unwrap());
        }
    }

    println!("\n\nitem {:?}", item);
    let mut index = 0;

    while index < item.len() {
        index = new_chunk(&mut item, index);
    }
}
