advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut r1 = Vec::new();
    let mut r2 = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        r1.push(nums.next().unwrap_or(0));
        r2.push(nums.next().unwrap_or(0));
    }
    r1.sort_unstable();
    r2.sort_unstable();
    r1.iter()
        .zip(r2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut r1 = Vec::new();
    let mut r2 = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        r1.push(nums.next().unwrap_or(0));
        r2.push(nums.next().unwrap_or(0));
    }
    r1.iter()
        .map(|r1| r1 * u32::try_from(r2.iter().filter(|r2| **r2 == *r1).count()).unwrap())
        .sum::<u32>()
        .into()
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
