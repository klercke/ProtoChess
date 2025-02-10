use std::io::{self, Write};
use std::sync::OnceLock;
use std::time::Instant;

use clap::{Parser, Subcommand};

/// Define command-line args
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Define commands
#[derive(Subcommand)]
enum Commands {
    /// Convert to/from square ID and standard algebraic notation
    Convert { notation: String },

    /// Exit the program
    #[clap(visible_aliases = ["quit", "q"])]
    Exit,

    /// For a given square ID, get all valid moves for a knight on that square
    #[clap(visible_alias = "gvm")]
    GetValidMoves {
        /// The chess piece type (KNIGHT, KING, OR PAWN (or N, K, or P))
        #[clap(value_parser = parse_lookup_table_piece)]
        piece: LookupTablePiece,

        /// The square to check
        square: usize,
    },
}

/// Pieces with lookup tables
#[derive(Debug, Clone, Copy, PartialEq)]
enum LookupTablePiece {
    Knight,
    King,
    Pawn,
}

/// Custom parser for LookupTablePiece
fn parse_lookup_table_piece(s: &str) -> Result<LookupTablePiece, String> {
    match s.to_uppercase().as_str() {
        "KNIGHT" | "N" => Ok(LookupTablePiece::Knight),
        "KING" | "K" => Ok(LookupTablePiece::King),
        "PAWN" | "P" => Ok(LookupTablePiece::Pawn),
        _ => Err(format!(
            "Invalid piece: {}. Must be KNIGHT, KING, or PAWN (or N, K, or P).",
            s
        )),
    }
}

/// Represents a chessboard as a u64
pub type Bitboard = u64;

/// Constants for each file/column
#[allow(dead_code)]
const FILE_A: Bitboard = 0x0101_0101_0101_0101;
#[allow(dead_code)]
const FILE_B: Bitboard = 0x0202_0202_0202_0202;
#[allow(dead_code)]
const FILE_C: Bitboard = 0x0404_0404_0404_0404;
#[allow(dead_code)]
const FILE_D: Bitboard = 0x0808_0808_0808_0808;
#[allow(dead_code)]
const FILE_E: Bitboard = 0x1010_1010_1010_1010;
#[allow(dead_code)]
const FILE_F: Bitboard = 0x2020_2020_2020_2020;
#[allow(dead_code)]
const FILE_G: Bitboard = 0x4040_4040_4040_4040;
#[allow(dead_code)]
const FILE_H: Bitboard = 0x8080_8080_8080_8080;

/// Constants for each rank/row
#[allow(dead_code)]
const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
#[allow(dead_code)]
const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
#[allow(dead_code)]
const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
#[allow(dead_code)]
const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
#[allow(dead_code)]
const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
#[allow(dead_code)]
const RANK_6: Bitboard = 0x0000_FF00_0000_0000;
#[allow(dead_code)]
const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
#[allow(dead_code)]
const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

/// Store move tables for knights, kings, and pawn attacks
static KNIGHT_MOVE_TABLE: OnceLock<[u64; 64]> = OnceLock::new();
static KING_MOVE_TABLE: OnceLock<[u64; 64]> = OnceLock::new();
static PAWN_ATTACK_TABLE: OnceLock<[u64; 64]> = OnceLock::new();

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

/// Calls the relevant function to convert between square ID and algebraic notation
fn convert_command(input: &String) {
    if let Ok(square) = input.parse::<usize>() {
        if let Some(notation) = Chessboard::notation_from_square(square) {
            println!("{}", notation);
        } else {
            println!("Invalid square ID: {}", square);
        }
    } else if let Some(square) = Chessboard::square_from_notation(input) {
        println!("{}", square);
    } else {
        println!(
            "Could not interpret {} as algebraic notation or square ID",
            input
        );
    }
}

/// Precompute move tables for knights
fn precompute_knight_moves() -> [Bitboard; 64] {
    let mut move_table = [0; 64];

    for (square, moves) in move_table.iter_mut().enumerate() {
        // Set the bit corresponding to the knight's position
        let bit = 1u64 << square;

        // Create edge masks
        let not_file_a: Bitboard = !FILE_A;
        let not_file_h: Bitboard = !FILE_H;
        let not_file_ab: Bitboard = !(FILE_A | FILE_B);
        let not_file_gh: Bitboard = !(FILE_G | FILE_H);

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

/// Pretty print Bitboard for debugging
fn pretty_print_bitboard(bitboard: Bitboard, marker: Option<usize>) {
    // Start from rank 8
    for rank in (0..8).rev() {
        print!("{} ", rank + 1); // Rank labels (1-indexed)
        for file in 0..8 {
            let square = rank * 8 + file;
            let mask: Bitboard = 1 << square;
            if Some(square) == marker {
                print!("* ") // Draw marker
            } else if bitboard & mask != 0 {
                print!("X "); // Occupied square
            } else {
                print!(". "); // Empty square
            }
        }
        println!(); // Each rank on a new line
    }
    println!("  a b c d e f g h "); // File labels
}

/// Prompts the user to run commands interactively
fn interactive_mode() {
    // Generate and print board
    let board = Chessboard::new();
    board.display();

    loop {
        // Get command from user
        print!("PC> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        // Process command
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        match Cli::try_parse_from(std::iter::once("").chain(args.iter().map(String::as_str))) {
            Ok(cli) => {
                if let Some(command) = cli.command {
                    handle_command(command);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

/// Processes commands and arguments
fn handle_command(command: Commands) {
    match command {
        Commands::Convert { notation } => {
            convert_command(&notation);
        }
        Commands::Exit => {
            println!("Bye!");
            std::process::exit(0);
        }
        Commands::GetValidMoves { square, piece } => {
            let move_table = match piece {
                LookupTablePiece::Knight => KNIGHT_MOVE_TABLE
                    .get()
                    .expect("Knight table not initialized."),
                LookupTablePiece::King => {
                    KING_MOVE_TABLE.get().expect("King table not initialized.")
                }
                LookupTablePiece::Pawn => PAWN_ATTACK_TABLE
                    .get()
                    .expect("Pawn table not initialized."),
            };

            pretty_print_bitboard(move_table[square], Some(square));
        }
    }
}

pub fn main() {
    // Precompute moves
    println!("Precompuing move tables...");
    let precompute_start_time = Instant::now();
    let _knight_move_table = KNIGHT_MOVE_TABLE.get_or_init(precompute_knight_moves);
    let precompute_duration = precompute_start_time.elapsed();
    println!(
        "Finished precomputing move tables in {}.{:03}ms ({}ns)",
        precompute_duration.as_millis(),
        precompute_duration.as_micros() % 1000,
        precompute_duration.as_nanos()
    );

    // Process command line args
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        handle_command(command);
    } else {
        // Go into interactive mode if no command is supplied
        interactive_mode();
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

    #[test]
    fn test_move_piece() {
        let mut board = Chessboard::new();
        assert!(board.is_square_occupied(1)); // Knight on b1
        assert!(!board.is_square_occupied(18)); // No piece on c3

        board.move_piece(1, 18); // Move knight from b1 to c3
        assert!(!board.is_square_occupied(1)); // Knight should be gone from b1
        assert!(board.is_square_occupied(18)); // Knight should be at c3
    }

    #[test]
    fn test_knight_move_generation() {
        let move_table = precompute_knight_moves();

        // Knight on b1 (square 1)
        let b1_moves = move_table[1];
        assert!(b1_moves & (1 << 16) != 0); // a3
        assert!(b1_moves & (1 << 18) != 0); // c3
        assert!(b1_moves & (1 << 11) != 0); // d2
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
    fn test_white_pieces_bitboard() {
        let board = Chessboard::new();
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
        let board = Chessboard::new();
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
        let board = Chessboard::new();
        let occupied = board.occupied();

        assert_eq!(occupied.count_ones(), 32); // 32 pieces total
        assert_eq!(occupied, board.white_pieces() | board.black_pieces()); // Should match all pieces combined
    }

    #[test]
    fn test_lookup_table_piece_parsing() {
        assert_eq!(parse_lookup_table_piece("N"), Ok(LookupTablePiece::Knight));
        assert_eq!(
            parse_lookup_table_piece("KNIGHT"),
            Ok(LookupTablePiece::Knight)
        );
        assert_eq!(parse_lookup_table_piece("K"), Ok(LookupTablePiece::King));
        assert_eq!(parse_lookup_table_piece("KING"), Ok(LookupTablePiece::King));
        assert_eq!(parse_lookup_table_piece("P"), Ok(LookupTablePiece::Pawn));
        assert_eq!(parse_lookup_table_piece("PAWN"), Ok(LookupTablePiece::Pawn));
        assert!(parse_lookup_table_piece("QUEEN").is_err());
        assert!(parse_lookup_table_piece("BISHOP").is_err());
        assert!(parse_lookup_table_piece("X").is_err());
        assert!(parse_lookup_table_piece("").is_err());
    }
}
