use std::io;
// use checkers::*;
use crate::*;

pub fn player_make_move(mut game_settings: GameSettings, mut wm_board: Board, mut bm_board: Board) -> (GameSettings, Board, Board) {
    print_board(wm_board, bm_board);
    let mut player_init_pos = String::new();
    let mut player_final_pos = String::new();
    
    if game_settings.turn_number % 2 == 0 {
        println!("White's turn!");
        println!("Please enter init pos: ");

        io::stdin()
        .read_line(&mut player_init_pos)
        .expect("Failed to read line");

        println!("Please enter final pos: ");

        io::stdin()
        .read_line(&mut player_final_pos)
        .expect("Failed to read line");
    } else if game_settings.turn_number % 2 != 0 {
        println!("Black's turn!");
        println!("Please enter init pos: ");

        io::stdin()
        .read_line(&mut player_init_pos)
        .expect("Failed to read line");

        println!("Please enter final pos: ");

        io::stdin()
        .read_line(&mut player_final_pos)
        .expect("Failed to read line");
    }

    let player_init_pos_int = translate_move(player_init_pos);
    let player_final_pos_int = translate_move(player_final_pos);
    let mask = 1u64 << player_init_pos_int;

    if game_settings.turn_number % 2 == 0 {
        if mask & wm_board.man_value != 0 {
            (wm_board, game_settings.board_representation) = move_man(player_init_pos_int, player_final_pos_int, wm_board, game_settings.board_representation);
            game_settings.turn_number += 1;
        } else if mask & wm_board.king_value != 0 {
            (wm_board, game_settings.board_representation) = move_king(player_init_pos_int, player_final_pos_int, wm_board, game_settings.board_representation);
            game_settings.turn_number += 1;
        } else {
            println!("Empty in init pos!");
        }
    } else if game_settings.turn_number % 2 != 0 {
        if mask & bm_board.man_value != 0 {
            (bm_board, game_settings.board_representation) = move_man(player_init_pos_int, player_final_pos_int, bm_board, game_settings.board_representation);
            game_settings.turn_number += 1;
        } else if mask & bm_board.king_value != 0 {
            (bm_board, game_settings.board_representation) = move_king(player_init_pos_int, player_final_pos_int, bm_board, game_settings.board_representation);
            game_settings.turn_number += 1;
        } else {
            println!("Empty in init pos!");
        }
    }

    return (game_settings, wm_board, bm_board);

}