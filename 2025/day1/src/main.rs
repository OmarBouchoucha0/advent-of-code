use std::fs;

fn parse_rotation(rotation: &str) -> (&str, u8) {
    let first_char = rotation.chars().next().unwrap();
    let (direction, distance) = rotation.split_at(first_char.len_utf8());
    let distance: u8 = distance.parse().unwrap();
    (direction, distance)
}

fn move_dial(dial: u8, rotation: &str) -> u8 {
    let (direction, distance) = parse_rotation(rotation);
    let upper: u8 = 99;
    let lower: u8 = 0;
    let mut sum = 0;
    if direction == "R" {
        sum = dial + distance;
        if sum >= upper {
            sum = sum - upper;
        }
    } else {
        sum = dial - distance;
        if sum <= lower {
            sum = sum + upper;
        }
    }
    sum
}

fn main() {
    let mut dial: u8 = 50;
    let mut password: u8 = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    for rotation in input.lines() {
        dial = move_dial(dial, rotation);
        if dial == 0 {
            password += 1;
        }
    }
    println!("Password : {password}");
}
