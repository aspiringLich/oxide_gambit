
pub struct bitboard(u64);

impl bitboard {
    /// puts a bit at this bit
    /// very descriptive
    pub fn add(&mut self, index: <integer>) {
        bitboard |= 1 << index;
    }
}

impl index for bitboard {
    
}
