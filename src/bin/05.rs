#![feature(let_chains)]
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_list = false;
    let mut sum_middle_pages = 0;
    for line in input.lines() {
        if line.is_empty() && !page_list {
            page_list = true;
            continue;
        } else if line.is_empty() {
            break;
        }
        if page_list {
            let mut pages: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
            let mut middle_page = Some(pages[pages.len() / 2]);
            'outer: for i in 0..pages.len() {
                let page = pages[i];
                if !map.contains_key(&page) {
                    continue;
                }
                let must_be_after = map.get(&page).unwrap();
                for j in 0..i {
                    let other_page = pages[j];
                    if must_be_after.contains(&other_page) {
                        middle_page = None;
                        break 'outer;
                    }
                }
            }
            if let Some(page) = middle_page {
                sum_middle_pages += page;
            }
        } else {
            let mut parts: Vec<u32> = line.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
            if !map.contains_key(&parts[0]) {
                map.insert(parts[0], vec![parts[1]]);
            } else {
                map.get_mut(&parts[0]).unwrap().push(parts[1]);
            }
        }
    }
    Some(sum_middle_pages)
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut page_list = false;
    let mut sum_middle_pages = 0;
    for line in input.lines() {
        if line.is_empty() && !page_list {
            page_list = true;
            continue;
        } else if line.is_empty() {
            break;
        }
        if page_list {
            let mut pages: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
            let mut bad = false;
            pages.sort_by(|a, b| {
                if let Some(must_be_before) = map.get(a) && must_be_before.contains(b) {
                    bad = true;
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            if bad {
                sum_middle_pages += pages[pages.len() / 2];
            }
        } else {
            let parts: Vec<u32> = line.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
            if !map.contains_key(&parts[0]) {
                map.insert(parts[0], vec![parts[1]]);
            } else {
                map.get_mut(&parts[0]).unwrap().push(parts[1]);
            }
        }
    }
    Some(sum_middle_pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
