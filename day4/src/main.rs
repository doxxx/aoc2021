use shared::Grid;
use std::collections::vec_deque::*;
use std::fmt;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(input.clone());
    println!();
    part2(input);

    Ok(())
}

fn read_input() -> Result<Bingo> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let numbers_line = lines.next().unwrap();
    let numbers: Vec<u32> = numbers_line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    lines.next(); // skip blank line

    let mut boards: Vec<Board> = Vec::new();
    let mut rows = Vec::new();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            boards.push(Board::new(rows));
            rows = Vec::new();
        } else {
            let row: Vec<BoardSlot> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .map(|num| BoardSlot::new(num))
                .collect();
            rows.push(row);
        }
    }

    if !rows.is_empty() {
        boards.push(Board::new(rows));
    }

    Ok(Bingo {
        boards,
        numbers: VecDeque::from(numbers),
    })
}

#[derive(Clone, Default)]
struct BoardSlot {
    num: u32,
    marked: bool,
}

impl BoardSlot {
    fn new(num: u32) -> Self {
        Self { num, marked: false }
    }
}

impl fmt::Display for BoardSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2}", self.num)?;
        if self.marked {
            write!(f, "*")
        } else {
            write!(f, " ")
        }
    }
}

#[derive(Clone)]
struct Board(Grid<BoardSlot>);

impl Board {
    pub fn new(rows: Vec<Vec<BoardSlot>>) -> Board {
        Self(Grid::new_square_with_rows(5, rows))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, y, v) in self.0.iter() {
            if x == 0 && y > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{} ", v)?;
        }

        Ok(())
    }
}

trait BoardMarking {
    fn mark_num(&mut self, num: u32);
    fn is_winner(&self) -> bool;
    fn sum_unmarked(&self) -> u32;
}

impl BoardMarking for Board {
    fn mark_num(&mut self, num: u32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.0[(x, y)].num == num {
                    self.0[(x, y)].marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for y in 0..5 {
            if self.0.get_row(y).iter().all(|s| s.marked) {
                return true;
            }
        }

        for x in 0..5 {
            if self.0.get_col(x).iter().all(|s| s.marked) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.0
            .cells
            .iter()
            .filter(|s| !s.marked)
            .map(|s| s.num)
            .sum()
    }
}

#[derive(Clone)]
struct Bingo {
    boards: Vec<Board>,
    numbers: VecDeque<u32>,
}

enum AdvanceResult {
    Winner(Board, u32),
    NoWinner,
    NoMoreNumbers,
}

impl Bingo {
    fn advance(&mut self) -> AdvanceResult {
        if let Some(num) = self.numbers.pop_front() {
            self.mark_boards(num);

            let winners = self.remove_winners();
            if let Some(winner) = winners.into_iter().nth(0) {
                return AdvanceResult::Winner(winner, num);
            }

            AdvanceResult::NoWinner
        } else {
            AdvanceResult::NoMoreNumbers
        }
    }

    fn mark_boards(&mut self, num: u32) {
        for board in self.boards.iter_mut() {
            board.mark_num(num);
        }
    }

    fn remove_winners(&mut self) -> Vec<Board> {
        let mut winners = Vec::new();

        while self.boards.len() > 0 {
            let winner = self
                .boards
                .iter()
                .enumerate()
                .find(|(_, b)| b.is_winner())
                .map(|(i, _)| i);

            if let Some(winner) = winner.map(|i| self.boards.remove(i)) {
                winners.push(winner);
            } else {
                break;
            }
        }

        winners
    }

    fn find_first_winning_board(&mut self) -> (Board, u32) {
        loop {
            match self.advance() {
                AdvanceResult::Winner(b, num) => return (b, num),
                AdvanceResult::NoWinner => {}
                AdvanceResult::NoMoreNumbers => panic!("no winning board!"),
            }
        }
    }

    fn find_last_winning_board(&mut self) -> (Board, u32) {
        let mut last_winner = None;

        loop {
            match self.advance() {
                AdvanceResult::Winner(b, num) => {
                    last_winner = Some((b, num));
                }
                AdvanceResult::NoWinner => {}
                AdvanceResult::NoMoreNumbers => return last_winner.expect("no winning board!"),
            }
        }
    }
}

fn part1(mut bingo: Bingo) {
    let (board, winning_num) = bingo.find_first_winning_board();

    println!("part1: winning num = {}", winning_num);
    println!("part1: winning board:\n{}", board);

    let result = board.sum_unmarked() * winning_num;

    println!("part1: result = {}", result);
}

fn part2(mut bingo: Bingo) {
    let (board, num) = bingo.find_last_winning_board();

    println!("part2: winning num = {}", num);
    println!("part2: winning board:\n{}", board);

    let result = board.sum_unmarked() * num;

    println!("part2: result = {}", result);
}
