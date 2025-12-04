use std::fs::{self};

fn main() {
    // Load the dataset
    let input =
        fs::read_to_string("src/day2.txt").expect("Should have been able to read the file.");

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

    let mut seq = &digits[0..length / 2];

    while seq.len() > 0 {
        // Set current index for eval
        let mut idx = seq.len();

        if idx + seq.len() > length {
            // Prevent index from going out of bounds, just continue with a smaller seq.
            seq = &seq[0..seq.len() - 1];
            continue;
        }

        let mut is_valid_id = false;

        while idx + seq.len() <= digits.len() {
            let eval = &digits[idx..idx + seq.len()];

            if eval == seq {
                idx += seq.len();
            } else {
                is_valid_id = true;
                break;
            }
        }

        if idx != digits.len() && idx + seq.len() > digits.len(){
            // This is actually a case where we have an uneven amount of digits, but we are skipping the last comparison.
            is_valid_id = true;
        }

        if is_valid_id {
            seq = &seq[0..seq.len() - 1];
        } else {
            return true
        }
    }

    return false;
}

#[test]
fn test_1() {
    assert_eq!(is_invalid_id(565656), true);
    assert_eq!(is_invalid_id(5656565), false);
    assert_eq!(is_invalid_id(1010), true);
    assert_eq!(is_invalid_id(999), true);
    assert_eq!(is_invalid_id(38593859), true);
    assert_eq!(is_invalid_id(2121212121), true);
}
