// move with http://0.0.0.0:8000/game/Pc2c4
// for back trace run with:
// RUST_BACKTRACE=full cargo run

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
                
                // let (piece, from, to);

                // // this should be written to move_Log

                // match parse_move(&move_data) {
                //     Ok(parsed) => {
                //         piece = parsed.0;
                //         from = parsed.1;
                //         to = parsed.2;
                //     }
                //     Err(_) => {
                //         let response = Response::from_string("Invalid move data.").with_status_code(400);
                //         if let Err(e) = request.respond(response) {
                //             eprintln!("Failed to respond to request: {}", e);
                //         }
                //         continue;
                //     }
                // }
                
                

                // Open CSV file for game in append mode, creating it if it doesn't exist

                let dir_path = format!("./games/{}", game_name);
                std::fs::create_dir_all(&dir_path).expect("Failed to create directory");
                
                let file_path = format!("{}/moves.csv", dir_path);

                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&file_path)
                    .unwrap();

                let mut wtr = csv::Writer::from_writer(file);

                // Write new move to CSV file
                wtr.write_record(&[move_data]).unwrap();
                wtr.flush().unwrap();


                // // Read all moves from CSV file and update the board accordingly
                // let mut rdr = csv::Reader::from_path(&file_path).unwrap();
                // let mut board = board.lock().unwrap();
                // for result in rdr.records() {
                //     let record = result.unwrap();
                //     let move_data = &record[0];

                //     let (piece, from, to);

                //     match parse_move(&move_data) {
                //         Ok(parsed) => {
                //             piece = parsed.0;
                //             from = parsed.1;
                //             to = parsed.2;
                //         }
                //         Err(_) => {
                //             let response = Response::from_string("Invalid move data.").with_status_code(400);
                //             if let Err(e) = request.respond(response) {
                //                 eprintln!("Failed to respond to request: {}", e);
                //             }
                //             continue;
                //         }
                //     }
                    

                //     match to_coords(&format!("{}{}", from.0, from.1)) {
                //         Ok(coords) => {
                //             let (x, y) = coords;
                //             board[x][y] = ' ';
                //         },
                //         Err(err) => {
                //             eprintln!("Error: {}", err);
                //             continue;
                //         },
                //     };
                    
                //     match to_coords(&format!("{}{}", to.0, to.1)) {
                //         Ok(coords) => {
                //             let (x, y) = coords;
                //             board[x][y] = piece;
                //         },
                //         Err(err) => {
                //             eprintln!("Error: {}", err);
                //             continue;
                //         },
                //     };
                    
                // }
                                // // Terminal-print the updated board
                                // print_board(&board);
                
                                // let board_string = board_to_string(&board);
                
                                // let response = Response::from_string(format!(
                                //     "Game: {}\nPiece: {}\nMove to: ({}, {})\n\n{}",
                                //     game_name,
                                //     piece,
                                //     to.1,
                                //     to.0,
                                //     board_string
                                // ));
                
                                // request.respond(response).unwrap();
                                
                // Read all moves from CSV file and update the board accordingly
                let mut rdr = csv::Reader::from_path(&file_path).unwrap();
                let mut board = board.lock().unwrap();
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
                        Err(_) => {
                            let response = Response::from_string("Invalid move data.").with_status_code(400);
                            if let Err(e) = request.respond(response) {
                                eprintln!("Failed to respond to request: {}", e);
                            }
                            continue;
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

                    let response = Response::from_string(format!(
                        "Game: {}\nPiece: {}\nMove to: ({}, {})\n\n{}",
                        game_name,
                        piece,
                        to.1,
                        to.0,
                        board_string
                    ));

                    request.respond(response).unwrap();
                }

            } else {
                let response = Response::from_string("Invalid request format. It should be '/{game_name}/{move_data}'");
                request.respond(response).unwrap();
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
