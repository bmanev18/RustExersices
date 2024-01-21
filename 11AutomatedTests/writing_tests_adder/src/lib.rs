pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn panics() {
    panic!("Panic!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Introduction and use of {panic!}
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Use of {assert!}
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Use of {asser_eq!} and {assert_ne}
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Custom failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting din not contain name, value was `{}`",
            result
        );
    }

    // Check for panic with {should_panic}
    #[test]
    #[should_panic(expected = "Panic!")]
    fn check_for_panic() {
        panics();
    }

    #[test]
    fn it_works()-> Result<(), String>{
        if 2+2 ==4 {
            Ok(())
        }else {
            Err(String::from("Does not equal four"))
        }
    }
}
