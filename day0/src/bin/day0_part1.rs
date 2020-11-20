fn day0_part1(input: &str) -> i64 {
    day0::add_one(input.len() as i64)
}

fn main() {
    let input = day0::INPUT.trim_end();
    println!("{:?}", day0_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day0_part1() {
        let input = day0::INPUT.trim_end();
        assert_eq!(day0_part1(input), 10);
    }
}
