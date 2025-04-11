use crate::board::Chessboard;

bitflags::bitflags! {
    /// Stores whether or not each castling option is valid
    #[derive(Debug)]
    pub struct CastlingRights:u8 {
        const WHITE_KINGSIDE  = 0b0001;
        const WHITE_QUEENSIDE = 0b0010;
        const BLACK_KINGSIDE  = 0b0100;
        const BLACK_QUEENSIDE = 0b1000;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    /// Tried to move from an empty square
    NoPieceAtSource,

    /// Tried to move opponent's piece
    NotYourTurn,

    /// Destination square is occupied by your own piece
    FriendlyFire,

    /// Move is not valid for that piece
    InvalidMoveForPiece,

    /// Move would leave or put king in check
    KingInCheck,

    /// Tried to castle illegally
    IllegalCastle,

    /// Tried to capture en passant when it's not available
    InvalidEnPassant,

    /// Tried to promote without a pawn or with invalid target
    BadPromotion,

    /// Fallback error with human-readable explanation
    InvalidMove { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveResult {
    pub capture: bool,
    pub check: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub promotion: Option<char>, // like 'Q', 'N', etc.
    pub castle: Option<CastleSide>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleSide {
    Kingside,
    Queenside,
}

/// Represents a chess game
#[derive(Debug)]
pub struct Game {
    /// The `Chessboard` used by this game
    board: Chessboard,
    /// Whether or not it's White's turn (if false, it is Black's turn)
    pub white_to_move: bool,
    /// A `Flags` representing whether or not each castling move is still valid
    pub castling_rights: CastlingRights,
    /// The target square of the valid en passant move, if any. Otherwise, `None`
    pub en_passant_square: Option<usize>,
    /// The number of halfmoves since the last capture or pawn advance
    pub halfmove_clock: u32,
    /// Fullmove number
    pub fullmove_number: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Creates a new game of chess
    pub fn new() -> Self {
        Self {
            board: Chessboard::new_starting(),
            white_to_move: true,
            castling_rights: CastlingRights::all(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Creates a new `Game` from a FEN string
    pub fn from_fen(fen_str: &str) -> Result<Self, String> {
        let mut board = Chessboard::new_empty();
        let mut rank: usize = 7;
        let mut file = 0;

        let fields: Vec<&str> = fen_str.split_whitespace().collect();
        if fields.len() != 6 {
            return Err("Invalid FEN: String must have exactly 6 fields".into());
        }

        let fen_piece_placement = fields[0];
        let fen_active_color = fields[1];
        let fen_castling_availability = fields[2];
        let fen_en_passant_target = fields[3];
        let fen_halfmove_clock = fields[4];
        let fen_fullmove_number = fields[5];

        for c in fen_piece_placement.chars() {
            match c {
                '1'..='8' => {
                    file += (c as u8 - b'0') as usize;
                }
                '/' => {
                    if file != 8 {
                        return Err(format!(
                            "Invalid FEN: Rank {} does not have 8 files",
                            rank + 1
                        ));
                    }
                    if rank == 0 {
                        return Err("Invalid FEN: String contains more than 8 ranks".into());
                    }
                    rank -= 1;
                    file = 0;
                }
                'p' | 'r' | 'n' | 'b' | 'q' | 'k' | 'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => {
                    if file >= 8 || rank > 7 {
                        return Err("Invalid FEN: Piece placement overflowed board".into());
                    }
                    let square = rank * 8 + file;
                    match c {
                        'p' => board.black_pawns |= 1 << square,
                        'r' => board.black_rooks |= 1 << square,
                        'n' => board.black_knights |= 1 << square,
                        'b' => board.black_bishops |= 1 << square,
                        'q' => board.black_queens |= 1 << square,
                        'k' => board.black_kings |= 1 << square,
                        'P' => board.white_pawns |= 1 << square,
                        'R' => board.white_rooks |= 1 << square,
                        'N' => board.white_knights |= 1 << square,
                        'B' => board.white_bishops |= 1 << square,
                        'Q' => board.white_queens |= 1 << square,
                        'K' => board.white_kings |= 1 << square,
                        _ => unreachable!(),
                    }
                    file += 1;
                }
                _ => {
                    return Err(format!(
                        "Invalid FEN: Invalid character '{}' in piece placement",
                        c
                    ))
                }
            }
        }
        if rank != 0 {
            return Err("Invalid FEN: Fewer than 8 ranks".into());
        }
        if file != 8 {
            return Err("Invalid FEN: Rank 1 does not have 8 files".into());
        }

        let white_to_move = match fen_active_color {
            "w" => true,
            "b" => false,
            _ => {
                return Err(format!(
                    "Invalid FEN: Unknown active color '{}'",
                    fen_active_color
                ))
            }
        };

        let mut castling_rights = CastlingRights::empty();
        if fen_castling_availability != "-" {
            for c in fen_castling_availability.chars() {
                match c {
                    'K' => castling_rights |= CastlingRights::WHITE_KINGSIDE,
                    'Q' => castling_rights |= CastlingRights::WHITE_QUEENSIDE,
                    'k' => castling_rights |= CastlingRights::BLACK_KINGSIDE,
                    'q' => castling_rights |= CastlingRights::BLACK_QUEENSIDE,
                    _ => {
                        return Err(format!(
                            "Invalid FEN: Invalid character '{}' in castling availability (hint: acceptable characters are 'K', 'Q', 'k', and 'q')",
                            c
                        ))
                    }
                }
            }
        }

        let en_passant_square = if fen_en_passant_target == "-" {
            None
        } else {
            let square =
                Chessboard::square_from_notation(fen_en_passant_target).ok_or_else(|| {
                    format!(
                        "Invalid FEN: Invalid en passant square '{}'",
                        fen_en_passant_target
                    )
                })?;
            let rank = square / 8;
            if rank != 2 && rank != 5 {
                return Err(format!(
                    "Invalid FEN: Invalid en passant square '{}' (hint: en passant is only valid on ranks 3 and 6)",
                    fen_en_passant_target
                ));
            }
            Some(square)
        };

        let halfmove_clock = fen_halfmove_clock.parse::<u32>().map_err(|_| {
            format!(
                "Invalid FEN: '{}' is not a valid u32 for halfmove clock",
                fen_halfmove_clock
            )
        })?;
        let fullmove_number = fen_fullmove_number.parse::<u32>().map_err(|_| {
            format!(
                "Invalid FEN: '{}' is not a valid u32 for fullmove number",
                fen_fullmove_number
            )
        })?;
        if fullmove_number == 0 {
            return Err("Invalid FEN: Fullmove number must be >= 1".into());
        }

        Ok(Game {
            board,
            white_to_move,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        })
    }

    pub fn board(&self) -> &Chessboard {
        &self.board
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<MoveResult, GameError> {
        // TODO: Validate move

        self.board.move_piece(from, to);

        // Return move result
        let result = MoveResult {
            //TODO: Actually populate these
            capture: false,
            check: false,
            checkmate: false,
            stalemate: false,
            promotion: None, // Some('Q') or None
            castle: None,    // Some(CastleSide::Kingside) or None
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if the error message contains an expected substring
    fn assert_err_contains(fen: &str, expected: &str) {
        let res = Game::from_fen(fen);
        assert!(
            res.is_err(),
            "expected error for fen: `{}`, but got ok",
            fen
        );
        let err = res.unwrap_err();
        assert!(
            err.to_lowercase().contains(&expected.to_lowercase()),
            "expected error to contain `{}`, got `{}`",
            expected,
            err
        );
    }

    #[test]
    fn test_missing_fields() {
        // Only 5 fields instead of 6
        let fen = "8/8/8/8/8/8/8/8 w - - 0";
        assert_err_contains(fen, "exactly 6 fields");
    }

    #[test]
    fn test_piece_placement_overflow() {
        // Rank 1: '8' fills file count to 8, then additional piece 'p' overflows
        let fen = "8p/8/8/8/8/8/8/8 w - - 0 1";
        assert_err_contains(fen, "piece placement overflowed board");
    }

    #[test]
    fn test_incomplete_rank() {
        // Rank 8: '3n' gives only 4 files
        let fen = "3n/8/8/8/8/8/8/8 w - - 0 1";
        assert_err_contains(fen, "does not have 8 files");
    }

    #[test]
    fn test_fewer_than_8_ranks() {
        // Only 7 ranks provided
        let fen = "8/8/8/8/8/8/8 w - - 0 1";
        assert_err_contains(fen, "fewer than 8 ranks");
    }

    #[test]
    fn test_unknown_active_color() {
        // 'x' is not a valid color
        let fen = "8/8/8/8/8/8/8/8 x - - 0 1";
        assert_err_contains(fen, "unknown active color 'x'");
    }

    #[test]
    fn test_invalid_castling_availability() {
        // 'a' is not a valid castling availability string
        let fen = "8/8/8/8/8/8/8/8 w a - 0 1";
        assert_err_contains(
            fen,
            "invalid character 'a' in castling availability (hint: acceptable characters are 'K', 'Q', 'k', and 'q')",
        );
    }

    #[test]
    fn test_invalid_character_in_piece_placement() {
        // 'x' is not a valid piece or digit
        let fen = "8x/8/8/8/8/8/8/8 w - - 0 1";
        assert_err_contains(fen, "invalid character 'x' in piece placement");
    }

    #[test]
    fn test_invalid_en_passant_square() {
        // 'z9' is not a valid square
        let fen = "8/8/8/8/8/8/8/8 w - z9 0 1";
        assert_err_contains(fen, "invalid en passant square 'z9'");
    }

    #[test]
    fn test_en_passant_square_wrong_rank() {
        // 'a4' is a valid square, but not for an en passant to occur
        let fen = "8/8/8/8/8/8/8/8 w - a4 0 1";
        assert_err_contains(
            fen,
            "invalid en passant square 'a4' (hint: en passant is only valid on ranks 3 and 6)",
        );
    }

    #[test]
    fn test_invalid_halfmove_clock() {
        // 'x' is not a number
        let fen = "8/8/8/8/8/8/8/8 w - - x 1";
        assert_err_contains(fen, "not a valid u32 for halfmove clock");
    }

    #[test]
    fn test_invalid_fullmove_number() {
        // 'x' is not a number
        let fen = "8/8/8/8/8/8/8/8 w - - 0 x";
        assert_err_contains(fen, "not a valid u32 for fullmove number");
    }

    #[test]
    fn test_fullmove_number_zero() {
        // Fullmove number cannot be 0
        let fen = "8/8/8/8/8/8/8/8 w - - 0 0";
        assert_err_contains(fen, "fullmove number must be >= 1");
    }

    #[test]
    fn test_valid_fen() {
        // Game starting conditions
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = Game::from_fen(fen);
        assert!(
            game.is_ok(),
            "expected valid FEN, got error: {:?}",
            game.unwrap_err()
        );
        let game = game.unwrap();
        assert!(game.white_to_move, "expected white to move");
        // Further validations about board state could be added here
    }
}
