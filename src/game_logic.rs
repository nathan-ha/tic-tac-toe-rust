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

pub fn get_input(board : &[char]) -> usize {
    loop {
        let mut input_str = String::new();
        std::io::stdin().read_line(&mut input_str).expect("error: unable to read user input");
        match input_str.trim().parse::<usize>() {
            Ok(input) => {
                if input <= 8 && board[input].is_digit(10) {
                    return input;
                }
            }
            Err(_) => {},
        }
        println!("Invalid Input, try again")
    }
}
