use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut increased_count = 0;
  let mut last_entry: Option<i32> = None;

  for (_index, line) in reader.lines().enumerate() {
    let number: i32 = line.unwrap().parse().unwrap();

    if let Some(last_number) = last_entry {
      if last_number < number {
        println!("{} (increased)", number);
        increased_count += 1;
      } else {
        println!("{} (decreased)", number)
      }
    } else {
      println!("{} (N/A - no previous measurement)", number)
    }
    last_entry = Some(number);
  }

  println!("{} entries increased", increased_count)
}
