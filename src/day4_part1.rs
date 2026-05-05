use std::{ fs };

fn main() -> std::io::Result<()> {
    // Load the dataset
    let input = fs::read_to_string("src/day4.txt")?;

    println!("Total number of papers: {}", count_paper_rolls(input.as_str()));

    Ok(())
}

fn count_paper_rolls(input: &str) -> i32 {
    let parsed = input.lines();

    let mut strVal: String = String::new();

    let mut width = 0;

    for line in parsed {
        if width == 0 {
            // Initialise the width once
            width = line.len()
        }

        strVal = strVal + &line.trim()
    }

    let length = strVal.len();
    let mut sum = 0;
    let char_array = strVal.as_bytes();

    println!("strVal: {}", strVal);

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
            if idx + width < length && char_array[idx + width + 1] as char == '@' {
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
            sum = sum + 1
        }

        println!(", sum_sur: {}, sum: {}", sum_surr, sum)
    }

    println!("sum: {}", sum);

    return sum;
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

        assert_eq!(count_paper_rolls(input), 13);
    }

    #[test]
    fn run_main() {
        main();
    }
}
