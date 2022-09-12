use crate::utils::print_board;


pub fn solve_full(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<Vec<i16>> {
    //Solves the board using the full algorithm

    let mut board = solve_deterministic(board, boardsize);

    //stores the board before using indeterministic solving
    let mut old_board = board.clone();

    //if the board is solved, return it
    if is_solved(board.clone()) {
        return board;
    }

    println!("Deterministic solving ended, beginning indeterministic solving:");
    print_board(&board);

    //loops until the board is solved
    loop {
        board = step_indeterministic(board, boardsize);

        //if the board is solved, return it
        if is_solved(board.clone()) {
            return board;
        }

        //if the board is not solved, but the board is the same as before, return the board
        if board == old_board {
            return board;
        }

        //if the board is not solved, and the board is not the same as before, continue
        old_board = board.clone();
    }

}

fn solve_deterministic(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<Vec<i16>> {
    //solves the board, as far as possible, deterministically.
    let mut board = board;
    let prev_board = board.clone();

    //Applies all functions to the board
    board = fill_full_rows_and_collumns(board, boardsize);
    board = no_three_in_a_row(board);

    //Checks if the board has changed
    if board == prev_board {
        board
    } else {
        solve_deterministic(board, boardsize)
    }

}

fn step_indeterministic(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<Vec<i16>> {
    // when all fast and deterministic ways to solve the board have been applied but the board is still not solved, the board is solved by trial and error

    let mut board = board;

    let mut test_spots = get_useful_spots(board.clone(), boardsize);
    test_spots.reverse(); //The spots are popped, so the last spot is the best spot to test


    loop {
        let mut test_board = board.clone();

        if test_spots.len() == 0 { //if there are no more spots to test, end the loop
            break;
        }

        let test_spot = test_spots.pop().unwrap();

        test_board[test_spot.0 as usize][test_spot.1 as usize] = 0; //sets the spot to 0

        let solved_test_board = solve_deterministic(test_board.clone(), boardsize); //solves the board

        if !board_is_valid(solved_test_board) {
            //if the board is not valid, the spot is set to 1
            board[test_spot.0 as usize][test_spot.1 as usize] = 1;
            board = solve_deterministic(board, boardsize); //the board is solved again, to propagate the changes
            // The list of spots to test is updated, if the board isn't solved yet
            if !is_solved(board.clone()) {
                test_spots = get_useful_spots(board.clone(), boardsize);
                test_spots.reverse();
            }

            // print_board(&board); //Debugging
            
            continue;
        }
        
        //However, if that was not the answer, the spot is set to 1 and the board is solved again
        
        test_board[test_spot.0 as usize][test_spot.1 as usize] = 1;
        
        let solved_test_board = solve_deterministic(test_board, boardsize);
        
        if !board_is_valid(solved_test_board) {
            //if the board is not valid, the spot is set to 0
            board[test_spot.0 as usize][test_spot.1 as usize] = 0;
            board = solve_deterministic(board, boardsize); //the board is solved again, to propagate the changes
            // The list of spots to test is updated, if the board isn't solved yet
            if !is_solved(board.clone()) {
                test_spots = get_useful_spots(board.clone(), boardsize);
                test_spots.reverse();
            }
            
            // print_board(&board); // Debugging
            
            
            continue;
        }
    }

    board //returns the board
}

fn get_useful_spots(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<(usize, usize, usize)> {
    // Returns a list with all open spots, ranked by usefulness
    // The current algorithms simply finds the spots that has the most neighbours that are solved (i.e. 0 or 1)
    // The number of filled in spots in its row and collumn is also taken into account
    // This is not the most efficient way to solve the board, but it is a good start

    let mut spot_filled_in_row = vec![0; boardsize as usize];
    let mut spot_filled_in_collumn = vec![0; boardsize as usize];

    //Finds the number of filled in spots in each row and collumn, to be used later

    for (i, row) in board.iter().enumerate() {
        for (j, spot) in row.iter().enumerate() {
            if *spot != -1 {
                spot_filled_in_row[i] += 1;
                spot_filled_in_collumn[j] += 1;
            }
        }
    }

    let mut score_list: Vec<(usize, usize, usize)> = Vec::new(); //stores the score of each spot, by x,y,score

    // The score of a spot is the number of neighbours that are solved * 100, plus the number of filled in spots in its row and collumn
    // This ensures that the spot with the most neighbours that are solved is chosen, and if there are multiple spots with the same number of neighbours that are solved, the spot with the most filled in spots in its row and collumn is chosen

    for (i, row) in board.iter().enumerate() {
        for (j, spot) in row.iter().enumerate() {
            if *spot == -1 {
                let mut score = 0usize;
                score += get_neighbours_solved(board.clone(), i, j) as usize* 100; //This is necessary because the default, i8, will overflow
                score += spot_filled_in_row[i];
                score += spot_filled_in_collumn[j];
                score_list.push((i, j, score));
            }
        }
    }

    if score_list.len() == 0 {
        println!("{:?}",board);
        panic!("No useful spot found: board is already solved, but not detected as such");
    }
    score_list.sort_by(|a, b| b.2.cmp(&a.2)); //sorts the list by score

    score_list

}

fn get_neighbours_solved(board: Vec<Vec<i16>>, row: usize, collumn: usize) -> i8 {
    // Returns how many of the four neighbours are solved (i.e. 0 or 1)

    let mut neighbours_solved = 0;

    //Checks the four neighbours
    if row != 0 {
        if board[row - 1][collumn] != -1 {
            neighbours_solved += 1;
        }
    }

    if row != board.len() - 1 {
        if board[row + 1][collumn] != -1 {
            neighbours_solved += 1;
        }
    }

    if collumn != 0 {
        if board[row][collumn - 1] != -1 {
            neighbours_solved += 1;
        }
    }

    if collumn != board.len() - 1 {
        if board[row][collumn + 1] != -1 {
            neighbours_solved += 1;
        }
    }

    neighbours_solved
}


fn fill_full_rows_and_collumns(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<Vec<i16>> {
    let mut board = board;
    board = fill_full_rows(board, boardsize);
    board = fill_full_collumns(board);
    board
}

fn fill_full_rows(board: Vec<Vec<i16>>, boardsize: i16) -> Vec<Vec<i16>> {
    // If a row already contains all its white or black circles, fill the rest with the opposite color
    let mut board = board;
    for row in 0..boardsize {
        let mut white = 0;
        let mut black = 0;
        for collumn in 0..boardsize {
            match board[row as usize][collumn as usize] {
                0 => white += 1,
                1 => black += 1,
                _ => (),
            }
        }
        if (white*2 == boardsize)&&(black*2 != boardsize) {
            for collumn in 0..boardsize {
                if board[row as usize][collumn as usize] == -1 {
                    board[row as usize][collumn as usize] = 1;
                }
            }
        } else if (black*2 == boardsize)&&(white*2 != boardsize) {
            for collumn in 0..boardsize {
                if board[row as usize][collumn as usize] == -1 {
                    board[row as usize][collumn as usize] = 0;
                }
            }
        }
    }

    board
}

fn fill_full_collumns(board: Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    // If a collumn already contains all its white or black circles, fill the rest with the opposite color
    let mut board = board;
    for collumn in 0..board.len() {
        let mut white = 0;
        let mut black = 0;
        for row in 0..board.len() {
            match board[row][collumn] {
                0 => white += 1,
                1 => black += 1,
                _ => (),
            }
        }
        if white*2 == board.len() {
            for row in 0..board.len() {
                if board[row][collumn] == -1 {
                    board[row][collumn] = 1;
                }
            }
        } else if black*2 == board.len() {
            for row in 0..board.len() {
                if board[row][collumn] == -1 {
                    board[row][collumn] = 0;
                }
            }
        }
    }

    board
}


fn no_three_in_a_row(board: Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    let mut board = board;
    board = two_in_rows(board);
    board = two_in_collumns(board);
    board
}

fn two_in_rows(board: Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    // Runs through all rows and checks if there are ever two circles of the same color back to back, with a free space at the end or beginning
    let mut board = board;
    for row in 0..board.len() {
        for collumn in 0..board.len()-2 {
            if (board[row][collumn] == board[row][collumn+1])&&(board[row][collumn+2] == -1)&&(board[row][collumn]!= -1) { // xx-
                board[row][collumn+2] = 1 - board[row][collumn];
            } else if (board[row][collumn] == board[row][collumn+2])&&(board[row][collumn+1] == -1)&&(board[row][collumn]!= -1) { // x-x
                board[row][collumn+1] = 1 - board[row][collumn];
            } else if (board[row][collumn+1] == board[row][collumn+2])&&(board[row][collumn] == -1)&&(board[row][collumn+1]!= -1) { // -xx
                board[row][collumn] = 1 - board[row][collumn+1];
            }
        }
    }
    board
}

fn two_in_collumns(board: Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    // Runs through all collumns and checks if there are ever two circles of the same color back to back, with a free space at the end or beginning
    let mut board = board;
    for collumn in 0..board.len() {
        for row in 0..board.len()-2 {
            if (board[row][collumn] == board[row+1][collumn])&&(board[row+2][collumn] == -1)&&(board[row][collumn]!= -1) { // xx-
                board[row+2][collumn] = 1 - board[row][collumn];
            } else if (board[row][collumn] == board[row+2][collumn])&&(board[row+1][collumn] == -1)&&(board[row][collumn]!= -1) { // x-x
                board[row+1][collumn] = 1 - board[row][collumn];
            } else if (board[row+1][collumn] == board[row+2][collumn])&&(board[row][collumn] == -1)&&(board[row+1][collumn]!= -1) { // -xx
                board[row][collumn] = 1 - board[row+1][collumn];
            }
        }
    }
    board
}


pub fn board_is_valid(board: Vec<Vec<i16>>) -> bool {
    // Checks if the board is valid
    
    // Checks if there are more than two circles of the same color in a row
    for row in 0..board.len() {
        for collumn in 0..board.len()-2 {
            if (board[row][collumn] == board[row][collumn+1])&&(board[row][collumn+1] == board[row][collumn+2])&&(board[row][collumn]!= -1) {
                return false;
            }
        }
    }

    // Checks if there are more than two circles of the same color in a collumn
    for collumn in 0..board.len() {
        for row in 0..board.len()-2 {
            if (board[row][collumn] == board[row+1][collumn])&&(board[row+1][collumn] == board[row+2][collumn])&&(board[row][collumn]!= -1) {
                return false;
            }
        }
    }

    // Sums the number of white and black circles in each row and collumn, if there are more than half of the boardsize of one color, the board is invalid
    for row in 0..board.len() {
        let mut white = 0;
        let mut black = 0;
        for collumn in 0..board.len() {
            match board[row][collumn] {
                0 => white += 1,
                1 => black += 1,
                _ => (),
            }
        }
        if (white > board.len()/2)||(black > board.len()/2) {
            return false;
        }
    }

    for collumn in 0..board.len() {
        let mut white = 0;
        let mut black = 0;
        for row in 0..board.len() {
            match board[row][collumn] {
                0 => white += 1,
                1 => black += 1,
                _ => (),
            }
        }
        if (white > board.len()/2)||(black > board.len()/2) {
            return false;
        }
    }

    board_has_no_duplicate_rows_or_collumns(board)
}

pub fn is_solved(board: Vec<Vec<i16>>) -> bool {
    // Checks if the board is solved
    for row in 0..board.len() {
        for collumn in 0..board.len() {
            if board[row][collumn] == -1 {
                return false;
            }
        }
    }

    board_is_valid(board) //Also checks if the board is valid
}

fn board_has_no_duplicate_rows_or_collumns(board: Vec<Vec<i16>>) -> bool {
    //The board is also invalid if there are duplicate rows or collumns
    let mut row_board = board.clone();

    // Sorts the rows first, they are much easier to compare
    row_board.sort();

    // Linear search for duplicate rows
    for row in 0..row_board.len()-1 {
        if row_board[row] == row_board[row+1] {
            return false;
        }
    }

    // Transposes the board, so that the collumns are now rows
    let mut collumn_board = vec![vec![0; board.len()]; board.len()];
    for row in 0..board.len() {
        for collumn in 0..board.len() {
            collumn_board[collumn][row] = board[row][collumn];
        }
    }

    // Sorts the collumns
    collumn_board.sort();

    // Linear search for duplicate collumns
    for collumn in 0..collumn_board.len()-1 {
        if collumn_board[collumn] == collumn_board[collumn+1] {
            return false;
        }
    }

    true


}