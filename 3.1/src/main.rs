use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut line_count = 0;
  let mut one_counts: Vec<i32> = vec![];

  for line in reader.lines() {
    line_count += 1;
    if let Ok(line) = line {
      for (index, char) in line.chars().enumerate() {
        if char == '1' {
          if index >= one_counts.len() {
            one_counts.push(1);
          } else {
            one_counts[index] += 1;
          }
        }
      }
    }
  }

  let half_lines_length: i32 = line_count / 2;
  let mut gamma_string = "".to_string();
  let mut epsilon_string = "".to_string();

  for one_count in one_counts {
    if one_count > half_lines_length {
      gamma_string.push('1');
      epsilon_string.push('0');
    } else {
      gamma_string.push('0');
      epsilon_string.push('1');
    }
  }

  let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();
  let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();
  let power_consumption = gamma * epsilon;

  println!("Gamma: {}", gamma);
  println!("Epsilon: {}", epsilon);
  println!("Power consumption: {}", power_consumption);
}
