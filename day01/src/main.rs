use std::fs::File;
use std::io::prelude::*;

fn traverse_digits(input: &Vec<u8>) -> u32
{
    let mut first: u8 = 0;
    let mut last_traversed: u8 = 0;
    let mut sum: u32 = 0;
    let mut last: u8 = 0;
    for (i, digit) in input.iter().enumerate() {
        let d = *digit;
        if i == 0 {
            first = d;
        } else if last_traversed == d {
            sum += d as u32;
        }

        if input.len() > i {
            last = d;
        }

        last_traversed = d;
    }

    if last == first {
        sum += last as u32;
    }

    sum
}

pub fn main() {
    const RADIX: u32 = 10;

    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut input: Vec<u8> = Vec::new();

    for c in contents.chars() {
        let d: u8 = c.to_digit(RADIX).unwrap() as u8;
        input.push(d);
    }


    let sum = traverse_digits(&input);

    println!("{:?}", sum);
}
