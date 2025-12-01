use std::fs;

fn parse_rotation(rotation: &str) -> (&str, u32) {
    let first_char = rotation.chars().next().unwrap();
    let (direction, distance) = rotation.split_at(first_char.len_utf8());
    let distance: u32 = distance.parse().unwrap();
    (direction, distance)
}

fn move_dial(dial: u32, rotation: &str) -> (u32, u32) {
    let (direction, distance) = parse_rotation(rotation);
    let upper = 99;
    let lower = 0;
    let modulo = upper + 1;
    let mut new_dial: i32 = dial as i32;
    let rotation_distance: i32 = distance as i32;
    let mut zero_crossings: u32 = 0;

    if direction == "R" {
        new_dial += rotation_distance;
        if new_dial > upper {
            zero_crossings = (new_dial / modulo) as u32;
            new_dial = new_dial % modulo;
        }
    } else {
        new_dial -= rotation_distance;
        if new_dial < lower {
            zero_crossings = (((-new_dial) + modulo - 1) / modulo) as u32;
            new_dial = ((new_dial % modulo) + modulo) % modulo;
        }
    }
    println!("rotation: {rotation}, new_dial: {new_dial}, zero_crossings: {zero_crossings}");
    (new_dial as u32, zero_crossings)
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
