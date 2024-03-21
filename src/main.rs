mod game_logic;
use game_logic::*;

fn main() {
    let mut board : [char; 9] = ['0', '1', '2',
                                 '3', '4', '5',
                                 '6', '7', '8'];
    let mut turn = 0;
    dislay_board(&board);
    loop {
        println!("--------------------------------");
        if turn % 2 == 0 {
            // player 1 move
            println!("Player 1 move");
            board[get_input(&board)] = 'x';
        } else {
            // player 2 move
            println!("Player 2 move");
            board[get_input(&board)] = 'o';
        }
        println!("--------------------------------");
        dislay_board(&board);
        let winner: char = check_winner(&board);
        if winner == 't' {
            println!("Tie!");
            return;
        }
        if winner == 'x' || winner == 'o' {
            println!("{} wins!", winner);
            return;
        }
        turn += 1;
    }
}
