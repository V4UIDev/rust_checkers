use maplit::hashmap;

use checkers::*;
use crate::check_attacks;

pub fn print_board(wm_board: Board, bm_board: Board) {
    let letter_array: [&str; 8] = ["H", "G", "F", "E", "D", "C", "B", "A"];
    let mut letter_value = 0;

    let mut temp_board: u64 = 0;

    temp_board |= wm_board.man_value;
    temp_board |= bm_board.man_value;
    temp_board |= wm_board.king_value;
    temp_board |= bm_board.king_value;

    for rank in 0..8 {
            print!("{} ", letter_array[letter_value]);
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = 
                    if temp_board & wm_board.man_value & mask != 0 {
                         '*' 
                    } else if temp_board & bm_board.man_value & mask != 0 {
                         '#' 
                    } else if temp_board & wm_board.king_value & mask != 0 {
                         'X' 
                    } else if temp_board & bm_board.king_value & mask != 0 {
                         'Z' 
                    } else { ' ' };
                print!("{char} ");
            }
            letter_value += 1;
            println!("");
        }
        println!("------------------");
        println!("  1 2 3 4 5 6 7 8");
    }

pub fn flush_boardrep(mut game_settings: GameSettings, wm_board: Board, bm_board: Board) -> GameSettings {
    game_settings.board_representation = 0;
    game_settings.board_representation |= wm_board.man_value;
    game_settings.board_representation |= bm_board.man_value;
    game_settings.board_representation |= wm_board.king_value;
    game_settings.board_representation |= bm_board.king_value;

    return game_settings;
}

pub fn translate_move(player_move_str: String) -> u8 {
     let mut player_move_int: u8 = 0;
     let default_move: u8 = 0;
     let row_values: std::collections::HashMap<&str, u8> = hashmap!{
          "A" => 0,
          "B" => 8,
          "C" => 16,
          "D" => 24,
          "E" => 32,
          "F" => 40,
          "G" => 48,
          "H" => 56
      };
  

     let mut position = player_move_str[0..1].to_owned();
     if position.chars().any(|c| matches!(c, 'a'..='h')) == false {
         if position.chars().any(|c| matches!(c, 'A'..='H')) == false {
             println!("Invalid position! Reverting to default (A1)");
             return default_move;
         }
     } else if position.chars().any(|c| matches!(c, 'a'..='h')) == true {
          position = position.to_uppercase();
     }

     player_move_int += row_values.get(&position.as_str()).copied().unwrap_or(0);

     let position_number = player_move_str[1..2].to_owned();
     if position_number.chars().any(|c| matches!(c, '1'..='8')) == false {
         println!("Invalid position! Reverting to default (A1)");
         return default_move;
     } else {
          let position_number_int = position_number.parse::<u8>();
          match position_number_int {
              Ok(value) => {
                  player_move_int += value - 1;
              },
              Err(e) => {
                  eprintln!("Error parsing number: {}", e);
              }
     }

}
return player_move_int;
}

pub fn game_checks(mut game_settings: GameSettings, mut wm_board: Board, mut bm_board: Board) -> (GameSettings, Board, Board) {

    if wm_board.score > 11 {
        println!("White wins!");
        game_settings.game_end = true;
    } else if bm_board.score > 11 {
        println!("Black wins!");
        game_settings.game_end = true;
    }

    if game_settings.turn_number > 100 {
        println!("Draw due to max amount of turns!");
        game_settings.game_end = true;
    }

    (wm_board, bm_board) = check_king_transform(wm_board, bm_board);

    if game_settings.turn_number % 2 == 0 {
        game_settings = check_attacks(wm_board, game_settings.board_representation, game_settings);
    } else {
        game_settings = check_attacks(bm_board, game_settings.board_representation, game_settings);
    }

    return (game_settings, wm_board, bm_board);
}

pub fn check_king_transform(mut wm_board: Board, mut bm_board: Board) -> (Board, Board){
    for iteration in 56..63 {
        let mask = 1u64 << iteration;
        let negmask = !(1u64 << iteration);
        if mask & wm_board.man_value != 0 {
            wm_board.man_value &= negmask;
            wm_board.king_value |= mask;
        }
    }

    for iteration in 0..7 {
        let mask = 1u64 << iteration;
        let negmask = !(1u64 << iteration);
        if mask & bm_board.man_value != 0 {
            bm_board.man_value &= negmask;
            bm_board.king_value |= mask;
        }
    }

    return (wm_board, bm_board);
}
