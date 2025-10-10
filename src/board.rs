use crate::masks;
use crate::pieces::PieceType;

use std::sync::OnceLock;

/// Store move tables for knig
pub static KNIGHT_MOVE_TABLE: OnceLock<[Bitboard; 64]> = OnceLock::new();
pub static KING_MOVE_TABLE: OnceLock<[Bitboard; 64]> = OnceLock::new();
pub static WHITE_PAWN_ATTACK_TABLE: OnceLock<[Bitboard; 64]> = OnceLock::new();
pub static BLACK_PAWN_ATTACK_TABLE: OnceLock<[Bitboard; 64]> = OnceLock::new();

/// Represents a chessboard as a u64
pub type Bitboard = u64;

/// Chessboard made of bitboards representing each piece
#[derive(Debug)]
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
        Self::new_starting()
    }
}

impl Chessboard {
    /// Creates a new, blank Chessboard
    pub fn new_empty() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_0000,
            white_rooks: 0x0000_0000_0000_0000,
            white_knights: 0x0000_0000_0000_0000,
            white_bishops: 0x0000_0000_0000_0000,
            white_queens: 0x0000_0000_0000_0000,
            white_kings: 0x0000_0000_0000_0000,

            black_pawns: 0x0000_0000_0000_0000,
            black_rooks: 0x0000_0000_0000_0000,
            black_knights: 0x0000_0000_0000_0000,
            black_bishops: 0x0000_0000_0000_0000,
            black_queens: 0x0000_0000_0000_0000,
            black_kings: 0x0000_0000_0000_0000,
        }
    }

    /// Creates a new Chessboard with all pieces in their starting positions
    pub fn new_starting() -> Self {
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
        // Start from rank 8
        for rank in (0..8).rev() {
            print!("{} ", rank + 1); // Rank labels (1-indexed)
            for file in 0..8 {
                let square = rank * 8 + file;
                print!("{} ", self.piece_at(square));
            }
            println!(); // Each rank on a new line
        }
        println!("  a b c d e f g h "); // File labels
        println!();
    }

    /// Return a character representation of the piece at a given square
    fn piece_at(&self, square: usize) -> char {
        let mask = 1 << square;

        let pieces = [
            (self.white_pawns, PieceType::WhitePawn),
            (self.white_knights, PieceType::WhiteKnight),
            (self.white_bishops, PieceType::WhiteBishop),
            (self.white_rooks, PieceType::WhiteRook),
            (self.white_queens, PieceType::WhiteQueen),
            (self.white_kings, PieceType::WhiteKing),
            (self.black_pawns, PieceType::BlackPawn),
            (self.black_knights, PieceType::BlackKnight),
            (self.black_bishops, PieceType::BlackBishop),
            (self.black_rooks, PieceType::BlackRook),
            (self.black_queens, PieceType::BlackQueen),
            (self.black_kings, PieceType::BlackKing),
        ];

        for (bitboard, piece_type) in pieces {
            if bitboard & mask != 0 {
                return piece_type.symbol();
            }
        }

        '.'
    }

    /// Converts standard notation (e4) to square number (28)
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

    /// Converts square number (28) to standard notation (e4)
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

/// Precompute move tables for knights
pub fn precompute_knight_moves() -> [Bitboard; 64] {
    let mut move_table = [0; 64];

    for (square, moves) in move_table.iter_mut().enumerate() {
        // Set the bit corresponding to the knight's position
        let bit = 1u64 << square;

        // Create edge masks
        let not_file_a: Bitboard = !masks::FILE_A;
        let not_file_h: Bitboard = !masks::FILE_H;
        let not_file_ab: Bitboard = !(masks::FILE_A | masks::FILE_B);
        let not_file_gh: Bitboard = !(masks::FILE_G | masks::FILE_H);

        // Calculate valid moves with edge masks
        *moves |= (bit & not_file_a) << 15; // NNW
        *moves |= (bit & not_file_h) << 17; // NNE
        *moves |= (bit & not_file_ab) << 6; // WNW
        *moves |= (bit & not_file_gh) << 10; // ENE
        *moves |= (bit & not_file_gh) >> 6; // ESE
        *moves |= (bit & not_file_ab) >> 10; // WSW
        *moves |= (bit & not_file_h) >> 15; // SSE
        *moves |= (bit & not_file_a) >> 17; // SSW
    }

    move_table
}

/// Precompute move tables for kings
pub fn precompute_king_moves() -> [Bitboard; 64] {
    let mut move_table = [0; 64];

    for (square, moves) in move_table.iter_mut().enumerate() {
        // Set the bit corresponding to the king's position
        let bit = 1u64 << square;

        // Create edge masks
        let not_file_a: Bitboard = !masks::FILE_A;
        let not_file_h: Bitboard = !masks::FILE_H;

        // Calculate valid moves with edge masks
        *moves |= (bit & not_file_a) << 7; // NW
        *moves |= bit << 8; // N
        *moves |= (bit & not_file_h) << 9; // NE
        *moves |= (bit & not_file_a) >> 1; // W
        *moves |= (bit & not_file_h) << 1; // E
        *moves |= (bit & not_file_a) >> 9; // SW
        *moves |= bit >> 8; //S
        *moves |= (bit & not_file_h) >> 7; // SE
    }

    move_table
}

/// Precompute attack tables for pawns
pub fn precompute_pawn_attacks() -> ([Bitboard; 64], [Bitboard; 64]) {
    let mut black_attack_table = [0; 64];
    let mut white_attack_table = [0; 64];

    for square in 0..64 {
        // Set the bit corresponding to the pawn's position
        let bit = 1u64 << square;

        // Create edge masks
        let not_file_a: Bitboard = !masks::FILE_A;
        let not_file_h: Bitboard = !masks::FILE_H;

        // Calculate valid moves with edge masks
        black_attack_table[square] |= (bit >> 7) & not_file_a;
        black_attack_table[square] |= (bit >> 9) & not_file_h;
        white_attack_table[square] |= (bit << 7) & not_file_h;
        white_attack_table[square] |= (bit << 9) & not_file_a;
    }

    (black_attack_table, white_attack_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_move_generation() {
        let move_table = precompute_knight_moves();

        // Knight on b1 (square 1)
        let b1_moves = move_table[1];
        assert!(b1_moves & (1 << 11) != 0); // d2
        assert!(b1_moves & (1 << 16) != 0); // a3
        assert!(b1_moves & (1 << 18) != 0); // c3
        assert_eq!(b1_moves.count_ones(), 3); // Three valid moves

        // Knight on d4 (square 27)
        let d4_moves = move_table[27];
        assert!(d4_moves & (1 << 10) != 0); // c2
        assert!(d4_moves & (1 << 12) != 0); // e2
        assert!(d4_moves & (1 << 17) != 0); // b3
        assert!(d4_moves & (1 << 21) != 0); // f3
        assert!(d4_moves & (1 << 33) != 0); // b5
        assert!(d4_moves & (1 << 37) != 0); // f5
        assert!(d4_moves & (1 << 42) != 0); // c6
        assert!(d4_moves & (1 << 44) != 0); // e6
        assert_eq!(d4_moves.count_ones(), 8); // Eight valid moves
    }

    #[test]
    fn test_king_move_generation() {
        let move_table = precompute_king_moves();

        // King on a1 (square 0)
        let a1_moves = move_table[0];
        assert!(a1_moves & (1 << 1) != 0); // b1
        assert!(a1_moves & (1 << 8) != 0); // a2
        assert!(a1_moves & (1 << 9) != 0); // b2
        assert_eq!(a1_moves.count_ones(), 3); // Three valid moves

        // King on b1 (square 1)
        let b1_moves = move_table[1];
        assert!(b1_moves & (1 << 0) != 0); // a1
        assert!(b1_moves & (1 << 2) != 0); // c1
        assert!(b1_moves & (1 << 8) != 0); // a2
        assert!(b1_moves & (1 << 9) != 0); // b2
        assert!(b1_moves & (1 << 10) != 0); // c2
        assert_eq!(b1_moves.count_ones(), 5); // Three valid moves

        // King on d4 (square 27)
        let d4_moves = move_table[27];
        assert!(d4_moves & (1 << 18) != 0); // c3
        assert!(d4_moves & (1 << 19) != 0); // d3
        assert!(d4_moves & (1 << 20) != 0); // e3
        assert!(d4_moves & (1 << 26) != 0); // c4
        assert!(d4_moves & (1 << 28) != 0); // e4
        assert!(d4_moves & (1 << 34) != 0); // c5
        assert!(d4_moves & (1 << 35) != 0); // d5
        assert!(d4_moves & (1 << 36) != 0); // e5
        assert_eq!(d4_moves.count_ones(), 8); // Eight valid moves
    }

    #[test]
    fn test_pawn_attack_generation() {
        let (black_attack_table, white_attack_table) = precompute_pawn_attacks();

        // Black pawn on a1 (square 0)
        let a1_black_attacks = black_attack_table[0];
        assert_eq!(a1_black_attacks.count_ones(), 0); // Zero valid moves

        // Black pawn on h8 (square 63)
        let h8_black_attacks = black_attack_table[63];
        assert!(h8_black_attacks & (1 << 54) != 0); // g7
        assert_eq!(h8_black_attacks.count_ones(), 1); // One valid move

        // Black pawn on d4 (square 27)
        let d4_black_attacks = black_attack_table[27];
        assert!(d4_black_attacks & (1 << 18) != 0); // c3
        assert!(d4_black_attacks & (1 << 20) != 0); // e3
        assert_eq!(d4_black_attacks.count_ones(), 2); // Two valid moves

        // White pawn on a1 (square 0)
        let a1_white_attacks = white_attack_table[0];
        assert!(a1_white_attacks & (1 << 9) != 0); // b2
        assert_eq!(a1_white_attacks.count_ones(), 1); // One valid move

        // White pawn on h8 (square 63)
        let h8_white_attacks = white_attack_table[63];
        assert_eq!(h8_white_attacks.count_ones(), 0); // Zero valid moves

        // White pawn on d4 (square 27)
        let d4_white_attacks = white_attack_table[27];
        assert!(d4_white_attacks & (1 << 34) != 0); // c5
        assert!(d4_white_attacks & (1 << 36) != 0); // e5
        assert_eq!(d4_white_attacks.count_ones(), 2); // Two valid moves
    }

    #[test]
    fn test_move_piece() {
        let mut board = Chessboard::new_starting();
        assert!(board.is_square_occupied(1)); // Knight on b1
        assert!(!board.is_square_occupied(18)); // No piece on c3

        board.move_piece(1, 18); // Move knight from b1 to c3
        assert!(!board.is_square_occupied(1)); // Knight should be gone from b1
        assert!(board.is_square_occupied(18)); // Knight should be at c3
    }

    #[test]
    fn test_white_pieces_bitboard() {
        let board = Chessboard::new_starting();
        let white_pieces = board.white_pieces();

        assert!(white_pieces & board.white_pawns != 0);
        assert!(white_pieces & board.white_rooks != 0);
        assert!(white_pieces & board.white_knights != 0);
        assert!(white_pieces & board.white_bishops != 0);
        assert!(white_pieces & board.white_queens != 0);
        assert!(white_pieces & board.white_kings != 0);
        assert_eq!(white_pieces.count_ones(), 16); // 16 white pieces at start
    }

    #[test]
    fn test_black_pieces_bitboard() {
        let board = Chessboard::new_starting();
        let black_pieces = board.black_pieces();

        assert!(black_pieces & board.black_pawns != 0);
        assert!(black_pieces & board.black_rooks != 0);
        assert!(black_pieces & board.black_knights != 0);
        assert!(black_pieces & board.black_bishops != 0);
        assert!(black_pieces & board.black_queens != 0);
        assert!(black_pieces & board.black_kings != 0);
        assert_eq!(black_pieces.count_ones(), 16); // 16 black pieces at start
    }

    #[test]
    fn test_occupied_bitboard() {
        let board = Chessboard::new_starting();
        let occupied = board.occupied();

        assert_eq!(occupied.count_ones(), 32); // 32 pieces total
        assert_eq!(occupied, board.white_pieces() | board.black_pieces()); // Should match all pieces combined
    }

    #[test]
    fn test_square_from_notation() {
        assert_eq!(Chessboard::square_from_notation("a1"), Some(0));
        assert_eq!(Chessboard::square_from_notation("h8"), Some(63));
        assert_eq!(Chessboard::square_from_notation("e4"), Some(28));
        assert_eq!(Chessboard::square_from_notation("d5"), Some(35));
        assert_eq!(Chessboard::square_from_notation("invalid"), None);
        assert_eq!(Chessboard::square_from_notation("a9"), None);
        assert_eq!(Chessboard::square_from_notation("z5"), None);
        assert_eq!(Chessboard::square_from_notation("e0"), None);
        assert_eq!(Chessboard::square_from_notation("h"), None);
        assert_eq!(Chessboard::square_from_notation("5"), None);
        assert_eq!(Chessboard::square_from_notation(""), None);
    }

    #[test]
    fn test_notation_from_square() {
        assert_eq!(Chessboard::notation_from_square(0), Some("a1".to_string()));
        assert_eq!(Chessboard::notation_from_square(63), Some("h8".to_string()));
        assert_eq!(Chessboard::notation_from_square(28), Some("e4".to_string()));
        assert_eq!(Chessboard::notation_from_square(35), Some("d5".to_string()));
        assert_eq!(Chessboard::notation_from_square(64), None);
        assert_eq!(Chessboard::notation_from_square(100), None);
        assert_eq!(Chessboard::notation_from_square(999), None);
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
