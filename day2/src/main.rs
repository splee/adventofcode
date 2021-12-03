use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone)]
struct MoveError;

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid move command")
    }
}

#[derive(Debug, Clone)]
enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn to_move(line: &str) -> Result<Move, MoveError> {
    let v: Vec<&str> = line.split(' ').collect();
    let cmd = v[0];
    let value = v[1].parse();
    let value = match value {
        Ok(v) => v,
        Err(_) => return Err(MoveError),
    };

    match cmd {
        "forward" => Ok(Move::Forward(value)),
        "down" => Ok(Move::Down(value)),
        "up" => Ok(Move::Up(value)),
        _ => Err(MoveError),
    }
}

#[derive(Debug, Clone)]
struct Sub {
    horiz: i32,
    depth: i32,
}

impl Sub {
    fn new() -> Self {
        Sub { horiz: 0, depth: 0 }
    }

    fn execute_move(&mut self, m: Move) {
        match m {
            Move::Forward(amount) => {
                self.horiz += amount; 
            },
            Move::Down(amount) => {
                self.depth += amount;
            },
            Move::Up(amount) => {
                self.depth -= amount;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let path = Path::new("./input");
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let inputs: Vec<Move> = file.lines().map(|l| {
        match l {
            Ok(v) => {
                match to_move(&v) {
                    Ok(m) => m,
                    Err(_) => panic!("invalid move")
                }
            }, 
            Err(_) => panic!("invalid line"),
        }
    }).collect();

    let mut sub = Sub::new();
    for m in inputs {
        println!("Move: {:?}", m);
        sub.execute_move(m);
    }
    println!("Sub is at position: {}, {}", sub.horiz, sub.depth);
    println!("Position * Depth: {}", sub.horiz * sub.depth);

    Ok(())
}

