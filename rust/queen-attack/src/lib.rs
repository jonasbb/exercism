pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<ChessPosition, &'static str> {
        if x < 0 || x >= 8 || y < 0 || y >= 8 {
            Err("Invalid Position")
        } else {
            Ok(ChessPosition { x: x, y: y })
        }
    }
}

pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen(pos)
    }

    pub fn can_attack(&self, other_queen: &Queen) -> bool {
        self.0.x == other_queen.0.x || // same row
        self.0.y == other_queen.0.y || // same column
        self.0.x - self.0.y == other_queen.0.x - other_queen.0.y || // same bottom-left top-right diag
        self.0.x + self.0.y == other_queen.0.x + other_queen.0.y // same top-left bottom-right diag
    }
}
