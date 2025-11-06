use core::panic;
use std::fs;

use regex::Regex;

fn check_syntax(word: &str) -> bool {
    if let Some(inner) = word.strip_prefix("mul(") {
        if inner.ends_with(')') {
            return true;
        }
    }
    false
}
fn check_ability(word: &str) -> bool {
    if word == "do()" {
        return true;
    } else if word == "dont()" {
        return true;
    }
    false
}
fn get_ability(word: &str) -> bool {
    if word == "do()" {
        return true;
    }
    false
}

fn calculate_result(word: &str) -> u64 {
    let Some(numbers) = word.strip_prefix("mul(") else {
        panic!("error");
    };
    let Some(numbers) = numbers.strip_suffix(")") else {
        panic!("error");
    };
    let numbers: Vec<&str> = numbers.split(',').collect();

    let x: u64 = match numbers[0].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("error"),
    };

    let y: u64 = match numbers[1].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("error"),
    };

    x * y
}

fn main() {
    // let input =
    //     String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let input = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => panic!("Error reading file : {e}"),
    };
    let mut mul_result: Vec<u64> = Vec::new();
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut ability = true;
    for mat in re.find_iter(&input) {
        if check_ability(mat.as_str()) {
            ability = get_ability(mat.as_str());
            println!("{:?}", ability);
        }
        if ability && check_syntax(mat.as_str()) {
            mul_result.push(calculate_result(&mat.as_str()));
        }
    }
    let final_sum: u64 = mul_result.iter().sum();
    println!("the sum is {final_sum}");
}
