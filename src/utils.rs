

pub fn print_board(board: &Vec<Vec<i16>>) {
    println!();
    for row in board {
        for char in row {
            match char {
                0 => print!("◼"),
                1 => print!("◻"),
                -1 => print!("-"),
                _ => panic!("Invalid character in board"),
            }
        }
        println!("");
    }
}