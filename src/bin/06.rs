advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let guard_poss = map.iter()
        .enumerate()
        .filter_map(|(i, row)| {
            let x = row.iter().position(|&c| c == '^')?;
            Some((i, x))
        })
        .collect::<Vec<(usize, usize)>>();
    for (mut y, mut x) in guard_poss {
        map[y][x] = 'X';
        let mut move_vec: (i64, i64) = (-1, 0);
        loop {
            let next_y = y as i64 + move_vec.0;
            let next_x = x as i64 + move_vec.1;
            if next_y < 0 || next_x < 0 || next_y >= map.len() as i64 || next_x >= map[0].len() as i64 {
                break;
            } else if map[next_y as usize][next_x as usize] == '#' {
                move_vec = (move_vec.1, -move_vec.0);
                continue;
            } else if map[next_y as usize][next_x as usize] == '.' {
                map[next_y as usize][next_x as usize] = 'X';
            };
            y = next_y as usize;
            x = next_x as usize;
        }
    }
    // let solved = map.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    // print!("{}", solved);
    Some(
        map.iter()
            .map(|row| row.iter().filter(|&&c| c == 'X').count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
