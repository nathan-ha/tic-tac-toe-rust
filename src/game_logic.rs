pub fn dislay_board(board : &[char]) {
    for i in 0..3 {
        print!(" {} ", board[i]);
    }
    println!();
    for i in 3..6 {
        print!(" {} ", board[i]);
    }
    println!();
    for i in 6..9 {
        print!(" {} ", board[i]);
    }
    println!();
}