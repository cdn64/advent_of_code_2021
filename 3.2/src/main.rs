use std::fs::File;
use std::io::{BufRead, BufReader};

enum LookupType {
  MostCommon,
  LeastCommon
}

fn find_rating(lookup_type: LookupType, lines: Vec<String>) -> Option<String> {
  let line_width = lines.first().unwrap().len();
  let mut current_lines = lines;
  for i in 0..line_width {
    let half_lines_length = current_lines.len() as f32 / 2.0;
    let ones_count = current_lines.clone().into_iter().filter(|line| line.chars().nth(i) == Some('1')).collect::<Vec<String>>().len();
    let keep_ones = match lookup_type {
      LookupType::MostCommon => ones_count as f32 >= half_lines_length,
      LookupType::LeastCommon => (ones_count as f32) < half_lines_length
    };
    let digit_to_filter_by = if keep_ones { '1' } else { '0' };
    current_lines = current_lines.into_iter().filter(|line| line.chars().nth(i) == Some(digit_to_filter_by)).collect::<Vec<String>>();

    if current_lines.len() == 1 {
      return Some(current_lines[0].clone())
    }
  }
  None
}

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

  let oxygen_generator_rating_string = find_rating(LookupType::MostCommon, lines.clone()).unwrap();
  let co2_scrubber_rating_string = find_rating(LookupType::LeastCommon, lines.clone()).unwrap();

  let co2_scrubber_rating = isize::from_str_radix(&oxygen_generator_rating_string, 2).unwrap();
  let oxygen_generator_rating = isize::from_str_radix(&co2_scrubber_rating_string, 2).unwrap();
  let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

  println!("Oxygen generator rating: {}", oxygen_generator_rating);
  println!("CO2 scrubber rating: {}", co2_scrubber_rating);
  println!("Life support rating: {}", life_support_rating);
}
