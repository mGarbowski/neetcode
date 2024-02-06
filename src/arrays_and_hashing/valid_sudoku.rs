//! https://leetcode.com/problems/valid-sudoku/

use std::collections::HashSet;
use std::ops::RangeInclusive;

const BOARD_SIZE: usize = 9;
const BOARD_SQUARES: usize = 3;
const DIGITS: RangeInclusive<char> = '1'..='9';


pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // Check rows
    for row in 0..BOARD_SIZE {
        let mut present = HashSet::new();
        for col in 0..BOARD_SIZE {
            let value = *board.get(row).unwrap().get(col).unwrap();
            if DIGITS.contains(&value) && !present.insert(value) {
                return false;
            }
        }
    }

    // Check cols
    for col in 0..BOARD_SIZE {
        let mut present = HashSet::new();
        for row in 0..BOARD_SIZE {
            let value = *board.get(row).unwrap().get(col).unwrap();
            if DIGITS.contains(&value) && !present.insert(value) {
                return false;
            }
        }
    }

    // Check squares
    for square_row in 0..BOARD_SQUARES {
        for square_col in 0..BOARD_SQUARES {
            if !is_valid_square(square_row, square_col, &board) {
                return false;
            }
        }
    }

    true
}

fn is_valid_square(square_row: usize, square_col: usize, board: &Vec<Vec<char>>) -> bool {
    let mut present = HashSet::new();
    let base_row = BOARD_SQUARES * square_row;
    let base_col = BOARD_SQUARES * square_col;

    for row in base_row..base_row + BOARD_SQUARES {
        for col in base_col..base_col + BOARD_SQUARES {
            let value = *board.get(row).unwrap().get(col).unwrap();
            if DIGITS.contains(&value) && !present.insert(value) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::valid_sudoku::*;

    #[test]
    fn is_valid_sudoku_true() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(is_valid_sudoku(board))
    }

    #[test]
    fn is_valid_sudoku_false() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!is_valid_sudoku(board))
    }
}