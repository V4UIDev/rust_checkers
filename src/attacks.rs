use checkers::*;
use maplit::hashmap;

use crate::flush_boardrep;

fn check_hop(marker: u64, board_rep: u64, position_value: u64, mut game_settings: GameSettings) -> GameSettings {

    if marker > position_value {
        let difference = position_value.abs_diff(marker);
        if marker + difference < 64 {
            let mask = 1u64 << (marker + difference);
            if board_rep & mask == 0 {
                game_settings.hop_needed = true;
                game_settings.update_hop_values(position_value, marker + difference, marker);
            }
        }
    } else if marker < position_value {
        let difference = position_value.abs_diff(marker);
        if let Some(result) = marker.checked_sub(difference) {
            if result > 0 {
                let mask = 1u64 << (marker - difference);
                if board_rep & mask == 0 {
                    game_settings.hop_needed = true;
                    game_settings.update_hop_values(position_value, marker - difference, marker);
                }
            }
        }
    }

    return game_settings;
}

fn check_attack_man(position_value: u64, board: Board, board_rep: u64, lh_marker: u64, rh_marker: u64, mut game_settings: GameSettings) -> GameSettings {

    let left_array: Vec<u64> = vec![0, 8, 16, 24, 32, 40, 48];
    let right_array: Vec<u64> = vec![7, 15, 23, 31, 39, 47, 55];

    if lh_marker != 256 && rh_marker != 256 {
        let mask = 1u64 << lh_marker;
        if mask & board.man_value == 0 && mask & board.king_value == 0 && board_rep & mask != 0 && !left_array.contains(&lh_marker) {
            game_settings = check_hop(lh_marker, board_rep, position_value, game_settings);
        }
        let mask = 1u64 << rh_marker;
        if mask & board.man_value == 0 && mask & board.king_value == 0 && board_rep & mask != 0 && !right_array.contains(&rh_marker) {
            game_settings = check_hop(rh_marker, board_rep, position_value, game_settings);
        }
    } else if lh_marker != 256 {
        let mask = 1u64 << lh_marker;
        if mask & board.man_value == 0 && mask & board.king_value == 0 && board_rep & mask != 0 && !left_array.contains(&lh_marker) {
            game_settings = check_hop(lh_marker, board_rep, position_value, game_settings);
        }
    } else if rh_marker != 256 {
        let mask = 1u64 << rh_marker;
        if mask & board.man_value == 0 && mask & board.king_value == 0 && board_rep & mask != 0 && !right_array.contains(&rh_marker) {
            game_settings = check_hop(rh_marker, board_rep, position_value, game_settings);
        }
    }
    return game_settings;
}

fn check_attack_king(position_value: u64, board: Board, board_rep: u64, marker_array: &Vec<u64>, mut game_settings: GameSettings) -> GameSettings {
    let side_array: Vec<u64> = vec![0, 8, 16, 24, 32, 40, 48, 7, 15, 23, 31, 39, 47, 55];

    for marker in marker_array{
        let mask = 1u64 << marker;
        if mask & board.man_value == 0 && mask & board.king_value == 0 && board_rep & mask != 0 && !side_array.contains(&marker){
            game_settings = check_hop(*marker, board_rep, position_value, game_settings);
        }
    }

    return game_settings;
}

fn attack_iterator_man(board: Board, board_rep: u64, mut game_settings: GameSettings) -> GameSettings {

    let left_array: Vec<u64> = vec![0, 8, 16, 24, 32, 40, 48, 56];
    let right_array: Vec<u64> = vec![7, 15, 23, 31, 39, 47, 55, 63];

    for iteration in 0..63 {
        let mask = 1u64 << iteration;
        if mask & board.man_value != 0 {
            let rh_marker: u64;
            let lh_marker: u64;

            if board.man_piece as u64 == 0 && iteration < 56 {
                rh_marker = iteration + 9;
                lh_marker = iteration + 7;
            } else if iteration > 9 {
                rh_marker = iteration - 7;
                lh_marker = iteration - 9;
            } else {
                continue;
            }

            if left_array.contains(&iteration){
                game_settings = check_attack_man(iteration, board, board_rep, 256, rh_marker, game_settings);
            } else if right_array.contains(&iteration){
                game_settings = check_attack_man(iteration, board, board_rep, lh_marker, 256, game_settings);
            } else {
                game_settings = check_attack_man(iteration, board, board_rep, lh_marker, rh_marker, game_settings);
            }
        }
    }

    return game_settings;
}

fn attack_iterator_king(board: Board, board_rep: u64, mut game_settings: GameSettings) -> GameSettings {

    let left_array: Vec<u64> = vec![8, 16, 24, 32, 40, 48];
    let right_array: Vec<u64> = vec![15, 23, 31, 39, 47, 55];
    let corner_array: Vec<u64> = vec![0, 7, 56, 63];
    let corner_association: std::collections::HashMap<&u64, u64> = hashmap!{
        &0 => 8,
        &7 => 14,
        &56 => 49,
        &63 => 54
    };

    for iteration in 0..63 {
        let mut marker_array: Vec<u64> = vec![];
        let mask = 1u64 << iteration;
        if mask & board.king_value != 0 {
            if left_array.contains(&iteration) {
                marker_array.push(iteration + 9);
                marker_array.push(iteration - 7);
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            } else if right_array.contains(&iteration) {
                marker_array.push(iteration - 9);
                marker_array.push(iteration + 7);
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            } else if corner_array.contains(&iteration) {
                marker_array.push(corner_association.get(&iteration).copied().unwrap_or(0));
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            } else if iteration > 0 && iteration < 8 {  
                marker_array.push(iteration + 7);
                marker_array.push(iteration + 9);
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            } else if iteration > 56 && iteration < 63 {
                marker_array.push(iteration - 9);
                marker_array.push(iteration - 7);
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            } else {
                marker_array.push(iteration - 9);
                marker_array.push(iteration + 7);
                marker_array.push(iteration + 9);
                marker_array.push(iteration - 7);
                game_settings = check_attack_king(iteration, board, board_rep, &marker_array, game_settings);
            }
        }
    }

    return game_settings;
}


pub fn check_attacks(board: Board, board_rep: u64, mut game_settings: GameSettings) -> GameSettings {

    game_settings = attack_iterator_man(board, board_rep, game_settings);
    game_settings = attack_iterator_king(board, board_rep, game_settings);
    return game_settings;

}

pub fn perform_hop(mut wm_board: Board, mut bm_board: Board, mut game_settings: GameSettings) -> (Board, Board, GameSettings){

    if let Some((init_pos, final_pos, death_piece)) = game_settings.get_hop_values() {
        let init_mask = !(1u64 << init_pos);
        let init_check = 1u64 << init_pos;
        let death_check = 1u64 << death_piece;
        let death_mask = !(1u64 << death_piece);
        let final_mask = 1u64 << final_pos;
        if game_settings.turn_number % 2 == 0 {
            if wm_board.man_value & init_check != 0 {
                wm_board.man_value &= init_mask;
                wm_board.man_value |= final_mask;
            } else {
                wm_board.king_value &= init_mask;
                wm_board.king_value |= final_mask;
            }
            if bm_board.man_value & death_check != 0 {
                bm_board.man_value &= death_mask;
            } else {
                bm_board.king_value &= death_mask;
            }
            wm_board.score += 1;
        } else {
            if bm_board.man_value & init_check != 0 {
                bm_board.man_value &= init_mask;
                bm_board.man_value |= final_mask;
            } else {
                bm_board.king_value &= init_mask;
                bm_board.king_value |= final_mask;
            }
            if wm_board.man_value & death_check != 0 {
                wm_board.man_value &= death_mask;
            } else {
                wm_board.king_value &= death_mask;
            }
            bm_board.score += 1;
        }
    }
    game_settings.hop_needed = false;
    game_settings = flush_boardrep(game_settings, wm_board, bm_board);
    return (wm_board, bm_board, game_settings);

    }