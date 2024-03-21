// displays the board in terminal
pub fn dislay_board(board: &[char]) {
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

// gets user input for the position on the board, and validates input
// returns the input
pub fn get_input(board: &[char]) -> usize {
    loop {
        let mut input_str = String::new();
        std::io::stdin()
            .read_line(&mut input_str)
            .expect("error: unable to read user input");
        match input_str.trim().parse::<usize>() {
            Ok(input) => {
                if input <= 8 && board[input].is_digit(10) {
                    return input;
                }
            }
            Err(_) => {}
        }
        println!("Invalid Input, try again")
    }
}

// returns 'x' or 'o' for the respective winner. 't' for tie.
pub fn check_winner(board: &[char]) -> char {
    // 8 checks
    if board[0] == board[1] && board[1] == board[2] {
        return board[0];
    }
    if board[3] == board[4] && board[4] == board[5] {
        return board[3];
    }
    if board[6] == board[7] && board[7] == board[8] {
        return board[6];
    }
    if board[0] == board[3] && board[3] == board[6] {
        return board[0];
    }
    if board[1] == board[4] && board[4] == board[7] {
        return board[1];
    }
    if board[2] == board[5] && board[5] == board[8] {
        return board[2];
    }
    if board[0] == board[4] && board[4] == board[8] {
        return board[0];
    }
    if board[2] == board[4] && board[4] == board[6] {
        return board[2];
    }
    // check for tie
    for i in 0..9 {
        if board[i].is_digit(10) {
            return 'n'; // if there is a digit left, then there is no tie
        }
    }
    return 't';
}
