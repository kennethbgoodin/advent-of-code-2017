use std::fs::File;
use std::io::prelude::*;

extern crate common;

fn traverse_digits(input: &Vec<u8>, part: usize) -> u32
{
    let mut sum: u32 = 0;

    let half = if part == 0 {
        1
    } else {
        input.len() / 2
    };

    for (i, digit) in input.iter().enumerate() {
        let d = *digit;
        let next = if i + half < input.len() {
            input[i + half]
        } else {
            let diff = input.len() - i;
            input[half - diff]
        };

        if next == d {
            sum += d as u32;
        }
    }

    sum
}

pub fn main() {
    const RADIX: u32 = 10;

    let part = common::get_part();

    let mut input_text = common::read_input("input.txt");
    let mut input: Vec<u8> = Vec::new();
    for c in input_text.chars() {
        let d: u8 = c.to_digit(RADIX).unwrap() as u8;
        input.push(d);
    }

    let sum = traverse_digits(&input, part);

    println!("sum: {:?}", sum);
}
