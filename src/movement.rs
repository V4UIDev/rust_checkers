use checkers::*;
use maplit::hashmap;

fn move_piece(init_position: u8, desired_position: u8, mut piece_value: u64, mut board_rep: u64) -> (u64, u64) {
    let mask = 1u64 << desired_position;
    let negmask = !(1u64 << init_position);
    piece_value |= mask;
    piece_value &= negmask;
    board_rep |= mask;
    board_rep &= negmask;

    return (piece_value, board_rep)
}

pub fn move_man(init_position: u8, desired_position: u8, mut manboard: Board, mut board_rep: u64) -> (Board, u64) {
    let left_array: Vec<u8> = vec![0, 8, 16, 24, 32, 40, 48, 56];
    let right_array: Vec<u8> = vec![7, 15, 23, 31, 39, 47, 55, 63];
    let lh_marker: u8;
    let rh_marker: u8;

    if manboard.man_piece as u64 == 0 {
        rh_marker = init_position + 9;
        lh_marker = init_position + 7;
    } else {
        rh_marker = init_position - 7;
        lh_marker = init_position - 9;
    }

    let legal_moves = [rh_marker, lh_marker];


    if left_array.contains(&init_position) {
        if desired_position == rh_marker {
            let mask = 1u64 << rh_marker;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            }   else {
                (manboard.man_value, board_rep) = move_piece(init_position, desired_position, manboard.man_value, board_rep);
            }
        }   else {
            println!("Invalid move!");
        }
    } else if right_array.contains(&init_position) {
        if desired_position == lh_marker {
            let mask = 1u64 << lh_marker;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            }   else {
                (manboard.man_value, board_rep) = move_piece(init_position, desired_position, manboard.man_value, board_rep);
            }
        }   else {
            println!("Invalid move!");
        }
    } else {
        for player_move in legal_moves {
            if desired_position == player_move {

                let mask = 1u64 << player_move;

                if mask & manboard.man_value & board_rep != 0 {
                    println!("Can't move, piece in way!")
                } else {
                    (manboard.man_value, board_rep) = move_piece(init_position, desired_position, manboard.man_value, board_rep);
                }
        } 
    }
}
    // board_rep |= manboard.man_value; Do we need this?
    return (manboard, board_rep);
}

pub fn move_king(init_position: u8, desired_position: u8, mut kingboard: Board, mut board_rep: u64) -> (Board, u64) {
    let left_array: Vec<u8> = vec![8, 16, 24, 32, 40, 48];
    let right_array: Vec<u8> = vec![15, 23, 31, 39, 47, 55];
    let corner_array: Vec<u8> = vec![0, 7, 56, 63];
    let corner_association: std::collections::HashMap<&u8, u8> = hashmap!{
        &0 => 8,
        &7 => 14,
        &56 => 49,
        &63 => 54
    };
    let mut check_array: Vec<u8> = vec![];

    if corner_array.contains(&init_position) {
        if corner_association.get(&init_position).copied().unwrap_or(0) == desired_position {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
        } else {
            println!("Invalid move!")
        }
    } else if left_array.contains(&init_position) {
        check_array.push(init_position - 7);
        check_array.push(init_position + 9);

        if check_array.contains(&desired_position) {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
            println!("Invalid move!")
        }
    } else if right_array.contains(&init_position) {
        check_array.push(init_position + 7);
        check_array.push(init_position - 9);

        if check_array.contains(&desired_position) {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
        } else {
            println!("Invalid move!")
        }
    } else if init_position > 0 && init_position < 7 {
        check_array.push(init_position + 7);
        check_array.push(init_position + 9);

        if check_array.contains(&desired_position) {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
        } else {
            println!("Invalid move!")
        }
    } else if init_position > 56 && init_position < 63 {
        check_array.push(init_position - 7);
        check_array.push(init_position - 9);

        if check_array.contains(&desired_position) {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
        } else {
            println!("Invalid move!")
        }
    } else {
        check_array.push(init_position - 7);
        check_array.push(init_position - 9);
        check_array.push(init_position + 7);
        check_array.push(init_position + 9);

        if check_array.contains(&desired_position) {
            let mask = 1u64 << desired_position;
            if mask & board_rep != 0 {
                println!("Can't move, piece in way!");
            } else {
                (kingboard.king_value, board_rep) = move_piece(init_position, desired_position, kingboard.king_value, board_rep);
            }
        } else {
            println!("Invalid move!")
        }
    }

return (kingboard, board_rep)
}
