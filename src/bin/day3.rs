use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> io::Result<()> {
    println!("--- Day 2 Solution ---");

    let path = Path::new("input/day3.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<u8>> = Vec::new();

    for line_res in reader.lines() {
        let line_res = line_res?;
        data.push(parse_line(&line_res));
    }

    println!("--- Part 1 ---");
    println!("Part 1: {}", q1(&data));

    Ok(())
}

fn parse_line(s: &str) -> Vec<u8> {
    s.trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect()
}

fn q1(data: &Vec<Vec<u8>>) -> u32 {
    let total: u32 = data
        .iter()
        .map(|line| max_joltage(line))
        .fold(0, |acc, x| acc + x as u32);
    total
}

fn max_joltage(line: &[u8]) -> u8 {

    let (max_idx, max_elem) = first_max(&line);

    // Seek to the right first, but if it's at the end then seek to the left
    if max_idx != line.len() - 1 {
        // Search second largest max to the right
        let (_, max_elem2) = first_max(&line[max_idx + 1..]);

        return max_elem * 10 + max_elem2;

    } else {
        // Search second largest max to the left
        let (_, max_elem2) = first_max(&line[..max_idx]);

        return max_elem2 * 10 + max_elem;
    }
}

fn first_max(slice: &[u8]) -> (usize, u8) {
    let mut max_elem = 0;
    let mut max_i = 0;

    for (i, &elem) in slice.iter().enumerate() {
        if elem > max_elem {
            max_elem = elem;
            max_i = i;
        }
    }
    (max_i, max_elem)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let expected: Vec<u8> = vec![0, 1, 2, 3];
        let ans = parse_line(&"0123");
        assert_eq!(expected, ans);
    }

    #[test]
    fn test_max_joltage() {
        let cases = vec![
            (vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 98),
            (vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 89),
            (vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 78),
            (vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 92),
        ];

        for (line, expected) in cases {
            assert_eq!(expected, max_joltage(&line));
        }
    }

    #[test]
    fn test_first_max() {
        let input = &[1, 2, 3, 9, 9];
        let expected: (usize, u8) = (3, 9);

        let ans = first_max(input);
        assert_eq!(expected.0, ans.0);
        assert_eq!(expected.1, ans.1);
    }
}
