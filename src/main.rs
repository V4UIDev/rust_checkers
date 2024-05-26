use checkers::*;
pub mod util;
pub mod movement;
pub mod attacks;
pub mod input;

use crate::util::*;
use crate::movement::*;
use crate::attacks::*;
use crate::input::*;

fn main() {

    let mut game_settings = GameSettings {
        turn_number: 0,
        hop_needed: false,
        hop_values: HopValues::Values{init_pos: 0, final_pos: 0, death_piece: 0},
        board_representation: 0,
        game_end: false
    };

    let mut whiteman_board = Board {
        man_piece: Pieces::WhiteMan,
        king_piece: Pieces::WhiteKing,
        man_value: 5614165,
        king_value: 0,
        score: 0
    };

    let mut blackman_board = Board {
        man_piece: Pieces::BlackMan,
        king_piece: Pieces::BlackKing,
        man_value: 12273903276444876800,
        king_value: 0,
        score: 0
    };

    game_settings.board_representation |= whiteman_board.man_value;
    game_settings.board_representation |= blackman_board.man_value;

    loop {
        print!("{}[2J", 27 as char);
        game_settings = flush_boardrep(game_settings, whiteman_board, blackman_board);
        (game_settings, whiteman_board, blackman_board) = game_checks(game_settings, whiteman_board, blackman_board);

        if game_settings.game_end == true {
            break;
        }

        if game_settings.hop_needed == true {
            (whiteman_board, blackman_board, game_settings) = perform_hop(whiteman_board, blackman_board, game_settings);
            continue;
        }

        (game_settings, whiteman_board, blackman_board) = player_make_move(game_settings, whiteman_board, blackman_board);
        print_board(whiteman_board, blackman_board);
    }
}
