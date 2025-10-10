#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Generic,
    GenericBlocker,
}

impl PieceType {
    pub fn name(self) -> &'static str {
        match self {
            PieceType::WhitePawn => "white pawn",
            PieceType::WhiteKnight => "white knight",
            PieceType::WhiteBishop => "white bishop",
            PieceType::WhiteRook => "white rook",
            PieceType::WhiteQueen => "white queen",
            PieceType::WhiteKing => "white king",
            PieceType::BlackPawn => "black pawn",
            PieceType::BlackKnight => "black knight",
            PieceType::BlackBishop => "black bishop",
            PieceType::BlackRook => "black rook",
            PieceType::BlackQueen => "black queen",
            PieceType::BlackKing => "black king",
            PieceType::Generic => "generic piece",
            PieceType::GenericBlocker => "generic blocker",
        }
    }

    pub fn symbol(self) -> char {
        match self {
            PieceType::WhitePawn => 'P',
            PieceType::WhiteKnight => 'N',
            PieceType::WhiteBishop => 'B',
            PieceType::WhiteRook => 'R',
            PieceType::WhiteQueen => 'Q',
            PieceType::WhiteKing => 'K',
            PieceType::BlackPawn => 'p',
            PieceType::BlackKnight => 'n',
            PieceType::BlackBishop => 'b',
            PieceType::BlackRook => 'r',
            PieceType::BlackQueen => 'q',
            PieceType::BlackKing => 'k',
            PieceType::Generic => '*',
            PieceType::GenericBlocker => 'x',
        }
    }

    pub fn unicode(self) -> char {
        match self {
            PieceType::WhitePawn => '♙',
            PieceType::WhiteKnight => '♘',
            PieceType::WhiteBishop => '♗',
            PieceType::WhiteRook => '♖',
            PieceType::WhiteQueen => '♕',
            PieceType::WhiteKing => '♔',
            PieceType::BlackPawn => '♟',
            PieceType::BlackKnight => '♞',
            PieceType::BlackBishop => '♝',
            PieceType::BlackRook => '♜',
            PieceType::BlackQueen => '♛',
            PieceType::BlackKing => '♚',
            PieceType::Generic => '⬢',
            PieceType::GenericBlocker => '✖',
        }
    }
}
