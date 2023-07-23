// move with http://0.0.0.0:8000/game/Pc2c4

/*
Rust-server chess board-game,
no 'game logic' only players moving pieces: a board

move with:
    http://0.0.0.0:8000/game/Pc2c4
    
for back trace run with:
    RUST_BACKTRACE=full cargo run

Goals & Rules:
- no unwrap
- stay slim, stay minimal, stay vanilla, small attack surface
- fewest libraries imported as possible
- fewest features possible
- speed is NOT important
- resource efficiency and robustness are important
game-state exists only in files in gamename directory:
- log of past moves
- array that stores current pieces on board
- svg image of board's last move


TODO:
- 
- move all operations in main route to separate functions
-- 
-- 
- error-page for invalid get requests
- get input start/gamename to set up game folder
- request queue system using directory of queue files
(ideally starting in RAM until threshold)
- game setup get
- get report (log) get
- remove games if inactive for a week
- game secure "login" via gamephrase and lossy-hashed user IP
(redirect to game-phase page if IP hash not recognized)
- load current game array vs. repeat past moves
(same game_state as board_game_state.txt) to load array?) 
- maybe add ascii-ramen board
- add svg board: letter number border, redish black, 
minimal pieces


in order to set up the board from a saved state,
that saved state needs to exist.


*/

// Import Packages / Dependencies
extern crate tiny_http;
extern crate csv;

use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use tiny_http::{Server, Response, Method};


// Variables
type Board = [[char; 8]; 8];

// Before Main...
fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    
    // Set up board
    let board: Arc<Mutex<Board>> = Arc::new(Mutex::new([
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
    ]));

    println!("Server >*< trench runnnnnnning at http://0.0.0.0:8000 |o| |o| " );

    for request in server.incoming_requests() {
        if request.method() == &Method::Get {
            let url_parts: Vec<&str> = request.url().split('/').collect();
            if url_parts.len() >= 3 {
                let game_name = url_parts[1].to_string();
                let move_data = url_parts[2].to_string();
                let mut response_string = String::new();  

                match validate_input(&move_data) {
                    Err(err_msg) => {
                        let response = Response::from_string(err_msg).with_status_code(400);
                        if let Err(e) = request.respond(response) {
                            eprintln!("Failed to respond to request: {}", e);
                        }
                        continue;
                    },
                    Ok(()) => {},
                }
                
                // Open CSV file for game in append mode, creating it if it doesn't exist

                let dir_path = format!("./games/{}", game_name);
                std::fs::create_dir_all(&dir_path).expect("Failed to create directory");
                
                let file_path = format!("{}/moves.csv", dir_path);


                let file = match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Failed to open file: {}", e);
                        continue;
                    }
                };
            
                let mut wtr = csv::Writer::from_writer(file);

                // Write new move to CSV file
                if let Err(e) = wtr.write_record(&[move_data]) {
                    eprintln!("Failed to write to file: {}", e);
                    continue;
                }
                
                if let Err(e) = wtr.flush() {
                    eprintln!("Failed to flush writer: {}", e);
                    continue;
                }
                
                                
                // Read all moves from CSV file and update the board accordingly
                let mut rdr = match csv::Reader::from_path(&file_path) {
                    Ok(reader) => reader,
                    Err(e) => {
                        eprintln!("Failed to create reader: {}", e);
                        continue;
                    }
                };

                let mut board = match board.lock() {
                    Ok(board) => board,
                    Err(e) => {
                        eprintln!("Failed to lock board: {}", e);
                        continue;
                    }
                };

                for result in rdr.records() {
                    let record = result.unwrap();
                    let move_data = &record[0];

                    let (piece, from, to);

                    match parse_move(&move_data) {
                        Ok(parsed) => {
                            piece = parsed.0;
                            from = parsed.1;
                            to = parsed.2;
                        }
                        Err(e) => {
                            eprintln!("Invalid move format: {}", e);
                            continue; // Skip this iteration and go to next loop iteration
                        }
                    }

                    match to_coords(&format!("{}{}", from.0, from.1)) {
                        Ok(coords) => {
                            let (x, y) = coords;
                            board[x][y] = ' ';
                        },
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            continue;
                        },
                    };

                    match to_coords(&format!("{}{}", to.0, to.1)) {
                        Ok(coords) => {
                            let (x, y) = coords;
                            board[x][y] = piece;
                        },
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            continue;
                        },
                    };



                    // Terminal-print the updated board
                    print_board(&board);

                    let board_string = board_to_string(&board);

       
                    response_string.push_str(&format!(
                        "Game: {}\nPiece: {}\nMove to: ({}, {})\n\n{}",
                        game_name,
                        piece,
                        to.1,
                        to.0,
                        board_string
                    ));
                }
                let response = Response::from_string(response_string);
                request.respond(response).unwrap();
            } else {
                // ... Invalid request format
            }
        }
    }
}


/////////////////////
// Helper Functions
/////////////////////

fn parse_move(move_data: &str) -> Result<(char, (char, u8), (char, u8)), String> {
    if move_data.len() != 5 {
        return Err(format!("Invalid input length. Input should be 5 characters. e.g. Pc2c4 or pc7c6 "));
    }

    let chars: Vec<char> = move_data.chars().collect();

    let piece = chars.get(0).ok_or("Failed to get piece")?;
    let from_col = chars.get(1).ok_or("Failed to get from_col")?;
    let from_row_digit = chars.get(2)
        .ok_or("Failed to get from_row_digit")?
        .to_digit(10)
        .ok_or("Failed to parse from_row_digit to number")?;
    let to_col = chars.get(3).ok_or("Failed to get to_col")?;
    let to_row_digit = chars.get(4)
        .ok_or("Failed to get to_row_digit")?
        .to_digit(10)
        .ok_or("Failed to parse to_row_digit to number")?;

    let from = (*from_col, from_row_digit as u8);
    let to = (*to_col, to_row_digit as u8);

    Ok((*piece, from, to))
}


fn to_coords(chess_notation: &str) -> Result<(usize, usize), String> {
    if chess_notation.len() != 2 {
        return Err(format!("Invalid chess notation: '{}'. It should be two characters long.", chess_notation));
    }
    let col = chess_notation.chars().nth(0).unwrap();
    let row = chess_notation.chars().nth(1).unwrap();

    if !('a'..='h').contains(&col) || !('1'..='8').contains(&row) {
        return Err(format!("Invalid chess notation: '{}'. It should be in the form 'e4'.", chess_notation));
    }

    let col = col as usize - 'a' as usize;
    let row = 8 - row.to_digit(10).unwrap() as usize;

    Ok((row, col))
}


fn print_board(board: &[[char; 8]; 8]) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}


fn piece_to_unicode(piece: char) -> &'static str {
    match piece {
        'r' => "♜",
        'n' => "♞",
        'b' => "♝",
        'q' => "♛",
        'k' => "♚",
        'p' => "♟",
        'R' => "♖",
        'N' => "♘",
        'B' => "♗",
        'Q' => "♕",
        'K' => "♔",
        'P' => "♙",
        _ => " ",
    }
}


fn board_to_string(board: &[[char; 8]; 8]) -> String {
    let mut board_string = String::new();
    board_string.push_str("  a b c d e f g h\n");
    for (i, row) in board.iter().enumerate() {
        board_string.push_str(&(8-i).to_string());
        board_string.push(' ');
        for &cell in row.iter() {
            let piece = piece_to_unicode(cell);
            board_string.push_str(piece);
            board_string.push(' ');
        }
        board_string.push_str(&(8-i).to_string());
        board_string.push('\n');
    }
    board_string.push_str("  a b c d e f g h\n");
    board_string
}



// Return Result with appropriate error messages instead of bool
fn validate_input(input: &str) -> Result<(), String> {
    if input.len() != 5 {
        return Err(format!("Invalid input length. Input should be 5 characters. e.g. Pc2c4 or pc7c6 "));
    }

    let chars: Vec<char> = input.chars().collect();

    let valid_pieces = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];
    let valid_cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let valid_rows = ['1', '2', '3', '4', '5', '6', '7', '8'];

    if !valid_pieces.contains(&chars[0]) {
        return Err(format!("Invalid piece identifier. The first character should be one of 'prnbqkPRNBQK'. e.g. Pc2c4 or pc7c6 "));
    }
    if !valid_cols.contains(&chars[1]) || !valid_cols.contains(&chars[3]) {
        return Err(format!("Invalid column identifier. The 2nd and 4th characters should be one of 'abcdefgh'. e.g. Pc2c4 or pc7c6 "));
    }
    if !valid_rows.contains(&chars[2]) || !valid_rows.contains(&chars[4]) {
        return Err(format!("Invalid row identifier. The 3rd and 5th characters should be one of '12345678'.e.g. Pc2c4 or pc7c6  "));
    }
    
    Ok(())
}
