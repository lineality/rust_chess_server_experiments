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

        // Check if the in-memory queue size exceeds the threshold, and if so, write a batch to disk
        if {
            let queue = in_memory_queue.lock().unwrap();
            queue.len() >= RAM_QUEUE_THRESHOLD
        } {
            // Start a new thread to write the batch to disk asynchronously
            let in_memory_queue_clone = in_memory_queue.clone();
            thread::spawn(move || {
                write_batch_to_disk(in_memory_queue_clone);
            });
            // Clear the in-memory queue after writing to disk
            let mut queue = in_memory_queue.lock().unwrap();
            queue.clear();
        }

        // if in_memory_queue.len() >= RAM_QUEUE_THRESHOLD {
        //     // Start a new thread to write the batch to disk asynchronously
        //     let in_memory_queue_clone = in_memory_queue.clone();
        //     thread::spawn(move || {
        //         write_batch_to_disk(in_memory_queue_clone);
        //     });
        //     // Clear the in-memory queue after writing to disk
        //     let mut queue = in_memory_queue.lock().unwrap();
        //     queue.clear();
        // }

        if in_memory_queue.len() >= RAM_QUEUE_THRESHOLD {
            // Start a new thread to write the batch to disk asynchronously
            let in_memory_queue_clone = in_memory_queue.clone();
            thread::spawn(move || {
                write_batch_to_disk(in_memory_queue_clone);
            });
            // Clear the in-memory queue after writing to disk
            let mut queue = in_memory_queue.lock().unwrap();
            queue.clear();
        }

fn to_chess_notation(coords: (usize, usize)) -> String {
    let col = (coords.1 as u8 + 'a' as u8) as char;
    let row = (8 - coords.0).to_string();
    format!("{}{}", col, row)
}


fn main() -> Result<(), std::io::Error> {
    let game_name = "game";

    // let board_orientation: bool = true; // 
    // generate_chessboard_backboards(game_name, board_orientation)?;

    // let board_orientation: bool = false; // 
    // generate_chessboard_backboards(game_name, board_orientation)?;

    /*
    [src/main.rs:1533] "from -> " = "from -> "
    [src/main.rs:1533] from = (
    'c',
    2,
    )
    [src/main.rs:1534] "to -> " = "to -> "
    [src/main.rs:1534] to = (
    'c',
    4,
    )
    from: (char, u8),
    to: (char, u8),
    */



    let from = Some((7, 1)); // Replace with your desired values
    let to = Some((5, 2));   // Replace with your desired values

    let from: (char, u8) = (
        'b',
        2,
        );
    let to: (char, u8) = (
        'c',
        3,
        );


    // Set up board
    let board: [[char; 8]; 8] = [
        ['r', 'n', 'b', 'q', 'k', 'b', ' ', 'r'],
        ['p', 'p', 'p', 'p', ' ', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', 'n', ' ', ' '],
        [' ', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
        [' ', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', 'N', ' ', ' ', ' ', ' ', ' '],
        ['P', ' ', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', ' ', 'B', 'Q', 'K', 'B', 'N', 'R']
    ];
    let game_board_state:[[char; 8]; 8] = board;


    let board_orientation: bool = false; // Black Pieces Orientation
    let board_orientation: bool = true; // White
    generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;

    


    Ok(())
}


// fn board_coordinates_char_u8_to_usize_tuple(input: (char, u8)) -> Option<(usize, usize)> {
//     let col = input.0 as u8 - b'a'; // Assuming 'a' maps to 0
//     let row = input.1;
//     Some((col as usize, row as usize))
// }


fn current_week_of_year() -> u32 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (duration_since_epoch.as_secs() / 60 / 60 / 24 / 7) as u32 % 52
}



// fn string_to_board(s: &str) -> [[char; 8]; 8] {
//     let mut board = [[' '; 8]; 8];
//     let s = s.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
//     for i in 0..8 {
//         for j in 0..8 {
//             board[i][j] = s[i * 8 + j];
//         }
//     }
//     board
// }

    // pub fn new(next_check_time_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let next_check_time = read_next_check_time_from_file(next_check_time_file)?;
    //     Ok(Self {
    //         next_check_time,
    //         expiration_by_name: HashMap::new(),
    //         names_by_expiration: BTreeMap::new(),
    //     })
    // }

// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     let queue = in_memory_queue.lock().unwrap();
//     // Implement the logic to write the batch of requests to disk asynchronously
//     // For example, you can iterate over the queue and write each request to a file or an on-disk database.
//     // Remember to handle errors and use proper I/O operations (e.g., buffered writes) to optimize performance.
// }

// fn process_in_memory_requests(in_memory_queue: &Arc<Mutex<VecDeque<Request>>>) {
//     let mut queue = in_memory_queue.lock().unwrap();
//     // Process requests in the in-memory queue
//     while let Some(request) = queue.pop_front() {
//         // Implement your request processing logic here
//         println!("Processing in-memory request: {:?}", request);
//         // For demonstration purposes, we'll just print the request content.
//     }
// }



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



use std::time::{SystemTime, UNIX_EPOCH};

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

    // first set the string-to-hash to be the input_string
    let mut string_to_hash = String::from(input_string);

    // then add the timestamp to the string
    string_to_hash.push_str(timestamp_string);

    // Before starting: set hash to value of 1
    let mut hash: u128 = 1;

    // begin iterating through the hashstring, on character at at time
    for this_character in string_to_hash.chars() {
        // println!("this_character {}", this_character);
        // println!("this_character as u128 {}", this_character as u128);


        /*  Step 1: Sum & Product
            Calculate the new hash value using integer arithmetic
            - Turn the character into a number
            - sum (add) the hash and that character
            - product: multiply that sum by 101 
            (31 can be used alternatiely)
        */
        hash = 101 * (hash + this_character as u128);
        // println!("step 1 hash {}", hash);


        /*  Step 2: Flip
            re-hash if the hash is even
        */
        if hash % 2 == 0 {
            hash = 101 * (hash + this_character as u128);
        }
        // println!("step 2 flip hash {}", hash);


        /*  Step 3: Trim 
            Reduce the hash to a 6-digit number by parsing it as a string
        */
        let hash_str = hash.to_string();


        // remove the first character from everything
        hash = match hash_str[1..].parse() {
            Ok(parsed_hash) => parsed_hash,
            Err(_) => {
                eprintln!("Failed to parse hash: {}", hash_str);
                0 // Set a default value or take appropriate action on parsing failure
            }
        };
    
        // remove 2 front characters from medium hashes
        if hash_str.len() > 6 {
            hash = match hash_str[2..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }

        // remove 3 front characters from long hashes
        if hash_str.len() > 20 {
            hash = match hash_str[3..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }
        
    }
    // println!("finished hash {}", hash);
    
    // return hash
    hash
}

fn main() {
    // get current timestamp
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH)
                       .expect("Time went backwards")
                       .as_millis()
                       .to_string();

    println!("{}", make_hash("123:123:243:234", &timestamp));

    // compare small change: just the last digit of the time
    println!("{}", make_hash("123:123:243:234", "20221008133518385205"));
    println!("{}", make_hash("123:123:243:234", "20221008133518385206"));

    // small number input: edge case check
    println!("{}", make_hash("1", "3"));
    println!("{}", make_hash("2", "2"));


    // small number input: edge case check
    println!("{}", make_hash("1", "3"));
    println!("{}", make_hash("1", "2"));
}

//svg chess board
use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::Document;

// Function to generate the SVG chessboard
fn generate_chessboard(chessboard: &[[char; 8]; 8]) -> String {
    let mut doc = Document::new()
        .set("width", "400")
        .set("height", "400");

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let x = col * 50;
            let y = row * 50;

            let square_color = if (row + col) % 2 == 0 {
                "#ccc"
            } else {
                "#666"
            };

            let square = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", 50)
                .set("height", 50)
                .set("fill", square_color);

            doc = doc.add(square);

            if piece != ' ' {
                let mut text = Text::new()
                    .set("x", x + 25)
                    .set("y", y + 35)
                    .set("text-anchor", "middle")
                    .set("font-size", 30)
                    .set("fill", if piece.is_lowercase() { "#cc0000" } else { "#ff9999" });

                if piece.is_uppercase() {
                    text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
                } else {
                    text = text.add(svg::node::Text::new(piece.to_string()));
                }

                doc = doc.add(text);
            }
        }
    }

    doc.to_string()
}

fn main() {
    let chessboard_state: [[char; 8]; 8] = [
        ['r', 'n', 'b', 'q', ' ', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', ' ', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', 'K', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'p', ' ', ' '],
        [' ', ' ', 'P', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', 'P', 'N'],
        ['P', 'P', 'P', 'P', 'P', 'P', ' ', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', ' ', 'R'],
    ];

    let svg_code = generate_chessboard(&chessboard_state);
    println!("{}", svg_code);
}


// // Function to generate the SVG chessboard with black orientation
// fn generate_white_oriented_chessboard(
//     chessboard: &[[char; 8]; 8], 
//     from: Option<(usize, usize)>, 
//     to: Option<(usize, usize)>
// ) -> Document {

//     let mut doc = Document::new()
//         .set("width", "500")  
//         .set("height", "500")  
//         .set("viewBox", (0, 0, 500, 500))
//         .set("style", "background-color: #2f0300;");  // Set background to dark red

//     // Define labels, reversed for black piece orientation
//     let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
//     let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

//     // Add column labels
//     for (idx, label) in column_labels.iter().enumerate() {
//         let label_text = Text::new()
//             .set("x", 50 + idx * 50 + 25)  
//             .set("y", 472)  
//             .set("text-anchor", "middle")
//             .set("font-size", 20)
//             .set("fill", "#757575")  // Set text color to dark grey
//             .add(svg::node::Text::new(label.to_string()));
//         doc = doc.add(label_text);
//     }

//     // Add row labels
//     for (idx, label) in row_labels.iter().enumerate() {
//         let label_text = Text::new()
//             .set("x", 32)  
//             .set("y", 50 + idx * 50 + 35)  
//             .set("text-anchor", "middle")
//             .set("font-size", 20)
//             .set("fill", "#757575")  
//             .add(svg::node::Text::new(label.to_string()));
//         doc = doc.add(label_text);
//     }

//     for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
//         for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
//             let x = 50 + col * 50;  
//             let y = 50 + row * 50;  

//             let square_color = if (row + col) % 2 == 0 {
//                 "#ccc"
//             } else {
//                 "#666"
//             };
            
//             let square = Rectangle::new()
//                 .set("x", x)
//                 .set("y", y)
//                 .set("width", 50)
//                 .set("height", 50)
//                 .set("fill", square_color);

//             doc = doc.add(square);

//             if piece != ' ' {

//                 // setting from an to color
//                 if let Some(from_coords) = from {
//                     let (row, col) = from_coords;
//                     let x = 50 + col * 50;
//                     let y = 50 + row * 50;
                
//                     let highlight = Rectangle::new()
//                         .set("x", x)
//                         .set("y", y)
//                         .set("width", 50)
//                         .set("height", 50)
//                         .set("fill", "none") // Transparent fill
//                         .set("stroke", "#3189D9")
//                         .set("stroke-width", 3);
                
//                     doc = doc.add(highlight);
//                 }
                
//                 if let Some(to_coords) = to {
//                     let (row, col) = to_coords;
//                     let x = 50 + col * 50;
//                     let y = 50 + row * 50;
                
//                     let highlight = Rectangle::new()
//                         .set("x", x)
//                         .set("y", y)
//                         .set("width", 50)
//                         .set("height", 50)
//                         .set("fill", "none") // Transparent fill
//                         .set("stroke", "#3189D9")
//                         .set("stroke-width", 3);
                
//                     doc = doc.add(highlight);
//                 }

                    
//                 let piece_color = if square_color == "#666" { // for darker background
//                     if piece.is_uppercase() {
//                         "#ffefc1" // lighter gray for light pieces
//                     } else {
//                         "#ff8e8e" // lighter red for dark pieces
//                     }
//                 } else { // for lighter background
//                     if piece.is_uppercase() {
//                         "#665628" // darker gray for light pieces
//                     } else {
//                         "#9e0b00" // darker red for dark pieces
//                     }
//                 };

//                 let mut text = Text::new()
//                     .set("x", x + 25)
//                     .set("y", y + 35)
//                     .set("text-anchor", "middle")
//                     .set("font-size", 30)
//                     .set("fill", piece_color);

//                 if piece.is_uppercase() {
//                     text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
//                 } else {
//                     text = text.add(svg::node::Text::new(piece.to_string()));
//                 }

//                 doc = doc.add(text);
//             }
//         }
//     }

//     doc
// }

// fn black_to_coords(chess_notation: &str) -> Result<(usize, usize), String> {
//     if chess_notation.len() != 2 {
//         return Err(format!("Invalid chess notation: '{}'. It should be two characters long.", chess_notation));
//     }
//     let col = chess_notation.chars().nth(0).unwrap();
//     let row = chess_notation.chars().nth(1).unwrap();

//     if !('a'..='h').contains(&col) || !('1'..='8').contains(&row) {
//         return Err(format!("Invalid chess notation: '{}'. It should be in the form 'e4'.", chess_notation));
//     }

//     let col = 'h' as usize - col as usize;  // Changed this line
//     let row = row.to_digit(10).unwrap() as usize - 1;  // And this line

//     Ok((row, col))
// }


// fn timestamp() -> u64 {
//     match SystemTime::now().duration_since(UNIX_EPOCH) {
//         Ok(duration) => duration.as_secs(),
//         Err(error) => {
//             // Handle the error (e.g., print an error message, log, etc.)
//             eprintln!("Error: {}", error);
//             1
//         }
//     }
// }

// fn check_ip_stamp_in_file(ip_stamp: &str, game_name: &str) -> Result<(), String> {
//     let file_path = format!("games/{}/ip_hash_list.txt", game_name);

//     match fs::read_to_string(&file_path) {
//         Ok(content) => {
//             // Split the file content into lines and check if ip_stamp is present in any of them
//             if content.lines().any(|line| line.trim() == ip_stamp) {
//                 Ok(())
//             } else {
//                 Err(String::from("IP not found in the list"))
//             }
//         }
//         Err(_) => Err(String::from("Could not read the file or the file does not exist")),
//     }
// }

// fn create_gamedata_json(dir_path: &str, game_name: &str, game_type: &str, move_number: u32) -> io::Result<()> {

//     let current_timestamp = timestamp();

//     // Create the JSON string
//     let json_data = format!(
//         r#"{{
//             "game_name": "{}",
//             "game_timestamp": {},
//             "activity_timestamp": {},
//             "game_type": "{}",
//             "move_number": {}
//         }}"#,
//         game_name,
//         current_timestamp,
//         current_timestamp,
//         game_type,
//         move_number
//     );

//     // Open the file for writing
//     let json_path = format!("{}/game_data.json", dir_path);
//     let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .truncate(true)
//         .open(json_path)?;

//     // Write the JSON data to the file
//     writeln!(file, "{}", json_data)?;

//     Ok(())
// }


    // // for delating old games...
    // fn read_last_timestamp(file_path: &str) -> Result<i64, io::Error> {
    //     let mut file = File::open(file_path)?;
    //     let mut timestamp_str = String::new();
    //     file.read_to_string(&mut timestamp_str)?;
    //     Ok(timestamp_str.trim().parse::<i64>().unwrap_or(0))
    // }


    // fn update_activity_timestamp(file_path: &str) -> Result<(), io::Error> {
    //     // TODO add gamename for dir-path ,which timestamp to use?
    //     let new_timestamp = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs() as i64;

    //     let mut file = File::create(file_path)?;
    //     file.write_all(new_timestamp.to_string().as_bytes())?;

    //     Ok(())
    // }





// // Return Result with appropriate error messages instead of bool
// fn validate_input(input: &str) -> Result<(), String> {
//     if input.len() != 5 {
//         return Err(format!("Invalid input length. Input should be 5 characters. e.g. Pc2c4 or pc7c6 "));
//     }

//     if input == "start" {
//         return Ok(());
//     }

//     let chars: Vec<char> = input.chars().collect();

//     let valid_pieces = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];
//     let valid_cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
//     let valid_rows = ['1', '2', '3', '4', '5', '6', '7', '8'];

//     if !valid_pieces.contains(&chars[0]) {
//         return Err(format!("Invalid piece identifier. The first character should be one of 'prnbqkPRNBQK'. e.g. Pc2c4 or pc7c6 "));
//     }
//     if !valid_cols.contains(&chars[1]) || !valid_cols.contains(&chars[3]) {
//         return Err(format!("Invalid column identifier. The 2nd and 4th characters should be one of 'abcdefgh'. e.g. Pc2c4 or pc7c6 "));
//     }
//     if !valid_rows.contains(&chars[2]) || !valid_rows.contains(&chars[4]) {
//         return Err(format!("Invalid row identifier. The 3rd and 5th characters should be one of '12345678'.e.g. Pc2c4 or pc7c6  "));
//     }
    
//     Ok(())
// }


    // pub fn access_directory(&mut self, name: &str) {
    //     if let Some(expiration_date) = self.expiration_by_name.get(name) {
    //         let new_expiration_date = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 30); // 30 days from now
    //         self.add_directory(name.to_string(), new_expiration_date);
    //     }
    // }
    pub fn access_directory(&mut self, name: &str) {
        if let Some(expiration_date) = self.expiration_by_name.get(name) {
            let new_expiration_date = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 30); // 30 days from now
            self.add_directory(name.to_string(), new_expiration_date);
        }
    }


// fn save_game_board_state(game_name: &str, board: [[char; 8]; 8]) -> std::io::Result<()> {
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

// fn parse_move(move_data: &str) -> Result<(char, (char, u8), (char, u8)), String> {
//     if move_data.len() != 5 {
//         return Err(format!("Invalid input length. Input should be 5 characters. e.g. Pc2c4 or pc7c6 "));
//     }

//     let chars: Vec<char> = move_data.chars().collect();

//     let piece = chars.get(0).ok_or("Failed to get piece")?;
//     let from_col = chars.get(1).ok_or("Failed to get from_col")?;
//     let from_row_digit = chars.get(2)
//         .ok_or("Failed to get from_row_digit")?
//         .to_digit(10)
//         .ok_or("Failed to parse from_row_digit to number")?;
//     let to_col = chars.get(3).ok_or("Failed to get to_col")?;
//     let to_row_digit = chars.get(4)
//         .ok_or("Failed to get to_row_digit")?
//         .to_digit(10)
//         .ok_or("Failed to parse to_row_digit to number")?;

//     let from = (*from_col, from_row_digit as u8);
//     let to = (*to_col, to_row_digit as u8);

//     Ok((*piece, from, to))
// }



        /*
        For testing, requires "mut request"
        for inspecting incoming requests for debugging
        */
        // // Inside your request handler function
        // println!("Method: {:?}", request.method());
        // println!("URL: {:?}", request.url());
        // println!("HTTP Version: {:?}", request.http_version());
        // println!("Headers: {:?}", request.headers());
        // println!("Remote address: {:?}", request.remote_addr());

        // let mut buffer = String::new();
        // match request.as_reader().read_to_string(&mut buffer) {
        //     Ok(_) => println!("Body: {}", buffer),
        //     Err(e) => eprintln!("Failed to read request body: {}", e),
        // };



        // call game move function
let response = match handle_chess_move(game_name, move_data) {
    Ok(svg_content) => {
        // Embed the SVG content in HTML with CSS to set its size
        let html_content = format!(r#"
        <html>
            <head>
                <style>
                    img {{
                        width: 300px;
                        height: 300px;
                    }}
                </style>
            </head>
            <body>
                <img src="data:image/svg+xml,{}" />
            </body>
        </html>
        "#, percent_encoding::percent_encode(svg_content.as_bytes(), percent_encoding::NON_ALPHANUMERIC));
        let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
            .unwrap_or_else(|_| panic!("Invalid header!")); // Maintain your error handling here

        Response::from_string(html_content).with_header(header).with_status_code(200)
    },
    Err(e) => {
        eprintln!("Failed to handle move: {}", e);
        Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
    }
};

if let Err(e) = request.respond(response) {
    eprintln!("Failed to respond to request: {}", e);
}

let response = match handle_chess_move(game_name, move_data) {
    Ok(svg_content) => {
        // Here we wrap the SVG content with HTML and a style element to adjust the SVG's size.
        let wrapped_content = format!(r#"
            <html>
                <head>
                    <style>
                        #my-svg {{
                            width: 300px;
                            height: 300px;
                        }}
                    </style>
                </head>
                <body>
                    <div id="my-svg">
                        {}
                    </div>
                </body>
            </html>
        "#, svg_content);

        let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
            .unwrap_or_else(|_| panic!("Invalid header!"));

        Response::from_string(wrapped_content).with_header(header).with_status_code(200)
    },
    Err(e) => {
        eprintln!("Failed to handle move: {}", e);
        Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
    }
};

if let Err(e) = request.respond(response) {
    eprintln!("Failed to respond to request: {}", e);
}






    if request.url() == "/favicon.ico" {
        let path = "favicon.ico";
        let response = match File::open(&path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                if file.read_to_end(&mut content).is_err() {
                    Response::empty(StatusCode(500))
                } else {
                    Response::new(200.into(), vec![], content.into(), Some(content.len()), None)
                }
            }
            Err(_) => Response::empty(StatusCode(404)),
        };
        if request.respond(response).is_err() {
            eprintln!("Error responding to request");
        }
    }



        // game_name
        let split_sub_segments: Vec<&str> = segment.split('_').collect();
        let game_name = split_sub_segments[0].to_string();

        
        // game_name
        let game_name = segments[0].to_string();
        println!("game_name:{}", game_name);
        
        
        


    // fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<Self> {
    //     let current_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
    //         Ok(duration) => duration.as_secs(),
    //         Err(_) => return None,
    //     };

    //     let segments: Vec<&str> = input.split('-').collect();

    //     if segments.len() < 2 {
    //         return None;
    //     }

    //     let mut white_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();
    //     let mut black_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();

    //     let mut white_temp_timecontrol: HashMap<u32, (u32, u32)> = HashMap::new();
    //     let mut black_temp_timecontrol: HashMap<u32, (u32, u32)> = HashMap::new();


    //     let mut white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();
    //     let mut black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();

    //     let mut white_time_remaining_sec: u32 = 0;
    //     let mut black_time_remaining_sec: u32 = 0;

    //     for segment in segments.iter().skip(1) {
    //         let elements: Vec<&str> = segment.split(',').collect();

    //         if *segment == "norway120" || *segment == "norwayarmageddon" {
    //             return TimedProject::from_preset_time_modes_chess(segment, &game_name);
    //         }

    //         if elements.len() < 2 {
    //             return None;
    //         }

    //         let key = elements[0].parse::<u32>().ok()?;
    //         let value1 = elements[1].parse::<u32>().ok()?;
    //         let value2 = elements.get(2).and_then(|x| x.parse().ok());

    //         match segments[0] {
    //             "incrimentsecsec" => {
    //                 white_increments_sec_sec_key_value_list.insert(value1, (value2));
    //                 black_increments_sec_sec_key_value_list.insert(value1, (value2));

    //             }
    //             "timecontrolmovemin" => {
    //                 if key == 0 {
    //                     white_time_remaining_sec = value1 * 60;
    //                     black_time_remaining_sec = value1 * 60;
    //                 }
    //                 white_temp_timecontrol.insert(value1, (value1, value2, value3));
    //                 black_temp_timecontrol.insert(value1, (value1, value2, value3));

    //             }
    //             _ => return None,
    //         }
    //     }

    //     // Convert the temporary HashMap to match the struct's type
    //     for (k, (v1, v2, v3, _)) in white_temp_timecontrol.iter() {
    //         white_timecontrol_move_min_incrsec_key_value_list.insert(k.clone(), (*v1, *v2, *v3));
    //     }


    //     // Convert the temporary HashMap to match the struct's type
    //     for (k, (v1, v2, v3, _)) in black_temp_timecontrol.iter() {
    //         black_timecontrol_move_min_incrsec_key_value_list.insert(k.clone(), (*v1, *v2, *v3));
    //     }

    //     Some(TimedProject {
    //         game_name: game_name.to_string(),
    //         project_start_time_timestamp: current_timestamp,
    //         white_time_remaining_sec,
    //         black_time_remaining_sec,
    //         white_increments_sec_sec_key_value_list,
    //         black_increments_sec_sec_key_value_list,
    //         white_timecontrol_move_min_incrsec_key_value_list,
    //         black_timecontrol_move_min_incrsec_key_value_list,
    //         last_move_time: 0,
    //         player_white: true,
    //         game_move_number: 0,
    //     })
    // }
    
    

//     /// Create a TimedProject with preset time modes for chess games
// pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<Self> {
//     // Key: move_number when increment starts
//     // Value: (seconds added at each turn)
//     let mut white_increments_map: HashMap<u32, u32> = HashMap::new();
//     let mut black_increments_map: HashMap<u32, u32> = HashMap::new();

//     // Key: move_number when time_control starts
//     // Value: (seconds added to clock, new increment in seconds)
//     let mut white_time_control_map: HashMap<u32, u32> = HashMap::new();
//     let mut black_time_control_map: HashMap<u32, u32> = HashMap::new();

//     // Match on provided preset string to initialize the settings
//     match preset {
//         // For "Norway 120" Classical Game
//         "norway120" => {


//             // Here we use 7200 seconds (120 minutes) as the initial time for both players.
//             Some(Self::new(game_name, 7200, white_increments_map, black_increments_map, white_time_control_map, black_time_control_map))
//         },

//         // For "Norway Chess Armageddon"
//         "norwayarmageddon" => {
//             white_increments_map.insert(61, 3);  // 3-sec increment starting from 61st move
//             black_increments_map.insert(61, 3);  // 3-sec increment starting from 61st move

//             // 300 seconds (5 mins) for White, 240 seconds (4 mins) for Black
//             Some(Self::new(game_name, 300, white_increments_map, black_increments_map, white_time_control_map, black_time_control_map))
//         },

//         // For "FIDE World Championship match"
//         "fideworldchampmatch" => {
//             white_increments_map.insert(61, 30);  // 30-sec increment starting from 61st move
//             black_increments_map.insert(61, 30);  // 30-sec increment starting from 61st move
            
//             // Add time control after 40th move: 60 minutes
//             white_time_control_map.insert(40, (3600, 0));
//             black_time_control_map.insert(40, (3600, 0));
            
//             // Add time control after 60th move: 15 minutes
//             white_time_control_map.insert(60, (900, 30));
//             black_time_control_map.insert(60, (900, 30));

//             // Here we use 7200 seconds (120 minutes) as the initial time for both players.
//             Some(Self::new(game_name, 7200, white_increments_map, black_increments_map, white_time_control_map, black_time_control_map))
//         },

//         // For any other presets, we return None
//         _ => None
//     }
// }


    // fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<TimedProject> {

    //     println!("starting from_increment_and_time_control() input: {}", input);
    //     println!("starting from_increment_and_time_control() game_name: {}", game_name);


    //     // Determine the current POSIX timestamp in seconds
    //     let current_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
    //         Ok(duration) => duration.as_secs(),
    //         Err(_) => {
    //             println!("An error occurred while obtaining system time");
    //             return None; // Return None to align with function's return type
    //         }
    //     };

    //     // Split the input into segments by underscores
    //     let segments: Vec<&str> = input.split('_').collect();
        
    //     if segments.len() < 2 {
    //         return None;
    //     }

    //     let project_start_time_timestamp: u64 = current_timestamp;
    //     let mut increments_sec_sec_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut white_time_remaining_sec: u32 = 0;
    //     let mut black_time_remaining_sec: u32 = 0;

    //     // Parse the remaining segments
    //     for segment in &segments[1..] {
    //         println!("in from_increment_and_time_control() this segment: {}", segment);

    //         if *segment == "norway120" || *segment == "norwayarmageddon" {
    //             return TimedProject::from_preset_time_modes_chess(segment, &game_name);
    //         }

    //         let mut iter = segment.split('(');
    //         let control_type = iter.next()?;
            
    //         // Gather all tuples
    //         let joined_tuples: String = iter.collect::<Vec<_>>().join("(");
    //         let tuple_strs: Vec<&str> = joined_tuples.split(')').collect();
    
    //         for tuple_str in tuple_strs {
    //             if tuple_str.is_empty() {
    //                 continue;
    //             }
                
    //             let elements: Vec<u32> = tuple_str.split(',')
    //                 .filter_map(|x| x.parse().ok())
    //                 .collect();
                
    //             if elements.len() != 2 {
    //                 return None;
    //             }

    //             // Match the control type and process accordingly
    //             match control_type {
    //                 "incrementseconds" => {
    //                     increments_sec_sec_key_value_list.push((elements[0], elements[1]));
    //                 },
    //                 "timecontrolmin" => {
    //                     if elements[0] == 0 {
    //                         white_time_remaining_sec = elements[1] * 60;  // Convert minutes to seconds
    //                         black_time_remaining_sec = elements[1] * 60;  // Convert minutes to seconds
    //                     }
    //                     timecontrol_move_min_key_value_list.push((elements[0] as u32, elements[1] as u32));
    //                 },
    //                 _ => return None,
    //             }
    //         }
    //     }

    //     // Create and return the TimedProject struct
    //     Some(TimedProject {
    //         game_name: game_name.to_string(),
    //         project_start_time_timestamp,
    //         white_time_remaining_sec,
    //         black_time_remaining_sec,
    //         increments_sec_sec_key_value_list,
    //         timecontrol_move_min_key_value_list,
    //         last_move_time: 0,
    //         player_white: true,
    //         game_move_number: 0,
    //     })
    // }

    

    // /// Create a TimedProject from a known preset
    // pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<TimedProject> {
        
      

    //     println!("starting from_preset_time_modes_chess()");


    //     match preset {
    //         "norway120" => Some(TimedProject {
    //             game_name: game_name.to_string(),
    //             project_start_time_timestamp: 7200, // 120 minutes in seconds
    //             white_time_remaining_sec: 7200, // 120 minutes in seconds
    //             black_time_remaining_sec: 7200, // 120 minutes in seconds
    //             increments_sec_sec_key_value_list: vec![(40, 1800)], // Add 30 minutes after move 40
    //             timecontrol_move_min_key_value_list: vec![],
    //             last_move_time: 0,  // Initialize missing field
    //             player_white: true, // Initialize missing field
    //             game_move_number: 0, // Initialize missing field
    //         }),
    //         "norwayarmageddon" => Some(TimedProject {
    //             game_name: game_name.to_string(),
    //             project_start_time_timestamp: 300, // 5 minutes in seconds
    //             white_time_remaining_sec: 300, // 5 minutes in seconds
    //             black_time_remaining_sec: 300, // 5 minutes in seconds
    //             increments_sec_sec_key_value_list: vec![], // No increment
    //             timecontrol_move_min_key_value_list: vec![],
    //             last_move_time: 0,  // Initialize missing field
    //             player_white: true, // Initialize missing field
    //             game_move_number: 0, // Initialize missing field
    //         }),
    //         _ => None, // Unknown preset
    //     }
    //     }


    // /// Create a TimedProject with preset time modes for chess games
    // pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<Self> {
    //             /*
    //     TODO: 
    //     update datastructures to be hashmaps not vec
        
    //     add other presets: fidewcmatch

    //     fideworldchampmatch
    //     QUote: FIDE 4. 2. Time control
    //     The time control for each game is 120 minutes for the first 40 moves, followed by 60 minutes for the next 20 moves and then 15
    //     minutes for the rest of the game with an increment of 30 seconds per move starting from move 61.
    //      */
    //     println!("starting from_preset_time_modes_chess()");
    //     // Initialize HashMaps for storing time control and increment settings
    //     let mut increments_map: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut time_control_map: HashMap<String, (u32, u32)> = HashMap::new();

    //     // Match on provided preset string
    //     match preset {
    //         // string notation of key values: timecontrolmovemin-wb,0,120,30-40,30

    //         /*
    //         Players start with 120 minutes on their clocks, and after each move, 
    //         they gain additional time, usually around 30 seconds per move. 
    //         This time increment continues until the end of the game.  
    //         */
    //         "norway120" => {
    //             increments_map.insert("move_40".to_string(), (40, 1800));  // 30 mins increment after 40th move
    //             Some(Self {
    //                 game_name: game_name.to_string(),
    //                 project_start_time_timestamp: 7200,  // 120 minutes in seconds
    //                 white_time_remaining_sec: 7200,  // 120 minutes in seconds
    //                 black_time_remaining_sec: 7200,  // 120 minutes in seconds
    //                 increments_sec_sec_key_value_list: increments_map,
    //                 timecontrol_move_min_key_value_list: time_control_map,
    //                 last_move_time: 0,
    //                 player_white: true,
    //                 game_move_number: 0,
    //             })
    //         },
    //         "norwayarmageddon" => {
    //             /*
    //             For Norway Chess Armageddon, there is indeed a time increment, 
    //             but it's a bit different from traditional chess time controls. 
    //             In Armageddon, White gets 5 minutes on the clock, 
    //             and Black gets 4 minutes. However, there's a crucial difference:

    //             White must win to claim victory, while Black only needs a draw to win the game.
    //             To compensate for this advantage, there is a time increment after move 60. 
    //             Starting from move 61, both players receive an additional 3 seconds per move. 
    //             This increment helps ensure that the game doesn't go on indefinitely 
    //             and adds a level of fairness to the Armageddon format.

    //             So, to summarize, there is a time increment in Norway Chess Armageddon, 
    //             but it starts after move 60, with both players receiving an extra 3 seconds per move.
    //              */

    //             // string notation of key values: timecontrolmovemin-w,0,5-b,0,4-wb,60,0,3
    //             Some(Self {
    //                 game_name: game_name.to_string(),
    //                 project_start_time_timestamp: 300,  // 5 minutes in seconds
    //                 white_time_remaining_sec: 300,  // 5 minutes in seconds
    //                 black_time_remaining_sec: 240,  // 4 minutes in seconds
    //                 increments_sec_sec_key_value_list: increments_map,
    //                 timecontrol_move_min_key_value_list: time_control_map,
    //                 last_move_time: 0,
    //                 player_white: true,
    //                 game_move_number: 0,
    //             })
    //         },
    //         "fideworldchampmatch" => {
    //             /*
    //             TODO: 
    //             fideworldchampmatch
    //             QUote: FIDE 4. 2. Time control
    //             The time control for each game is 120 minutes for the first 40 moves, 
    //             followed by 60 minutes for the next 20 moves and then 
    //             15 minutes for the rest of the game 
    //             with an increment of 30 seconds per move starting from move 61.

    //             string input format:
    //             timecontrolmovemin-61,30
    //             incrimentsecsec-wb,0,7200-wb,40,3600-wb,60,900
    //             */
    //             increments_map.insert("move_40".to_string(), (40, 1800));  // 30 mins increment after 40th move
    //             Some(Self {
    //                 game_name: game_name.to_string(),
    //                 project_start_time_timestamp: 7200,  // 120 minutes in seconds
    //                 white_time_remaining_sec: 7200,  // 120 minutes in seconds
    //                 black_time_remaining_sec: 7200,  // 120 minutes in seconds
    //                 increments_sec_sec_key_value_list: increments_map,
    //                 timecontrol_move_min_key_value_list: time_control_map,
    //                 last_move_time: 0,
    //                 player_white: true,
    //                 game_move_number: 0,
    //             })
    //         },
    //         _ => None // Unknown preset returns None
    //     }
    // }

    
        
    // use std::collections::HashMap;
    // use std::io::{self, BufReader, Lines};
    // use std::fs::File;

    // use std::collections::HashMap;
    // use std::fs::File;
    // use std::io::{self, BufRead, BufReader};
    
    // fn load_timedata_from_txt(game_name: &str) -> io::Result<TimedProject> {
    //     let path = format!("games/{}/time_data.txt", game_name);
    //     let file = File::open(&path)?;
    //     let reader = BufReader::new(file);
        
    //     let mut project_start_time_timestamp: u64 = 0;
    //     let mut white_time_remaining_sec: u32 = 0;
    //     let mut black_time_remaining_sec: u32 = 0;
    //     let mut increments_sec_sec_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut last_move_time: u64 = 0;
    //     let mut player_white: bool = true;
    //     let mut game_move_number: usize = 0;
    
    //     for line in reader.lines() {
    //         let line = line?;
    //         let parts: Vec<&str> = line.split(": ").collect();
            
    //         match parts[0] {
    //             "project_start_time_timestamp" => project_start_time_timestamp = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid project_start_time_timestamp"))?,
    //             "white_time_remaining_sec" => white_time_remaining_sec = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "black_time_remaining_sec" => black_time_remaining_sec = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid black_time_remaining_sec"))?,
    //             "increments_sec_sec_key_value_list" => {
    //                 let increments_map: HashMap<u32, u32> = string_to_hashmap(parts[1]);
    //                 increments_sec_sec_key_value_list = convert_to_named_tuple_map(increments_map);
    //             },
    //             "timecontrol_move_min_key_value_list" => {
    //                 let timecontrol_map: HashMap<u32, u32> = string_to_hashmap(parts[1]);
    //                 timecontrol_move_min_key_value_list = convert_to_named_tuple_map(timecontrol_map);
    //             },
    //             "last_move_time" => last_move_time = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid last_move_time"))?,
    //             "player_white" => player_white = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid player_white"))?,
    //             "game_move_number" => game_move_number = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid game_move_number"))?,
    //             _ => {}
    //         }
    //     }
    
    //     Ok(TimedProject {
    //         game_name: game_name.to_string(),
    //         project_start_time_timestamp,
    //         white_time_remaining_sec,
    //         black_time_remaining_sec,
    //         increments_sec_sec_key_value_list,
    //         timecontrol_move_min_key_value_list,
    //         last_move_time,
    //         player_white,
    //         game_move_number,
    //     })
    // }
    

    // fn load_timedata_from_txt(game_name: &str) -> io::Result<TimedProject> {
    //     println!("Starting load_timedata_from_txt()");

    //     // Define the path to read from
    //     let path = format!("games/{}/time_data.txt", game_name);
        
    //     // Initialize variables to hold data read from the file
    //     let mut project_start_time_timestamp: u64 = 0;
    //     let mut white_time_remaining_sec: u32 = 0;
    //     let mut black_time_remaining_sec: u32 = 0;
    //     let mut increments_sec_sec_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
    //     let mut last_move_time: u64 = 0;
    //     let mut player_white: bool = true;
    //     let mut game_move_number: usize = 0;

    //     // Open and read the file line by line
    //     let file = File::open(&path)?;
    //     let reader = BufReader::new(file);

    //     for line in reader.lines() {
    //         let line = line?;
    //         let parts: Vec<&str> = line.split(": ").collect();


    //         // let increments_str = "0,30-300,10-30,5";
    //         let increments_map: HashMap<u32, u32> = string_to_hashmap(&increments_str);
            
    //         // let timecontrol_str = "40,60-100,15";
    //         let timecontrol_map: HashMap<u32, u32> = string_to_hashmap(&timecontrol_str);
            
    //         // let increments_str_converted = hashmap_to_string(&increments_map);
    //         // let timecontrol_str_converted = hashmap_to_string(&timecontrol_map);
            

    //         match parts[0] {
    //             "project_start_time_timestamp" => project_start_time_timestamp = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "white_time_remaining_sec" => white_time_remaining_sec= parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "black_time_remaining_sec" => black_time_remaining_sec= parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,

    //             "increments_sec_sec_key_value_list" => increments_sec_sec_key_value_list = increments_map,
    //             "timecontrol_move_min_key_value_list" => timecontrol_move_min_key_value_list = timecontrol_map,
    

    //             // "increments_sec_sec_key_value_list" => increments_sec_sec_key_value_list= parts[4].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             // "timecontrol_move_min_key_value_list" => timecontrol_move_min_key_value_list= parts[5].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "last_move_time" => last_move_time= parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "player_white" => player_white= parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             "game_move_number" => game_move_number= parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid white_time_remaining_sec"))?,
    //             _ => {}
    //         }
    //     }

    //     Ok(TimedProject {
    //         game_name: game_name.to_string(),
    //         project_start_time_timestamp,
    //         white_time_remaining_sec,
    //         black_time_remaining_sec,
    //         increments_sec_sec_key_value_list,
    //         timecontrol_move_min_key_value_list,
    //         last_move_time,
    //         player_white,
    //         game_move_number,
    //     })
    // }


    // fn process_chess_time_file(game_name: &str) -> String {
    //     println!("starting process_chess_time_file()");


    //     let path = format!("games/{}/time_data.txt", game_name);
    
    //     if !Path::new(&path).exists() {
    //         return "".to_string();
    //     }
    
    //     let file = match File::open(&path) {
    //         Ok(f) => f,
    //         Err(_) => return "".to_string(),
    //     };
    
    //     let mut reader = BufReader::new(file);
    //     let mut time_struct = TimedProject {
    //         game_name: "".to_string(),
    //         project_start_time_timestamp: 0,
    //         white_time_remaining_sec:
    //         black_time_remaining_sec:
    //         increments_sec_sec_key_value_list: vec![],
    //         timecontrol_move_min_key_value_list: vec![],
    //         last_move_time: 0,
    //         player_white: true,
    //         game_move_number: 0,
    //     };
    
    //     // Initialize `new_posix` and `current_increment` to remove "cannot find value" errors
    //     let new_posix: u64 = 0; // Initialize properly
    //     let current_increment: u64 = 0; // Initialize properly
    
    //     let mut this_player_time_spent = new_posix.saturating_sub(time_struct.last_move_time);
    
    //     // ... (rest of your code, make sure to handle errors without unwrap)
    
    //     "Some HTML or response".to_string() // Return value
    // }    
    
    
// use core::hash::Hash;
// // use std::hash::Hash;

// /// Converts a specialized file-string to a HashMap
// pub fn string_to_hashmap<V1, V2>(file_str: &str) -> HashMap<V1, V2>
//     where
//         V1: std::str::FromStr + Hash + Eq,
//         V2: std::str::FromStr,
//         <V1 as std::str::FromStr>::Err: std::fmt::Debug,
//         <V2 as std::str::FromStr>::Err: std::fmt::Debug,
//     {
//         let mut map = HashMap::new();
//         let pairs = file_str.split('-').collect::<Vec<&str>>();
//         for pair in pairs.chunks(2) {
//             if pair.len() == 2 {
//                 if let (Ok(key), Ok(value)) = (pair[0].parse(), pair[1].parse()) {
//                     map.insert(key, value);
//                 }
//             }
//         }
//         map
//     }


// use std::collections::HashMap;

    
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct TimedProject {
    game_name: String,
    project_start_time_timestamp: u64,
    white_time_remaining_sec: u32,
    black_time_remaining_sec: u32,
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)>,
    last_move_time: u64,
    player_white: bool,
    game_move_number: u16,
}

fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Note: These are placeholder variables. You will need to calculate them based on your game logic.
    let moves_to_next_time_control = 0;
    let next_time_control_min = 0;
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Generate the HTML content.
    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div>
                <p>White time remaining: {}</p>
                <p>Black time remaining: {}</p>
                <p>This turn so far: {}</p>
                <p>Total time since start: {}</p>
                <p>Moves to next time control: {}</p>
                <p>Next time control (min): {}</p>
                <p>Current increment: {}</p>
                <p>Next increment at time (sec): {}</p>
                <p>Next increment on move: {}</p>
            </div>
        </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    project.white_time_remaining_sec,
    project.black_time_remaining_sec,
    time_this_turn,
    time_since_start,
    moves_to_next_time_control,
    next_time_control_min,
    current_increment,
    next_increment_time,
    next_increment_move
    );

    html_content
}

fn main() {
    // Example of how to get the current timestamp
    let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    // Dummy TimedProject for demonstration purposes
    let project = TimedProject {
        game_name: "t6".to_string(),
        project_start_time_timestamp: 1695509181,
        white_time_remaining_sec: 7200,
        black_time_remaining_sec: 7200,
        white_increments_sec_sec_key_value_list: HashMap::new(),
        black_increments_sec_sec_key_value_list: HashMap::new(),
        white_timecontrol_move_min_incrsec_key_value_list: HashMap::new(),
        black_timecontrol_move_min_incrsec_key_value_list: HashMap::new(),
        last_move_time: 1695509181,
        player_white: true,
        game_move_number: 0,
    };

    let html_content = generate_html_with_time_data(&project, current_timestamp);
    println!("{}", html_content);
}



use std::collections::HashMap;

struct TimedProject {
    game_name: String,
    project_start_time_timestamp: u64,
    white_time_remaining_sec: u32,
    black_time_remaining_sec: u32,
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    white_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    last_move_time: u64,
    player_white: bool,
    game_move_number: u16,
}

pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;
    
    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Calculate next time control move number and time in minutes
    let (moves_to_next_time_control, next_time_control_min) = if project.player_white {
        if let Some((&move_number, &(minutes, _))) = project.white_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (move_number, minutes)
        } else {
            (0, 0)
        }
    } else {
        if let Some((&move_number, &(minutes, _))) = project.black_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (move_number, minutes)
        } else {
            (0, 0)
        }
    };

    // Calculate current increment and the next increment
    let (current_increment, next_increment_time, next_increment_move) = if project.player_white {
        if let Some((&next_move, &increment)) = project.white_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], next_move, increment)
        } else {
            (0, 0, 0)
        }
    } else {
        if let Some((&next_move, &increment)) = project.black_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (project.black_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], next_move, increment)
        } else {
            (0, 0, 0)
        }
    };

    // TODO: Generate the HTML here based on the calculated variables and the rules you specified.

    "HTML_PLACEHOLDER".to_string()
}



pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {


    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;



    // Note: These are placeholder variables. You will need to calculate them based on your game logic.
    let moves_to_next_time_control = 0;
    let next_time_control_min = 0;
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Generate the HTML content.
    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div>
                <p>White time remaining: {}</p>
                <p>Black time remaining: {}</p>
                <p>This turn so far: {}</p>
                <p>Total time since start: {}</p>
                <p>Moves to next time control: {}</p>
                <p>Next time control (min): {}</p>
                <p>Current increment: {}</p>
                <p>Next increment at time (sec): {}</p>
                <p>Next increment on move: {}</p>
            </div>
        </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    project.white_time_remaining_sec,
    project.black_time_remaining_sec,
    time_this_turn,
    time_since_start,
    moves_to_next_time_control,
    next_time_control_min,
    current_increment,
    next_increment_time,
    next_increment_move
    );

    html_content
}


pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {


    let mut html_output = String::new();

    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Here, you can set the placeholder variables according to your game's specific logic.
    // For example:
    let moves_to_next_time_control = 0;
    let next_time_control_min = 0;
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Generate HTML based on conditions
    html_output.push_str("<div>\n");
    
    // Always show remaining time for both players
    html_output.push_str(&format!("- White Time Remaining: {} sec\n", project.white_time_remaining_sec));
    html_output.push_str(&format!("- Black Time Remaining: {} sec\n", project.black_time_remaining_sec));
    
    // Always show these
    html_output.push_str(&format!("- Time Spent This Turn So Far: {} sec\n", time_this_turn));
    html_output.push_str(&format!("- Total Time Since Start of Game: {} sec\n", time_since_start));
    html_output.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

    // Conditional HTML generation for time controls and increments
    if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Next Time-Control at Move: {}\n", moves_to_next_time_control));
        html_output.push_str(&format!("- Next Time-Control (in minutes): {}\n", next_time_control_min));
    }
    
    if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Current Increment: {}\n", current_increment));
        html_output.push_str(&format!("- Next Increment at time (sec): {}\n", next_increment_time));
        html_output.push_str(&format!("- Next Increment on Move: {}\n", next_increment_move));
    }

    html_output.push_str("</div>\n");
    
    html_output
}


// v3
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    /* 
    html time_bar_html items:
    - White Time Remaining:
    - Black Time Remaining:
    \n
    - Time Spent This Turn so Far:
    - Total Time Since Start of Game:
    - This Game Move: int
    - Next (White/Black) Time-Control at Move: int
    - Next (White/Black) Time-Control (in minutes): 
    - Current (White/Black) Increment:
    - Next (White/Black) Increment at time (sec):
    - Next (White/Black) Increment on Move: int


    making other helper function if needed is fine

    Rules: 
    1. If time controls or incriments are blank, generate no html.
    2. If black and white time contorls or incriments are the same, 
    then just print without 'black or white.'
    If they are different, print separately.
    3. 
    */
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Calculate moves to the next time control and time at the next time control
    let (moves_to_next_time_control, next_time_control_seconds) = if project.player_white {
        find_next_time_control(&project.white_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    } else {
        find_next_time_control(&project.black_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    };

    let next_time_control_min = next_time_control_seconds / 60;

    // Initialize other variables as placeholders for now.
    let current_increment = 0;  // TODO: Calculate current increment
    let next_increment_time = 0;  // TODO: Calculate next increment time
    let next_increment_move = 0;  // TODO: Calculate next increment move

    // Existing HTML generation logic here

    // Generate HTML based on conditions
    html_output.push_str("<div>\n");
    
    // Always show remaining time for both players
    html_output.push_str(&format!("- White Time Remaining: {} sec\n", project.white_time_remaining_sec));
    html_output.push_str(&format!("- Black Time Remaining: {} sec\n", project.black_time_remaining_sec));
    
    // Always show these
    html_output.push_str(&format!("- Time Spent This Turn So Far: {} sec\n", time_this_turn));
    html_output.push_str(&format!("- Total Time Since Start of Game: {} sec\n", time_since_start));
    html_output.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

    // Conditional HTML generation for time controls and increments
    if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Next Time-Control at Move: {}\n", moves_to_next_time_control));
        html_output.push_str(&format!("- Next Time-Control (in minutes): {}\n", next_time_control_min));
    }
    
    if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Current Increment: {}\n", current_increment));
        html_output.push_str(&format!("- Next Increment at time (sec): {}\n", next_increment_time));
        html_output.push_str(&format!("- Next Increment on Move: {}\n", next_increment_move));
    }

    html_output.push_str("</div>\n");
    
    html_output

    // Existing HTML generation logic here
    // Existing HTML generation logic here

    // Generate the HTML content.
    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div>
                <p>White time remaining: {}</p>
                <p>Black time remaining: {}</p>
                <p>This turn so far: {}</p>
                <p>Total time since start: {}</p>
                <p>Moves to next time control: {}</p>
                <p>Next time control (min): {}</p>
                <p>Current increment: {}</p>
                <p>Next increment at time (sec): {}</p>
                <p>Next increment on move: {}</p>
            </div>
        </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    project.white_time_remaining_sec,
    project.black_time_remaining_sec,
    time_this_turn,
    time_since_start,
    moves_to_next_time_control,
    next_time_control_min,
    current_increment,
    next_increment_time,
    next_increment_move
    );


    String::from(html_content, html_output)  
}



// v4
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    let mut html_output = String::new();

    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Find the next time control based on the player's color
    let (moves_to_next_time_control, next_time_control_seconds) = if project.player_white {
        find_next_time_control(&project.white_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    } else {
        find_next_time_control(&project.black_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    };

    let next_time_control_min = next_time_control_seconds / 60;

    // TODO: Calculate current_increment, next_increment_time, next_increment_move based on your game logic
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Start generating HTML content
    html_output.push_str("<div>\n");
    html_output.push_str(&format!("<p>White Time Remaining: {} sec</p>\n", project.white_time_remaining_sec));
    html_output.push_str(&format!("<p>Black Time Remaining: {} sec</p>\n", project.black_time_remaining_sec));
    html_output.push_str(&format!("<p>Time Spent This Turn So Far: {} sec</p>\n", time_this_turn));
    html_output.push_str(&format!("<p>Total Time Since Start of Game: {} sec</p>\n", time_since_start));
    html_output.push_str(&format!("<p>This Game Move: {}</p>\n", project.game_move_number));

    // Conditional generation for time control
    if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
        html_output.push_str(&format!("<p>Next Time-Control at Move: {}</p>\n", moves_to_next_time_control));
        html_output.push_str(&format!("<p>Next Time-Control (in minutes): {}</p>\n", next_time_control_min));
    }

    // Conditional generation for increments
    if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
        html_output.push_str(&format!("<p>Current Increment: {}</p>\n", current_increment));
        html_output.push_str(&format!("<p>Next Increment at Time (sec): {}</p>\n", next_increment_time));
        html_output.push_str(&format!("<p>Next Increment on Move: {}</p>\n", next_increment_move));
    }

    html_output.push_str("</div>\n");

    // Final HTML content
    let html_content = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="Current Game Board" />
        <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <body style="background-color:black;">
        <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
        {}
    </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    html_output
    );

    html_content
}


use std::collections::HashMap;

struct TimedProject {
    game_name: String,
    project_start_time_timestamp: u64,
    white_time_remaining_sec: u32,
    black_time_remaining_sec: u32,
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    white_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    last_move_time: u64,
    player_white: bool,
    game_move_number: u16,
}

pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;
    
    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Calculate next time control move number and time in minutes
    let (moves_to_next_time_control, next_time_control_min) = if project.player_white {
        if let Some((&move_number, &(minutes, _))) = project.white_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (move_number, minutes)
        } else {
            (0, 0)
        }
    } else {
        if let Some((&move_number, &(minutes, _))) = project.black_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (move_number, minutes)
        } else {
            (0, 0)
        }
    };

    // Calculate current increment and the next increment
    let (current_increment, next_increment_time, next_increment_move) = if project.player_white {
        if let Some((&next_move, &increment)) = project.white_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], next_move, increment)
        } else {
            (0, 0, 0)
        }
    } else {
        if let Some((&next_move, &increment)) = project.black_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32) {
            (project.black_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], next_move, increment)
        } else {
            (0, 0, 0)
        }
    };

    // TODO: Generate the HTML here based on the calculated variables and the rules you specified.

    "HTML_PLACEHOLDER".to_string()
}


use std::collections::HashMap;

struct TimedProject {
    game_name: String,
    project_start_time_timestamp: u64,
    white_time_remaining_sec: u32,
    black_time_remaining_sec: u32,
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    white_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    last_move_time: u64,
    player_white: bool,
    game_move_number: u16,
}

pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Initialize the HTML string
    let mut html_string = String::new();

    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;
    
    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Add time information to the HTML string
    html_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
    html_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));

    // Add move number
    html_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

    // Calculate and add next time control and increment details
    let (moves_to_next_time_control, next_time_control_min) = if project.player_white {
        project.white_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (k, v.0)).unwrap_or((0, 0))
    } else {
        project.black_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (k, v.0)).unwrap_or((0, 0))
    };

    let (current_increment, next_increment_time, next_increment_move) = if project.player_white {
        project.white_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v)).unwrap_or((0, 0, 0))
    } else {
        project.black_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (project.black_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v)).unwrap_or((0, 0, 0))
    };

    // Add to HTML string
    html_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
    html_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

    html_string
}

fn main() {
    // Initialize an example TimedProject struct
    let project = TimedProject {
        game_name: "t6".to_string(),
        project_start_time_timestamp: 1695509181,
        white_time_remaining_sec: 7200,
        black_time_remaining_sec: 7200,
        white_increments_sec_sec_key_value_list: HashMap::new(),
        black_increments_sec_sec_key_value_list: HashMap::new(),
        white_timecontrol_move_min_incrsec_key_values_list: [(41, (0, 10))].iter().cloned().collect(),
        black_timecontrol_move_min_incrsec_key_values_list: [(41, (0, 10))].iter().cloned().collect(),
        last_move_time: 0,
        player_white: true,
        game_move_number: 0,
    };

    // Generate the HTML
    let html = generate_html_with_time_data(&project, 1695509182);
    println!("{}", html);
}

/// Generates the HTML time bar.
///
/// This function uses the provided TimedProject instance and current timestamp
/// to generate an HTML time bar as described.
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Initialize the HTML string
    let mut html_string = String::new();
    
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;
    
    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Add time information to the HTML string
    html_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
    html_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));

    // Add move number
    html_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));
    
    // Calculate and add next time control and increment details
    // Logic to determine moves to next time control, next time control in minutes, and increments.
    let (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move) = calculate_time_control_and_increment_details(project);
    
    // Add to HTML string
    html_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
    html_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

    // Final HTML content
    let html_content = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="Current Game Board" />
        <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <body style="background-color:black;">
        <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
        {}
    </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    html_string,
    );

    html_content
}

/// Helper function to calculate time control and increment details.
///
/// This function takes a reference to a TimedProject instance and returns a tuple
/// containing moves to the next time control, next time control in minutes, current increment,
/// next increment time in seconds, and next increment move.
fn calculate_time_control_and_increment_details(project: &TimedProject) -> (u32, u32, u32, u32, u32) {
    let (moves_to_next_time_control, next_time_control_min) = project.white_timecontrol_move_min_incrsec_key_values_list
        .iter()
        .find(|&&(k, _)| k > project.game_move_number as u32)
        .map(|(&k, &v)| (k, v.0))
        .unwrap_or((0, 0));
    
    let (current_increment, next_increment_time, next_increment_move) = project.white_increments_sec_sec_key_value_list
        .iter()
        .find(|&&(k, _)| k > project.game_move_number as u32)
        .map(|(&k, &v)| (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v))
        .unwrap_or((0, 0, 0));
    
    (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move)
}


pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {


    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;



    // Note: These are placeholder variables. You will need to calculate them based on your game logic.
    let moves_to_next_time_control = 0;
    let next_time_control_min = 0;
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Generate the HTML content.
    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div>
                <p>White time remaining: {}</p>
                <p>Black time remaining: {}</p>
                <p>This turn so far: {}</p>
                <p>Total time since start: {}</p>
                <p>Moves to next time control: {}</p>
                <p>Next time control (min): {}</p>
                <p>Current increment: {}</p>
                <p>Next increment at time (sec): {}</p>
                <p>Next increment on move: {}</p>
            </div>
        </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    project.white_time_remaining_sec,
    project.black_time_remaining_sec,
    time_this_turn,
    time_since_start,
    moves_to_next_time_control,
    next_time_control_min,
    current_increment,
    next_increment_time,
    next_increment_move
    );

    html_content
}


pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {


    let mut html_output = String::new();

    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Here, you can set the placeholder variables according to your game's specific logic.
    // For example:
    let moves_to_next_time_control = 0;
    let next_time_control_min = 0;
    let current_increment = 0;
    let next_increment_time = 0;
    let next_increment_move = 0;

    // Generate HTML based on conditions
    html_output.push_str("<div>\n");
    
    // Always show remaining time for both players
    html_output.push_str(&format!("- White Time Remaining: {} sec\n", project.white_time_remaining_sec));
    html_output.push_str(&format!("- Black Time Remaining: {} sec\n", project.black_time_remaining_sec));
    
    // Always show these
    html_output.push_str(&format!("- Time Spent This Turn So Far: {} sec\n", time_this_turn));
    html_output.push_str(&format!("- Total Time Since Start of Game: {} sec\n", time_since_start));
    html_output.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

    // Conditional HTML generation for time controls and increments
    if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Next Time-Control at Move: {}\n", moves_to_next_time_control));
        html_output.push_str(&format!("- Next Time-Control (in minutes): {}\n", next_time_control_min));
    }
    
    if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Current Increment: {}\n", current_increment));
        html_output.push_str(&format!("- Next Increment at time (sec): {}\n", next_increment_time));
        html_output.push_str(&format!("- Next Increment on Move: {}\n", next_increment_move));
    }

    html_output.push_str("</div>\n");
    
    html_output
}


// v3
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
  
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;

    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Calculate moves to the next time control and time at the next time control
    let (moves_to_next_time_control, next_time_control_seconds) = if project.player_white {
        find_next_time_control(&project.white_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    } else {
        find_next_time_control(&project.black_timecontrol_move_min_incrsec_key_value_list, project.game_move_number)
    };

    let next_time_control_min = next_time_control_seconds / 60;

    // Initialize other variables as placeholders for now.
    let current_increment = 0;  // TODO: Calculate current increment
    let next_increment_time = 0;  // TODO: Calculate next increment time
    let next_increment_move = 0;  // TODO: Calculate next increment move

    // Existing HTML generation logic here

    // Generate HTML based on conditions
    html_output.push_str("<div>\n");
    
    // Always show remaining time for both players
    html_output.push_str(&format!("- White Time Remaining: {} sec\n", project.white_time_remaining_sec));
    html_output.push_str(&format!("- Black Time Remaining: {} sec\n", project.black_time_remaining_sec));
    
    // Always show these
    html_output.push_str(&format!("- Time Spent This Turn So Far: {} sec\n", time_this_turn));
    html_output.push_str(&format!("- Total Time Since Start of Game: {} sec\n", time_since_start));
    html_output.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

    // Conditional HTML generation for time controls and increments
    if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Next Time-Control at Move: {}\n", moves_to_next_time_control));
        html_output.push_str(&format!("- Next Time-Control (in minutes): {}\n", next_time_control_min));
    }
    
    if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
        html_output.push_str(&format!("- Current Increment: {}\n", current_increment));
        html_output.push_str(&format!("- Next Increment at time (sec): {}\n", next_increment_time));
        html_output.push_str(&format!("- Next Increment on Move: {}\n", next_increment_move));
    }

    html_output.push_str("</div>\n");
    
    html_output

    // Existing HTML generation logic here
    // Existing HTML generation logic here

    // Generate the HTML content.
    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div>
                <p>White time remaining: {}</p>
                <p>Black time remaining: {}</p>
                <p>This turn so far: {}</p>
                <p>Total time since start: {}</p>
                <p>Moves to next time control: {}</p>
                <p>Next time control (min): {}</p>
                <p>Current increment: {}</p>
                <p>Next increment at time (sec): {}</p>
                <p>Next increment on move: {}</p>
            </div>
        </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    project.white_time_remaining_sec,
    project.black_time_remaining_sec,
    time_this_turn,
    time_since_start,
    moves_to_next_time_control,
    next_time_control_min,
    current_increment,
    next_increment_time,
    next_increment_move
    );


    String::from(html_content, html_output)  
}



// v4
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {

// Initialize the HTML string
let mut html_string = String::new();

// Calculate the time since the start of the game.
let time_since_start = current_timestamp - project.project_start_time_timestamp;

// Calculate the time used so far in this turn.
let time_this_turn = current_timestamp - project.last_move_time;

// Add time information to the HTML string
html_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
html_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));

// Add move number
html_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

// Calculate and add next time control and increment details
let (moves_to_next_time_control, next_time_control_min) = if project.player_white {
    project.white_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (k, v.0)).unwrap_or((0, 0))
} else {
    project.black_timecontrol_move_min_incrsec_key_values_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (k, v.0)).unwrap_or((0, 0))
};

let (current_increment, next_increment_time, next_increment_move) = if project.player_white {
    project.white_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v)).unwrap_or((0, 0, 0))
} else {
    project.black_increments_sec_sec_key_value_list.iter().find(|&&(k, _)| k > project.game_move_number as u32).map(|(&k, &v)| (project.black_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v)).unwrap_or((0, 0, 0))
};

// Add to HTML string
html_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
html_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

// Start generating HTML content
html_string.push_str("<div>\n");
html_string.push_str(&format!("<p>White Time Remaining: {} sec</p>\n", project.white_time_remaining_sec));
html_string.push_str(&format!("<p>Black Time Remaining: {} sec</p>\n", project.black_time_remaining_sec));
html_string.push_str(&format!("<p>Time Spent This Turn So Far: {} sec</p>\n", time_this_turn));
html_string.push_str(&format!("<p>Total Time Since Start of Game: {} sec</p>\n", time_since_start));
html_string.push_str(&format!("<p>This Game Move: {}</p>\n", project.game_move_number));

// Conditional generation for time control
if !project.white_timecontrol_move_min_incrsec_key_value_list.is_empty() || !project.black_timecontrol_move_min_incrsec_key_value_list.is_empty() {
    html_string.push_str(&format!("<p>Next Time-Control at Move: {}</p>\n", moves_to_next_time_control));
    html_string.push_str(&format!("<p>Next Time-Control (in minutes): {}</p>\n", next_time_control_min));
}

// Conditional generation for increments
if !project.white_increments_sec_sec_key_value_list.is_empty() || !project.black_increments_sec_sec_key_value_list.is_empty() {
    html_string.push_str(&format!("<p>Current Increment: {}</p>\n", current_increment));
    html_string.push_str(&format!("<p>Next Increment at Time (sec): {}</p>\n", next_increment_time));
    html_string.push_str(&format!("<p>Next Increment on Move: {}</p>\n", next_increment_move));
}

html_string.push_str("</div>\n");

// Final HTML content
let html_content = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
</head>
<body style="background-color:black;">
    <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
    {}
</body>
</html>
"#,
project.game_name,
project.game_name,
html_string,
);

html_content
}


// Wrapper for use-case 1: Create or update struct and make HTML
pub fn wrapper_move_update_and_make_html(game_name: &str, move_data: &str) -> io::Result<String> {
    let mut project = Self::load_timedata_from_txt(game_name)?;

    // Update the struct using the update_timedata_after_move function
    project.update_timedata_after_move(move_data);
    
    // Generate the HTML content using the updated struct
    Ok(Self::generate_html_with_time_data(&project, timestamp_64()))
}


// Wrapper for use-case 2: Load text file and make HTML
pub fn wrapper_no_move_load_and_make_html(game_name: &str) -> io::Result<String> {
    let current_timestamp = timestamp_64();

    // Load the TimedProject struct from the text file
    let project = Self::load_timedata_from_txt(game_name)?;

    // Generate the HTML content using the loaded struct
    Ok(Self::generate_html_with_time_data(&project, current_timestamp))

}

let response = match handle_chess_move(game_name.clone(), move_data) {
    Ok(_) => {
        // Handle the Result returned by the function and propagate any errors using the ? operator
        let html_content = wrapper_move_update_and_make_html(&game_name, &move_data)?;
        
        match Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]) {
            Ok(header) => Response::from_string(html_content).with_header(header).with_status_code(200),
            Err(_) => Response::from_string("Failed to create header").with_status_code(500),
        }
    },
    Err(e) => {
        eprintln!("Failed to handle move: {}", e);
        Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
    }
};


use std::time::{SystemTime, UNIX_EPOCH};

fn posix_to_readable_datetime(posix_time: u64) -> String {
    let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(posix_time);
    match time.duration_since(UNIX_EPOCH) {
        Ok(system_time) => {
            let since_the_epoch = system_time.as_secs();
            let secs = since_the_epoch % 60;
            let minutes = (since_the_epoch / 60) % 60;
            let hours = (since_the_epoch / 3600) % 24;
            let days_since_epoch = since_the_epoch / 86400;
            // 1970-01-01 was a Thursday
            let day_of_week = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][(days_since_epoch % 7) as usize];
            format!("{} {:02}:{:02}:{:02}", day_of_week, hours, minutes, secs)
        },
        Err(e) => {
            eprintln!("Error calculating time: {}", e);
            String::from("Invalid time")
        }
    }
}

fn seconds_to_hms(seconds: u64) -> String {
    let hours = seconds / 3600;
    let remainder = seconds % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}


/// Generates the HTML time bar.
///
/// This function uses the provided TimedProject instance and current timestamp
/// to generate an HTML time bar as described.
pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
    // Initialize the HTML string
    let mut html_string = String::new();
    
    // Calculate the time since the start of the game.
    let time_since_start = current_timestamp - project.project_start_time_timestamp;
    
    // Calculate the time used so far in this turn.
    let time_this_turn = current_timestamp - project.last_move_time;

    // Add time information to the HTML string
    html_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
    html_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));

    // Add move number
    html_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));
    
    // Calculate and add next time control and increment details
    // Logic to determine moves to next time control, next time control in minutes, and increments.
    let (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move) = calculate_time_control_and_increment_details(project);
    
    // Add to HTML string
    html_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
    html_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

    // Final HTML content
    let html_content = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="Current Game Board" />
        <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <body style="background-color:black;">
        <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
        {}
    </body>
    </html>
    "#,
    project.game_name,
    project.game_name,
    html_string,
    );

    html_content
}

/// Helper function to calculate time control and increment details.
///
/// This function takes a reference to a TimedProject instance and returns a tuple
/// containing moves to the next time control, next time control in minutes, current increment,
/// next increment time in seconds, and next increment move.
fn calculate_time_control_and_increment_details(project: &TimedProject) -> (u32, u32, u32, u32, u32) {
    let (moves_to_next_time_control, next_time_control_min) = project.white_timecontrol_move_min_incrsec_key_values_list
        .iter()
        .find(|&&(k, _)| k > project.game_move_number as u32)
        .map(|(&k, &v)| (k, v.0))
        .unwrap_or((0, 0));
    
    let (current_increment, next_increment_time, next_increment_move) = project.white_increments_sec_sec_key_value_list
        .iter()
        .find(|&&(k, _)| k > project.game_move_number as u32)
        .map(|(&k, &v)| (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v))
        .unwrap_or((0, 0, 0));
    
    (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move)
}

<!DOCTYPE html>
<html>
<head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="/metatag_timetest1.png" />
    <!-- Styling for text inputs -->
    <style>
        input[type=text] {
            color: gray;
            font-size: 18px;
        }
    </style>
</head>
<body style="background-color:black;">
    <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
    <div style="color: gray; font-size: 42px;">
        <br> White Time Remaining: <span id="white_time_remaining">7200</span></br>
        <br> Black Time Remaining: <span id="black_time_remaining">7200</span></br>
        <br> Time Spent This Turn So Far: <span id="time_spent_this_turn">0</span></br>
        <br> Total Time Since Start of Game: <span id="total_time_since_start">1695595577</span></br>
        <br> This Game Move: <span id="this_game_move">0</span></br>
        <br> Next Time-Control at Move: <span id="next_time_control_move">0</span></br>
        <br> Next Time-Control (in minutes): <span id="next_time_control_min">0</span></br>
        <br> Current Increment: <span id="current_increment">0</span></br>
        <br> Next Increment at time (sec): <span id="next_increment_time">0</span></br>
        <br> Next Increment on Move: <span id="next_increment_move">0</span></br>
    </div>
</body>
</html>


// use std::time::{SystemTime, UNIX_EPOCH};
fn posix_to_readable_datetime(posix_time: u64) -> String {
    let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(posix_time);
    match time.duration_since(UNIX_EPOCH) {
        Ok(system_time) => {
            let since_the_epoch = system_time.as_secs();
            let secs = since_the_epoch % 60;
            let minutes = (since_the_epoch / 60) % 60;
            let hours = (since_the_epoch / 3600) % 24;
            let days_since_epoch = since_the_epoch / 86400;
            // 1970-01-01 was a Thursday
            let day_of_week = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][(days_since_epoch % 7) as usize];
            format!("{} {:02}:{:02}:{:02}", day_of_week, hours, minutes, secs)
        },
        Err(e) => {
            eprintln!("Error calculating time: {}", e);
            String::from("Invalid time")
        }
    }
}

fn seconds_to_hms(seconds: u64) -> String {
    let hours = seconds / 3600;
    let remainder = seconds % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// calculate time control and increment details.
/// This function takes a reference to a TimedProject instance and returns a tuple
/// containing moves to the next time control, next time control in minutes, current increment,
/// next increment time in seconds, and next increment move.
fn calculate_time_control_and_increment_details(project: &TimedProject) -> (u32, u32, u32, u32, u32) {
    // Print for debugging: display current game move number
    println!("Current game move number: {}", project.game_move_number);
    
    let (moves_to_next_time_control, next_time_control_min) = project.white_timecontrol_move_min_incrsec_key_values_list
        .iter()
        .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
        .map(|(&k, &v)| {
            println!("Found next time control move: {} min: {}", k, v.0);  // Print for debugging
            (k, v.0)
        })
        .unwrap_or_else(|| {
            println!("No next time control move found.");  // Print for debugging
            (0, 0)
        });

    let (current_increment, next_increment_time, next_increment_move) = project.white_increments_sec_sec_key_value_list
        .iter()
        .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
        .map(|(&k, &v)| {
            println!("Found next increment: {} time: {} move: {}", project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v);  // Print for debugging
            (project.white_increments_sec_sec_key_value_list[&(project.game_move_number as u32)], k, v)
        })
        .unwrap_or_else(|| {
            println!("No next increment found.");  // Print for debugging
            (0, 0, 0)
        });
    
    // Print for debugging: display all calculated values
    println!(
        "Calculated details: moves_to_next_time_control: {}, next_time_control_min: {}, current_increment: {}, next_increment_time: {}, next_increment_move: {}",
        moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move
    );

    (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move)
}


  pub fn generate_html_with_time_data(project: &TimedProject, current_timestamp: u64) -> String {
          /* 
        html time_bar_html items:
        - White Time Remaining:
        - Black Time Remaining:
        \n
        - Time Spent This Turn so Far:
        - Total Time Since Start of Game:
        - This Game Move: int
        - Next (White/Black) Time-Control at Move: int
        - Next (White/Black) Time-Control (in minutes): 
        - Current (White/Black) Increment:
        - Next (White/Black) Increment at time (sec):
        - Next (White/Black) Increment on Move: int

        making other helper function if needed is fine

        Rules: 
        1. If time controls or incriments are blank, generate no html.
        2. If black and white time contorls or incriments are the same, 
        then just print without 'black or white.'
        If they are different, print separately.
        3. 

        example of timedata.txt
        game_name: t2
        project_start_time_timestamp: 1695688402
        white_time_remaining_sec: 7200
        black_time_remaining_sec: 7200
        white_time_timecontrolmin_incrsec_key_values_list: {}
        black_time_timecontrolmin_incrsec_key_values_list: {}
        white_move_timecontrolmin_incrsec_key_values_list: {41: (0, 10)}
        black_move_timecontrolmin_incrsec_key_values_list: {41: (0, 10)}
        previous_move_timestamp: 0
        player_white: true
        game_move_number: 0
        */
        println!("=== Start generate_html_with_time_data ===");

        
        // Initialize the HTML string
        let mut html_timedata_string = String::new();
        
        // Calculate the time since the start of the game.
        let time_since_start = current_timestamp - project.project_start_time_timestamp;
        
        // Calculate the time used so far in this turn.
        let time_this_turn = current_timestamp - project.previous_move_timestamp;


        // Print statements for debugging
        println!("Current timestamp: {}", current_timestamp);
        println!("Time since start: {}", time_since_start);
        println!("Time this turn: {}", time_this_turn);
        println!("White time remaining: {}", project.white_time_remaining_sec);
        println!("Black time remaining: {}", project.black_time_remaining_sec);
        println!("player_white: {}", project.player_white);
        println!("player_white: {}", project.player_white);
        println!("project_start_time_timestamp: {}", project.project_start_time_timestamp);
        println!("white_time_timecontrolmin_incrsec_key_values_list: {:#?}", project.white_time_timecontrolmin_incrsec_key_values_list);
        println!("black_time_timecontrolmin_incrsec_key_values_list: {:#?}", project.black_time_timecontrolmin_incrsec_key_values_list);
        println!("white_move_timecontrolmin_incrsec_key_values_list: {:#?}", project.white_move_timecontrolmin_incrsec_key_values_list);
        println!("black_move_timecontrolmin_incrsec_key_values_list: {:#?}", project.black_move_timecontrolmin_incrsec_key_values_list);
        println!("current_move_timestamp: {}", project.current_move_timestamp);       
        println!("previous_move_timestamp: {}", project.previous_move_timestamp);       
        println!("player_white: {}", project.player_white);
        println!("Current game move number: {}", project.game_move_number);


        // // Include time increments for White and Black if available.
        // // Loop through white_move_timecontrolmin_incrsec_key_values_list to dynamically include information
        // for (move_num, (min, sec)) in &project.white_move_timecontrolmin_incrsec_key_values_list {
        //     html_timedata_string.push_str(&format!(" White Time Increment starts on move {}: {} min {} sec\n", move_num, min, sec));
        // }
        
        // // Loop through black_move_timecontrolmin_incrsec_key_values_list to dynamically include information
        // for (move_num, (min, sec)) in &project.black_move_timecontrolmin_incrsec_key_values_list {
        //     html_timedata_string.push_str(&format!(" Black Time Increment starts on move {}: {} min {} sec\n", move_num, min, sec));
        // }

        // y0ur m0ve
        let next_move = if project.player_white {
            "White"
        } else {
            "Black"
        };
        html_timedata_string.push_str(&format!("<li>Next Move: {}</li>", next_move));
                
                
        // // Add time information to the HTML string
        // html_timedata_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
        // html_timedata_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));

        // // Add move number
        // html_timedata_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));
        
        // // Calculate and add next time control and increment details
        // // Logic to determine moves to next time control, next time control in minutes, and increments.
        // let (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move) = calculate_time_control_and_increment_details(project);
        
        // // Add to HTML string
        // html_timedata_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
        // html_timedata_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));


        // Format time using helper functions
        let time_since_start_str = posix_to_readable_datetime(time_since_start);
        let time_this_turn_str = seconds_to_hms(time_this_turn);
        let white_time_str = seconds_to_hms(project.white_time_remaining_sec.into());  // <-- Modification here
        let black_time_str = seconds_to_hms(project.black_time_remaining_sec.into());  // <-- And here, if black_time_remaining_sec is also u32
        
        
        // html_timedata_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", white_time_str, black_time_str));
        // html_timedata_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn_str, time_since_start_str));

        // html_timedata_string.push_str(&format!("- This Game Move: {}\n", project.game_move_number));

        // let (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move) = calculate_time_control_and_increment_details(project);

        // html_timedata_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
        // html_timedata_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

        // call helper function to process time controll data

        html_timedata_string.push_str(&Self::generate_html_timedata(project));
 

        html_timedata_string.push_str(&format!("<li>White Time Remaining: {}</li><li>Black Time Remaining: {}</li>", white_time_str, black_time_str));
        html_timedata_string.push_str(&format!("<li>This Game Move: {}</li>", project.game_move_number));
        
        // TODO this needs to be updated for black and white separate settings
        let (
            white_moves_to_next_time_control, 
            black_moves_to_next_time_control,
    
            white_next_time_control_min, 
            black_next_time_control_min, 
    
            white_current_increment, 
            black_current_increment, 
    
            white_next_increment_time, 
            black_next_increment_time, 
            
            white_next_increment_move,
            black_next_increment_move,

        ) = calculate_time_control_and_increment_details(project);


        //#############





        // Conditionally append time control and increment details
        if white_moves_to_next_time_control > 0 || white_next_time_control_min > 0 {
            html_timedata_string.push_str(&format!("<li>White Next Time-Control at Move: {}</li><li>White Next Time-Control (in minutes): {}</li>", white_moves_to_next_time_control, white_next_time_control_min));
        }
        if black_moves_to_next_time_control > 0 || black_next_time_control_min > 0 {
            html_timedata_string.push_str(&format!("<li>Black Next Time-Control at Move: {}</li><li>Black Next Time-Control (in minutes): {}</li>", black_moves_to_next_time_control, black_next_time_control_min));
        }

        if white_current_increment > 0 {
            html_timedata_string.push_str(&format!("<li>White Current Increment: {}</li><li>White Next Increment at time (sec): {}</li><li>White Next Increment on Move: {}</li>", white_current_increment, white_next_increment_time, white_next_increment_move));
        }    
        if black_current_increment > 0 {
            html_timedata_string.push_str(&format!("<li>Black Current Increment: {}</li><li>Black Next Increment at time (sec): {}</li><li>Black Next Increment on Move: {}</li>", black_current_increment, black_next_increment_time, black_next_increment_move));
        }    


        // Include time increments for White and Black if available.
        // Loop through white_move_timecontrolmin_incrsec_key_values_list to dynamically include information
        for (move_num, (min, sec)) in &project.white_move_timecontrolmin_incrsec_key_values_list {
            html_timedata_string.push_str(&format!("<li>White Time Increment starts on move {}: adding {} sec per move.</li>", move_num, sec));
            html_timedata_string.push_str(&format!("<li>White Time Control starts on move {}: adding {} min.</li>", move_num, min));
        }

        // Loop through black_move_timecontrolmin_incrsec_key_values_list to dynamically include information
        for (move_num, (min, sec)) in &project.black_move_timecontrolmin_incrsec_key_values_list {
            html_timedata_string.push_str(&format!("<li>Black Time Increment starts on move {}: adding {} sec per move.</li>", move_num, sec));
            html_timedata_string.push_str(&format!("<li>Black Time Control starts on move {}: adding {} min.</li>", move_num, min));
        }


        //#############


        html_timedata_string.push_str(&format!("<li>Time Spent This Turn so Far: {}</li><li>Total Time Since Start of Game: {}</li>", time_this_turn_str, time_since_start_str));
        
        // Final HTML content
        let html_content = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta property="og:title" content="Current Game Board" />
            <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />

        </head>
        <body style="background-color:black;">
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <div style="color: gray; font-size: 42px;"> 
            <ul style="list-style-type: none;">
            {}
            </ul>
        </body>
        </html>
        "#,
        project.game_name,
        project.game_name,
        html_timedata_string,
        );

        html_content
    }

// // Wrapper for use-case 1: Create or update struct and make HTML
// pub fn wrapper_move_update_and_make_html(game_name: &str, move_data: &str) -> io::Result<String> {
//     let mut project = Self::load_timedata_from_txt(game_name)?;

//     // Update the struct using the update_timedata_before_move function
//     project.update_timedata_before_move(move_data);
    
//     // Generate the HTML content using the updated struct
//     Ok(Self::generate_html_with_time_data(&project, timestamp_64()))
// }


// // Wrapper for use-case 2: Load text file and make HTML
// pub fn wrapper_no_move_load_and_make_html(game_name: &str) -> io::Result<String> {
//     let current_timestamp = timestamp_64();

//     // Load the TimedProject struct from the text file
//     let project = Self::load_timedata_from_txt(game_name)?;

//     // Generate the HTML content using the loaded struct
//     Ok(Self::generate_html_with_time_data(&project, current_timestamp))
// }




// /// Converts a specialized file-string to a HashMap<u32, u32>
// pub fn string_to_hashmap_timedata(file_str: &str) -> HashMap<u32, u32> {
//     let mut map = HashMap::new();
//     let pairs = file_str.split('-').collect::<Vec<&str>>();
    
//     for pair in pairs.chunks(2) {
//         if pair.len() == 2 {
//             if let (Ok(key), Ok(value)) = (pair[0].parse::<u32>(), pair[1].parse::<u32>()) {
//                 map.insert(key, value);
//             }
//         }
//     }
//     map
// }



// /// Converts a HashMap to a specialized file-string
// pub fn hashmap_to_string_timedata<V1, V2>(map: &HashMap<V1, V2>) -> String
//     where
//         V1: std::fmt::Display,
//         V2: std::fmt::Display,
//     {
//         let entries: Vec<String> = map
//             .iter()
//             .map(|(key, value)| format!("{},{}", key, value))
//             .collect();
//         entries.join("-")
//     }


// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     let mut map = HashMap::new();
//     for item in input.split(',') {
//         let parts: Vec<&str> = item.split(' ').collect();
//         if parts.len() == 3 {
//             if let (Ok(key), Ok(value1), Ok(value2)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
//                 map.insert(key, (value1, value2));
//             }
//         }
//     }
//     map
// }


// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     let mut map = HashMap::new();
//     for item in input.split(',') {
//         let parts: Vec<&str> = item.split(' ').collect();
//         if parts.len() == 3 {
//             if let (Ok(key), Ok(value1), Ok(value2)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
//                 map.insert(key, (value1, value2));
//             }
//         }
//     }
//     map
// }



// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Raw Input: {}", input);

//     let mut map = HashMap::new();

//     // Remove spaces, curly braces, and parentheses
//     let stripped_input = input.replace(" ", "").replace("{", "").replace("}", "");

    
//     // Split by `),` and then add `)` back to each segment (except the last one)
//     let segments: Vec<String> = stripped_input.split("),").enumerate().map(|(i, seg)| {
//         if i == stripped_input.split("),").count() - 1 {
//             seg.to_string()
//         } else {
//             format!("{})", seg)
//         }
//     }).collect();

//     for pair in &segments {
//         let parts: Vec<&str> = pair.split(":").collect();
//         if parts.len() != 2 {
//             println!("   Error: Malformed input in pair.");
//             continue;  // Move to the next pair
//         }

//         let key: u32 = match parts[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Error: Key parsing error");
//                 continue;  // Move to the next pair
//             },
//         };

//         let values: Vec<u32> = parts[1].trim_start_matches("(").split(',').filter_map(|x| x.trim().parse().ok()).collect();
//         if values.len() != 2 {
//             println!("   Error: Malformed tuple in pair.");
//             continue;  // Move to the next pair
//         }

//         map.insert(key, (values[0], values[1]));
//     }

//     println!("Processed map: {:?}", map);
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }



// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Raw Input: {}", input);

//     let mut map = HashMap::new();

//     // Remove spaces, curly braces, and parentheses
//     let stripped_input = input.replace(" ", "").replace("{", "").replace("}", "");

//     // Split by `),` to get individual key-tuple-value pairs
//     for pair in stripped_input.split("),") {
//         let parts: Vec<&str> = pair.split(":").collect();
//         if parts.len() != 2 {
//             println!("   Error: Malformed input in pair.");
//             continue;  // Move to the next pair
//         }

//         let key: u32 = match parts[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Error: Key parsing error");
//                 continue;  // Move to the next pair
//             },
//         };

//         let values: Vec<u32> = parts[1].trim_start_matches("(").split(',').filter_map(|x| x.trim().parse().ok()).collect();
//         if values.len() != 2 {
//             println!("   Error: Malformed tuple in pair.");
//             continue;  // Move to the next pair
//         }

//         map.insert(key, (values[0], values[1]));
//     }

//     println!("Processed map: {:?}", map);
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }

// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata() ===");
//     println!("Raw Input: {}", input);

//     let mut map = HashMap::new();
//     let stripped_input = input.replace("{", "").replace("}", "").replace("(", "").replace(")", "").replace(" ", "");
    
//     // Split by comma to separate each key-value pair
//     for pair in stripped_input.split(",") {
//         let parts: Vec<&str> = pair.split(":").collect();
//         if parts.len() != 2 {
//             println!("   Error: Malformed input in pair.");
//             continue;  // Move to the next pair
//         }

//         let key: u32 = match parts[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Error: Key parsing error");
//                 continue;  // Move to the next pair
//             },
//         };

//         let values: Vec<u32> = parts[1].splitn(2, ',').filter_map(|x| x.trim().parse().ok()).collect();
//         if values.len() != 2 {
//             println!("   Error: Malformed tuple in pair.");
//             continue;  // Move to the next pair
//         }

//         map.insert(key, (values[0], values[1]));
//     }

//     println!("Processed map: {:?}", map);
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }



// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata() ===");
//     println!("Raw Input: {}", input);

//     let mut map = HashMap::new();
//     let data_str = input.trim_matches(|c| c == '{' || c == '}').trim();

//     // Split by comma to separate each key-value pair
//     for pair in data_str.split(",") {
//         let parts: Vec<&str> = pair.split(": ").collect();
//         if parts.len() != 2 {
//             println!("   Error: Malformed input in pair.");
//             continue;  // Move to the next pair
//         }

//         let key: u32 = match parts[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Error: Key parsing error in pair.");
//                 continue;  // Move to the next pair
//             },
//         };

//         let tuple_str = parts[1].trim().trim_matches(|c| c == '(' || c == ')');
//         let values: Vec<u32> = tuple_str.split(',').filter_map(|x| x.trim().parse().ok()).collect();

//         if values.len() != 2 {
//             println!("   Error: Malformed tuple in pair.");
//             continue;  // Move to the next pair
//         }

//         map.insert(key, (values[0], values[1]));
//     }

//     println!("Processed map: {:?}", map);
//     println!("--- End string_to_tuple_hashmap_timedata() ---");
//     map
// }


// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Raw Input: {}", input);

//     let mut map = HashMap::new();
//     let data_str = input.trim_matches(|c| c == '{' || c == '}').trim();

//     let parts: Vec<&str> = data_str.split(": ").collect();
//     if parts.len() != 2 {
//         println!("   Error: Malformed input.");
//         return map;
//     }

//     let key: u32 = match parts[0].trim().parse() {
//         Ok(k) => k,
//         Err(_) => {
//             println!("   Error: Key parsing error");
//             return map;
//         },
//     };

//     let tuple_str = parts[1].trim().trim_matches(|c| c == '(' || c == ')');
//     let values: Vec<u32> = tuple_str.split(',').filter_map(|x| x.trim().parse().ok()).collect();

//     if values.len() != 2 {
//         println!("   Error: Malformed tuple");
//         return map;
//     }

//     map.insert(key, (values[0], values[1]));

//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }



// // use std::collections::HashMap;

// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Raw Input: {}", input);

//     // Extract the substring between `{}` brackets.
//     let start_index = input.find('{');
//     let end_index = input.rfind('}');
    
//     if start_index.is_none() || end_index.is_none() {
//         println!("   Skipping, no valid input format found");
//         return HashMap::new();
//     }

//     let substring = &input[start_index.unwrap() + 1..end_index.unwrap()];

//     let mut map = HashMap::new();

//     let items: Vec<&str> = substring.split(',').collect();
//     for item in items {
//         println!("-- Processing item: {}", item);

//         let key_value: Vec<&str> = item.splitn(2, ":").collect();
//         if key_value.len() != 2 {
//             println!("   Skipping, key_value.len() != 2");
//             continue;
//         }

//         let key: u32 = match key_value[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Skipping, key parsing error");
//                 continue;
//             },
//         };

//         // Clean the tuple and split by ','
//         let clean_tuple = key_value[1].trim().trim_start_matches('(').trim_end_matches(')');
//         let values: Vec<u32> = clean_tuple
//             .split(',')
//             .filter_map(|x| x.trim().parse().ok())
//             .collect();

//         if values.len() != 2 {
//             println!("   Skipping, values.len() != 2");
//             continue;
//         }

//         map.insert(key, (values[0], values[1]));
//     }
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }


// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Input to string_to_tuple_hashmap_timedata: {}", input);

//     let mut map = HashMap::new();
//     let clean_input = input.trim_start_matches('{').trim_end_matches('}');

//     // Splitting by ',' but making sure it's outside of the tuple
//     let items = clean_input.splitn(2, |c: char| c == ',' && !clean_input.chars().take_while(|&x| x != c).filter(|&x| x == '(').count() % 2 == 1);

//     for item in items {
//         println!("-- Processing item: {}", item);

//         let key_value: Vec<&str> = item.splitn(2, ":").collect();
//         if key_value.len() != 2 {
//             println!("   Skipping, key_value.len() != 2");
//             continue;
//         }

//         let key: u32 = match key_value[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => {
//                 println!("   Skipping, key parsing error");
//                 continue;
//             },
//         };

//         // Clean the tuple and split by ','
//         let clean_tuple = key_value[1].trim().trim_start_matches('(').trim_end_matches(')');
//         let values: Vec<u32> = clean_tuple
//             .split(',')
//             .filter_map(|x| x.trim().parse().ok())
//             .collect();

//         if values.len() != 2 {
//             println!("   Skipping, values.len() != 2");
//             continue;
//         }

//         map.insert(key, (values[0], values[1]));
//     }
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }



// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Input to string_to_tuple_hashmap_timedata: {}", input);

//     let mut map = HashMap::new();

//     // Split the input into lines and process each line
//     for line in input.lines() {
//         // Check for lines that are similar to: {41: (0, 10)}
//         if line.contains("{") && line.contains("}") {
//             // Extracting the content between '{' and '}'
//             let clean_input = line.split('{').nth(1).unwrap_or_default().split('}').next().unwrap_or_default();

//             for item in clean_input.split(',') {
//                 println!("-- Processing item: {}", item);

//                 let key_value: Vec<&str> = item.split(":").collect();
//                 if key_value.len() != 2 {
//                     println!("   Skipping, key_value.len() != 2");
//                     continue;
//                 }

//                 let key: u32 = match key_value[0].trim().parse() {
//                     Ok(k) => k,
//                     Err(_) => {
//                         println!("   Skipping, failed to parse key");
//                         continue;
//                     },
//                 };

//                 // Extracting the tuple values between '(' and ')'
//                 let clean_tuple = key_value[1].trim().trim_start_matches('(').trim_end_matches(')');
//                 let values: Vec<u32> = clean_tuple
//                     .split(',')
//                     .filter_map(|x| x.trim().parse().ok())
//                     .collect();

//                 if values.len() != 2 {
//                     println!("   Skipping, values.len() != 2");
//                     continue;
//                 }

//                 map.insert(key, (values[0], values[1]));
//             }
//         }
//     }

//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }




// pub fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     println!("=== string_to_tuple_hashmap_timedata ===");
//     println!("Input to string_to_tuple_hashmap_timedata: {}", input);

//     let mut map = HashMap::new();
//     // Assuming that the input string format is {41: (0, 10)}, stripping '{' and '}'.
//     let clean_input = input.trim_start_matches('{').trim_end_matches('}');
//     for item in clean_input.split(',') {
//         println!("-- Processing item: {}", item);

//         let key_value: Vec<&str> = item.split(":").collect();
//         if key_value.len() != 2 {
//             println!("   Skipping, key_value.len() != 2");
//             continue;
//         }

//         let key: u32 = match key_value[0].trim().parse() {
//             Ok(k) => k,
//             Err(_) => continue,
//         };

//         // Assuming that the tuple format is (0, 10), stripping '(' and ')'.
//         let clean_tuple = key_value[1].trim().trim_start_matches('(').trim_end_matches(')');
//         let values: Vec<u32> = clean_tuple
//             .split(',')
//             .filter_map(|x| x.trim().parse().ok())
//             .collect();

//         if values.len() != 2 {
//             println!("   Skipping, values.len() != 2");
//             continue;
//         }

//         map.insert(key, (values[0], values[1]));
//     }
//     println!("=== End string_to_tuple_hashmap_timedata ===");
//     map
// }



// fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     let mut result = HashMap::new();
//     println!("\n=== string_to_tuple_hashmap_timedata ===");
//     println!("Input to string_to_tuple_hashmap_timedata: {}", input);

//     let trimmed = input.trim_matches(|c| c == '{' || c == '}');
//     for item in trimmed.split(", ") {
//         println!("-- Processing item: {}", item);
//         let parts: Vec<&str> = item.split(": ").collect();
//         if parts.len() != 2 {
//             println!("   Skipping, parts.len() != 2");
//             continue;
//         }

//         let tuple_str = parts[1].trim_matches(|c| c == '(' || c == ')');
//         let tuple_parts: Vec<&str> = tuple_str.split(", ").collect();
//         if tuple_parts.len() != 2 {
//             println!("   Skipping, tuple_parts.len() != 2");
//             continue;
//         }

//         if let (Ok(key), Ok(value1), Ok(value2)) = (
//             parts[0].parse::<u32>(),
//             tuple_parts[0].parse::<u32>(),
//             tuple_parts[1].parse::<u32>(),
//         ) {
//             println!("   Parsed values: key = {}, value1 = {}, value2 = {}", key, value1, value2);
//             result.insert(key, (value1, value2));
//         }
//     }
//     println!("=== End string_to_tuple_hashmap_timedata ===\n");
//     result
// }


// fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     let mut result = HashMap::new();
//     println!("Input to string_to_tuple_hashmap_timedata: {}", input);

//     let trimmed = input.trim_matches(|c| c == '{' || c == '}');
//     for item in trimmed.split(", ") {
//         println!("Processing item: {}", item);
//         let parts: Vec<&str> = item.split(": ").collect();
//         if parts.len() != 2 {
//             continue;
//         }

//         let tuple_str = parts[1].trim_matches(|c| c == '(' || c == ')');
//         let tuple_parts: Vec<&str> = tuple_str.split(", ").collect();
//         if tuple_parts.len() != 2 {
//             continue;
//         }

//         if let (Ok(key), Ok(value1), Ok(value2)) = (
//             parts[0].parse::<u32>(),
//             tuple_parts[0].parse::<u32>(),
//             tuple_parts[1].parse::<u32>(),
//         ) {
//             println!("Parsed values: key = {}, value1 = {}, value2 = {}", key, value1, value2);
//             result.insert(key, (value1, value2));
//         }
//     }

//     result
// }



// // Converts a string in the form of "{key1: (value1, value2), key2: (value3, value4)}" to a HashMap<u32, (u32, u32)>
// fn string_to_tuple_hashmap_timedata(input: &str) -> HashMap<u32, (u32, u32)> {
//     let mut result = HashMap::new();

//     let trimmed = input.trim_matches(|c| c == '{' || c == '}');
//     for item in trimmed.split(", ") {
//         let parts: Vec<&str> = item.split(": ").collect();
//         if parts.len() != 2 {
//             continue;
//         }

//         let tuple_str = parts[1].trim_matches(|c| c == '(' || c == ')');
//         let tuple_parts: Vec<&str> = tuple_str.split(", ").collect();
//         if tuple_parts.len() != 2 {
//             continue;
//         }

//         if let (Ok(key), Ok(value1), Ok(value2)) = (
//             parts[0].parse::<u32>(),
//             tuple_parts[0].parse::<u32>(),
//             tuple_parts[1].parse::<u32>(),
//         ) {
//             result.insert(key, (value1, value2));
//         }
//     }

//     result
// }



// fn calculate_time_control_and_increment_details(project: &TimedProject) -> (u32, u32, u32, u32, u32,u32, u32, u32, u32, u32) {
//     // Print for debugging: display current game move number
//     println!("Current game move number: {}", project.game_move_number);
    
//     let (moves_to_next_time_control, next_time_control_min) = project.white_move_timecontrolmin_incrsec_key_values_list
//         .iter()
//         .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
//         .map(|(&k, &v)| {
//             println!("Found next time control move: {} min: {}", k, v.0);  // Print for debugging
//             (k, v.0)
//         })
//         .unwrap_or_else(|| {
//             println!("No next time control move found.");  // Print for debugging
//             (0, 0)
//         });

//     let (current_increment, next_increment_time, next_increment_move) = project.white_time_timecontrolmin_incrsec_key_values_list
//         .iter()
//         .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
//         .map(|(&k, &v)| {
//             println!("Found next increment: {} time: {} move: {}", project.white_time_timecontrolmin_incrsec_key_values_list[&(project.game_move_number as u32)], k, v);  // Print for debugging
//             (project.white_time_timecontrolmin_incrsec_key_values_list[&(project.game_move_number as u32)], k, v)
//         })
//         .unwrap_or_else(|| {
//             println!("No next increment found.");  // Print for debugging
//             (0, 0, 0)
//         });
    
//     // Print for debugging: display all calculated values
//     println!(
//         "Calculated details: moves_to_next_time_control: {}, next_time_control_min: {}, current_increment: {}, next_increment_time: {}, next_increment_move: {}",
//         moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move
//     );

//     (
//         white_moves_to_next_time_control, 
//         black_moves_to_next_time_control,

//         white_next_time_control_min, 
//         black_next_time_control_min, 

//         white_current_increment, 
//         black_current_increment, 

//         white_next_increment_time, 
//         black_next_increment_time, 

//         white_next_increment_move,
//         black_next_increment_move,        
//     )

// }



/// calculate time control and increment details.
/// This function takes a reference to a TimedProject instance and returns a tuple
/// containing moves to the next time control, next time control in minutes, current increment,
/// next increment time in seconds, and next increment move.
// Helper function
// fn calculate_time_control_and_increment_details(project: &TimedProject) -> (u32, u32, u32, u32, u32) {
//     let (moves_to_next_time_control, next_time_control_min) = project.white_move_timecontrolmin_incrsec_key_values_list
//         .iter()
//         .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
//         .map(|(&k, &v)| (k, v.0))
//         .unwrap_or((0, 0));

//     let (current_increment, next_increment_time, next_increment_move) = project.white_time_timecontrolmin_incrsec_key_values_list
//         .iter()
//         .find(|&(&k, _)| k > project.game_move_number as u32)  // Fixed pattern matching
//         .map(|(&k, &v)| (project.white_time_timecontrolmin_incrsec_key_values_list[&(project.game_move_number as u32)], k, v))
//         .unwrap_or((0, 0, 0));
    
//     (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move)
// }



    // // Wrapper for use-case 1: Create or update struct and make HTML
    // pub fn wrapper_move_update_and_make_html(game_name: &str, move_data: &str) -> io::Result<String> {
    //     let mut project = Self::load_timedata_from_txt(game_name)?;

    //     // Update the struct using the update_timedata_before_move function
    //     project.update_timedata_before_move(move_data);
        
    //     // Generate the HTML content using the updated struct
    //     Ok(Self::generate_html_with_time_data(&project, timestamp_64()))
    // }


    // // Wrapper for use-case 2: Load text file and make HTML
    // pub fn wrapper_no_move_load_and_make_html(game_name: &str) -> io::Result<String> {
    //     let current_timestamp = timestamp_64();

    //     // Load the TimedProject struct from the text file
    //     let project = Self::load_timedata_from_txt(game_name)?;

    //     // Generate the HTML content using the loaded struct
    //     Ok(Self::generate_html_with_time_data(&project, current_timestamp))
    // }
    
    
    // fn generate_html_timedata(project: &TimedProject) -> String {
    //     let mut html_timedata_string = String::new();
    
    //     let mut time_controls_combined: HashMap<u32, (u32, u32)> = HashMap::new();
    
    //     for (&move_num, &(min, sec)) in &project.white_move_timecontrolmin_incrsec_key_values_list {
    //         time_controls_combined.insert(move_num, (min, sec));
    //     }
    
    //     for (&move_num, &(min, sec)) in &project.black_move_timecontrolmin_incrsec_key_values_list {
    //         if let Some(val) = time_controls_combined.get_mut(&move_num) {
    //             if *val != (min, sec) {
    //                 // If the values are different, set to a flag value to identify later.
    //                 *val = (u32::MAX, u32::MAX);
    //             }
    //         } else {
    //             time_controls_combined.insert(move_num, (min, sec));
    //         }
    //     }
    
    //     for (move_num, (min, sec)) in time_controls_combined {
    //         if min != 0 {
    //             if sec != 0 && sec != u32::MAX {
    //                 html_timedata_string.push_str(&format!("<li>Time Control on move {}, adds {} min and {} sec per move.</li>", move_num, min, sec));
    //             } else if sec == u32::MAX {
    //                 // White and Black have different values.
    //                 let white_val = project.white_move_timecontrolmin_incrsec_key_values_list.get(&move_num).unwrap();
    //                 let black_val = project.black_move_timecontrolmin_incrsec_key_values_list.get(&move_num).unwrap();
    //                 html_timedata_string.push_str(&format!("<li>White Time Control on move {}, adds {} min.</li>", move_num, white_val.0));
    //                 html_timedata_string.push_str(&format!("<li>Black Time Control on move {}, adds {} min.</li>", move_num, black_val.0));
    //             } else {
    //                 html_timedata_string.push_str(&format!("<li>Time Control on move {}, adds {} min.</li>", move_num, min));
    //             }
    //         }
    //     }
    
    //     let mut increments_combined: HashMap<u32, u32> = HashMap::new();
    
    //     for (&move_num, &sec) in &project.white_time_timecontrolmin_incrsec_key_values_list {
    //         increments_combined.insert(move_num, sec);
    //     }
    
    //     for (&move_num, &sec) in &project.black_time_timecontrolmin_incrsec_key_values_list {
    //         if let Some(val) = increments_combined.get_mut(&move_num) {
    //             if *val != sec {
    //                 *val = u32::MAX;
    //             }
    //         } else {
    //             increments_combined.insert(move_num, sec);
    //         }
    //     }
    
    //     for (move_num, sec) in increments_combined {
    //         if sec != 0 {
    //             if sec != u32::MAX {
    //                 html_timedata_string.push_str(&format!("<li>Increment starts on move {}, adding {} sec per move.</li>", move_num, sec));
    //             } else {
    //                 // White and Black have different values.
    //                 let white_val = project.white_time_timecontrolmin_incrsec_key_values_list.get(&move_num).unwrap();
    //                 let black_val = project.black_time_timecontrolmin_incrsec_key_values_list.get(&move_num).unwrap();
    //                 html_timedata_string.push_str(&format!("<li>White Increment starts on move {}, adding {} sec per move.</li>", move_num, *white_val));
    //                 html_timedata_string.push_str(&format!("<li>Black Increment starts on move {}, adding {} sec per move.</li>", move_num, *black_val));
    //             }
    //         }
    //     }
    
    //     html_timedata_string
    // }
    
    // fn generate_html_timedata(project: &TimedProject) -> String {

    //     let mut html_timedata_string = String::new();
    
    //     // Extracting the current move number for clarity and ease of use
    //     let current_move = project.game_move_number;
    //     let current_move = project.game_move_number;

    
    //     println!("Current Move: {}", current_move);  // Debugging print


    //     // For time controls
    //     let mut time_controls: HashMap<u32, (Option<u32>, Option<u32>)> = HashMap::new();
    
    //     for (&move_num, &(min, _)) in &project.white_move_timecontrolmin_incrsec_key_values_list {
    //         time_controls.entry(move_num).or_insert((Some(min), None)).0 = Some(min);
    //     }
    
    //     for (&move_num, &(min, _)) in &project.black_move_timecontrolmin_incrsec_key_values_list {
    //         time_controls.entry(move_num).or_insert((None, Some(min))).1 = Some(min);
    //     }
    
    //     for (move_num, (white_min, black_min)) in time_controls {
    //         match (white_min, black_min) {
    //             (Some(wm), Some(bm)) if wm == bm => {
    //                 html_timedata_string.push_str(&format!("<li>Time Control on move {}: adds {} min.</li>", move_num, wm));
    //             }
    //             (Some(wm), Some(bm)) => {
    //                 html_timedata_string.push_str(&format!("<li>White Time Control on move {}: adds {} min.</li>", move_num, wm));
    //                 html_timedata_string.push_str(&format!("<li>Black Time Control on move {}: adds {} min.</li>", move_num, bm));
    //             }
    //             (Some(wm), None) => {
    //                 html_timedata_string.push_str(&format!("<li>White Time Control on move {}: adds {} min.</li>", move_num, wm));
    //             }
    //             (None, Some(bm)) => {
    //                 html_timedata_string.push_str(&format!("<li>Black Time Control on move {}: adds {} min.</li>", move_num, bm));
    //             }
    //             _ => {}
    //         }
    //     }
    

    //     // Filtering out past rules
    //     let future_time_controls: Vec<_> = time_controls.iter()
    //         .filter(|&&(move_num, _)| move_num > current_move)
    //         .collect();
    
    //     let future_increments: Vec<_> = increments.iter()
    //         .filter(|&&(move_num, _)| move_num > current_move)
    //         .collect();

    //     // For increments
    //     let mut increments: HashMap<u32, (Option<u32>, Option<u32>)> = HashMap::new();
    
    //     for (&move_num, &sec) in &project.white_time_timecontrolmin_incrsec_key_values_list {
    //         if sec != 0 {
    //             increments.entry(move_num).or_insert((Some(sec), None)).0 = Some(sec);
    //         }
    //     }
    
    //     for (&move_num, &sec) in &project.black_time_timecontrolmin_incrsec_key_values_list {
    //         if sec != 0 {
    //             increments.entry(move_num).or_insert((None, Some(sec))).1 = Some(sec);
    //         }
    //     }
    

    //     // increment details
    //     let white_current_increment = project.white_time_timecontrolmin_incrsec_key_values_list
    //         .iter()
    //         .filter(|&&(move_num, _)| move_num <= current_move)
    //         .map(|(_, &incr)| incr)
    //         .last()
    //         .unwrap_or(0);
    
    //     let black_current_increment = project.black_time_timecontrolmin_incrsec_key_values_list
    //         .iter()
    //         .filter(|&&(move_num, _)| move_num <= current_move)
    //         .map(|(_, &incr)| incr)
    //         .last()
    //         .unwrap_or(0);
    
    //     if white_current_increment == black_current_increment {
    //         if white_current_increment != 0 {
    //             html_timedata_string.push_str(&format!("<li>Current Increment: {} sec per move.</li>", white_current_increment));
    //         }
    //     } else {
    //         if white_current_increment != 0 {
    //             html_timedata_string.push_str(&format!("<li>White's Current Increment: {} sec per move.</li>", white_current_increment));
    //         }
    //         if black_current_increment != 0 {
    //             html_timedata_string.push_str(&format!("<li>Black's Current Increment: {} sec per move.</li>", black_current_increment));
    //         }
    //     }




    //     for (move_num, (white_sec, black_sec)) in increments {
    //         match (white_sec, black_sec) {
    //             (Some(ws), Some(bs)) if ws == bs => {
    //                 html_timedata_string.push_str(&format!("<li>Increment starts on move {}: adding {} sec per move.</li>", move_num, ws));
    //             }
    //             (Some(ws), Some(bs)) => {
    //                 html_timedata_string.push_str(&format!("<li>White Increment starts on move {}: adding {} sec per move.</li>", move_num, ws));
    //                 html_timedata_string.push_str(&format!("<li>Black Increment starts on move {}: adding {} sec per move.</li>", move_num, bs));
    //             }
    //             (Some(ws), None) => {
    //                 html_timedata_string.push_str(&format!("<li>White Increment starts on move {}: adding {} sec per move.</li>", move_num, ws));
    //             }
    //             (None, Some(bs)) => {
    //                 html_timedata_string.push_str(&format!("<li>Black Increment starts on move {}: adding {} sec per move.</li>", move_num, bs));
    //             }
    //             _ => {}
    //         }
    //     }
    
    //     html_timedata_string
    // }
    

    // fn generate_html_timedata(project: &TimedProject) -> String {
    //     let mut html_timedata_string = String::new();
    
    //     let current_move = project.game_move_number;
    //     println!("Current Move: {}", current_move); // Debugging print
    
    //     // Merge time controls and increments into unified HashMaps for easier processing
    //     let mut combined_time_controls: HashMap<u32, (u32, u32)> = project.white_move_timecontrolmin_incrsec_key_values_list
    //         .clone()
    //         .into_iter()
    //         .collect();
    
    //     for (&key, &value) in &project.black_move_timecontrolmin_incrsec_key_values_list {
    //         combined_time_controls.insert(key, value);
    //     }
    
    //     let mut combined_increments: HashMap<u32, (u32, u32)> = project.white_time_timecontrolmin_incrsec_key_values_list
    //         .clone()
    //         .into_iter()
    //         .collect();
    
    //     for (&key, &value) in &project.black_time_timecontrolmin_incrsec_key_values_list {
    //         combined_increments.insert(key, value);
    //     }
    
    //     // Time control messages
    //     for (move_num, (white_min, black_min)) in combined_time_controls {
    //         if move_num > current_move {
    //             if white_min == black_min && white_min != 0 {
    //                 html_timedata_string.push_str(&format!("<li>Time Control on move {}: adds {} min.</li>", move_num, white_min / 60));
    //             } else {
    //                 if white_min != 0 {
    //                     html_timedata_string.push_str(&format!("<li>White Time Control on move {}: adds {} min.</li>", move_num, white_min / 60));
    //                 }
    //                 if black_min != 0 {
    //                     html_timedata_string.push_str(&format!("<li>Black Time Control on move {}: adds {} min.</li>", move_num, black_min / 60));
    //                 }
    //             }
    //         }
    //     }
    
    //     // Increment messages
    //     for (move_num, (white_sec, black_sec)) in combined_increments {
    //         if move_num > current_move {
    //             if white_sec == black_sec && white_sec != 0 {
    //                 html_timedata_string.push_str(&format!("<li>Increment starts on move {}: adding {} sec per move.</li>", move_num, white_sec));
    //             } else {
    //                 if white_sec != 0 {
    //                     html_timedata_string.push_str(&format!("<li>White Increment starts on move {}: adding {} sec per move.</li>", move_num, white_sec));
    //                 }
    //                 if black_sec != 0 {
    //                     html_timedata_string.push_str(&format!("<li>Black Increment starts on move {}: adding {} sec per move.</li>", move_num, black_sec));
    //                 }
    //             }
    //         }
    //     }
    
    //     html_timedata_string
    // }
    
    
    
    