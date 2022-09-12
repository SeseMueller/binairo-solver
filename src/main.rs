use crate::solver::board_is_valid;

mod task;
mod utils;
mod solver;

fn main() {
    
    let mode = 11;

    if mode == 12 {
        panic!("Mode 12 is not supported!");
    }

    let boardsizes = [
        6, //6x6 easy
        6, //6x6 hard
        8, //8x8 easy
        8, //8x8 hard
        10, //10x10 easy
        10, //10x10 hard
        14, //14x14 easy
        14, //14x14 hard
        20, //20x20 easy
        20, //20x20 hard
        24, //24x24 special daily
        30 //30x30 special weeky
            // Theoretically 30x40 special monthly, but it's not supported
        ]; 

    let boardsize = boardsizes[mode as usize];

    let body = task::fetch_body(mode);
    assert!(body.is_ok());
    let body = body.unwrap();

    //Starts measuring the time, because the task is fetched
    let start = std::time::Instant::now();

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

    let end = std::time::Instant::now();
    println!("Time elapsed: {}ms", end.duration_since(start).as_millis());
}

