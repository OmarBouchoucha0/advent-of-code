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
            zero_crossings = (((-new_dial) + modulo) / modulo) as u32;
            new_dial = ((new_dial % modulo) + modulo) % modulo;
            if dial == 0 {
                zero_crossings -= 1;
            }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_right_no_crossing() {
        let (pos, crossings) = move_dial(50, "R30");
        assert_eq!(pos, 80);
        assert_eq!(crossings, 0);
    }

    #[test]
    fn test_simple_left_no_crossing() {
        let (pos, crossings) = move_dial(50, "L30");
        assert_eq!(pos, 20);
        assert_eq!(crossings, 0);
    }

    #[test]
    fn test_right_single_crossing() {
        let (pos, crossings) = move_dial(50, "R50");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_right_single_crossing_from_99() {
        let (pos, crossings) = move_dial(99, "R1");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_right_multiple_crossings() {
        let (pos, crossings) = move_dial(50, "R150");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 2);
    }

    #[test]
    fn test_right_multiple_crossings_land_mid() {
        let (pos, crossings) = move_dial(50, "R175");
        assert_eq!(pos, 25);
        assert_eq!(crossings, 2);
    }

    #[test]
    fn test_left_single_crossing() {
        let (pos, crossings) = move_dial(10, "L15");
        assert_eq!(pos, 95);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_left_multiple_crossings() {
        let (pos, crossings) = move_dial(10, "L110");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 2);
    }

    #[test]
    fn test_left_multiple_crossings_land_mid() {
        let (pos, crossings) = move_dial(10, "L115");
        assert_eq!(pos, 95);
        assert_eq!(crossings, 2);
    }

    #[test]
    fn test_left_exactly_100() {
        let (pos, crossings) = move_dial(0, "L100");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_right_exactly_100() {
        let (pos, crossings) = move_dial(0, "R100");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_right_exactly_200() {
        let (pos, crossings) = move_dial(0, "R200");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 2);
    }

    #[test]
    fn test_edge_case_99_to_0() {
        let (pos, crossings) = move_dial(99, "R1");
        assert_eq!(pos, 0);
        assert_eq!(crossings, 1);
    }

    #[test]
    fn test_edge_case_0_to_99() {
        let (pos, crossings) = move_dial(0, "L1");
        assert_eq!(pos, 99);
        assert_eq!(crossings, 0);
    }

    #[test]
    fn test_parse_rotation() {
        let (dir, dist) = parse_rotation("R50");
        assert_eq!(dir, "R");
        assert_eq!(dist, 50);

        let (dir, dist) = parse_rotation("L123");
        assert_eq!(dir, "L");
        assert_eq!(dist, 123);
    }
    #[test]
    fn test_sequence() {
        let mut dial: u32 = 50;
        let mut password: u32 = 0;
        let mut zero;
        let input = "L68 L30 R48 L5 R60 L55 L1 L99 R14 L82";
        for rotation in input.split_whitespace() {
            (dial, zero) = move_dial(dial, rotation);
            password += zero;
            if dial == 0 {
                password += 1;
            }
        }
        assert_eq!(password, 6);
        assert_eq!(dial, 32);
    }
}
