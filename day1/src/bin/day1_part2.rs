fn day1_part2(input: &str) -> i64 {
    let vs: Vec<i64> = input
        .split('\n')
        .map(|s| s.parse::<i64>().expect("not a number"))
        .collect();

    for (i, v1) in vs.iter().enumerate() {
        for (j, v2) in vs.iter().enumerate() {
            if i == j {
                continue;
            }
            for (k, v3) in vs.iter().enumerate() {
                if k == i || k == j {
                    continue;
                }
                if v1 + v2 + v3 == 2020 {
                    return v1 * v2 * v3;
                }
            }
        }
    }
    panic!("no solution")
}

fn main() {
    let input = day1::INPUT.trim_end();
    println!("{:?}", day1_part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part2_example() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(day1_part2(input), 241861950)
    }

    #[test]
    fn test_solve_day1_part2() {
        let input = day1::INPUT.trim_end();
        assert_eq!(day1_part2(input), 232508760);
    }
}
