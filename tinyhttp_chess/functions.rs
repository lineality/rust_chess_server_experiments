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


/// Convert a string in algebraic chess notation to a coordinate pair.
///
/// # Arguments
///
/// * `chess_notation` - A string slice that holds the chess notation (e.g. "e4").
///
/// # Returns
///
/// * `Ok((usize, usize))` - If the input is a valid chess notation, return a tuple with row and column as usize.
/// * `Err(String)` - If the input is invalid, return an error message.
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

// no new libraries
fn validate_input(input: &str) -> bool {
    if input.len() != 5 {
        return false;
    }

    let chars: Vec<char> = input.chars().collect();

    let valid_pieces = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];
    let valid_cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let valid_rows = ['1', '2', '3', '4', '5', '6', '7', '8'];

    valid_pieces.contains(&chars[0])
        && valid_cols.contains(&chars[1])
        && valid_rows.contains(&chars[2])
        && valid_cols.contains(&chars[3])
        && valid_rows.contains(&chars[4])
}

// Return Result with appropriate error messages instead of bool
fn validate_input(input: &str) -> Result<(), String> {
    if input.len() != 5 {
        return Err(format!("Invalid input length. Input should be 5 characters."));
    }

    let chars: Vec<char> = input.chars().collect();

    let valid_pieces = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];
    let valid_cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let valid_rows = ['1', '2', '3', '4', '5', '6', '7', '8'];

    if !valid_pieces.contains(&chars[0]) {
        return Err(format!("Invalid piece identifier. The first character should be one of 'prnbqkPRNBQK'."));
    }
    if !valid_cols.contains(&chars[1]) || !valid_cols.contains(&chars[3]) {
        return Err(format!("Invalid column identifier. The 2nd and 4th characters should be one of 'abcdefghABCDEFGH'."));
    }
    if !valid_rows.contains(&chars[2]) || !valid_rows.contains(&chars[4]) {
        return Err(format!("Invalid row identifier. The 3rd and 5th characters should be one of '12345678'."));
    }
    
    Ok(())
}


// old
fn parse_move(move_data: &str) -> (char, (char, u8), (char, u8)) {
    let piece = move_data.chars().nth(0).unwrap();
    let from = (move_data.chars().nth(1).unwrap(), move_data.chars().nth(2).unwrap().to_digit(10).unwrap() as u8);
    let to = (move_data.chars().nth(3).unwrap(), move_data.chars().nth(4).unwrap().to_digit(10).unwrap() as u8);

    (piece, from, to)
}


match parse_move(move_data) {
    Err(err_msg) => {
        eprintln!("Failed to parse move: {}", err_msg);
        continue;
    },
    Ok((piece, from, to)) => {
        let from_num = match from.1 {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse from coordinate: {}", from.1);
                continue;
            }
        };
        let to_num = match to.1 {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Failed to parse to coordinate: {}", to.1);
                continue;
            }
        };
        let from = (from.0, from_num);
        let to = (to.0, to_num);

        // Your code that uses piece, from, and to goes here...
    }
}



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



for request in server.incoming_requests() {
    if request.method() == &Method::Get {
        let url_parts: Vec<&str> = request.url().split('/').collect();
        if url_parts.len() >= 3 {
            let game_name = url_parts[1].to_string();
            let move_data = url_parts[2].to_string();
            let mut response_string = String::new();  // Create a string to accumulate the response

            // ... validate input, open and write to CSV file

            for result in rdr.records() {
                // ... update the board accordingly
                // ... instead of responding immediately, append to the response_string
                
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
