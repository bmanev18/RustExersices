pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(number: i32) -> i32 {
    number + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
