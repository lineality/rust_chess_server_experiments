/*
http://0.0.0.0:8000/game/Pc2c4
RUST_BACKTRACE=full cargo run
*/


/* 
TODO:
- game setup:
    how to do as page or get request?
    what to do if files no found...redirect to setup-page...or instructions raw text/hmtl?


    setup/type_type/game_name/game_phrase

if error
return text (not html) "that option is now available or allowed, try:
    setup/type_type/game_name/game_phrase


    if move len 7 & == "restart"
        reset board

- load game_board_state (instead of replaying all past moves)
- new check and new system to start game:
    start/gamename get request
- remove all uses of "unwrap"
- error-page for invalid get requests
- get input start/gamename to set up game folder
- 'request queue' system, using directory of queue files
(ideally starting in RAM until threshold)

- remove games if inactive for a week
- game secure "login" via gamephrase and lossy-hashed user IP
(redirect to game-phase page if IP hash not recognized)
- load current game array vs. repeat past moves
(same game_board_state.txt) to load array?) 
- maybe add ascii-ramen board
- add svg board: letter number border, redish black, 
minimal pieces
- move all operations in main route to separate functions
- get report (log) get
-- 
-- 
*/



/*
Rust-server chess board-game:

Minimal
Amnesiac
Stateless
Turn-based

move with:
    http://0.0.0.0:8000/game/Pc2c4
    
for back trace run with:
    RUST_BACKTRACE=full cargo run

Goals & Rules:
- make only one changea at a time
- no unwrap
- speed is NOT important
- resource efficiency and robustness are important

Process outline: (draft):
- get game-state from saved file
- load game state
- get move data in a get-request
- add new move to log
- make a new board array based on old array + new move data
- make svg image of board's last move
- show svg to user
**
- stay slim, stay minimal, stay vanilla, small attack surface
- fewest libraries imported as possible
- fewest features possible
-- game-state exists only in files in gamename directory
- no 'game logic,' only players moving pieces: a board

    // // Set up board
    // let board: Arc<Mutex<Board>> = Arc::new(Mutex::new([
    //     ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    //     ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    //     ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    //     ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
    // ]));

Note:

This code works. The next task is to keep improving 
how the game-board is handled. 
use (or alter) the load_game_board_state() function: 
with two immutable arrays, 
one for the original loaded state, and one for the 
state that is altered by the new move.

write a new function: board_state_after_move, which creates
(or passes data to) a new immuntable board array

the datatypes are as here:
```
type Board = [[char; 8]; 8];
fn parse_move(move_data: &str) -> Result<(char, (char, u8), (char, u8)), String> {
```
does this need to changed throughout the code?
*/



// Import Packages / Dependencies
extern crate tiny_http;
extern crate csv;

// use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use tiny_http::{Server, Response, Method};
use std::io::prelude::*;


// Variables
type Board = [[char; 8]; 8];


fn main() {
    let server = match Server::http("0.0.0.0:8000") {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            return;
        }
    };
    
    println!("Server >*< trench runnnnnnning at http://0.0.0.0:8000 |o| |o| " );

    for request in server.incoming_requests() {

        // get request containing game and move
        if request.method() == &Method::Get {
            let url_parts: Vec<&str> = request.url().split('/').collect();


            // if chess game maove
            if url_parts.len() == 3 {

                let game_name = url_parts[1].to_string();
                let move_data = url_parts[2].to_string();  

                // // // declare response outside match blocks so we can assign it in each match block
                // let response = Response::from_string(response_string);

                // sanitize and validate inputs from get request
                match validate_input(&move_data) {
                    Err(err_msg) => {
                        let response = Response::from_string(err_msg).with_status_code(400);
                        if let Err(e) = request.respond(response) {
                            eprintln!("Failed to respond to request: {}", e);
                        }
                        continue; // No need to run the rest of the loop for invalid inputs
                    },
                    Ok(()) => {},
                }

                // call game move function
                // call game move function
                let response = match handle_chess_move(game_name, move_data) {
                    Ok(response_string) => {
                        Response::from_string(response_string).with_status_code(200)
                    },
                    Err(e) => {
                        eprintln!("Failed to handle move: {}", e);
                        Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                    }
                };

                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
            } else {
                // ... Invalid request format
            }
        }
    }
}


/////////////////////
// Helper Functions
/////////////////////


fn handle_chess_move(game_name: String, move_data: String) -> Result<String, Box<dyn std::error::Error>> {

    let mut response_string = String::new();


    // File Setup
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
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Failed to open file: {}", e))));
    }};

    let mut wtr = csv::Writer::from_writer(file);


    // Write new move to CSV file
    if let Err(e) = wtr.write_record(&[move_data.clone()]) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other, format!("Failed to write to file: {}", e))));
    }

    if let Err(e) = wtr.flush() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other, format!("Failed to flush writer: {}", e))));
    }


    // // Load the game board state
    // let mut board = match load_game_board_state(&game_name) {
    //     Ok(board) => board,
    //     Err(e) => {
    //         return Err(Box::new(std::io::Error::new(
    //             std::io::ErrorKind::Other, format!("Failed to load game board state: {}", e))));
    //     }
    // };

    // let (piece, from, to);

    // match parse_move(&move_data) {
    //     Ok(parsed) => {
    //         piece = parsed.0;
    //         from = parsed.1;
    //         to = parsed.2;
    //     }
    //     Err(e) => {
    //         eprintln!("Invalid move format: {}", e);
    //     }
    // }

    // match to_coords(&format!("{}{}", from.0, from.1)) {
    //     Ok(coords) => {
    //         let (x, y) = coords;
    //         board[x][y] = ' ';
    //     },
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     },
    // };

    // match to_coords(&format!("{}{}", to.0, to.1)) {
    //     Ok(coords) => {
    //         let (x, y) = coords;
    //         board[x][y] = piece;
    //     },
    //     Err(err) => {
    //         eprintln!("Error: {}", err);
    //     },
    // };

    // Load the game board state
    let mut board = match load_game_board_state(&game_name) {
        Ok(board) => board,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other, format!("Failed to load game board state: {}", e))));
        }
    };

    match parse_move(&move_data) {
        Ok((piece, from, to)) => {
            // This is the successful case. `piece`, `from`, and `to` are guaranteed to be initialized.
            match to_coords(&format!("{}{}", from.0, from.1)) {
                Ok(coords) => {
                    let (x, y) = coords;
                    board[x][y] = ' ';
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                },
            };

            match to_coords(&format!("{}{}", to.0, to.1)) {
                Ok(coords) => {
                    let (x, y) = coords;
                    board[x][y] = piece;
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                },
            };

            // Save game (save game_board_state to .txt file)
            if let Err(e) = save_game_board_state(&game_name, board) {
                eprintln!("Failed to save game state: {}", e);
            }

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
        Err(e) => {
            // This is the error case. Return or handle the error in some way here.
            eprintln!("Invalid move format: {}", e);
            // If you want to return error to the caller, you could do so here:
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Invalid move format: {}", e))));
        }
    }


    // // Save game (save game_board_state to .txt file)
    // if let Err(e) = save_game_board_state(&game_name, board) {
    //     eprintln!("Failed to save game state: {}", e);
    // }


    // // Terminal-print the updated board
    // print_board(&board);

    // let board_string = board_to_string(&board);

    // response_string.push_str(&format!(
    //     "Game: {}\nPiece: {}\nMove to: ({}, {})\n\n{}",
    //     game_name,
    //     piece,
    //     to.1,
    //     to.0,
    //     board_string
    // ));

    // return response string
    Ok(response_string)

}


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


fn save_game_board_state(game_name: &str, board: [[char; 8]; 8]) -> std::io::Result<()> {
    let dir_path = format!("./games/{}", game_name);
    std::fs::create_dir_all(&dir_path)?;

    let file_path = format!("{}/game_board_state.txt", dir_path);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    for row in board.iter() {
        let line: String = row.iter().collect();
        writeln!(file, "{}", line)?;
    }
    
    Ok(())
}

fn load_game_board_state(game_name: &str) -> std::io::Result<Board> {
    let dir_path = format!("./games/{}", game_name);
    let file_path = format!("{}/game_board_state.txt", dir_path);
    let file_content = std::fs::read_to_string(file_path)?;

    let mut board: Board = [[' '; 8]; 8];
    for (i, line) in file_content.lines().enumerate() {
        for (j, square) in line.chars().enumerate() {
            board[i][j] = square;
        }
    }

    Ok(board)
}
