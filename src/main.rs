use crate::solver::board_is_valid;

mod task;
mod utils;
mod solver;

fn main() {
    
    let boardsize = 30;

    let body = task::fetch_body();
    assert!(body.is_ok());
    let body = body.unwrap();

    let mytask = task::fetch_task(&body);
    let puzzle_id = task::get_puzzle_id(&body);

    println!("Running with task: {}", mytask);
    println!("Puzzle ID: {}", puzzle_id);

    let board = task::parse_task(mytask, boardsize);

    assert_eq!(board.len(), boardsize as usize);
    assert_eq!(board[0].len(), boardsize as usize);
    assert!(board[(boardsize-1) as usize][(boardsize-1) as usize] != -2); //asserts that the last element was written to, meaning the task was most likely parsed correctly

    //prints the board as 2d Vector
    utils::print_board(&board);

    println!("Solving...\n\n");

    let solution = solver::solve_full(board, boardsize);

    println!("\nThe solution was calculated:");

    if !board_is_valid(solution.clone()){
        println!("The solution is invalid!");
    }

    utils::print_board(&solution);
}

