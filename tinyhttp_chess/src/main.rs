// move with http://0.0.0.0:8000/game/Pc2c4
// for back trace run with:
// RUST_BACKTRACE=full cargo run

// Import Packages / Dependencies
extern crate tiny_http;
extern crate csv;

use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use tiny_http::{Server, Response, Method};
use std::io::Write;
use csv::Writer;

// Variables
type Position = (char, char, u8);
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
                let (piece, _from, to) = parse_move(&move_data);

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


                // Read all moves from CSV file and update the board accordingly
                let mut rdr = csv::Reader::from_path(&file_path).unwrap();
                let mut board = board.lock().unwrap();
                for result in rdr.records() {
                    let record = result.unwrap();
                    let move_data = &record[0];
                    let (piece, from, to) = parse_move(move_data);

                    let from_coords = to_coords(&format!("{}{}", from.0, from.1));
                    let to_coords = to_coords(&format!("{}{}", to.0, to.1));

                    board[from_coords.0][from_coords.1] = ' ';
                    board[to_coords.0][to_coords.1] = piece;
                }
                
                // Terminal-print the updated board
                print_board(&board);
                
                // // simply reports last move...
                // let response = Response::from_string(format!("Game: {}\nPiece: {}\nMove to: ({}, {})", game_name, piece, to.1, to.0));
                // request.respond(response).unwrap();
                
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

            } else {
                let response = Response::from_string("Invalid request format. It should be '/{game_name}/{move_data}'");
                request.respond(response).unwrap();
            }
        }
    }
}


/////////////////////
// Helper Functions!
/////////////////////
fn parse_move(move_data: &str) -> (char, (char, u8), (char, u8)) {
    let piece = move_data.chars().nth(0).unwrap();
    let from = (move_data.chars().nth(1).unwrap(), move_data.chars().nth(2).unwrap().to_digit(10).unwrap() as u8);
    let to = (move_data.chars().nth(3).unwrap(), move_data.chars().nth(4).unwrap().to_digit(10).unwrap() as u8);

    (piece, from, to)
}


fn to_chess_notation(coords: (usize, usize)) -> String {
    let col = (coords.1 as u8 + 'a' as u8) as char;
    let row = (8 - coords.0).to_string();
    format!("{}{}", col, row)
}


fn to_coords(chess_notation: &str) -> (usize, usize) {
    let col = chess_notation.chars().nth(0).unwrap() as usize - 'a' as usize;
    let row = 8 - chess_notation.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    (row, col)
}


fn print_board(board: &[[char; 8]; 8]) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn board_to_string_simple(board: &[[char; 8]; 8]) -> String {
    let mut board_string = String::new();
    for row in board.iter() {
        for &cell in row.iter() {
            board_string.push(cell);
            board_string.push(' ');
        }
        board_string.push('\n');
    }
    board_string
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


