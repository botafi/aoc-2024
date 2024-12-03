advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect();
                let first_diff = nums[0] - nums[1];
                for i in 1..nums.len() {
                    let prev = nums[i - 1];
                    let n = nums[i];
                    let next = nums.get(i + 1);
                    if prev.abs_diff(n) < 1 || prev.abs_diff(n) > 3 {
                        return false;
                    }
                    if let Some(next) = next {
                        let next_diff = n - *next;
                        if next_diff < 0 && first_diff > 0 {
                            return false;
                        }
                        if next_diff > 0 && first_diff < 0 {
                            return false;
                        }
                        if next.abs_diff(n) < 1 || next.abs_diff(n) > 3 {
                            return false;
                        }
                    }
                }
                true
            })
            .count() as u32,
    )
}

fn solve(nums: &Vec<i32>) -> bool {
    let first_diff = nums[0] - nums[1];
    for i in 1..nums.len() {
        let prev = nums[i - 1];
        let n = nums[i];
        let next = nums.get(i + 1);
        if prev.abs_diff(n) < 1 || prev.abs_diff(n) > 3 {
            return false;
        }
        if let Some(next) = next {
            let next_diff = n - *next;
            if next_diff < 0 && first_diff > 0 {
                return false;
            }
            if next_diff > 0 && first_diff < 0 {
                return false;
            }
            if next.abs_diff(n) < 1 || next.abs_diff(n) > 3 {
                return false;
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect();
                if solve(&nums) {
                    return true;
                }
                for i in 0..nums.len() {
                    let mut nums = nums.clone();
                    nums.remove(i);
                    if solve(&nums) {
                        return true;
                    }
                }
                false
            })
            .count() as u32,
    )
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
