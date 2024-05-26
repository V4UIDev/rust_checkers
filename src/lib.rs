#[derive(Copy, Clone)]
pub enum Pieces {
    WhiteMan,
    WhiteKing,
    BlackMan,
    BlackKing
}

#[derive(Copy, Clone, Debug)]
pub enum HopValues {
    Values {
        init_pos: u64,
        final_pos: u64,
        death_piece: u64

    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub man_piece: Pieces,
    pub king_piece: Pieces,
    pub man_value: u64,
    pub king_value: u64,
    pub score: u64
}

#[derive(Copy, Clone)]
pub struct GameSettings {
    pub turn_number: u8,
    pub hop_needed: bool,
    pub hop_values: HopValues,
    pub board_representation: u64,
    pub game_end: bool
}

impl GameSettings {
    pub fn update_hop_values(&mut self, init_pos: u64, final_pos: u64, death_piece: u64) {
        self.hop_values = HopValues::Values {
            init_pos,
            final_pos,
            death_piece
        };
    }

    pub fn get_hop_values(&mut self) -> Option<(u64, u64, u64)> {
        match self.hop_values {
            HopValues::Values {
                init_pos,
                final_pos,
                death_piece
            } => Some((init_pos, final_pos, death_piece)),
        }
    }

}

pub const LAST_BIT: u64 = 63;