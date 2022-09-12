use std::{error::Error};


///Fetches the task from the website and scrapes the response for the task
pub fn fetch_task(body: &str) -> String {

    let task = scrape(&body);

    return task;
}

pub fn fetch_body(size: i16) -> Result<String, Box<dyn Error>> {
    let url = format!("https://www.puzzle-binairo.com/?size={}", size);
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response.escape_default().to_string())
}

fn scrape(body: &str) -> String {
    //Scrapes the body for the task
    //The task is in the body between "var task = \'" and "\';"

    // println!("Scraping body for task: {}", body);

    let start = body.find("var task = ").unwrap() + 13;

    let mut content = String::new();

    for c in body.chars().skip(start) { //Skip to the start of the task
        if c == '\\' { //Stop when the end of the task is reached
            break;
        }
        content.push(c);
    }

    content

}

pub fn get_puzzle_id(body: &str) -> String {
    //Gets the puzzle id from the body
    //The puzzle id is in the body between 'id=\"puzzleID\">' and '</span>'
    // println!("Scraping body for puzzle id: {}", body);

    let value = body.find("id=\\\"puzzleID\\\">");

    if value.is_none() {
        return String::from("Special puzzle");
    }

    let start = value.unwrap() + 16;

    let mut content = String::new();

    for c in body.chars().skip(start) { //Skip to the start of the puzzle id
        if c == '<' { //Stop when the end of the puzzle id is reached
            break;
        }
        content.push(c);
    }

    content
}

/// Given a Task in the form of a String, returns a 2d Vector of i8 representing the board
pub fn parse_task(task: String, size: i16) -> Vec<Vec<i16>> {
    
    //Creates a 1d Vector of i8 with the size of the board
    let mut board: Vec<i16> = vec![-1; (size*size) as usize];

    //The current position in the board, as in the index of the 1d Vector
    let mut pos: usize = 0;

    //Runs through the task String. 
    //Whenever there is a 1, it sets the current position in the board to 1
    //Whenever there is a 0, it sets the current position in the board to 0
    //When there is a letter, increase the index by the number of the letter

    for c in task.chars() {
        match c {
            '1' => {board[pos] = 1; pos +=1},
            '0' => {board[pos] = 0; pos +=1},
            'a' => pos += 1,
            'b' => pos += 2,
            'c' => pos += 3,
            'd' => pos += 4,
            'e' => pos += 5,
            'f' => pos += 6,
            'g' => pos += 7,
            'h' => pos += 8,
            'i' => pos += 9,
            'j' => pos += 10,
            'k' => pos += 11,
            'l' => pos += 12,
            'm' => pos += 13,
            'n' => pos += 14,
            'o' => pos += 15,
            'p' => pos += 16,
            'q' => pos += 17,
            'r' => pos += 18,
            's' => pos += 19,
            't' => pos += 20,
            'u' => pos += 21,
            'v' => pos += 22,
            'w' => pos += 23,
            'x' => pos += 24,
            'y' => pos += 25,
            'z' => pos += 26,
            _ => panic!("Invalid character in task {} : {}", task ,c),
        }
    }

    //Converts the 1d Vector to a 2d Vector
    let mut board_2d: Vec<Vec<i16>> = vec![vec![-2; size as usize]; size as usize];
    for i in 0..size {
        for j in 0..size {
            board_2d[i as usize][j as usize] = board[(i*size + j) as usize];
        }
    }

    return board_2d;
}
