use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let rgx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(rgx.captures_iter(input).map(|cap| {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        a * b
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let rgx = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();
    Some(rgx.captures_iter(input).map(|cap| {
        let ins = &cap[1];
        if ins == "do" {
            enabled = true;
            return 0;
        } else if ins == "don't" {
            enabled = false;
            return 0;
        }
        if !enabled {
            return 0;
        }
        let a = cap[2].parse::<u32>().unwrap();
        let b = cap[3].parse::<u32>().unwrap();
        a * b
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
