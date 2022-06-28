#[allow(dead_code)]

pub fn add(num_one: u8, num_two: u8) -> u8 {
    num_one + num_two
}

pub fn i_throw_an_error() {
    panic!("im an error message");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Rectangle;

    #[test]
    fn sum() {
        assert_eq!(2, add(1, 1));
        assert_ne!(2, add(2, 2));
    }

    #[test]
    fn larger_can_hold() {
        let small: Rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        let big: Rectangle = Rectangle {
            width: 20,
            height: 20,
        };
        assert!(big.can_hold(&small));
    }

    #[test]
    fn smaller_cannot_hold() {
        let small: Rectangle = Rectangle {
            width: 10,
            height: 10,
        };
        let big: Rectangle = Rectangle {
            width: 20,
            height: 20,
        };
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn greeting_contains_name() {
        let result: String = String::from("Bob");
        assert!(
            result.contains("Bob"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "im an error message")]
    fn should_throw() {
        i_throw_an_error();
    }
}
