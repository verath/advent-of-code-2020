fn is_valid(line: &day2::Line) -> bool {
    let cs: Vec<char> = line.password.chars().collect();
    let pos1 = line.min_num as usize - 1; // 1-indexed
    let pos2 = line.max_num as usize - 1; // 1-indexed
    cs[pos1] == line.letter && cs[pos2] != line.letter
        || cs[pos2] == line.letter && cs[pos1] != line.letter
}

fn day2_part2(input: &str) -> i64 {
    input
        .lines()
        .map(|l| day2::parse_line(l).expect("invalid line"))
        .filter(is_valid)
        .count() as i64
}

fn main() {
    let input = day2::INPUT.trim_end();
    println!("{:?}", day2_part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // is valid: position 1 contains a and position 3 does not.
        let line = day2::parse_line("1-3 a: abcde").unwrap();
        assert_eq!(is_valid(&line), true);

        // is invalid: neither position 1 nor position 3 contains b.
        let line = day2::parse_line("1-3 b: cdefg").unwrap();
        assert_eq!(is_valid(&line), false);

        // is invalid: both position 2 and position 9 contain c.
        let line = day2::parse_line("2-9 c: ccccccccc").unwrap();
        assert_eq!(is_valid(&line), false);
    }

    #[test]
    fn test_day2_part2_example() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day2_part2(input), 1);
    }

    #[test]
    fn test_solve_day2_part2() {
        let input = day2::INPUT.trim_end();
        assert_eq!(day2_part2(input), 616);
    }
}
