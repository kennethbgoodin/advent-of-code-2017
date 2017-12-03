use std;

pub fn get_part() -> usize {
    let mut args = std::env::args();
    assert!(args.len() == 2);
    let part = args.nth(1).expect("get_part: bad argument")
                .parse().expect("get_part: not a number");
    assert!(part == 1 || part == 2);
    part
}