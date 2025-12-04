use std::fs::{self};

fn main() {
    // Load the dataset
    let input = fs::read_to_string("src/day2.txt").expect("Should have been able to read the file.");

    let mut sum: u64 = 0;

    // Iterate over the input
    for range in input.split(',') {
        let boundaries: Vec<&str> = range.split('-').collect();
        let left = boundaries[0].to_string().parse::<u64>().unwrap();
        let right = boundaries[1].to_string().parse::<u64>().unwrap();

        let mut current = left;
        while current <= right {
            if is_invalid_id(current) {
                sum += current;
            }
            current += 1;
        }
    }

    println!("Done iterating. Sum of all invalid IDs: {}", sum)
}

fn is_invalid_id(val: u64) -> bool {
    let digits = val.to_string();
    let length = digits.len();
    if length % 2 != 0 {
        return false;
    }

    let left = &digits[0..(length / 2)];
    let right = &digits[(length / 2)..];

    return left == right;
}

#[test]
fn test_1() {
    main();
}
