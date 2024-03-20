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
        let input: usize = input_str.trim().parse().expect("Invalid input");
        let out_of_range: bool = input > 8;
        // TODO: panics on letter inputs
        if out_of_range || !(board[input].is_digit(10)) {
            println!("Invalid Input, try again");
        } else {
            return input;
        }
    }
}