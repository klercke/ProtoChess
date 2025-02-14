#![allow(dead_code)]

use crate::board::Bitboard;

use std::sync::OnceLock;

/// Constants for each file/column
pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_B: Bitboard = 0x0202_0202_0202_0202;
pub const FILE_C: Bitboard = 0x0404_0404_0404_0404;
pub const FILE_D: Bitboard = 0x0808_0808_0808_0808;
pub const FILE_E: Bitboard = 0x1010_1010_1010_1010;
pub const FILE_F: Bitboard = 0x2020_2020_2020_2020;
pub const FILE_G: Bitboard = 0x4040_4040_4040_4040;
pub const FILE_H: Bitboard = 0x8080_8080_8080_8080;

/// Constants for each rank/row
pub const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
pub const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
pub const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
pub const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
pub const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
pub const RANK_6: Bitboard = 0x0000_FF00_0000_0000;
pub const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
pub const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

/// Siding piece blocker masks
pub static BISHOP_BLOCKER_MASKS: OnceLock<[Bitboard; 64]> = OnceLock::new();

/// Generate the blocker masks for bishops in each square
pub fn precompute_bishop_blocker_masks() -> [Bitboard; 64] {
    let mut blocker_masks = [0; 64];

    for (square, blockers) in blocker_masks.iter_mut().enumerate() {
        let rank = square % 8;
        let file = square / 8;

        // All of the below functions will only evaluate the center 6x6 square
        // since edge pieces cannot be blockers

        // Northeast
        for i in 1..7 {
            if file + i >= 7 || rank + i >= 7 {
                break;
            }
            *blockers |= 1 << ((rank + i) * 8 + (file + i));
        }

        // Northwest
        for i in 1..7 {
            if file <= i || rank + i >= 7 {
                break;
            }
            *blockers |= 1 << ((rank + i) * 8 + (file - i));
        }

        // Southwest
        for i in 1..7 {
            if file <= i || rank <= i {
                break;
            }
            *blockers |= 1 << ((rank - i) * 8 + (file - i));
        }

        // Southeast
        for i in 1..7 {
            if file + i >= 7 || rank <= i {
                break;
            }
            *blockers |= 1 << ((rank - i) * 8 + (file + i));
        }
    }

    blocker_masks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bishop_blocker_mask_generation() {
        let blocker_table = precompute_bishop_blocker_masks();

        // Bishop on a1 (square 0)
        let a1_blockers = blocker_table[0];
        assert!(a1_blockers & (1 << 9) != 0); // b2
        assert!(a1_blockers & (1 << 18) != 0); // c3
        assert!(a1_blockers & (1 << 27) != 0); // d4
        assert!(a1_blockers & (1 << 36) != 0); // e5
        assert!(a1_blockers & (1 << 45) != 0); // f6
        assert!(a1_blockers & (1 << 54) != 0); // g7
        assert_eq!(a1_blockers.count_ones(), 6); // Six valid blockers

        // Bishop on h8 (square 63)
        // Since edges are exluded, this should equal the a1 table
        let h8_blockers = blocker_table[63];
        assert_eq!(a1_blockers, h8_blockers);

        // Bishop on h1 (square 7)
        let h1_blockers = blocker_table[7];
        assert!(h1_blockers & (1 << 14) != 0); // g2
        assert!(h1_blockers & (1 << 21) != 0); // f3
        assert!(h1_blockers & (1 << 28) != 0); // e4
        assert!(h1_blockers & (1 << 35) != 0); // d5
        assert!(h1_blockers & (1 << 42) != 0); // c6
        assert!(h1_blockers & (1 << 49) != 0); // b7
        assert_eq!(h1_blockers.count_ones(), 6); // Six valid blockers

        // Bishop on d4 (square 27)
        let d4_blockers = blocker_table[27];
        assert!(d4_blockers & (1 << 9) != 0); // b2
        assert!(d4_blockers & (1 << 18) != 0); // c3
        assert!(d4_blockers & (1 << 36) != 0); // e5
        assert!(d4_blockers & (1 << 45) != 0); // f6
        assert!(d4_blockers & (1 << 54) != 0); // g7
        assert!(d4_blockers & (1 << 13) != 0); // f2
        assert!(d4_blockers & (1 << 20) != 0); // e3
        assert!(d4_blockers & (1 << 34) != 0); // c5
        assert!(d4_blockers & (1 << 41) != 0); // b6
        assert_eq!(d4_blockers.count_ones(), 9); // Nine blockers
    }
}
