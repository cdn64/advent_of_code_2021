use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq)]
enum BingoNumber {
  Unmarked(i32),
  Marked(i32)
}
#[derive(Debug)]
struct BingoBoardRow([BingoNumber; 5]);
#[derive(Debug)]
struct BingoBoard([BingoBoardRow; 5]);

fn main() {
  let filename = "src/input.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut lines = reader.lines().filter_map(Result::ok);

  let mut boards: Vec<BingoBoard> = vec![];
  let mut current_board_rows = Vec::<BingoBoardRow>::with_capacity(5);

  // Read in the numbers
  let numbers: Vec<i32> = lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

  // Read in the bingo boards
  for line in lines {
    if line.len() == 0 { continue }

    let row_numbers: Vec<BingoNumber> = line.split_ascii_whitespace().map(|s| BingoNumber::Unmarked(s.parse::<i32>().unwrap())).take(5).collect();
    current_board_rows.push(BingoBoardRow([
      row_numbers[0],
      row_numbers[1],
      row_numbers[2],
      row_numbers[3],
      row_numbers[4]
    ]));

    if current_board_rows.len() == 5 {
      let rows = [
        current_board_rows.pop().unwrap(),
        current_board_rows.pop().unwrap(),
        current_board_rows.pop().unwrap(),
        current_board_rows.pop().unwrap(),
        current_board_rows.pop().unwrap()
      ];
      boards.push(BingoBoard(rows))
    }
  }

  let win_conditions = [
    [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
    [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)],
    [(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)],
    [(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)],
    [(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)],
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
    [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)],
    [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
    [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)],
    [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)],
  ];

  let mut winning_board_index: Option<usize> = None;
  let mut last_number: Option<i32> = None;

  // Draw numbers
  for number in numbers {

    // Mark number on all boards
    for board in 0..boards.len() {
      for y in 0..5 {
        for x in 0..5 {
          if boards[board].0[y].0[x] == BingoNumber::Unmarked(number) {
            boards[board].0[y].0[x] = BingoNumber::Marked(number);
          }
        }
      }
    }

    // Check win conditions on all boards
    for board in 0..boards.len() {
      for win_condition in &win_conditions {
        let mut won = true;
        for (x, y) in win_condition {
          match boards[board].0[*y as usize].0[*x as usize] {
            BingoNumber::Unmarked(_) => won = false,
            _ => {}
          };
        }
        if won {
          last_number = Some(number);
          winning_board_index = Some(board);
          break;
        }
      }
    }
    if winning_board_index != None {
      break;
    }
  }

  // Sum up all unmarked numbers on winning board
  let mut unmarked_sum = 0;
  let winning_board = &boards[winning_board_index.unwrap()];
  for y in 0..5 {
    for x in 0..5 {
      match winning_board.0[y].0[x] {
        BingoNumber::Unmarked(n) => unmarked_sum += n,
        _ => {}
      }
    }
  }

  println!("Last number: {}", last_number.unwrap());
  println!("Sum of unmarked numbers on winning board: {}", unmarked_sum);
  println!("Multiplied (= end result): {}", last_number.unwrap() * unmarked_sum);
}
