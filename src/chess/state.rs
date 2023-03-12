use super::piece::PieceInfo;

pub struct Rules {
    pub pieces: Vec<PieceInfo>,
}

impl Rules {
    /// A set of rules that are the same as normal chess
    pub fn normal() -> Rules {
        Rules { 
            pieces: Vec::new() 
        }
    }
}