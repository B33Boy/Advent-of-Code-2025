use std::{fs, io, path::Path};

fn main() -> io::Result<()> {
    println!("--- Day 2 Solution ---");

    let path = Path::new("input/day2.txt");

    let line = fs::read_to_string(path)
        .expect("Could not read file")
        .trim()
        .to_string();

    // println!("--- Part 1 ---");
    // let sol1 = q1(&line);
    // println!("sol1: {sol1}");

    println!("--- Part 2 ---");
    let sol2 = q2(&line);
    println!("sol1: {sol2}");

    Ok(())
}

#[allow(dead_code)]
fn q1(line: &str) -> i64 {
    let ranges = get_ranges(&line);
    let mut total: i64 = 0;

    for range in ranges {
        let ids = get_invalid_ids(&range);
        total += ids.iter().sum::<i64>();
    }
    total
}

fn get_ranges(line: &str) -> Vec<(i64, i64)> {
    line.split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}

fn get_invalid_ids(range: &(i64, i64)) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = Vec::new();

    let (start, end) = *range;
    for i in start..=end {
        if is_valid(i) {
            invalid_ids.push(i);
        }
    }
    invalid_ids
}

fn is_valid(id: i64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();

    if id_len % 2 != 0 {
        // Odd number of characters can't be split in half
        return true;
    }

    let (a, b) = id_str.split_at(id_len / 2);

    a != b
}

fn q2(line: &str) -> i64 {
    let ranges = get_ranges(&line);
    let mut total: i64 = 0;

    for range in ranges {
        let ids = get_invalid_ids2(&range);
        total += ids.iter().sum::<i64>();
    }
    total
}

fn get_invalid_ids2(range: &(i64, i64)) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = Vec::new();

    let (start, end) = *range;
    for i in start..=end {
        if !is_valid2(i) {
            invalid_ids.push(i);
        }
    }
    invalid_ids
}

fn is_valid2(id: i64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();

    for win_size in 1..=(id_len / 2) {
        // only consider win_sizes that evenly fit in (e.g. skip if string is of length 5 and win_size = 2)
        if id_len % win_size != 0 {
            continue;
        }

        let pattern = &id_str[..win_size];
        if pattern.repeat(id_len / win_size) == id_str {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ranges() {
        let input = "11-22,95-115,998-1012";
        let expected = vec![(11, 22), (95, 115), (998, 1012)];

        let ans = get_ranges(input);

        assert_eq!(expected, ans);
    }

    #[test]
    fn test_range_contains_invalid_ids() {
        let input = vec![(11, 22), (95, 115), (998, 1012)];

        let expected = vec![vec![11, 22], vec![99], vec![1010]];

        for (idx, range) in input.iter().enumerate() {
            let ans = get_invalid_ids(range);
            assert_eq!(expected[idx], ans);
        }
    }

    #[test]
    fn test_is_valid_id() {
        let cases = vec![
            (1, true),
            (21, true),
            (22, false),
            (333, true),
            (4545, false),
        ];

        for case in cases {
            let ans = is_valid(case.0);
            assert_eq!(ans, case.1);
        }
    }

    #[test]
    fn test_is_valid_id2() {
        let cases = vec![
            (11, false),
            (22, false),
            (999, false),
            (1010, false),
            (1188511885, false),
            (12, true),
            (1000, true),
        ];

        for case in cases {
            let ans = is_valid2(case.0);
            assert_eq!(ans, case.1);
        }
    }
}
