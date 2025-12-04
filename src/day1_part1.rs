use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial = Dial::new();
    
    if let Ok(lines) = read_lines("day1.txt") {
        for line in lines.map_while(Result::ok) {
            let rotation = Rotation{
                direction: if line.starts_with('L') { Direction::Left } else { Direction::Right},
                ticks: line[1..].parse::<i32>().unwrap()
            };

            dial.rotate(rotation);
        }
    }

    println!("Dialing completed. The password is {}", dial.counter)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Dial {
    position: i32,
    counter: u32,
}

struct Rotation {
    direction: Direction,
    ticks: i32,
}

enum Direction {
    Left,
    Right,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            counter: 0,
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let mut ticks = rotation.ticks;
        match rotation.direction {
            Direction::Left => {
                while ticks > 0 {
                    if self.position - ticks < 0 {
                        ticks -= self.position + 1;
                        self.position = 99;
                    } else {
                        self.position -= ticks;
                        ticks = 0;
                    };
                }
            },
            Direction::Right => {
                while ticks > 0 {
                    if self.position + ticks > 99 {
                        ticks -= 99 - self.position + 1;
                        self.position = 0;
                    } else {
                        self.position += ticks;
                        ticks = 0;
                    };
                }
            },
        }
        
        if self.position == 0 {
            self.counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_rotation_to_position {
        ($($name:ident: $direction:expr, $ticks:literal, $expected_position:literal),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                let mut dial = Dial::new();

                dial.rotate(Rotation {
                    direction: $direction,
                    ticks: $ticks,
                });

                assert_eq!(dial.position, $expected_position);
                }
            )*
        }
    }

    test_rotation_to_position! {
        test_rotation_left_1: Direction::Left, 10, 40,
        test_rotation_left_2: Direction::Left, 20, 30,
        test_rotation_left_3: Direction::Left, 40, 10,
        test_rotation_left_4: Direction::Left, 50, 0,
        test_rotation_left_5: Direction::Left, 60, 90,

        test_rotation_right_1: Direction::Right, 10, 60,
        test_rotation_right_2: Direction::Right, 20, 70,
        test_rotation_right_3: Direction::Right, 40, 90,
        test_rotation_right_4: Direction::Right, 49, 99,
        test_rotation_right_5: Direction::Right, 60, 10,
    }


    macro_rules! test_rotation_to_counter {
        ($($name:ident: $direction:expr, $ticks:literal, $expected_count:literal),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                let mut dial = Dial::new();

                dial.rotate(Rotation {
                    direction: $direction,
                    ticks: $ticks,
                });

                assert_eq!(dial.counter, $expected_count);
                }
            )*
        }
    }

    test_rotation_to_counter! {
        test_counter_left_1: Direction::Left, 49, 0,
        test_counter_left_2: Direction::Left, 50, 1,
        test_counter_left_3: Direction::Left, 51, 0,

        test_counter_right_1: Direction::Right, 49, 0,
        test_counter_right_2: Direction::Right, 50, 1,
        test_counter_right_3: Direction::Right, 51, 0,
    }

    #[test]
    fn example_test(){
        let mut dial = Dial::new();
        dial.rotate(Rotation { direction: Direction::Left, ticks: 68 });
        assert_eq!(dial.position, 82);
        assert_eq!(dial.counter, 0);
        
        dial.rotate(Rotation { direction: Direction::Left, ticks: 30 });
        assert_eq!(dial.position, 52);
        assert_eq!(dial.counter, 0);
        
        dial.rotate(Rotation { direction: Direction::Right, ticks: 48 });
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 1);

        dial.rotate(Rotation { direction: Direction::Left, ticks: 5 });
        assert_eq!(dial.position, 95);
        assert_eq!(dial.counter, 1);

        dial.rotate(Rotation { direction: Direction::Right, ticks: 60 });
        assert_eq!(dial.position, 55);
        assert_eq!(dial.counter, 1);

        dial.rotate(Rotation { direction: Direction::Left, ticks: 55 });
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 2);

        dial.rotate(Rotation { direction: Direction::Left, ticks: 1 });
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 2);

        dial.rotate(Rotation { direction: Direction::Left, ticks: 99 });
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 3);

        dial.rotate(Rotation { direction: Direction::Right, ticks: 14 });
        assert_eq!(dial.position, 14);
        assert_eq!(dial.counter, 3);

        dial.rotate(Rotation { direction: Direction::Left, ticks: 82 });
        assert_eq!(dial.position, 32);
        assert_eq!(dial.counter, 3);
    }
}
