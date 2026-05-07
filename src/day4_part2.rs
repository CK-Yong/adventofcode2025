use std::fs;

fn main() -> std::io::Result<()> {
    // Load the dataset
    let input = fs::read_to_string("src/day4.txt")?;

    let result = clean_up_paper_rolls(input.as_str());
    println!("Total number of papers: {}", result);

    Ok(())
}

fn clean_up_paper_rolls(input: &str) -> i32 {
    let mut sum = 0;
    let mut area = String::from(input);

    let mut width = 0;

    let mut str_val: String = String::new();

    for line in input.lines() {
        if width == 0 {
            // Initialise the width once
            width = line.len()
        }

        str_val = str_val + &line.trim()
    }

    loop {
        let result = count_paper_rolls(&str_val, width);
        sum = sum + result.0;

        println!("removed: {} - sum {}", result.0, sum);

        for idx in result.1 {
            area.replace_range(idx..idx, ".");
        }

        if result.0 == 0 {
            return sum;
        }
    }
}

fn count_paper_rolls(input: &str, width: usize) -> (i32, Vec<usize>) {
    let length = input.len();
    let mut sum = 0;
    let char_array = input.as_bytes();

    // println!("strVal: {}", str_val);

    let mut remove_targets = Vec::new();

    for (idx, val) in char_array.iter().enumerate() {
        print!("idx: {}", idx);

        // Surrounding rolls
        let mut sum_surr = 0;

        if *val as char != '@' {
            println!(" - Skipped");
            continue;
        }

        if idx >= width {
            // Count the row above
            if char_array[idx - width] as char == '@' {
                sum_surr = sum_surr + 1;
            }
        }

        // Case: We are not on the right boundary
        if idx % width < width - 1 {
            // Count the one on the right
            if char_array[idx + 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }

            // Check top right
            if idx > width && char_array[idx - width + 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }

            // Check bottom right
            if idx + width + 1 < length && char_array[idx + width + 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }
        }

        // Case: We are not on the left boundary
        if idx % width != 0 {
            // Count the one on the left
            if char_array[idx - 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }

            // Check top left
            if idx > width && char_array[idx - width - 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }

            // Check bottom left
            if idx + width < length && char_array[idx + width - 1] as char == '@' {
                sum_surr = sum_surr + 1;
            }
        }

        if idx < length - width {
            // Count the row on the bottom
            if char_array[idx + width] as char == '@' {
                sum_surr = sum_surr + 1;
            }
        }

        if sum_surr < 4 {
            sum = sum + 1;
            remove_targets.push(idx);
        }

        println!(", sum_sur: {}, sum: {}", sum_surr, sum)
    }

    println!("sum: {}", sum);

    return (sum, remove_targets);
}

mod tests {
    use super::*;

    #[test]
    fn test_count_paper_rolls() {
        let input = "..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.";

        let width = 10;
        let mut str_val: String = String::new();
        for line in input.lines() {
            str_val = str_val + &line.trim()
        }

        let mut result = count_paper_rolls(&str_val, width);
        assert_eq!(result.0, 13);
        assert_eq!(result.1, [2, 3, 5, 6, 8, 10, 26, 40, 49, 70, 90, 92, 98]);
    }

    #[test]
    fn run_main() {
        main();
    }
}
