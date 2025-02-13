use protochess::board::{self, Bitboard, Chessboard};
use protochess::init;

use std::io::{self, Write};

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

/// Pieces with lookup tables
#[derive(Debug, Clone, Copy, PartialEq)]
enum LookupTablePiece {
    Knight,
    King,
    WhitePawn,
    BlackPawn,
}

/// Custom parser for LookupTablePiece
fn parse_lookup_table_piece(s: &str) -> Result<LookupTablePiece, String> {
    match s.to_uppercase().as_str() {
        "KNIGHT" | "N" => Ok(LookupTablePiece::Knight),
        "KING" | "K" => Ok(LookupTablePiece::King),
        "WHITEPAWN" | "W" => Ok(LookupTablePiece::WhitePawn),
        "BLACKPAWN" | "B" => Ok(LookupTablePiece::BlackPawn),
        _ => Err(format!(
            "Invalid piece: {}. Must be KNIGHT, KING, or PAWN (or N, K, or P).",
            s
        )),
    }
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
        // TODO: Let this take standard notation as well (ex. a6)
        Commands::GetValidMoves { square, piece } => {
            if square > 63 {
                println!("Invalid square: {}", square)
            } else {
                let move_table = match piece {
                    LookupTablePiece::Knight => board::KNIGHT_MOVE_TABLE
                        .get()
                        .expect("Knight table not initialized."),
                    LookupTablePiece::King => board::KING_MOVE_TABLE
                        .get()
                        .expect("King table not initialized."),
                    LookupTablePiece::WhitePawn => board::WHITE_PAWN_ATTACK_TABLE
                        .get()
                        .expect("Pawn table not initialized."),
                    LookupTablePiece::BlackPawn => board::BLACK_PAWN_ATTACK_TABLE
                        .get()
                        .expect("Pawn table not initialized."),
                };

                pretty_print_bitboard(move_table[square], Some(square));
            }
        }
    }
}

pub fn main() {
    init::precompute();

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
    fn test_lookup_table_piece_parsing() {
        assert_eq!(parse_lookup_table_piece("N"), Ok(LookupTablePiece::Knight));
        assert_eq!(
            parse_lookup_table_piece("KNIGHT"),
            Ok(LookupTablePiece::Knight)
        );
        assert_eq!(parse_lookup_table_piece("K"), Ok(LookupTablePiece::King));
        assert_eq!(parse_lookup_table_piece("KING"), Ok(LookupTablePiece::King));
        assert_eq!(
            parse_lookup_table_piece("B"),
            Ok(LookupTablePiece::BlackPawn)
        );
        assert_eq!(
            parse_lookup_table_piece("BLACKPAWN"),
            Ok(LookupTablePiece::BlackPawn)
        );
        assert_eq!(
            parse_lookup_table_piece("W"),
            Ok(LookupTablePiece::WhitePawn)
        );
        assert_eq!(
            parse_lookup_table_piece("WHITEPAWN"),
            Ok(LookupTablePiece::WhitePawn)
        );
        assert!(parse_lookup_table_piece("QUEEN").is_err());
        assert!(parse_lookup_table_piece("BISHOP").is_err());
        assert!(parse_lookup_table_piece("X").is_err());
        assert!(parse_lookup_table_piece("").is_err());
    }
}
