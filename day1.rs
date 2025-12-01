fn main() {
    let dial = Dial::new();

}

struct Dial {
    position: u32,
    counter: u32,
}

struct Rotation {
    direction: Direction,
    ticks: u32,
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

    fn rotate(&mut self, rotation: Rotation) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_rotation() {
        let mut dial = Dial::new();
        dial.rotate(Rotation {
            direction: Direction.Left,
            ticks: 10,
        })

        assert_eq!(dial.position, 40)
    }
}