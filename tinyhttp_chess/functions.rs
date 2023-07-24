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


// apply_move takes an existing board state and a move, and returns a new
// board state that reflects the outcome of the move.
fn board_state_after_move1(board: &Board, piece: char, from: (usize, usize), to: (usize, usize)) -> Board {
    let mut new_board = board.clone(); // Clone the board to ensure immutability

    // Empty the 'from' cell
    new_board[from.0][from.1] = ' ';

    // Place the piece in the 'to' cell
    new_board[to.0][to.1] = piece;

    new_board
}

// We can then define a function like this

fn board_state_after_move(board: &Board, move_data: &str) -> Result<Board, String> {
    let parsed_move = parse_move(move_data)?;
    
    // Now, create a new board and apply the move on this new board
    let mut new_board = *board; // copy the original board
    // Now apply the parsed move on new_board
    // let's assume the parsed_move indicates a move from ('a',1) to ('b',2)
    
    new_board[parsed_move.1.1 as usize][parsed_move.1.0 as usize] = ' '; // remove the piece from the original location
    new_board[parsed_move.2.1 as usize][parsed_move.2.0 as usize] = parsed_move.0; // place the piece at the new location

    Ok(new_board)
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



// fn save_game_board_state(game_name: &str, board: &Board) -> std::io::Result<()> {
//     let dir_path = format!("./games/{}", game_name);
//     std::fs::create_dir_all(&dir_path)?;

//     let file_path = format!("{}/game_board_state.txt", dir_path);
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(file_path)?;

//     for row in board.iter() {
//         let line: String = row.iter().collect();
//         writeln!(file, "{}", line)?;
//     }
    
//     Ok(())
// }


// fn load_game_board_state(game_name: &str, board: &mut Board) -> std::io::Result<()> {
//     let file_path = format!("./games/{}/game_state.txt", game_name);
//     let file = std::fs::File::open(file_path)?;
//     let reader = std::io::BufReader::new(file);

//     for (i, line) in reader.lines().enumerate() {
//         let line = line?;
//         if i < 8 { // make sure we don't index out of the board
//             for (j, piece) in line.chars().enumerate() {
//                 if j < 8 { // make sure we don't index out of the board
//                     board[i][j] = piece;
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// apply_move takes an existing board state and a move, and returns a new
// board state that reflects the outcome of the move.
fn board_state_after_move1(board: &Board, piece: char, from: (usize, usize), to: (usize, usize)) -> Board {
    let mut new_board = board.clone(); // Clone the board to ensure immutability

    // Empty the 'from' cell
    new_board[from.0][from.1] = ' ';

    // Place the piece in the 'to' cell
    new_board[to.0][to.1] = piece;

    new_board
}

// We can then define a function like this

fn board_state_after_move(board: &Board, move_data: &str) -> Result<Board, String> {
    let parsed_move = parse_move(move_data)?;
    
    // Now, create a new board and apply the move on this new board
    let mut new_board = *board; // copy the original board
    // Now apply the parsed move on new_board
    // let's assume the parsed_move indicates a move from ('a',1) to ('b',2)
    
    new_board[parsed_move.1.1 as usize][parsed_move.1.0 as usize] = ' '; // remove the piece from the original location
    new_board[parsed_move.2.1 as usize][parsed_move.2.0 as usize] = parsed_move.0; // place the piece at the new location

    Ok(new_board)
}

// // New function for applying a move to a board
// fn apply_move(board: &Board, chess_move: ChessMove) -> Board {
//     let mut new_board = *board; // Create a copy of the board
//     let ChessMove { piece, from, to } = chess_move; // Destructure the ChessMove struct
//     new_board[from.0][from.1] = ' '; // Remove piece from starting position
//     new_board[to.0][to.1] = piece; // Place piece in ending position
//     new_board // Return the new board
// }


// fn make_move(original_state: &Board, move_: Move) -> Board {
//     // Create a new game board that results from making the move
//     let new_state = apply_move_to_state(original_state, &move_);
    
//     new_state
// }

// fn apply_move_to_state(original_state: &Board, move_: &Move) -> Board {
//     // Function that applies a move to the original state and returns a new state
//     // Logic of applying move goes here...
    
//     let new_state = Board {
//         // fields updated based on the move
//     };
    
//     new_state
// }


// fn load_game_board_state(game_name: &str, board: &mut Board) -> std::io::Result<()> {
//     let dir_path = format!("./games/{}", game_name);
//     std::fs::create_dir_all(&dir_path)?;

//     let file_path = format!("{}/game_board_state.txt", dir_path);
//     let file = match OpenOptions::new().read(true).open(&file_path) {
//         Ok(file) => file,
//         Err(_) => return Ok(()),  // if file does not exist, return Ok and continue with an empty board
//     };

//     let reader = std::io::BufReader::new(file);
//     let mut lines = reader.lines();

//     for row in board.iter_mut() {
//         let line = match lines.next() {
//             Some(line) => line?,
//             None => break,  // if there are no more lines, break the loop
//         };
//         if line.len() != row.len() {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 "Invalid board state data",
//             ));
//         }
//         row.copy_from_slice(line.as_bytes());
//     }

//     Ok(())
// }

fn load_game_board_state(game_name: &str) -> std::io::Result<Board> {
    let dir_path = format!("./games/{}", game_name);
    let file_path = format!("{}/game_state.txt", dir_path);
    let file_content = std::fs::read_to_string(file_path)?;

    let mut board: Board = [[' '; 8]; 8];
    for (i, line) in file_content.lines().enumerate() {
        for (j, square) in line.chars().enumerate() {
            board[i][j] = square;
        }
    }

    Ok(board)
}


// apply_move takes an existing board state and a move, and returns a new
// board state that reflects the outcome of the move.
fn board_state_after_move1(board: &Board, piece: char, from: (usize, usize), to: (usize, usize)) -> Board {
    let mut new_board = board.clone(); // Clone the board to ensure immutability

    // Empty the 'from' cell
    new_board[from.0][from.1] = ' ';

    // Place the piece in the 'to' cell
    new_board[to.0][to.1] = piece;

    new_board
}

// We can then define a function like this

fn board_state_after_move(board: &Board, move_data: &str) -> Result<Board, String> {
    let parsed_move = parse_move(move_data)?;
    
    // Now, create a new board and apply the move on this new board
    let mut new_board = *board; // copy the original board
    // Now apply the parsed move on new_board
    // let's assume the parsed_move indicates a move from ('a',1) to ('b',2)
    
    new_board[parsed_move.1.1 as usize][parsed_move.1.0 as usize] = ' '; // remove the piece from the original location
    new_board[parsed_move.2.1 as usize][parsed_move.2.0 as usize] = parsed_move.0; // place the piece at the new location

    Ok(new_board)
}

if url_parts.len() == 0 {
    // This is the landing page (base domain only)
    // Handle the landing page logic here
    // For example, display a welcome message or show some introductory content.
} else if url_parts.len() == 3 {
    // This is a valid URL with exactly three parts
    // Extract the data from the URL and perform the corresponding action
    let game_name = url_parts[1].to_string();
    let move_data = url_parts[2].to_string();
    let mut response_string = String::new();
} else {
    // Handle the case when the URL has more than three parts or invalid URL
    // For example, return an error response or provide a default action.
}


// Check if it's the landing page (base domain only)
if path == "/" {
    // Here, you can read the pre-existing HTML script from a file or use a hardcoded string.
    // For this example, I'll provide a simple response with a "Hello, World!" message.
    let landing_page_content = "<html><body><h1>Hello, World!</h1></body></html>";

    // Create an HTTP response with the pre-existing HTML content
    let response = Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(landing_page_content))
        .unwrap();

    Ok(response)



fn landing_page()

            // landing page
            // Check if it's the landing page (base domain only)
            if url_parts.len() == 0 {
                // Here, you can read the pre-existing HTML script from a file or use a hardcoded string.
                // For this example, I'll provide a simple response with a "Hello, World!" message.
                let landing_page_content = r#"<html>
                <body>
                  <body style="background-color:black;">
                  <font color="00FF00">  
                        <div style="line-height:1px">
                    <tt> 
                    <p style="font-size:38px; "> r n b q k b n r </p>
                    <p style="font-size:38px; "> p p p p p p p p </p>
                    <p style="font-size:38px; "> . . . . . . . . </p>
                    <p style="font-size:38px; "> . . . . . . . . </p>
                    <p style="font-size:38px; "> . . . P . . . . </p>
                    <p style="font-size:38px; "> . . . . . . . . </p>
                    <p style="font-size:38px; "> P P P . P P P P </p>
                    <p style="font-size:38px; "> R N B Q K B N R </p>
                    
                    <p style="font-size:18px; "> 鰻　み　岡　野　エ　た　お　天　ラ　白 </p>
                    <p style="font-size:18px; "> 丼　そ　山　菜　ビ　こ　で　丼　ー　竜 </p>
                    <p style="font-size:18px; "> 八　カ　の　天　フ　焼　ん　八　メ　 </p>
                    <p style="font-size:18px; "> 三　ツ　ラ　ぷ　ラ　き　四　円　ン </p>
                    <p style="font-size:18px; "> 百　ラ　ー　ら　イ　三　円 </p>
                    <p style="font-size:18px; "> 六　ー　メ　八　十　円 </p>
                    <p style="font-size:18px; "> 十　メ　ン　五　円 </p>
                    <p style="font-size:18px; "> 三　ン　十　円 </p>
                    <p style="font-size:18px; "> 八　万　円 </p>
                    <p style="font-size:18px; "> 万　円 </p>
                    <p style="font-size:18px; "> 円　</p>
                        </div>
                        </body>
                    </html>
                    "#;


                let response = Response::from_string(landing_page_content);

                match request.respond(response) {
                    Ok(_) => {
                        // Successfully responded, do something if needed
                    }
                    Err(error) => {
                        // Handle the error gracefully
                        println!("Error: {:?}", error);
                        // Or perform some other error handling actions
                    }
                }
            }
            // game setup



                // sanitize and validate inputs from get request
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
                        eprintln!("Failed to open file: {}", e);
                        continue;
                    }
                };
            
                let mut wtr = csv::Writer::from_writer(file);

                // Write new move to CSV file
                if let Err(e) = wtr.write_record(&[move_data.clone()]) {
                    eprintln!("Failed to write to file: {}", e);
                    continue;
                }
                
                if let Err(e) = wtr.flush() {
                    eprintln!("Failed to flush writer: {}", e);
                    continue;
                }
                

                // Load the game board state
                let mut board = match load_game_board_state(&game_name) {
                    Ok(board) => board,
                    Err(e) => {
                        eprintln!("Failed to load game state: {}", e);
                        continue;
                    }
                };

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

                // // New use of the apply_move function
                // let new_board = board_state_after_move(&board, piece, from, to);
                // *board = new_board;  // Assign the new board back to the shared board state

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

                let response = Response::from_string(response_string);




                fn handle_chess_move(game_name: String, move_data: String, request: &Request) {
                    // This is where all the logic for handling a chess move will go.
                
                        
                    // sanitize and validate inputs from get request
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
                            eprintln!("Failed to open file: {}", e);
                            continue;
                        }
                    };
                
                    let mut wtr = csv::Writer::from_writer(file);
                
                    // Write new move to CSV file
                    if let Err(e) = wtr.write_record(&[move_data.clone()]) {
                        eprintln!("Failed to write to file: {}", e);
                        continue;
                    }
                
                    if let Err(e) = wtr.flush() {
                        eprintln!("Failed to flush writer: {}", e);
                        continue;
                    }
                
                
                    // Load the game board state
                    let mut board = match load_game_board_state(&game_name) {
                        Ok(board) => board,
                        Err(e) => {
                            eprintln!("Failed to load game state: {}", e);
                            continue;
                        }
                    };
                
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
                
                    // // New use of the apply_move function
                    // let new_board = board_state_after_move(&board, piece, from, to);
                    // *board = new_board;  // Assign the new board back to the shared board state
                
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
                
                    let response = Response::from_string(response_string);
                
                }


                for request in server.incoming_requests() {
                    // get request containing game and move
                    if request.method() == &Method::Get {
                        let url_parts: Vec<&str> = request.url().split('/').collect();
                        let mut response_string = String::new();  // moved to here
                
                        // if chess game move
                        if url_parts.len() == 3 {
                            let game_name = url_parts[1].to_string();
                            let move_data = url_parts[2].to_string();  
                
                            // sanitize and validate inputs from get request
                            match validate_input(&move_data) {
                                Err(err_msg) => {
                                    response_string = err_msg;  // update the response string
                                    // continue;  // note: `continue` might skip the response, which might not be what you want
                                },
                                Ok(()) => {},
                            }
                
                            // call game move function
                            match handle_chess_move(game_name, move_data) {
                                Ok(result_string) => {
                                    response_string = result_string;  // update the response string
                                },
                                Err(e) => {
                                    eprintln!("Failed to handle move: {}", e);
                                    response_string = format!("Failed to handle move: {}", e);  // update the response string
                                }
                            }
                        } else {
                            // ... Invalid request format
                            response_string = "Invalid request format".to_string();  // update the response string
                        }
                
                        // use request.respond() only once
                        let response = Response::from_string(response_string);
                        if let Err(error) = request.respond(response) {
                            eprintln!("Failed to respond to request: {}", error);
                        }
                    }
                }


        // landing page (NOT HTML, keep it)
        // Check if it's the landing page (base domain only)
        if url_parts.len() == 2 {
            let response = match landing_page() {
                Ok(response_string) => {
                    Response::from_string(response_string).with_status_code(200)
                },
                Err(e) => {
                    eprintln!("Failed to generate landing page: {}", e);
                    Response::from_string(format!("Failed to generate landing page: {}", e)).with_status_code(500)
                }
            };

            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {}", e);
            }
            continue; // No need to run the rest of the loop for the landing page
        }


        // access json like this:
        fn main() {
            let dir_path = "./games/my_game";
            match read_gamedata_json(dir_path) {
                Ok(parsed_json) => {
                    let game_name = &parsed_json["game_name"];
                    let activity_timestamp = &parsed_json["activity_timestamp"];
                    let game_type = &parsed_json["game_type"];
                    let move_number = &parsed_json["move_number"];
        
                    println!("Game Name: {}", game_name);
                    println!("Activity Timestamp: {}", activity_timestamp);
                    println!("Game Type: {}", game_type);
                    println!("Move Number: {}", move_number);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        

        use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::SystemTime;

fn read_gamedata_json(dir_path: &str) -> Result<String, io::Error> {
    let json_path = format!("{}/game_data.json", dir_path);
    std::fs::read_to_string(json_path)
}

fn update_activity_timestamp(dir_path: &str) -> Result<(), io::Error> {
    // Read the existing JSON data
    let mut json_data = read_gamedata_json(dir_path)?;

    // Update the activity_timestamp field with the current timestamp
    if let Some(pos_start) = json_data.find("\"activity_timestamp\":") {
        if let Some(pos_end) = json_data[pos_start..].find(',') {
            let old_timestamp = &json_data[pos_start + "\"activity_timestamp\":".len()..pos_start + pos_end];
            let new_timestamp = format!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64);
            json_data = json_data.replace(old_timestamp, &new_timestamp);
        }
    }

    // Open the file for writing
    let json_path = format!("{}/game_data.json", dir_path);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(json_path)?;

    // Write the updated JSON data back to the file
    file.write_all(json_data.as_bytes())?;

    Ok(())
}


use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn read_last_timestamp(file_path: &str) -> Result<i64, io::Error> {
    let mut file = File::open(file_path)?;
    let mut timestamp_str = String::new();
    file.read_to_string(&mut timestamp_str)?;
    Ok(timestamp_str.trim().parse::<i64>().unwrap_or(0))
}

fn update_activity_timestamp(file_path: &str) -> Result<(), io::Error> {
    let new_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut file = File::create(file_path)?;
    file.write_all(new_timestamp.to_string().as_bytes())?;

    Ok(())
}




/*
    designed to be a good-enough hash, not relying on libraries
    that includes string and timestamp

    the timestamp kind of functions as a 'secret key'
    as in crypographic 'signing' and verification
    again: not meant to be super world class,
    but light weight and easy to debug,
    not likely to have unexpected or incomprehensible issues
    no external libraries, trust issues, etc. 
    
    the first few digits are less random, so -> removed
    this also keeps the hash from getting huge so quickly

    to make the numbers more significantly different if even one
    input character is changed: add an additional hash if the 
    current hash is odd/even (even picked here)

    Recommended:
    for timestamp: use this to get sub-second depth in a string

    from datetime import datetime
    # get time
    timestamp_raw = datetime.utcnow()
    # make readable string
    timestamp = timestamp_raw.strftime('%Y%m%d%H%M%S%f')
*/
fn make_hash(input_string: &str, timestamp_string: &str) -> u128 {
    let mut string_to_hash = String::from(input_string);
    string_to_hash.push_str(timestamp_string);

    let mut hash: u128 = 1;
    for this_character in string_to_hash.chars() {
        // println!("this_character {}", this_character);
        // println!("this_character as u128 {}", this_character as u128);

        // Calculate the new hash value using integer arithmetic
        hash = 101 * (hash + this_character as u128);

        // println!("step 1 hash {}", hash);

        // reflip if the hash is even
        if hash % 2 == 0 {
            hash = 101 * (hash + this_character as u128);
        }
        println!("step 2 flip hash {}", hash);

        // Reduce the hash to a 6-digit number by parsing it as a string
        let hash_str = hash.to_string();
        if hash_str.len() > 6 {
            hash = match hash_str[2..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }
        if hash_str.len() > 15 {
            hash = match hash_str[4..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }
        
        
        
    }
    println!("finished hash {}", hash);
    
    hash
}
