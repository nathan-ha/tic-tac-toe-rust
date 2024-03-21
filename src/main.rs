mod game_logic;
use game_logic::*;

fn main() {
    let mut board : [char; 9] = ['0', '1', '2',
                                 '3', '4', '5',
                                 '6', '7', '8'];
    let mut turn = 0;
    loop {
        println!("--------------------------------");
        dislay_board(&board);
        if turn % 2 == 0 {
            // player 1 move
            println!("Player 1 move");
            board[get_input(&board)] = 'X';
        } else {
            // player 2 move
            println!("Player 2 move");
            board[get_input(&board)] = 'O';
        }
        println!("--------------------------------");
        turn += 1;
    }
}
