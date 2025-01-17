use std::io::{self, Write};

/// Represents a bitboard
pub type Bitboard = u64;

///// Constants for the bitboard's size and bitmasks
//const BOARD_SIZE: usize = 64;
//const FILE_A: Bitboard = 0x0101_0101_0101_0101;
//const FILE_H: Bitboard = 0x8080_8080_8080_8080;
//const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
//const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

/// Chessboard made of bitboards representing each piece
pub struct Chessboard {
    pub white_pawns: Bitboard,
    pub white_rooks: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_kings: Bitboard,
    pub black_pawns: Bitboard,
    pub black_rooks: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_kings: Bitboard,
}

impl Default for Chessboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Chessboard {
    /// Creates a new chessboard with all pieces in their starting positions
    pub fn new() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_FF00,   // ... 1111 1111 ...
            white_rooks: 0x0000_0000_0000_0081,   // ... 1000 0001
            white_knights: 0x0000_0000_0000_0042, // ... 0100 0010
            white_bishops: 0x0000_0000_0000_0024, // ... 0010 0100
            white_queens: 0x0000_0000_0000_0008,  // ... 0000 1000
            white_kings: 0x0000_0000_0000_0010,   // ... 0001 0000

            black_pawns: 0x00FF_0000_0000_0000, // ... 1111 1111 ...
            black_rooks: 0x8100_0000_0000_0000, // 1000 0001 ...
            black_knights: 0x4200_0000_0000_0000, // 0100 0010 ...
            black_bishops: 0x2400_0000_0000_0000, // 0010 0100 ...
            black_queens: 0x0800_0000_0000_0000, // 0000 1000 ...
            black_kings: 0x1000_0000_0000_0000, // 0001 0000 ...
        }
    }

    /// Checks if a given square is occupied
    pub fn is_square_occupied(&self, square: usize) -> bool {
        let mask = 1 << square;
        (self.occupied() & mask) != 0
    }

    /// Returns a bitboard of all occupied squares
    pub fn occupied(&self) -> Bitboard {
        self.white_pieces() | self.black_pieces()
    }

    /// Returns a bitboard of all white pieces
    pub fn white_pieces(&self) -> Bitboard {
        self.white_pawns
            | self.white_rooks
            | self.white_knights
            | self.white_bishops
            | self.white_queens
            | self.white_kings
    }

    /// Returns a bitboard of all black pieces
    pub fn black_pieces(&self) -> Bitboard {
        self.black_pawns
            | self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_queens
            | self.black_kings
    }

    /// Moves the piece at a given square to another
    pub fn move_piece(&mut self, from: usize, to: usize) {
        let mask_from = 1 << from;
        let mask_to = 1 << to;

        // Search through each bitboard to find the one with our target piece
        for piece in [
            &mut self.white_pawns,
            &mut self.white_rooks,
            &mut self.white_knights,
            &mut self.white_bishops,
            &mut self.white_queens,
            &mut self.white_kings,
            &mut self.black_pawns,
            &mut self.black_rooks,
            &mut self.black_knights,
            &mut self.black_bishops,
            &mut self.black_queens,
            &mut self.black_kings,
        ] {
            if *piece & mask_from != 0 {
                *piece &= !mask_from; // Remove the piece from its original position
                *piece |= mask_to; // Put the piece down in its destination
                break;
            }
        }
    }

    /// Pretty(-ish) print the board
    pub fn display(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                print!("{} ", self.piece_at(square));
            }
            println!();
        }
    }

    /// Return a character representation of the piece at a given square
    fn piece_at(&self, square: usize) -> char {
        let mask = 1 << square;
        if self.white_pawns & mask != 0 {
            'X'
        } else if self.white_rooks & mask != 0 {
            'R'
        } else if self.white_knights & mask != 0 {
            'N'
        } else if self.white_bishops & mask != 0 {
            'B'
        } else if self.white_queens & mask != 0 {
            'Q'
        } else if self.white_kings & mask != 0 {
            'K'
        } else if self.black_pawns & mask != 0 {
            'x'
        } else if self.black_rooks & mask != 0 {
            'r'
        } else if self.black_knights & mask != 0 {
            'n'
        } else if self.black_bishops & mask != 0 {
            'b'
        } else if self.black_queens & mask != 0 {
            'q'
        } else if self.black_kings & mask != 0 {
            'k'
        } else {
            '.'
        }
    }

    /// Convert standard notation (e4) to square number (28)
    pub fn square_from_notation(notation: &str) -> Option<usize> {
        if notation.len() != 2 {
            // TODO: Throw an error
            return None;
        }
        let file = notation.chars().nth(0)?;
        let rank = notation.chars().nth(1)?;

        if !("a"..="h").contains(&file.to_string().as_str())
            || !("1"..="8").contains(&rank.to_string().as_str())
        {
            // TODO: Throw an error
            return None;
        }

        let file_index = (file as u8 - b'a') as usize;
        let rank_index = (rank as u8 - b'1') as usize;

        Some(rank_index * 8 + file_index)
    }

    /// Convert square number (28) to standard notation (e4)
    pub fn notation_from_square(square: usize) -> Option<String> {
        if square >= 64 {
            // TODO: Throw an error
            return None;
        }

        let file = (square % 8) as u8 + b'a';
        let rank = (square / 8) as u8 + b'1';

        Some(format!("{}{}", file as char, rank as char))
    }
}

pub fn main() {
    let board = Chessboard::new();
    board.display();

    loop {
        print!("Enter a command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "help" => {
                println!("Available commands:");
                println!("  help    - Show this menu");
                println!("  quit    - Exits protochess");
                println!("  convert - Converts between internal square ID and algebraic notation");
            }
            "quit" => {
                println!("Bye!");
                break;
            }
            "convert" => {
                print!("Enter square ID or algebraic square name: ");
                io::stdout().flush().unwrap();

                let mut convert_input = String::new();
                io::stdin().read_line(&mut convert_input).unwrap();
                let convert_input = convert_input.trim();

                if let Ok(square) = convert_input.parse::<usize>() {
                    if let Some(notation) = Chessboard::notation_from_square(square) {
                        println!("Square #{} is {}", square, notation);
                    } else {
                        println!("Invalid square ID");
                    }
                } else if let Some(square) = Chessboard::square_from_notation(convert_input) {
                    println!("Square {} is #{}", convert_input, square);
                } else {
                    println!("Invalid input");
                }
            }
            _ => {
                println!("Unknown/invalid command. Type 'help' for a list of commands");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_notation() {
        assert_eq!(Chessboard::square_from_notation("a1"), Some(0));
        assert_eq!(Chessboard::square_from_notation("h8"), Some(63));
        assert_eq!(Chessboard::square_from_notation("e4"), Some(28));
        assert_eq!(Chessboard::square_from_notation("d5"), Some(35));
        assert_eq!(Chessboard::square_from_notation("invalid"), None);
        assert_eq!(Chessboard::square_from_notation("a9"), None);
    }

    #[test]
    fn test_notation_from_square() {
        assert_eq!(Chessboard::notation_from_square(0), Some("a1".to_string()));
        assert_eq!(Chessboard::notation_from_square(63), Some("h8".to_string()));
        assert_eq!(Chessboard::notation_from_square(28), Some("e4".to_string()));
        assert_eq!(Chessboard::notation_from_square(35), Some("d5".to_string()));
        assert_eq!(Chessboard::notation_from_square(64), None);
    }

    #[test]
    fn test_round_trip_conversion() {
        for square in 0..64 {
            let notation = Chessboard::notation_from_square(square).unwrap();
            let back_to_square = Chessboard::square_from_notation(&notation).unwrap();
            assert_eq!(square, back_to_square);
        }
    }
}
