/*
http://0.0.0.0:8000/game/Pc2c4
RUST_BACKTRACE=full cargo run
*/


/* 
TODO:

- make and get game_data json!
    - ip hash list
    - move log?
    - last move?
    - game_board_state
    - game timestamp
    - activity timestamp
    - hashed_gamephrase
    - 

- get user IP and hash it

- add b in 

- pass 'b' somehow to move_handler

- check user IP somewhere

- create a game_phrase resirect thing...or instructions to put in

- then gamephrase_get will set their hashed IP in the game_data json




- add restart...
    - trigger set up net game

- get ip

Qd1c2
start

- make white and black display
- show html...

- complete secure 'login' for game
- move to loading in game_data
- check user IP-lossy-hash
- make somewhat longer (20-3?)
- 

- other secure-server aspects?

- think about mudd game
-- mudd-builder...
- Doore
- 


- game setup:
    how to do as page or get request?
    what to do if files no found...redirect to setup-page...or instructions raw text/hmtl?


    setup/type_type/game_name/game_phrase

if error
return text (not html) "that option is now available or allowed, try:
    setup/type_type/game_name/game_phrase

- again    
- start
if move len 7 & == "restart"
    reset board


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
use tiny_http::{Server, Response, Method, Header}; 
use std::path::Path;

// Variables
type Board = [[char; 8]; 8];

use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::Text;

// struct GameData {
//     game_name: String,
//     activity_timestamp: i64,
//     game_type: String,
//     move_number: u32,
// }


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

            // match request.remote_addr() {
            //     Some(socket_addr) => {
            //         let user_ip = socket_addr.ip();
            //         println!("Client IP: {:?}", user_ip);
        
            //         // Place your code here where you handle the request using the IP information
            //         // ...
        
            //     },
            //     None => println!("Could not retrieve client IP"),
            // }
            

            let ip_hash = match request.remote_addr() {
                Some(socket_addr) => {
                    let ip_string = socket_addr.ip().to_string();
                    
                    // Assuming you have a timestamp string at this point
                    let timestamp_string = "your_timestamp_string_here";
        
                    // Now you can make a hash from the IP string and the timestamp string
                    make_hash(&ip_string, &timestamp_string)

                },
                None => {
                    println!("Could not retrieve client IP");
                    continue;
                },
            };

            println!("ip_hash: {:?}", ip_hash);
            

            // // landing page html
            // // Check if it's the landing page (base domain only)
            // if url_parts.len() == 2 {
            //     let response = match landing_page() {
            //         Ok(response_string) => {
            //             Response::from_data(response_string)
            //                 .with_header(Header::from_bytes("Content-Type", "text/html").unwrap())
            //                 .with_status_code(200)
            //         },
            //         Err(e) => {
            //             eprintln!("Failed to generate landing page: {}", e);
            //             Response::from_string(format!("Failed to generate landing page: {}", e)).with_status_code(500)
            //         }
            //     };

            //     if let Err(e) = request.respond(response) {
            //         eprintln!("Failed to respond to request: {}", e);
            //     }
            //     continue; // No need to run the rest of the loop for the landing page
            // }


        // landing page (NOT HTML, keep it)
        // Check if it's the landing page (base domain only)
        if url_parts.len() == 2 {
            let response = match landing_page_no_html() {
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

        // if chess game maove
        else if url_parts.len() == 3 {

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

            // if 'start' reset and return blank board




            // // call game move function
            // let response = match handle_chess_move(game_name, move_data) {
            //     Ok(response_string) => {
            //         Response::from_string(response_string).with_status_code(200)
            //     },
            //     Err(e) => {
            //         eprintln!("Failed to handle move: {}", e);
            //         Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
            //     }
            // };


            let response = match handle_chess_move(game_name, move_data) {
                Ok(svg_content) => {
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                        .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
            
                    Response::from_string(svg_content).with_header(header).with_status_code(200)
                },
                Err(e) => {
                    eprintln!("Failed to handle move: {}", e);
                    Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                }
            };
            
            
            

            if let Err(e) = request.respond(response) {
                eprintln!("Failed to respond to request: {}", e);
            }

        } 


        // setup (new game)
        else if url_parts.len() == 5 {
            let mode = url_parts[1].to_string();
            if mode != "setup" {
                // Return an error response if the mode is not "setup"
                let response = Response::from_string("Invalid mode for setup.")
                    .with_status_code(400);
                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
            } else {
                let game_type = url_parts[2].to_string();
                let game_name = url_parts[3].to_string();
                let game_phrase = url_parts[4].to_string();

                // Call setup_new_game here
                let response = match setup_new_game(&game_type, &game_name, &game_phrase) {
                    Ok(_) => Response::from_string("Game setup successfully.")
                        .with_status_code(200),

                    Err(e) => Response::from_string(format!("Failed to set up game: {}", e))
                        .with_status_code(500),
                };

                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
            }
        }

        

        // // setup (new game)
        // else if url_parts.len() == 5 {
        //     let mode = url_parts[1].to_string();
        //     if mode != "setup"...exit error

        //     let game_type = url_parts[2].to_string();            
        //     let game_name = url_parts[3].to_string();
        //     let game_phrase = url_parts[4].to_string();

        //     // Call setup_new_game here
        //     let response = match setup_new_game(game_type, game_name, game_phrase) {
        //         Ok(_) => Response::from_string("Game setup successfully.")
        //             .with_status_code(200),

        //         Err(e) => Response::from_string(format!("Failed to set up game: {}", e))
        //             .with_status_code(500),
        //     };

        //     if let Err(e) = request.respond(response) {
        //         eprintln!("Failed to respond to request: {}", e);
        //     }
        // }




        else {
            // ... Invalid request format
            let response = match landing_page_no_html() {
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
        }
    }
}


/////////////////////
// Helper Functions
/////////////////////

// fn landing_page() -> Result<String, Box<dyn std::error::Error>> {

//     // Here, you can read the pre-existing HTML script from a file or use a hardcoded string.
//     // For this example, I'll provide a simple response with a "Hello, World!" message.
//     let response_string = r#"<html>
//     <body>
//         <body style="background-color:black;">
//         <font color="00FF00">  
//             <div style="line-height:1px">
//         <tt> 
//         <p style="font-size:38px; "> r n b q k b n r </p>
//         <p style="font-size:38px; "> p p p p p p p p </p>
//         <p style="font-size:38px; "> . . . . . . . . </p>
//         <p style="font-size:38px; "> . . . . . . . . </p>
//         <p style="font-size:38px; "> . . P . . . . . </p>
//         <p style="font-size:38px; "> . . . . . . . . </p>
//         <p style="font-size:38px; "> P P . P P P P P </p>
//         <p style="font-size:38px; "> R N B Q K B N R </p>
        
//         <p style="font-size:18px; "> 鰻　み　岡　野　エ　た　お　天　ラ　白 </p>
//         <p style="font-size:18px; "> 丼　そ　山　菜　ビ　こ　で　丼　ー　竜 </p>
//         <p style="font-size:18px; "> 八　カ　の　天　フ　焼　ん　八　メ　 </p>
//         <p style="font-size:18px; "> 三　ツ　ラ　ぷ　ラ　き　四　円　ン </p>
//         <p style="font-size:18px; "> 百　ラ　ー　ら　イ　三　円 </p>
//         <p style="font-size:18px; "> 六　ー　メ　八　十　円 </p>
//         <p style="font-size:18px; "> 十　メ　ン　五　円 </p>
//         <p style="font-size:18px; "> 三　ン　十　円 </p>
//         <p style="font-size:18px; "> 八　万　円 </p>
//         <p style="font-size:18px; "> 万　円 </p>
//         <p style="font-size:18px; "> 円　</p>
//             </div>
//             </body>
//         </html>
//         "#.to_string();

//         // return response string
//         Ok(response_string)
//     }

fn landing_page_no_html() -> Result<String, Box<dyn std::error::Error>> {

    // Here, you can read the pre-existing HTML script from a file or use a hardcoded string.
    // For this example, I'll provide a simple response with a "Hello, World!" message.
    let response_string = r#"
    Try https://y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE & https://y0urm0ve.com/YOUR_GAME_NAME/Pc2c4

        r n b q k b n r
        p p p p p p p p
        . . . . . . . .
        . . . . . . . .
        . . P . . . . .
        . . . . . . . .
        P P . P P P P P
        R N B Q K B N R
        
        天　ラ　白
        丼　ー　竜
        ん　八　メ
        八　ン
        万
        円

        "#.to_string();

        // return response string
        Ok(response_string)
    }

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

    // Load the game board state
    let mut board = match load_game_board_state(&game_name) {
        Ok(board) => board,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other, format!("Failed to load game board state: {}", e))));
        }
    };

    match parse_move(&move_data) {
        Ok((is_black, piece, from, to)) => {
            // This is the successful case. `piece`, `from`, and `to` are guaranteed to be initialized.

            // Initialize variables to hold converted coordinates
            let from_x_y_coordinates: (usize, usize);
            let to_x_y_coordinates: (usize, usize);

            // "FROM" moves
            let from_coords_result = to_coords(&format!("{}{}", from.0, from.1));
            let from_coords = from_coords_result?;
            let (x, y) = from_coords;
            board[x][y] = ' ';
            from_x_y_coordinates = from_coords;


            // "TO" moves
            let to_coords_result = to_coords(&format!("{}{}", to.0, to.1));
            let to_coords = to_coords_result?;
            let (x, y) = to_coords;
            board[x][y] = piece;
            to_x_y_coordinates = to_coords;


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

            // generate svg




            // Inverting the coordinates for black's perspective
            let from_black_oriented = (7 - from_x_y_coordinates.0, 7 - from_x_y_coordinates.1);
            let to_black_oriented = (7 - to_x_y_coordinates.0, 7 - to_x_y_coordinates.1);

            // Generate SVG with these coordinates
            let doc = if is_black {
                generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented))
            } else {
                generate_white_oriented_chessboard(&board, Some(from_x_y_coordinates), Some(to_x_y_coordinates))  
            };

            // Generate SVG with these coordinates
            let doc = generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented));

            // Define the file name
            let file_name = "board.svg";

            // Write the svg code to the file
            svg::save(file_name, &doc).expect("Unable to write to file");

            println!("SVG file has been created successfully.");
    

            // return svg...
            // After generating the SVG...
            let svg_content = doc.to_string();
            Ok(svg_content)


            
        }
        Err(e) => {
            // This is the error case. Return or handle the error in some way here.
            eprintln!("Invalid move format: {}", e);
            // If you want to return error to the caller, you could do so here:
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Invalid move format: {}", e))));
        }
    }

    // // return response string
    // Ok(response_string)

    // // After generating the SVG...
    // let svg_content = doc.to_string();
    // Ok(svg_content)

}


// Function to generate the SVG chessboard with black orientation
fn generate_white_oriented_chessboard(
    chessboard: &[[char; 8]; 8], 
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>
) -> Document {

    let mut doc = Document::new()
        .set("width", "500")  
        .set("height", "500")  
        .set("viewBox", (0, 0, 500, 500))
        .set("style", "background-color: #2f0300;");  // Set background to dark red

    // Define labels, reversed for black piece orientation
    let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
    let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 50 + idx * 50 + 25)  
            .set("y", 472)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  // Set text color to dark grey
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 32)  
            .set("y", 50 + idx * 50 + 35)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
        for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
            let x = 50 + col * 50;  
            let y = 50 + row * 50;  

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

                if let Some(from_coords) = from {
                    let (row, col) = from_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }
                
                if let Some(to_coords) = to {
                    let (row, col) = to_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }

                    
                let piece_color = if square_color == "#666" { // for darker background
                    if piece.is_uppercase() {
                        "#ffefc1" // lighter gray for light pieces
                    } else {
                        "#ff8e8e" // lighter red for dark pieces
                    }
                } else { // for lighter background
                    if piece.is_uppercase() {
                        "#665628" // darker gray for light pieces
                    } else {
                        "#9e0b00" // darker red for dark pieces
                    }
                };

                let mut text = Text::new()
                    .set("x", x + 25)
                    .set("y", y + 35)
                    .set("text-anchor", "middle")
                    .set("font-size", 30)
                    .set("fill", piece_color);

                if piece.is_uppercase() {
                    text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
                } else {
                    text = text.add(svg::node::Text::new(piece.to_string()));
                }

                doc = doc.add(text);
            }
        }
    }

    doc
}

fn black_to_coords(chess_notation: &str) -> Result<(usize, usize), String> {
    if chess_notation.len() != 2 {
        return Err(format!("Invalid chess notation: '{}'. It should be two characters long.", chess_notation));
    }
    let col = chess_notation.chars().nth(0).unwrap();
    let row = chess_notation.chars().nth(1).unwrap();

    if !('a'..='h').contains(&col) || !('1'..='8').contains(&row) {
        return Err(format!("Invalid chess notation: '{}'. It should be in the form 'e4'.", chess_notation));
    }

    let col = 'h' as usize - col as usize;  // Changed this line
    let row = row.to_digit(10).unwrap() as usize - 1;  // And this line

    Ok((row, col))
}


// Function to generate the SVG chessboard with black orientation
fn generate_black_oriented_chessboard(
    chessboard: &[[char; 8]; 8], 
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>
) -> Document {

    let mut doc = Document::new()
        .set("width", "500")  
        .set("height", "500")  
        .set("viewBox", (0, 0, 500, 500))
        .set("style", "background-color: #2f0300;");  // Set background to dark red

    // Define labels, reversed for black piece orientation
    let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
    let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 50 + idx * 50 + 25)  
            .set("y", 472)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  // Set text color to dark grey
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 32)  
            .set("y", 50 + idx * 50 + 35)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
        for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
            let x = 50 + col * 50;  
            let y = 50 + row * 50;  

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

                if let Some(from_coords) = from {
                    let (row, col) = from_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }
                
                if let Some(to_coords) = to {
                    let (row, col) = to_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }

                    
                let piece_color = if square_color == "#666" { // for darker background
                    if piece.is_uppercase() {
                        "#ffefc1" // lighter gray for light pieces
                    } else {
                        "#ff8e8e" // lighter red for dark pieces
                    }
                } else { // for lighter background
                    if piece.is_uppercase() {
                        "#665628" // darker gray for light pieces
                    } else {
                        "#9e0b00" // darker red for dark pieces
                    }
                };

                let mut text = Text::new()
                    .set("x", x + 25)
                    .set("y", y + 35)
                    .set("text-anchor", "middle")
                    .set("font-size", 30)
                    .set("fill", piece_color);

                if piece.is_uppercase() {
                    text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
                } else {
                    text = text.add(svg::node::Text::new(piece.to_string()));
                }

                doc = doc.add(text);
            }
        }
    }

    doc
}



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

fn parse_move(move_data: &str) -> Result<(bool, char, (char, u8), (char, u8)), String> {
    let mut adjusted_move_data = String::from(move_data);
    let is_black;

    if move_data.len() > 6 {
        return Err(format!("Invalid input length. Input should be 5 or 6 characters."));
    }
    
    if move_data.len() == 6 {
        if move_data.starts_with('b') {
            adjusted_move_data.remove(0);  // Remove first character 'b'
            is_black = true;
        } else {
            return Err(format!("If input length is 6, it must start with 'b'."));
        }
    } else {
        is_black = false;
    }

    let chars: Vec<char> = adjusted_move_data.chars().collect();

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

    Ok((is_black, *piece, from, to))
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



fn validate_input(input: &str) -> Result<(), String> {
    // Check if the input is "start"
    if input == "start" {
        return Ok(());
    }

    let mut adjusted_input = String::from(input);

    if input.len() > 6 {
        return Err(format!("Invalid input length. Input should be 5 or 6 characters."));
    }

    if input.len() == 6 {
        if input.starts_with('b') {
            adjusted_input.remove(0);  // Remove first character 'b'
        } else {
            return Err(format!("If input length is 6, it must start with 'b'."));
        }
    }

    let chars: Vec<char> = adjusted_input.chars().collect();

    let valid_pieces = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];
    let valid_cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let valid_rows = ['1', '2', '3', '4', '5', '6', '7', '8'];

    if !valid_pieces.contains(&chars[0]) {
        return Err(format!("Invalid piece identifier. The first character should be one of 'prnbqkPRNBQK'. e.g. Pc2c4 or pc7c6 "));
    }
    if !valid_cols.contains(&chars[1]) || !valid_cols.contains(&chars[3]) {
        return Err(format!("Invalid column identifier. The 2nd and 4th characters should be one of 'abcdefgh'. e.g. Pc2c4 or pc7c6 "));
    }
    if !valid_rows.contains(&chars[2]) || !valid_rows.contains(&chars[4]) {
        return Err(format!("Invalid row identifier. The 3rd and 5th characters should be one of '12345678'. e.g. Pc2c4 or pc7c6  "));
    }
    
    Ok(())
}



fn save_game_board_state(game_name: &str, board: [[char; 8]; 8]) -> io::Result<()> {
    let dir_path = format!("./games/{}", game_name);
    std::fs::create_dir_all(&dir_path).expect("Failed to create directory");

    let file_path = format!("{}/game_board_state.txt", dir_path);
    let mut file = File::create(&file_path).expect("Failed to create file");

    for row in &board {
        let line: String = row.iter().collect();
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
    
    Ok(())
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





fn setup_new_game(game_type: &str, game_name: &str, game_phrase: &str) -> std::io::Result<()> {

    // Validate game_name: novel, permitted, ascii etc.
    if !is_valid_game_name(game_name) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
            "Invalid game name: ascii abc_123 novel names, try again. "));
        }
    /*

    make files and folders...
    set up and save initial game board

    store date in a file:
    last_activity = posix timestamp

    // make a game_data json:
    activity_timestamp: posic timestamp
    game_type: chess
    move_number: 0
    set game type = chess

    */

    // make a game_data json

    // Posix Timestamp in gamedata json file:

    // Set up board
    let board: [[char; 8]; 8] = [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
    ];
    

    // Save game (save game_board_state to .txt file)
    if let Err(e) = save_game_board_state(&game_name, board) {
        eprintln!("Failed to save game state: {}", e);
    }

    // create folder for game_name
    let dir_path = format!("./games/{}", game_name);
    std::fs::create_dir_all(&dir_path)?;

    let file_path = format!("{}/game_type.txt", dir_path);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    writeln!(file, "{}", game_type)?;

    // write gametype, timestamp
    if let Err(e) = create_gamedata_json(&dir_path, game_name, game_type, 0) {
        eprintln!("Failed to create game data: {}", e);
    }

    Ok(())
}


// Helper function to validate game_name
fn is_valid_game_name(game_name: &str) -> bool {
    // sanity check
    println!("game name: {:?}", game_name);

    // Check if game_name is a reserved word
    let reserved_words = vec!["setup", "restart", "y0ur_m0ve"];
    // sanity check
    println!("Reserved words: {:?}", reserved_words);

    if reserved_words.contains(&game_name) {
        eprintln!("error # 1: Invalid game name: Reserved word used.");
        return false;
    }

    // Check if game_name contains only alphanumeric characters and underscores
    for c in game_name.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            eprintln!("error # 2: Invalid game name: wrong characters.");
            return false;
        }
    }

    // Check if a directory with this game_name already exists
    let game_dir = format!("./games/{}", game_name);
    if Path::new(&game_dir).exists() {
        eprintln!("error # 3: Invalid game name: already exists!");
        return false;
    }

    true
}


fn create_gamedata_json(dir_path: &str, game_name: &str, game_type: &str, move_number: u32) -> io::Result<()> {
    // Get the current time
    let now = SystemTime::now();
    // Calculate the duration since UNIX EPOCH
    let duration = now.duration_since(UNIX_EPOCH).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Error while getting duration: {}", err),
        )
    })?;
    // Extract the seconds from the duration
    let timestamp_secs = duration.as_secs() as i64;

    // Create the JSON string
    let json_data = format!(
        r#"{{
            "game_name": "{}",
            "game_timestamp": {},
            "activity_timestamp": {},
            "game_type": "{}",
            "move_number": {}
        }}"#,
        game_name,
        timestamp_secs,
        timestamp_secs,
        game_type,
        move_number
    );

    // Open the file for writing
    let json_path = format!("{}/game_data.json", dir_path);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(json_path)?;

    // Write the JSON data to the file
    writeln!(file, "{}", json_data)?;

    Ok(())
}




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



// use std::time::{SystemTime, UNIX_EPOCH};

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

