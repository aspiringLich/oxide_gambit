use super::piece_info::PieceInfo;
use super::*;
use crate::chess::board::Board;
use crate::chess::state::{Index, State};
use crate::chess::Team::{*, self};
use crate::rules::piece::Piece;
use anyhow::{bail, Context, Result};

impl<'a> State<'a> {
    /// loads a FEN string into the board state
    ///
    /// |#|description|example|
    /// |---|---|---|
    /// |0| pieces on the board|rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR|
    /// |1| turn|w|
    /// |2| castling rights|KQkq|
    /// |3| en passant|-|
    /// |4| halfmove clock|0|
    /// |5| move counter|1|
    ///
    /// # Example
    ///
    /// ```
    /// // returns the standard chess starting position
    /// from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ")
    /// ```
    ///
    /// TODO: implement other things
    #[allow(non_snake_case)]
    pub fn from_FEN(str: &str) -> Result<Self> {
        let mut board: Board<Piece> = Board::new();

        let mut sections = str.split(" ");
        
        let mut add_piece_char = |ch: char, square: u8| {
            let piece = match ch {
                'p' => Piece::BlackPawn,
                'r' => Piece::BlackRook,
                'n' => Piece::BlackKnight,
                'b' => Piece::BlackBishop,
                'q' => Piece::BlackQueen,
                'k' => Piece::BlackKing,
                'P' => Piece::WhitePawn,
                'R' => Piece::WhiteRook,
                'N' => Piece::WhiteKnight,
                'B' => Piece::WhiteBishop,
                'Q' => Piece::WhiteQueen,
                'K' => Piece::WhiteKing,
                _ => bail!("invalid piece character encountered"),
            };
            board[square as usize] = piece;
            Ok(())
        };
        
        let piece_section = sections.next().expect("piece section exists");
        let mut square = 0;
        for ch in piece_section.chars() {
            match ch {
                // skip <x> squares
                '1'..='8' => square += ch as u8 - '0' as u8,
                // next rank
                '/' => square += 8 - square % 8,
                // wow something else
                _ => {
                    add_piece_char(ch, (square % 8) + (7 - (square / 8)) * 8)?;
                    square += 1;
                }
            }
        }

        // who's g dang turn is it??
        let turn_section = sections.next().context("turn section exists")?;
        let ch = turn_section
            .chars()
            .next()
            .context("turn section has a char")?;
        let turn = match ch {
            'b' => Team::Black,
            'w' => Team::White,
            _ => bail!("invalid turn section"),
        };

        // castling rights
        let castling_section = sections.next().context("castling section exists")?;
        for ch in castling_section.chars() {
            match ch {
                // TODO: implement castling
                // 'q' => state.castling[0] = true,
                // 'k' => state.castling[1] = true,
                // 'Q' => state.castling[2] = true,
                // 'K' => state.castling[3] = true,
                '-' => {}
                _ => bail!("invalid castling section"),
            };
        }

        // TODO: implement en passant
        // en passant
        let en_passant_section = sections.next().context("en passant section exists")?;
        for [rank, file] in en_passant_section.chars().array_chunks() {
            // state.en_passant = Some(std_position_to_pos(rank, file)?)
        }

        // TODO: implement halfmove clock
        // halfmove clock
        let halfmove_section = sections.next().context("halfmove section exists")?;
        // state.halfmove_clock = halfmove_section.parse::<u32>()?;

        // TODO: implement fullmove counter
        // fullmove counter
        let fullmove_section = sections.next().context("fullmove section exists")?;
        // state.fullmove_counter = fullmove_section.parse::<u32>()?;

        if let Some(_) = sections.next() {
            bail!("encountered too many sections in FEN string")
        }
        
        // Ok(state)
        todo!()
    }
}
