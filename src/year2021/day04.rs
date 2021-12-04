use crate::utils::utils;
use std::collections::HashMap;

struct BingoBoard {
    value_loc: HashMap<u32, (u32, u32)>,
}

struct MarkingBingoBoard<'a> {
    board: &'a BingoBoard,
    is_checked: HashMap<(u32, u32), bool>,
}

impl BingoBoard {
    fn new(board_str: &[&str]) -> Self {
        Self {
            value_loc: board_str.iter()
                .enumerate()
                .map(|(i, line)| line.split(" ")
                     .filter(|str| str != &"")
                     .map(|str| str.parse().unwrap())
                     .enumerate()
                     .map(move |(j, entry)| (i as u32, j as u32, entry)))
                .flatten()
                .map(|(i, j, entry)| (entry, (i, j)))
                .collect(),
        }
    }
}

impl<'a> MarkingBingoBoard<'a> {
    fn new(board: &'a BingoBoard) -> Self {
        let is_checked = board.value_loc
            .values()
            .map(|loc| (loc.clone(), false))
            .collect();
        Self {
            board,
            is_checked,
        }
    }

    fn call(&mut self, num: &u32) -> bool {
        match self.board.value_loc.get(num) {
            Some(i) => {
                self.is_checked.insert(*i, true);
                (0..5).any(|i| (0..5).all(|j| self.is_checked[&(i,j)]))
                    || (0..5).any(|j| (0..5).all(|i| self.is_checked[&(i, j)]))
            }
            None => false
        }
    }
}

fn get_draw(draw: &str) -> Vec<u32> {
    draw.split(",")
        .map(|str| str.parse().unwrap())
        .collect()
}

fn parse_input(input: &Vec<&str>) -> (Vec<u32>, Vec<BingoBoard>) {
    let draw = get_draw(&input[0]);
    let boards = (2..input.len())
        .step_by(6)
        .map(|i| BingoBoard::new(&input[i..i+5]))
        .collect();
    (draw, boards)
}

fn get_winning_board_score(draws: &Vec<u32>, bingo_boards: &Vec<BingoBoard>) -> u32 {
    let mut marking_bingo_boards: Vec<MarkingBingoBoard> = bingo_boards.iter()
        .map(|board| MarkingBingoBoard::new(board))
        .collect();
    let (i, win_num) = draws.iter()
        .find_map(|draw| marking_bingo_boards.iter_mut()
                  .enumerate()
                  .find_map(|(i, board)| 
                            if board.call(draw) {
                                Some((i, draw))
                            } else {
                                None
                            }))
        .unwrap();
    let unmark_number_sum = bingo_boards[i].value_loc
        .iter()
        .filter(|(_, loc)| !marking_bingo_boards[i].is_checked[loc])
        .fold(0, |sum, (value, _)| sum + value);
    win_num * unmark_number_sum
}

fn get_last_winning_board_score(draws: &Vec<u32>, bingo_boards: &Vec<BingoBoard>) -> u32 {
    let mut marking_bingo_boards: Vec<MarkingBingoBoard> = bingo_boards.iter()
        .map(|board| MarkingBingoBoard::new(board))
        .collect();
    let win_num = draws.iter()
        .find_map(|draw| {
            if marking_bingo_boards.len() > 1 {
                for i in (0..marking_bingo_boards.len()).rev() {
                    if marking_bingo_boards[i].call(draw) {
                        marking_bingo_boards.remove(i);
                    }
                }
                None
            } else {
                if marking_bingo_boards[0].call(draw) {
                    Some(draw)
                } else {
                    None
                }
            }})
        .unwrap();
    let unmark_number_sum = marking_bingo_boards[0].board.value_loc
        .iter()
        .filter(|(_, loc)| !marking_bingo_boards[0].is_checked[loc])
        .fold(0, |sum, (value, _)| sum + value);
    win_num * unmark_number_sum
}

pub fn run() {
    let input: Vec<String> = utils::read_file_to_lines("data/year2021/day04");
    let input = input.iter()
        .map(AsRef::as_ref)
        .collect();
    let (draw, boards) = parse_input(&input);
    println!("{}", get_winning_board_score(&draw, &boards));
    println!("{}", get_last_winning_board_score(&draw, &boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day04_test() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7"];
        let (draws, boards) = parse_input(&input);
        assert_eq!(get_winning_board_score(&draws, &boards), 4512);
        assert_eq!(get_last_winning_board_score(&draws, &boards), 1924);
    }
}
