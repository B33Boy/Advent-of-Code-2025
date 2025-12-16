use std::{fs, io, path::Path};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(()),
        }
    }
}

impl Op {
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

fn main() -> io::Result<()> {
    println!("--- Day 6 Solution ---");

    let path = Path::new("input/day6.txt");
    let content = fs::read_to_string(path)?;

    println!("--- Part 1 ---");
    println!("{}", a1(&content));

    Ok(())
}

fn a1(content: &str) -> i64 {
    let mut total = 0;

    let (operands, operators) = ingest_data(content);

    for col in 0..operands[0].len() {
        let mut acc: Option<i64> = None;
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

fn ingest_data(content: &str) -> (Vec<Vec<i64>>, Vec<Op>) {
    let mut operands: Vec<Vec<i64>> = Vec::new();
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

fn parse_operands(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|token| token.parse::<i64>().unwrap())
        .collect()
}
