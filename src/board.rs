use rand::seq::SliceRandom;
use std::ptr;

pub enum BoardError {
    InvalidDimension,
}

pub struct Board {
    board: Vec<u32>,
    size: usize,
    dimension: usize,
}

impl Board {
    pub fn new(dimension: usize) -> Result<Self, BoardError> {
        if dimension == 0 {
            return Err(BoardError::InvalidDimension);
        }

        let size = dimension
            .checked_mul(dimension)
            .ok_or(BoardError::InvalidDimension)?;

        let mut board: Vec<u32> = (1..size as u32).collect();
        board.push(0);

        Ok(Self {
            board,
            size,
            dimension,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn board(&self) -> &[u32] {
        &self.board
    }

    pub fn cal_index(&self, &row: &usize, &col: &usize) -> usize {
        row * self.dimension + col
    }

    pub fn print_it(&self) {
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                let idx = self.cal_index(&i, &j);
                let value = self.board[idx];
                let c = if value == 0 {
                    '_'.to_string()
                } else {
                    value.to_string()
                };
                print!(" {} ", c);
            }
            println!();
        }
    }

    pub fn is_goal(&self) -> bool {
        if self.board[self.size() - 1] != 0 {
            return false;
        }
        for i in 1..self.size() {
            if self.board[i - 1] != i as u32 {
                return false;
            }
        }

        true
    }

    pub fn humming(&self) -> u32 {
        let mut humming_distance: u32 = 0;
        if self.board[self.size() - 1] != 0 {
            humming_distance += 1;
        }

        for i in 1..self.size() {
            if self.board[i - 1] != i as u32 {
                humming_distance += 1;
            }
        }

        humming_distance
    }

    pub fn manhattan(&self) -> u64 {
        let mut manhattan: u64 = 0;

        let dim = self.dimension as i64;
        let mut expected_row: i64;
        let mut expected_col: i64;
        let mut current_row: i64;
        let mut current_col: i64;

        for i in 0..self.size() {
            let tile = self.board[i] as i64;
            if tile == 0 {
                expected_row = dim - 1;
                expected_col = dim - 1;
            } else {
                let tile = tile - 1;
                expected_row = tile / dim;
                expected_col = tile % dim;
            }

            current_row = i as i64 / dim;
            current_col = i as i64 % dim;

            manhattan += (expected_row - current_row).abs().unsigned_abs()
                + (expected_col - current_col).unsigned_abs();
        }
        manhattan
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.board.shuffle(&mut rng);
    }

    pub fn is_equal(&self, other: &Board) -> bool {
        if ptr::eq(self, other) {
            return true;
        }

        if self.size() != other.size() {
            return false;
        }

        for i in 0..self.size() {
            if self.board[i] != other.board[i] {
                return false;
            }
        }
        true
    }
}
