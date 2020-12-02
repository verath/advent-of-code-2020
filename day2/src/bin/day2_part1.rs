fn is_valid(line: &day2::Line) -> bool {
    let letter_count = line.password.chars().filter(|&c| c == line.letter).count() as i32;
    letter_count >= line.min_num && letter_count <= line.max_num
}

fn day2_part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| day2::parse_line(l).expect("invalid line"))
        .filter(is_valid)
        .count() as i64
}

fn main() {
    let input = day2::INPUT.trim_end();
    println!("{:?}", day2_part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let valid = day2::parse_line("1-3 a: abcde").unwrap();
        let invalid = day2::parse_line("1-3 b: cdefg").unwrap();
        assert_eq!(is_valid(&valid), true);
        assert_eq!(is_valid(&invalid), false);
    }

    #[test]
    fn test_day2_part1_example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day2_part1(input), 2);
    }

    #[test]
    fn test_solve_day2_part1() {
        let input = day2::INPUT.trim_end();
        assert_eq!(day2_part1(input), 418);
    }
}
