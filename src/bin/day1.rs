use core::num;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("--- Day 1 Solution ---");

    let path = Path::new("input/day1.txt");
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut val = 50;
    let mut num_zeros = 0;

    print!("Starting at 50\n");

    for line_res in reader.lines() {
        let line_res = line_res?;
        let line = line_res.trim();

        // Q1
        // update(&mut val, line);
        // if val == 0{
        //     num_zeros+=1;
        // }

        // Q2
        update2(&mut val, line, &mut num_zeros);
    }

    print!("num_zeros: {num_zeros}\n");

    Ok(())
}

#[allow(dead_code)]
fn update(val: &mut i32, line: &str) {
    let delta = parse_dir(&line).unwrap() as i32;
    let mag: i32 = line[1..].trim().parse().ok().unwrap();
    *val = (*val + delta * mag).rem_euclid(100);
    println!("Delta: {}, updated val is {}", delta * mag, *val);
}

fn update2(val: &mut i32, line: &str, num_zeros: &mut i32) {
    let dir = parse_dir(&line).unwrap() as i32;
    let mag: i32 = line[1..].trim().parse().ok().unwrap();
    let delta = dir * mag;

    let new_val = *val + delta;

    // If new val is outside of -99 to 99 (not inclusive) then we have crossed zeros
    *num_zeros += (new_val / 100).abs();

    // Account for new_val dropping to within in the range -99 to 0 (inclusive)
    if *val != 0 && new_val <= 0 {
        *num_zeros += 1;
    }

    // Use % to get remaining value that is within 0-99 (inclusive)
    *val = new_val.rem_euclid(100);
}

fn parse_dir(inp: &str) -> Option<i8> {
    match inp.chars().next() {
        Some('R') => Some(1),
        Some('L') => Some(-1),
        _ => panic!("Invalid sign"),
    }
}
