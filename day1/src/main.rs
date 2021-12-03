use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;


fn main() -> Result<(), Error> {
    // Open input file.
    let path = Path::new("./input");
    let file = File::open(path)?;
    let file = BufReader::new(file);

    // Parse each line to ints.
    let inputs: Vec<i64> = file.lines().map(|x| {
        match x {
            Ok(v) => v.parse().unwrap(),
            Err(_) => panic!("Unparsable line"),
        }
    }).collect();

    println!("== Part 1 ==");
    // convert to a slice for fun with windows.
    part1(&inputs[..]);

    println!("\n\n== Part 2 ==");
    part2(&inputs[..]);

    Ok(())
}

fn part1(inputs: &[i64]) {
    let mut depth_inc_count: i64 = 0;
    // iterate over a sliding window of 2 elements from the slice.
    for window in inputs.windows(2) {
        if window[0] < window[1] {
            depth_inc_count += 1;
        }
    }

    println!("Depth increased {} times!", depth_inc_count);
}

fn part2(inputs: &[i64]) {
    let mut window_inc_count: i64 = 0;
    // Capture the previous window in the loop
    let mut prev_window_sum: Option<i64> = None;
    for window in inputs.windows(3) {
        let window_sum = window[0] + window[1] + window[2];
        if let Some(prev) = prev_window_sum {
            if window_sum > prev {
                window_inc_count += 1;
            }
        }
        prev_window_sum = Some(window_sum);
    }

    println!("Depth increased {} times!", window_inc_count);
}
