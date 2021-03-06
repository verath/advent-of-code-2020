#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Debug)]
pub struct Line<'a> {
    pub min_num: i32,
    pub max_num: i32,
    pub letter: char,
    pub password: &'a str,
}

pub fn parse_line(line: &str) -> Option<Line> {
    lazy_static! {
        // 1-3 a: abcde
        static ref RE: Regex = Regex::new(
            r"^(?P<min_num>\d+)-(?P<max_num>\d+) (?P<letter>\w): (?P<password>\w+)$").unwrap();
    }
    let caps = RE.captures(line)?;
    let min_num = caps.name("min_num")?.as_str().parse::<i32>().ok()?;
    let max_num = caps.name("max_num")?.as_str().parse::<i32>().ok()?;
    let letter = caps.name("letter")?.as_str().chars().next()?;
    let password = caps.name("password")?.as_str();
    Some(Line {
        min_num,
        max_num,
        letter,
        password,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "1-3 a: abcde";
        let expected = Line {
            min_num: 1,
            max_num: 3,
            letter: 'a',
            password: "abcde",
        };
        assert_eq!(parse_line(line), Some(expected));
        assert_eq!(parse_line("Not a valid line"), None);
    }
}
