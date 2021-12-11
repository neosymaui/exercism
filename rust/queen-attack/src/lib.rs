#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    x: i32,
    y: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { x: rank, y: file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            x: position.x,
            y: position.y,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.x == other.x || self.y == other.y {
            return true;
        } else {
            for i in -7..7 {
                if (self.x - i == other.x || self.x + i == other.x) && self.y - i == other.y {
                    return true;
                }
            }
        }
        false
    }
}
