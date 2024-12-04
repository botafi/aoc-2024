advent_of_code::solution!(4);

const DIRECTIONS : [(i32, i32); 8] = [
    (0, 1), (1, 0), (0, -1), (-1, 0),
    (1, 1), (1, -1), (-1, 1), (-1, -1)
];
pub fn part_one(input: &str) -> Option<u32> {
    let text_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let text_to_find: Vec<char> = "XMAS".chars().collect();
    let mut texts_found_count = 0;
    for i in 0..text_map.len() {
        for j in 0..text_map[i].len() {
            for dir in DIRECTIONS.iter() {
                let mut x = i as i32;
                let mut y = j as i32;
                let mut found = true;
                for c in text_to_find.iter() {
                    if x < 0 || y < 0 || x >= text_map.len() as i32 || y >= text_map[x as usize].len() as i32 {
                        found = false;
                        break;
                    }
                    if *text_map[x as usize].get(y as usize).unwrap() != *c {
                        found = false;
                        break;
                    }
                    x += dir.0;
                    y += dir.1;
                }
                if found {
                    texts_found_count += 1;
                }
            }
        }
    }

    Some(texts_found_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let text_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut texts_found_count = 0;
    for i in 1..text_map.len() {
        for j in 1..text_map[i].len() {
            let mid = text_map[i][j];
            if mid != 'A' {
                continue;
            }
            let upper_left = text_map.get(i-1).and_then(|text_map| text_map.get(j-1)).unwrap_or(&'0');
            let upper_right = text_map.get(i-1).and_then(|text_map| text_map.get(j+1)).unwrap_or(&'0');
            let lower_left = text_map.get(i+1).and_then(|text_map| text_map.get(j-1)).unwrap_or(&'0');
            let lower_right = text_map.get(i+1).and_then(|text_map| text_map.get(j+1)).unwrap_or(&'0');

            let first_line = (*upper_left == 'S' && *lower_right == 'M') || (*upper_left == 'M' && *lower_right == 'S');
            let second_line = (*upper_right == 'S' && *lower_left == 'M') || (*upper_right == 'M' && *lower_left == 'S');
            if first_line && second_line {
                texts_found_count += 1;
            }
        }
    }
    Some(texts_found_count)
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
