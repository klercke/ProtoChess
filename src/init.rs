use crate::{board, masks};

use std::sync::OnceLock;
use std::time::Instant;

pub static PRECOMPUTATION_COMPLETE: OnceLock<()> = OnceLock::new();

pub fn precompute() {
    PRECOMPUTATION_COMPLETE.get_or_init(|| {
        let total_start = Instant::now();
        println!("Initializing precomputation...");

        let knight_start = Instant::now();
        let _knight_move_table =
            board::KNIGHT_MOVE_TABLE.get_or_init(board::precompute_knight_moves);
        println!(
            "Knight move table computed in {}.{:03}ms",
            knight_start.elapsed().as_millis(),
            knight_start.elapsed().as_micros() % 1000
        );

        let king_start = Instant::now();
        let _king_move_table = board::KING_MOVE_TABLE.get_or_init(board::precompute_king_moves);
        println!(
            "King move table computed in {}.{:03}ms",
            king_start.elapsed().as_millis(),
            king_start.elapsed().as_micros() % 1000
        );

        let pawn_start = Instant::now();
        let (black_pawn_table, white_pawn_table) = board::precompute_pawn_attacks();
        let _black_pawn_attack_table =
            board::BLACK_PAWN_ATTACK_TABLE.get_or_init(|| black_pawn_table);
        let _white_pawn_attack_table =
            board::WHITE_PAWN_ATTACK_TABLE.get_or_init(|| white_pawn_table);
        println!(
            "Pawn attack tables computed in {}.{:03}ms",
            pawn_start.elapsed().as_millis(),
            pawn_start.elapsed().as_micros() % 1000
        );

        let bishop_start = Instant::now();
        let _bishop_start_table =
            masks::BISHOP_BLOCKER_MASKS.get_or_init(masks::precompute_bishop_blocker_masks);
        println!(
            "Bishop blocker table computed in {}.{:03}ms",
            bishop_start.elapsed().as_millis(),
            bishop_start.elapsed().as_micros() % 1000
        );

        let rook_start = Instant::now();
        let _rook_start_table =
            masks::ROOK_BLOCKER_MASKS.get_or_init(masks::precompute_rook_blocker_masks);
        println!(
            "Rook blocker table computed in {}.{:03}ms",
            rook_start.elapsed().as_millis(),
            rook_start.elapsed().as_micros() % 1000
        );

        let total_time = total_start.elapsed();
        println!(
            "Total initialization time: {}.{:03}ms",
            total_time.as_millis(),
            total_time.as_micros() % 1000
        );
    });
}
