mod game_logic;
use game_logic::*;

fn main() {
    let mut board : [char; 9] = ['0', '1', '2',
                                 '3', '4', '5',
                                 '6', '7', '8'];
    dislay_board(&board);
}
