use crate::*;

use super::direction::Direction;

#[derive(Deref, DerefMut)]
pub struct Square(pub u8);

impl Square {
    pub fn from_xy(x: u8, y: u8) -> Option<Square> {
        Self::valid_xy(x, y).then(|| Square(x + y * 8))
    }
    
    pub fn to_xy(&self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }
    
    pub fn valid_xy(x: u8, y: u8) -> bool {
        x < 8 && y < 8
    }
    
    /// Returns the square thats 1 square in this direction
    pub fn dir(&self, dir: Direction) -> Option<Square> {
        let (x, y) = self.to_xy();
        let (dx, dy) = dir.xy();
        let x = x as i8 + dx;
        let y = y as i8 + dy;
        
        if x < 0 || y < 0 {
            return None;
        }
        
        Self::from_xy(x as u8, y as u8)
    }
}