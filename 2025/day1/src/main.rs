use std::fs;

fn parse_rotation(rotation: &str) -> (&str, u32) {
    let first_char = rotation.chars().next().unwrap();
    let (direction, distance) = rotation.split_at(first_char.len_utf8());
    let distance: u32 = distance.parse().unwrap();
    (direction, distance)
}

fn move_dial(dial: u32, rotation: &str) -> (u32, u32) {
    let (direction, distance) = parse_rotation(rotation);
    let upper: i16 = 100;
    let lower: i16 = 0;
    let current_dial: i16 = dial as i16;
    let rotation_distance: i16 = distance as i16;
    let mut sum: i16;
    let mut zero: i16 = 0;

    if direction == "R" {
        sum = current_dial + rotation_distance;
        if sum > upper {
            zero += sum / upper;
        }
        sum = sum % upper;
    } else {
        sum = current_dial - rotation_distance;
        if sum < lower {
            zero += (sum / upper).abs();
        }
        sum = (sum % upper + upper) % upper;
    }
    (sum as u32, zero as u32)
}

fn main() {
    let mut dial: u32 = 50;
    let mut password: u32 = 0;
    let mut zero;
    let input = fs::read_to_string("input.txt").unwrap();
    for rotation in input.lines() {
        (dial, zero) = move_dial(dial, rotation);
        password += zero;
        if dial == 0 {
            password += 1;
        }
    }
    println!("Password : {password}");
}
