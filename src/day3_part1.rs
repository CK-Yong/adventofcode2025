use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> std::io::Result<()> {
    // Load the dataset
    let file = File::open("src/day3.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut sum: u32 = 0;
    for line in lines.map_while(Result::ok) {
        let joltage = find_highest_joltage(&line);
        sum += joltage;
        println!("line: {} - joltage: {}", line, joltage);
    }

    println!("The sum of all joltages is {}", sum);

    Ok(())
}

fn find_highest_joltage(val: &str) -> u32 {
    let mut first_digit = '0';
    let mut second_digit = '0';

    for (idx, digit) in val.chars().enumerate() {
        if second_digit > first_digit {
            first_digit = second_digit;
            second_digit = digit;
            continue;
        }

        if digit > second_digit {
            if second_digit > first_digit {
                first_digit = second_digit;
            }
            second_digit = digit;
        } else if digit > first_digit {
            first_digit = second_digit;
            second_digit = digit;
        }
    }

    let mut str = String::with_capacity(2);
    str.push(first_digit);
    str.push(second_digit);
    str.parse::<u32>().expect("Failed to parse result")
}

mod tests {
    use super::*;

    #[test]
    fn test_highest_joltage() {
        assert_eq!(find_highest_joltage("987654321111111"), 98);
        assert_eq!(find_highest_joltage("234234234234278"), 78);
    }

    #[test]
    fn test_highest_joltage_2() {
        assert_eq!(find_highest_joltage("811111111111119"), 89);
        assert_eq!(find_highest_joltage("811111111111191"), 91);
    }

    #[test]
    fn test_highest_joltage_3() {
        assert_eq!(find_highest_joltage("818181911112111"), 92);
    }


    #[test]
    fn test_highest_joltage_4() {
        assert_eq!(find_highest_joltage("46434344354985593"), 99);
    }
    
    #[test]
    fn run_main() {
        main();
    }
}
