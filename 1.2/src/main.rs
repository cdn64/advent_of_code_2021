use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
  let mut sums: Vec<i32> = vec![];

  for window_start in 2..lines.len() {
    let sum = lines[window_start].parse::<i32>().unwrap() +
              lines[window_start - 1].parse::<i32>().unwrap() +
              lines[window_start - 2].parse::<i32>().unwrap();
    sums.push(sum);
  }

  let mut increased_count = 0;

  let mut last_entry: Option<i32> = None;
  for sum in sums {
    if let Some(last_sum) = last_entry {
      if sum > last_sum {
        println!("{} (increased)", sum);
        increased_count += 1;
      } else if sum == last_sum {
        println!("{} (no change)", sum);
      } else {
        println!("{} (decreased)", sum);
      }
    } else {
      println!("{} (N/A - no previous sum)", sum);
    }
    last_entry = Some(sum)
  }

  println!("{} entries increased", increased_count)
}
