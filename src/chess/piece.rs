
#[derive(Debug)]
pub struct PieceInfo {
    pub name: &'static str,
    pub icon: char,
    pub value: u8,
    pub moves: &'static [(i8, i8)]
}