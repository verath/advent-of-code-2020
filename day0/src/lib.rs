pub const INPUT: &str = include_str!("../input.txt");

pub fn add_one(n: i64) -> i64 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(0), 1);
        assert_eq!(add_one(-1), 0);
    }
}
