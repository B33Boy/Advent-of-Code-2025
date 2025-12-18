use std::{fs, io, path::Path};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

impl Op {
    fn apply(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }

    fn identity(self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

fn main() -> io::Result<()> {
    println!("--- Day 6 Solution ---");

    let path = Path::new("input/day6.txt");
    let content = fs::read_to_string(path)?;

    let (operands, operators) = ingest_data(&content);

    println!("--- Part 1 ---");
    println!("{}", a1(&operands, &operators));

    let (operands, operators) = ingest_data2(&content);
    println!("--- Part 2 ---");
    println!("{}", a2(&operands, &operators));

    Ok(())
}

fn ingest_data(content: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let mut operands: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<Op> = Vec::new();

    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let is_last = lines.peek().is_none();
        if is_last {
            operators = parse_operators(line);
            break;
        }
        operands.push(parse_operands(line));
    }

    (operands, operators)
}

fn parse_operators(line: &str) -> Vec<Op> {
    line.split_whitespace()
        .map(|token| Op::try_from(token).unwrap())
        .collect()
}

fn parse_operands(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .map(|token| token.parse::<u64>().unwrap())
        .collect()
}

fn a1(operands: &[Vec<u64>], operators: &[Op]) -> u64 {
    let mut total = 0;

    for col in 0..operands[0].len() {
        let mut acc: Option<u64> = None;
        let cur_op = operators[col];

        for row in operands.iter() {
            // first value of acc will be None so we just keep the number, otherwise apply the operation
            acc = Some(match acc {
                None => row[col],
                Some(prev) => cur_op.apply(prev, row[col]),
            });
        }

        total += acc.unwrap();
    }

    total
}

fn ingest_data2(content: &str) -> (Vec<Vec<char>>, Vec<Op>) {
    let mut operands: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<Op> = Vec::new();

    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        let is_last = lines.peek().is_none();
        if is_last {
            operators = parse_operators2(line);
            break;
        }
        operands.push(line.chars().collect());
    }

    (operands, operators)
}

fn parse_operators2(line: &str) -> Vec<Op> {
    let mut out = Vec::with_capacity(line.len());
    let mut prev = ' ';

    for c in line.chars() {
        if c == '+' || c == '*' {
            prev = c;
        }

        let cur_op = Op::try_from(prev).unwrap();
        out.push(cur_op);
    }

    out
}

fn a2(operands: &[Vec<char>], operators: &[Op]) -> u64 {
    // Iterate through columns, keep track of prev operand
    // Join all chars per column, convert to int, add it to a buffer vec
    // When current operand is different from prev, take each num in the buffer vec and reduce with operand
    // Add the value to total

    let mut total: u64 = 0;

    let operands = transpose(operands); // Now each row represents a column
    let mut buffer: Vec<u64> = Vec::new();

    for (col_idx, col) in operands.iter().enumerate() {
        let col_string: String = col.iter().collect();
        let col_str = col_string.trim();

        let cur_op = operators[col_idx];

        // Push to buffer as long as column is not empty
        if !col_str.is_empty() {
            let col_num: u64 = col_str.parse().unwrap();
            buffer.push(col_num);
            continue;
        }

        // Compute all the values in the buffer with the current operator
        total += acc_buffer(&mut buffer, cur_op);
    }

    // Buffer for last block needs to be processed
    let final_op = operators.last().copied().unwrap();
    total += acc_buffer(&mut buffer, final_op);

    total
}

#[allow(clippy::needless_range_loop)]
fn transpose<T: Clone>(mat: &[Vec<T>]) -> Vec<Vec<T>> {
    if mat.is_empty() {
        return Vec::new();
    }

    let rows = mat.len();
    let cols = mat[0].len();

    let mut t = vec![Vec::with_capacity(rows); cols];

    for row in 0..rows {
        for col in 0..cols {
            t[col].push(mat[row][col].clone());
        }
    }

    t
}

fn acc_buffer(buffer: &mut Vec<u64>, op: Op) -> u64 {
    let acc = buffer
        .iter()
        .fold(op.identity(), |acc, &x| op.apply(acc, x));

    buffer.clear();

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let given = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];

        let ans = transpose(&given);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_parse_operator2() {
        let given = "+ * ";
        let expected = vec![Op::Add, Op::Add, Op::Mul, Op::Mul];
        let ans = parse_operators2(given);

        assert_eq!(given.len(), ans.len());
        assert_eq!(ans, expected);
    }
}
