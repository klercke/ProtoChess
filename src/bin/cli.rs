use protochess::board::{self, Bitboard, Chessboard};
use protochess::game::Game;
use protochess::init;
use protochess::masks;

use std::io::{self, Write};

use clap::{Parser, Subcommand};
use shellish_parse::parse;

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
    #[clap(visible_alias = "c")]
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
        square_alg: String,
    },

    /// Import board state from a FEN string
    Fen { fen_str: String },

    /// Prints the game state
    #[clap(visible_aliases = ["show", "p", "s"])]
    Print,

    /// Moves a piece from `from` to `to`
    #[clap(visible_aliases = ["m", "mv"])]
    Move {
        /// The square the piece currently resides on
        from: String,

        /// The target square for the piece to move to
        to: String,
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
    Bishop,
    Rook,
    WhitePawn,
    BlackPawn,
}

/// Custom parser for LookupTablePiece
fn parse_lookup_table_piece(s: &str) -> Result<LookupTablePiece, String> {
    match s.to_uppercase().as_str() {
        "KNIGHT" | "N" => Ok(LookupTablePiece::Knight),
        "KING" | "K" => Ok(LookupTablePiece::King),
        "BISHOP" | "B" => Ok(LookupTablePiece::Bishop),
        "ROOK" | "R" => Ok(LookupTablePiece::Rook),
        "WHITEPAWN" | "WP" => Ok(LookupTablePiece::WhitePawn),
        "BLACKPAWN" | "BP" => Ok(LookupTablePiece::BlackPawn),
        _ => Err(format!(
            "Invalid piece: {}. Must be KNIGHT, KING, BISHOP, ROOK, WHITEPAWN, or BLACKPAWN (or N, K, B, R, WP, or BP).",
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
    let mut game = Game::new();
    game.board().display();

    loop {
        print!("PC> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("failed to read input");
            continue;
        }

        let input = input.trim();
        let args = match parse(input, false) {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("parse error: {}", e);
                continue;
            }
        };

        match Cli::try_parse_from(std::iter::once("").chain(args.iter().map(String::as_str))) {
            Ok(cli) => {
                if let Some(command) = cli.command {
                    handle_command(command, &mut game);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

/// Processes commands and arguments
fn handle_command(command: Commands, game: &mut Game) {
    match command {
        Commands::Convert { notation } => {
            convert_command(&notation);
        }

        Commands::Exit => {
            println!("Bye!");
            std::process::exit(0);
        }

        Commands::GetValidMoves { square_alg, piece } => {
            let square_id = Chessboard::square_from_notation(&square_alg).unwrap();
            let move_table = match piece {
                LookupTablePiece::Knight => board::KNIGHT_MOVE_TABLE
                    .get()
                    .expect("Knight table not initialized."),
                LookupTablePiece::King => board::KING_MOVE_TABLE
                    .get()
                    .expect("King table not initialized."),
                // TODO: This is not technically a move table. Fix that.
                LookupTablePiece::Bishop => masks::BISHOP_BLOCKER_MASKS
                    .get()
                    .expect("Bishop table not initialized."),
                LookupTablePiece::Rook => masks::ROOK_BLOCKER_MASKS
                    .get()
                    .expect("Rook table not initialized."),
                LookupTablePiece::WhitePawn => board::WHITE_PAWN_ATTACK_TABLE
                    .get()
                    .expect("White pawn table not initialized."),
                LookupTablePiece::BlackPawn => board::BLACK_PAWN_ATTACK_TABLE
                    .get()
                    .expect("Black pawn table not initialized."),
            };

            pretty_print_bitboard(move_table[square_id], Some(square_id));
        }

        Commands::Fen { fen_str } => match Game::from_fen(&fen_str) {
            Ok(new_game) => *game = new_game,
            Err(e) => {
                eprintln!("{}", e);
            }
        },

        Commands::Print => {
            game.board().display();
        }

        Commands::Move { from, to } => {
            let from_id = Chessboard::square_from_notation(&from).unwrap();
            let to_id = Chessboard::square_from_notation(&to).unwrap();

            match game.make_move(from_id, to_id) {
                Ok(_) => game.board().display(),
                Err(e) => {
                    eprintln!("Invalid move: {:?}", e);
                }
            }
        }
    }
}

pub fn main() {
    init::precompute();

    // Process command line args
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        let mut game = Game::new();
        handle_command(command, &mut game);
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
        assert_eq!(parse_lookup_table_piece("B"), Ok(LookupTablePiece::Bishop));
        assert_eq!(
            parse_lookup_table_piece("BISHOP"),
            Ok(LookupTablePiece::Bishop)
        );
        assert_eq!(
            parse_lookup_table_piece("BP"),
            Ok(LookupTablePiece::BlackPawn)
        );
        assert_eq!(
            parse_lookup_table_piece("BLACKPAWN"),
            Ok(LookupTablePiece::BlackPawn)
        );
        assert_eq!(
            parse_lookup_table_piece("WP"),
            Ok(LookupTablePiece::WhitePawn)
        );
        assert_eq!(
            parse_lookup_table_piece("WHITEPAWN"),
            Ok(LookupTablePiece::WhitePawn)
        );
        assert!(parse_lookup_table_piece("QUEEN").is_err());
        assert!(parse_lookup_table_piece("X").is_err());
        assert!(parse_lookup_table_piece("").is_err());
    }
}
