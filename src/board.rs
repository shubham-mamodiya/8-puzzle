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
}
