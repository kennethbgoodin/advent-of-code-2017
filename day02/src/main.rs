extern crate common;

fn checksum(input: Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;

    for vec in input.iter() {
        let mut lowest = 0;
        let mut highest = 0;
        for digit in vec.iter() {
            let d = *digit;
            println!("d: {:?}, highest: {:?}, lowest: {:?}", d, highest, lowest);
            if highest == 0 || d > highest {
                highest = d;
            }

            if lowest == 0 || d < lowest {
                lowest = d;
            }

            println!("highest: {:?}", highest);
            println!("lowest: {:?}", lowest);
        }

        sum += highest as u32 - lowest as u32;
        println!("cur sum: {:?}", sum);
        println!("\n")
    }

    sum
}

pub fn main() {
    let input_text = common::read_input("input.txt");
    let mut input: Vec<Vec<u32>> = Vec::new();

    for (i, s) in input_text.split("\n").enumerate() {
        for num in s.split_whitespace() {
            let d: u32 = num.parse().unwrap();
            match input.get_mut(i) {
                Some(ref mut v) => {
                    v.push(d);
                    continue;
                }
                None => { }
            }
            let mut v = Vec::new();
            v.push(d);
            input.push(v);
        }
    }

    println!("{:?}", input);

    let checksum = checksum(input);
    println!("checksum: {:?}", checksum);
}
