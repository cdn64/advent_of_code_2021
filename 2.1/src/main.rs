use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut position = 0;
  let mut depth = 0;

  for line in reader.lines() {
    if let Ok(command) = line {
      if command.starts_with("forward ") {
        position += command[8..].parse::<i32>().unwrap();
      } else if command.starts_with("down ") {
        depth += command[5..].parse::<i32>().unwrap();
      } else if command.starts_with("up ") {
        depth -= command[3..].parse::<i32>().unwrap();
      }
    }
  }

  println!("Final position: {}", position);
  println!("Final depth: {}", depth);
  println!("Multiplied: {}", position * depth);
}
