use std::fs;

fn range_limits(range: &str) -> Vec<&str> {
    let limits: Vec<&str> = range.split("-").collect();
    limits
}

fn validate_id(id: i128) -> bool {
    let id = id.to_string();
    for i in 0..id.len() {
        let (left, right) = id.split_at(i);
        if left == right {
            return false;
        }
    }
    return true;
}

fn validate_range(range: &str) -> i128 {
    let limits = range_limits(range);
    let lower: i128 = limits[0].parse().unwrap();
    let upper: i128 = limits[1].parse().unwrap();
    let mut sum: i128 = 0;
    for id in lower..=upper {
        if !validate_id(id) {
            sum += id;
        }
    }
    sum
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for range in content.trim().split(",") {
        println!("Range : {range}");
        println!("Limits : {:?}", range_limits(range));
        println!("Sum : {}", validate_range(range));
        sum += validate_range(range);
    }
    println!("THE FINAL SUM : {sum}");
}
