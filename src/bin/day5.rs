use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{cmp, io};

fn main() -> io::Result<()> {
    println!("--- Day 5 Solution ---");

    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut unordered_ranges = parse_ranges(&mut lines);
    let ranges = merge_ranges(&mut unordered_ranges);

    println!("--- Part 1 ---");
    println!("{}", a1(&ranges, &mut lines));

    println!("--- Part 2 ---");
    println!("{}", a2(&ranges));

    Ok(())
}

fn parse_ranges(lines: &mut io::Lines<BufReader<File>>) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let range = parse_range(&line);
        ranges.push(range);
    }
    ranges
}

fn parse_range(range_str: &str) -> (i64, i64) {
    let (x_str, y_str) = range_str.split_once('-').unwrap();
    let x = x_str.parse::<i64>().unwrap();
    let y = y_str.parse::<i64>().unwrap();
    (x, y)
}

fn merge_ranges(ranges: &mut [(i64, i64)]) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    // First sort the ranges by x where (x, y)
    ranges.sort();

    for (i, &cur) in ranges.iter().enumerate() {
        if i == 0 {
            res.push(cur);
            continue;
        }

        let back = res.last_mut().unwrap();

        if back.1 < cur.0 {
            res.push(cur);
            continue;
        }
        back.1 = cmp::max(back.1, cur.1);
    }

    res
}

fn a1(ranges: &[(i64, i64)], lines: &mut io::Lines<BufReader<File>>) -> i64 {
    let mut cnt = 0;

    while let Some(Ok(line)) = lines.next() {
        let val = line.parse::<i64>().unwrap();

        if val_in_range(ranges, val) {
            cnt += 1;
        }
    }
    cnt
}

fn val_in_range(ranges: &[(i64, i64)], val: i64) -> bool {
    for range in ranges {
        if range.0 <= val && val <= range.1 {
            return true;
        }
    }

    false
}


fn a2(ranges: &[(i64, i64)]) -> i64 {
    let mut cnt = 0;

    for range in ranges {
        cnt += range.1 - range.0 + 1;
    }

    cnt
}