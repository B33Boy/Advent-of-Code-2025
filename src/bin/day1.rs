
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()>{
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

        update(&mut val, line);

        if val == 0{
            num_zeros+=1;
        }

    }

    print!("{num_zeros}\n");

    Ok(())
}


fn update(val: &mut i32, line: &str) {

    let delta =  parse(&line).unwrap() as i32;
    let mag: i32 = line[1..].trim().parse().ok().unwrap();
    *val = (*val + delta * mag).rem_euclid(100);  
    println!("Delta: {}, updated val is {}", delta * mag, *val);
}

fn parse(inp: &str) -> Option<i8>{
    match inp.chars().next() {
        Some('R') => Some(1),
        Some('L') => Some(-1),
        _ => panic!("Invalid sign")
    }
}
