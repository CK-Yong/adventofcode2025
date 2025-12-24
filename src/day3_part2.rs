use std::{
    array,
    fs::File,
    io::{self, BufRead},
};

fn main() -> std::io::Result<()> {
    // Load the dataset
    let file = File::open("src/day3.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    for line in lines.map_while(Result::ok) {
        let joltage = find_highest_joltage(&line);
        sum += joltage;
        println!("line: {} - joltage: {}", line, joltage);
    }

    println!("The sum of all joltages is {}", sum);

    Ok(())
}

fn find_highest_joltage(val: &str) -> u64 {
    let mut buffer: [char; 12] = array::repeat('0');
    let length = val.len();
    let mut rotations = 0;

    let mut from = 0;
    for i in 0..11 {
        let mut max = '0';
        let mut index = 0;
        let potential: Vec<char> = val.chars().collect();
        for (idx, digit) in potential[from..length-i].iter().enumerate() {
            if buffer.len() + idx < length {
                break;   
            }
            
            if max < *digit {
                max = *digit;
                from = idx;
            }
        }

        buffer[i] = max;

    }

    let mut str = String::with_capacity(12);
    for ch in buffer {
        str.push(ch);
    }
    str.parse::<u64>().expect("Failed to parse result")
}

mod tests {
    use super::*;

    #[test]
    fn test_highest_joltage() {
        assert_eq!(find_highest_joltage("987654321111111"), 987654321111);
        assert_eq!(find_highest_joltage("234234234234278"), 434234234278);
    }

    #[test]
    fn test_highest_joltage_2() {
        assert_eq!(find_highest_joltage("811111111111119"), 811111111119);
    }

    #[test]
    fn test_highest_joltage_3() {
        assert_eq!(find_highest_joltage("818181911112111"), 888911112111);
        assert_eq!(find_highest_joltage("818181911112111111"), 911112111111);
    }

    #[test]
    fn run_main() {
        main();
    }
}
