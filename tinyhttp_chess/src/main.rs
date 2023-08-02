/*
http://0.0.0.0:8000/game/Pc2c4
RUST_BACKTRACE=full cargo run

http://0.0.0.0:8000/setup/chess/katsu/katsudan
http://0.0.0.0:8000/game/Pc2c4

http://0.0.0.0:8000/setup/chess960/ramen/two
http://0.0.0.0:8000/game/Pc2c4
*/


/* TODO:
https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces 


for just game_name
show two svg images...black and white...



- is game_data ip_hash
list .csv based?

- maybe make ip-hash function
    - game setup

- handle game move

    - game restart

- public_board
- not_public_board

- game_erase

start
erase

page:

instructions page

- make ip_hash based on 

1. raw password hash
2. timestamp ip hash 

- what to call 'secure' version?
- incovenient_chess?
- maybe...setup area mode
    - public, hidden, lockdown
    - 

maybe delete request after
using data...



rust question:
can e.g. a password struct
be set to only be accessible
from specific functions?

- drop move-history

- revisit the whole struct impl catastrophy
probably go back to just writing files

- add scrambled eggs to the board reader writer.


- secure mode?

- make a password blacklist

- encript 


- encrypt-scramble game
    - ...what mode? or settings?

- turn off saving svg, turn off terminal print of game,



where is gamae-data set...
https://chat.openai.com/share/f877e8d6-754b-47e6-90ec-4588223b893d 

task scheduling...

Duty Roster:
    events have startby dates listed in duty_roster directory,
    check duty_roster directory at start of each action
    to see if there are tasks to do (e.g. delete old games)
    if any startby date is smaller number than today's date,
    make a gueue
    open that folder
    do the task


slightly more orange ivory color for background
try
shapes...
during setup...
type of pieces...


game_data:
function to save game_data
function to load game_data
testing?

...maybe add this to setup-game...
...maybe part of 'start' game?


where to call functions...


game_phrase route...
maybe fork of setup?


- make and get game_data json!
    - ip hash list
    - move log?
    - last move?
    - game_board_state
    - game timestamp
    - activity timestamp
    - hashed_gamephrase
    - 

game setup get endpoint...
- route call setup game for that game-name?

requeest queue system
two part
1. light: 
2  heavy, save request as files in a directory

- get user IP and hash it

- check user IP somewhere:
first part of...parse input...
if that game_name's ip_hash list 

- create a game_phrase resirect thing...or instructions to put in

- then gamephrase_get will set their hashed IP in the game_data json


- add restart...
    - trigger set up net game

- get ip



- complete secure 'login' for game
- move to loading in game_data
- check user IP-lossy-hash
- make somewhat longer (20-3?) 

- other secure-server aspects?

- think about mudd game
-- mudd-builder...
- Doore



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



github open source rust server code:


data privacy policy/procedures:
    The intent is to store as little (as few) data as possbile.
    An encrypted game-state is stored.
    No game history is stored.
    No record of 'who' made any more is stored.
    There is no information of any kind about
    wins, losses, move-legality, etc.,
    only a 'game-board'

    
    Given that manually typing in a long secure password every time
    you made a chess-move would make the process to cumbersome.

there is an inherent paradox here,
either you are bad because A. you are identifying and tracking the uesr,
making it possible to associate a specific user with information used by the system

or you are bad because B. you are NOT identifying and tracking the uesr, making it 
possible for some OTHER person to enguage with their server sessions, because you
did not identify the individual person and track them with enough detail. 

But you cannot be bath in both of these two ways at the same time.

This design errs on the side of not storing any private information,
which means a highly likelihood of accidentally confusing you with someone else
(because the site delibrorately pays not attention to who you are.)

If you want more security, you can opt in to the secure-chess mode, 
which will be more secure but also more annoying to use.






data retention policy/procedures:
    There is no personal information. 
    There is no account.
    There are no cookies.
    There is no user information or user identification.

secure-mode gaming:
- check entropy of passphrase
- encrypt game-state
- use passphrase for every move

privacy:


As the lesser of two wrongs, 
Default (GPT-3.5)

User
could every other digit from an ip be used to identify an ip?

ChatGPT
No, using every other digit from an IP address would not be sufficient to identify the IP address accurately. An IP (Internet Protocol) address is a numerical label assigned to each device connected to a computer network that uses the Internet Protocol for communication. IP addresses are used to identify and locate devices on the internet.

An IP address consists of four sets of numbers, separated by periods (e.g., 192.168.1.1). Each set can range from 0 to 255, and there are over four billion possible combinations.

If you were to use every other digit from an IP address, you would lose critical information and could potentially identify multiple IP addresses with the same pattern. For example:

Original IP address: 192.168.1.1
Using every other digit: 92811

There could be other IP addresses like 9.2.8.1.1, 9.2.8.11, etc., that would also share the same pattern of digits. Hence, using every other digit would not uniquely identify an IP address.

To accurately identify an IP address, you need to consider all four sets of numbers in the correct order. This is why the complete IP address is necessary for network communication and administration.

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
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::Text;
// use zeroize::Zeroize;


struct GameData {
    game_name: String,
    hashed_gamephrase: u128,
    game_type: String,
    game_timestamp: u128,
    activity_timestamp: u128,
    ip_hash_list: Vec<u128>,
    game_board_state: [[char; 8]; 8],
}


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


            // get reduced ip_stamp rather than whole ip
            let ip_stamp = match request.remote_addr() {
                Some(socket_addr) => {
                    
                    let ip_string = socket_addr.ip().to_string();
        
                    let alternating_stamp = str_filter_alternating(&ip_string);

                    let reduced_ip_stamp = remove_duplicates(&alternating_stamp);

                    reduced_ip_stamp

                },
                None => {
                    // println!("Could not retrieve client IP");
                    continue;
                },
                 
            };
            println!("ip_stamp: {}", ip_stamp);


            

            // let ip_stamp = match request.remote_addr() {
            //     Some(socket_addr) => {
            //         let ip_string = socket_addr.ip().to_string();
                    
        
            //         // Now you can make a hash from the IP string and the timestamp string
            //         make_hash(&ip_string, &timestamp_string)

            //     },
            //     None => {
            //         println!("Could not retrieve client IP");
            //         continue;
            //     },
            // };

            // println!("ip_stamp: {:?}", ip_stamp);

        ////////////////
        // landing page
        ////////////////
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


        /////////
        // m0ve
        ////////
        else if url_parts.len() == 3 {

            let game_name = url_parts[1].to_string();
            let move_data = url_parts[2].to_string();  

            // // // declare response outside match blocks so we can assign it in each match block
            // let response = Response::from_string(response_string);
            if is_existing_game_name(&game_name) {
                // println!("Game exists, proceed with the game logic.");
            } else {
                println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
            }

            // validate_ip_hash
            let dir_path = format!("./games/{}", game_name);
            let game_data = match GameData::from_txt(&dir_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Failed to validate_ip_hash: {}", e);
                    return; // or handle the error in a way that's appropriate for your application
                }
            };

            let is_valid = game_data.validate_ip_hash(&ip_stamp);

            if !is_valid {
                println!("Failed to validate_ip_hash");
                return; // Return success: we've handled the request by generating a response
            }




            // if 'start' reset and return blank board


            // // call game move function
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


        //////////////
        // login m0ve
        //////////////
        else if url_parts.len() == 4 {

            let game_name = url_parts[1].to_string();
            let move_data = url_parts[2].to_string();  
            let game_phrase = url_parts[3].to_string();  

            if is_existing_game_name(&game_name) {
                // println!("Game exists, proceed with the game logic.");
            } else {
                println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
            }


            // // validate game_phrase
            let dir_path = format!("./games/{}", game_name);
            let game_data = match GameData::from_txt(&dir_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("validate game_phrase, Failed to load game data: {}", e);
                    return; // or handle the error in a way that's appropriate for your application
                }
            };

            let is_valid = game_data.validate_game_phrase(&game_phrase);

            if is_valid {
                println!("Game phrase is valid.");
                // add ip_hash to list...


            } else {
                println!("Game phrase is not valid.");
            }


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


            // match check_ip_stamp_in_file(&ip_stamp, &game_name) {
            //     Ok(_) => println!("OK"),
            //     Err(message) => println!("Try: y0urm0ve.com/YOUR_GAME_NAME/Pc2c4/GAME_PHRASE{}", message),
            // }


            // if 'start' reset and return blank board


            // // call game move function
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

        //////////
        // setup    (new game)
        /////////
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
                
                
                // Check the game type and call the corresponding function
                let response = if game_type == "chess960" {
                    match setup_new_chess960_game(&game_type, &game_name, &game_phrase, &ip_stamp) {
                        Ok(_) => Response::from_string("Chess960 game setup successfully.")
                            .with_status_code(200),
                        Err(e) => Response::from_string(format!("Failed to set up Chess960 game: {}", e))
                            .with_status_code(500),
                    }
                } else if game_type == "chess" {
                    match setup_new_game(&game_type, &game_name, &game_phrase, &ip_stamp) {
                        Ok(_) => Response::from_string("Chess game setup successfully.")
                            .with_status_code(200),
                        Err(e) => Response::from_string(format!("Failed to set up Chess game: {}", e))
                            .with_status_code(500),
                    }
                } else {
                    Response::from_string("Invalid game type.")
                        .with_status_code(400)
                };

                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
            }
        }

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

        r n b q k b n r
        p p p p p p p p
        . . . . . . . .
        . . . . . . . .
        . . P . . . . .
        . . . . . . . .
        P P . P P P P P
        R N B Q K B N R
        
        た　天　海　ラ　白
        こ　丼　老　ー　竜
        焼　ん　二　八　メ
        き　八　匹　ン
        三　万　
        百　円
        円

        y0urm0ve.com/setup/chess/
        　　　　YOUR_GAME_NAME/
        　　　　　　YOUR_GAME_PHRASE 

        y0urm0ve.com/
        　　YOUR_GAME_NAME/
        　　　　Pc2c4

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
            // let doc = generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented));

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


// fn board_to_unicode(board: &[[char; 8]; 8]) -> [[&'static str; 8]; 8] {
//     let mut board_unicode: [[&'static str; 8]; 8] = [[" "; 8]; 8];

//     for (i, row) in board.iter().enumerate() {
//         for (j, cell) in row.iter().enumerate() {
//             let piece = piece_to_unicode(*cell);
//             board_unicode[i][j] = piece;
//         }
//     }

//     board_unicode
// }



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





fn setup_new_chess960_game(game_type: &str, game_name: &str, game_phrase: &str, ip_stamp: &str) -> std::io::Result<()> {


    // Validate game_name: novel, permitted, ascii etc.
    if !is_valid_game_name(game_name) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
            "Invalid game name: ascii abc_123 novel names, try again. "));
        }
    /*

    make files and folders...
    set up and save initial game board

    store date in a json file:

    last_activity = posix timestamp

    // make a game_data json:
    activity_timestamp: posic timestamp
    game_type: chess
    move_number: 0
    set game type = chess

    two gametypes for now

    chess
    chess960
        chess_gomeclock

    */

    // make a game_data json

    // Posix Timestamp for game_timestamp and activity_timestamp
    let this_timestamp: u128 = timestamp();


    // make gamephrase hash: to verify if just the 'unknown' game_phrase is correct
    let hashed_gamephrase = make_hash(game_phrase, this_timestamp, 10);


    /* make ip_hash
    ip_hash made with only half of ip, every other number, so the actual ip is never used at all

    Step 1. the ip_stamp needs to be unique enough to have few collisions between people
    but non-unique enough to not be 'personal data'

    Step 2. user game_timestamp (or what will be used to set game_timestamp) to make hashed_ip_stamp
    */
    let hashed_ip_stamp: u128 = make_hash(&ip_stamp, this_timestamp, 10);
    

    // Set up board
    let game_board_state_result = generate_chess960();
    let game_board_state = match game_board_state_result {
        Ok(board) => board,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    };


    // // This is now done with game_data struct/impl
    // // Save game (save game_board_state to .txt file)
    // if let Err(e) = save_game_board_state(&game_name, board) {
    //     eprintln!("Failed to save game state: {}", e);
    // }


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

    // create list for initial game_data list
    let ip_hash_list = vec![hashed_ip_stamp];
    


    // create first game_data struct
    let game_data = GameData::new(
        game_name.to_string(),
        hashed_gamephrase, 
        game_type.to_string(), 
        this_timestamp,
        this_timestamp,
        ip_hash_list, 
        game_board_state
    );


    // write
    game_data.to_txt()?;

    Ok(())
}


fn setup_new_game(game_type: &str, game_name: &str, game_phrase: &str, ip_stamp: &str) -> std::io::Result<()> {

    // Validate game_name: novel, permitted, ascii etc.
    if !is_valid_game_name(game_name) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
            "Invalid game name: ascii abc_123 novel names, try again. "));
        }
    /*

    make files and folders...
    set up and save initial game board

    store date in a json file:

    last_activity = posix timestamp

    // make a game_data json:
    activity_timestamp: posic timestamp
    game_type: chess
    move_number: 0
    set game type = chess

    two gametypes for now

    chess
    chess960
        chess_gomeclock

    */

    // make a game_data json

    // Posix Timestamp for game_timestamp and activity_timestamp
    let this_timestamp: u128 = timestamp();


    // make gamephrase hash: to verify if just the 'unknown' game_phrase is correct
    let hashed_gamephrase = make_hash(game_phrase, this_timestamp, 10);


    /* make ip_hash
    ip_hash made with only half of ip, every other number, so the actual ip is never used at all

    Step 1. the ip_stamp needs to be unique enough to have few collisions between people
    but non-unique enough to not be 'personal data'

    Step 2. user game_timestamp (or what will be used to set game_timestamp) to make hashed_ip_stamp
    */
    let hashed_ip_stamp: u128 = make_hash(&ip_stamp, this_timestamp, 10);
    

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
    let game_board_state:[[char; 8]; 8] = board;


    // // This is now done with game_data struct/impl
    // // Save game (save game_board_state to .txt file)
    // if let Err(e) = save_game_board_state(&game_name, board) {
    //     eprintln!("Failed to save game state: {}", e);
    // }


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

    // create list for initial game_data list
    let ip_hash_list = vec![hashed_ip_stamp];
    


    // create first game_data struct
    let game_data = GameData::new(
        game_name.to_string(),
        hashed_gamephrase, 
        game_type.to_string(), 
        this_timestamp,
        this_timestamp,
        ip_hash_list, 
        game_board_state
    );


    // write
    game_data.to_txt()?;

    Ok(())
}



impl GameData {
    /*
    struct GameData {
    game_name: String,
    hashed_gamephrase: u128,
    game_type: String,
    game_timestamp: u128,
    activity_timestamp: u128,
    ip_hash_list: Vec<u128>,
    game_board_state: [[char; 8]; 8],
    } */

    fn new(
        game_name: String,
        hashed_gamephrase: u128, 
        game_type: String, 
        game_timestamp: u128,
        activity_timestamp: u128,
        ip_hash_list: Vec<u128>, 
        game_board_state: [[char; 8]; 8]
    ) -> Self {
        Self {
            game_name,
            hashed_gamephrase,
            game_type,
            game_timestamp,
            activity_timestamp,
            ip_hash_list,
            game_board_state,
        }
    }

    fn to_txt(&self) -> io::Result<()> {
        let dir_path = format!("./games/{}", self.game_name);

        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/hashed_gamephrase.txt", dir_path))?
            .write_all(self.hashed_gamephrase.to_string().as_bytes())?;
    
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/game_type.txt", dir_path))?
            .write_all(self.game_type.as_bytes())?;
    
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/game_timestamp.txt", dir_path))?
            .write_all(self.game_timestamp.to_string().as_bytes())?;
    
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/activity_timestamp.txt", dir_path))?
            .write_all(self.activity_timestamp.to_string().as_bytes())?;
    
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}/ip_hash_list.txt", dir_path))?
            .write_all(self.ip_hash_list.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ").as_bytes())?;
    
        // Save game (save game_board_state to .txt file)
        std::fs::create_dir_all(&dir_path).expect("Failed to create directory");

        let file_path = format!("{}/game_board_state.txt", dir_path);
        let mut file = File::create(&file_path).expect("Failed to create file");

        for row in &self.game_board_state {
            let line: String = row.iter().collect();
            writeln!(file, "{}", line).expect("Failed to write to file");
        }

        Ok(())
    }

    fn from_txt(dir_path: &str) -> io::Result<Self> {
        let game_name = dir_path.split('/').last().unwrap_or("").to_string();

        let hashed_gamephrase = fs::read_to_string(format!("{}/hashed_gamephrase.txt", dir_path))?
            .trim().parse::<u128>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let game_type = fs::read_to_string(format!("{}/game_type.txt", dir_path))?;

        let game_timestamp = fs::read_to_string(format!("{}/game_timestamp.txt", dir_path))?
            .trim().parse::<u128>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let activity_timestamp = fs::read_to_string(format!("{}/activity_timestamp.txt", dir_path))?
            .trim().parse::<u128>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let ip_hash_list = fs::read_to_string(format!("{}/ip_hash_list.txt", dir_path))?
            .split(',').map(|n| n.trim().parse::<u128>().map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
            .collect::<Result<Vec<_>, _>>()?;

        // let game_board_state_str = fs::read_to_string(format!("{}/game_board_state.txt", dir_path))?;
        // let game_board_state_rows: Vec<Vec<char>> = game_board_state_str.split(',')
        //     .map(|row_str| row_str.chars().collect())
        //     .collect();

        let game_board_state_str = fs::read_to_string(format!("{}/game_board_state.txt", dir_path))?;
        let game_board_state_rows: Vec<Vec<char>> = game_board_state_str.lines()
            .map(|row_str| row_str.chars().collect())
            .collect();

        if game_board_state_rows.len() != 8 || game_board_state_rows.iter().any(|row| row.len() != 8) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "game_board_state must be 8x8"));
        }

        let mut game_board_state: [[char; 8]; 8] = [[' '; 8]; 8];
        for (i, row) in game_board_state_rows.into_iter().enumerate() {
            for (j, c) in row.into_iter().enumerate() {
                game_board_state[i][j] = c;
            }
        }
    
        Ok(Self {
            game_name,
            hashed_gamephrase,
            game_type,
            game_timestamp,
            activity_timestamp,
            ip_hash_list,
            game_board_state,
        })
    }

    fn validate_game_phrase(&self, game_phrase: &str) -> bool {
        // Step 1: get the game_timestamp from the file
        let game_timestamp = self.game_timestamp;

        // Step 2: make hash of game_phrase
        let check_this_hashed_gamephrase = make_hash(game_phrase, game_timestamp, 10);

        // Check if check_this_hashed_gamephrase equals games/game_Name/hashed_gamephrase.txt
        let game_dir = format!("./games/{}", self.game_name);
        let hashed_gamephrase_path = format!("{}/hashed_gamephrase.txt", game_dir);

        if let Ok(contents) = fs::read_to_string(&hashed_gamephrase_path) {
            if contents.trim() == check_this_hashed_gamephrase.to_string() {
                return true;
            }
        }

        eprintln!("Invalid game phrase.");
        false
    }

    fn validate_ip_hash(&self, ip_stamp: &str) -> bool {


        // make ip_hash from game_timestamp and ip_stamp
        let check_this_hashed_ip_stamp: u128 = make_hash(&ip_stamp, self.game_timestamp, 10);

        // Check if a directory with this game_name exists
        let game_dir = format!("./games/{}", self.game_name);
        if !Path::new(&game_dir).exists() {
            eprintln!("error # 3: Game name does not exist.");
            return false;
        }

        // Check if ip_hash is in the ip_hash_list
        if self.ip_hash_list.contains(&check_this_hashed_ip_stamp) {
            return true;
        }

        eprintln!("Invalid IP hash.");
        false
    }

}


// Helper function to validate game_name
fn is_existing_game_name(game_name: &str) -> bool {

    // Check if game_name is a reserved word
    let reserved_words = vec![
        "setup", 
        "games",
        "chess",
        "y0ur_m0ve",
        "start",
        "erase"
        ];
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
    if !Path::new(&game_dir).exists() {
        eprintln!("error # 3: Game name does not exist.");
        return false;
    }

    true
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


fn timestamp() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as u128,
        Err(error) => {
            eprintln!("Error: {}", error);
            0
        }
    }
}



fn remove_duplicates(input_string: &str) -> String {
    // Convert the string into a vector of characters
    let chars: Vec<char> = input_string.chars().collect();

    // Create a new vector, filtering out duplicates
    let mut unique_chars = Vec::new();
    for &c in chars.iter() {
        if !unique_chars.contains(&c) {
            unique_chars.push(c);
        }
    }

    // Convert the vector of characters back into a string
    let unique_string: String = unique_chars.into_iter().collect();

    unique_string
}


fn str_filter_alternating(input_string: &str) -> String {
    // Filter out periods and convert the string into a vector of characters
    let chars: Vec<char> = input_string.chars().filter(|&c| c != '.').collect();

    // Create a new vector containing every other character from the filtered vector
    let mut shorter_chars = Vec::new();
    for (index, &c) in chars.iter().enumerate() {
        if index % 2 == 0 {
            shorter_chars.push(c);
        }
    }

    // Convert the vector of characters back into a string
    let shorter_string: String = shorter_chars.into_iter().collect();

    shorter_string
}




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





// Function to generate the SVG chessboard
fn generate_white_oriented_chessboard(
    chessboard: &[[char; 8]; 8], 
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>
) -> Document {
let mut doc = Document::new()
    .set("width", "500")  // Adjusting the width to account for labels
    .set("height", "500")  // Adjusting the height to account for labels
    .set("viewBox", (0, 0, 500, 500))
    .set("style", "background-color: #000;");  // Set background to black

// Define labels
let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

// Add column labels
for (idx, label) in column_labels.iter().enumerate() {
    let label_text = Text::new()
        .set("x", 50 + idx * 50 + 25)  // Adjusting the x-coordinate to account for labels
        .set("y", 472)  // Positioning the label slightly above the bottom edge
        .set("text-anchor", "middle")
        .set("font-size", 20)
        .set("fill", "#757575")  // Set text color to white
        .add(svg::node::Text::new(label.to_string()));
    doc = doc.add(label_text);
}

// Add row labels
for (idx, label) in row_labels.iter().enumerate() {
    let label_text = Text::new()
        .set("x", 32)  // Positioning the label slightly to the right of the left edge
        .set("y", 50 + idx * 50 + 35)  // Adjusting the y-coordinate to account for labels
        .set("text-anchor", "middle")
        .set("font-size", 20)
        .set("fill", "#757575")  // Set text color to white
        .add(svg::node::Text::new(label.to_string()));
    doc = doc.add(label_text);
}

for (row, row_pieces) in chessboard.iter().enumerate() {
    for (col, &piece) in row_pieces.iter().enumerate() {
        let x = 50 + col * 50;  // Adjusting the x-coordinate to account for labels
        let y = 50 + row * 50;  // Adjusting the y-coordinate to account for labels

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


            // setting from an to color
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


            let piece_color = if square_color == "#ccc" { // for lighter background
                if piece.is_lowercase() {
                    "#9e0b00" // darker red for dark pieces
                } else {
                    "#665628" // darker gray for light pieces
                }
            } else { // for darker background
                if piece.is_lowercase() {
                    "#ff8e8e" // lighter red for dark pieces
                } else {
                    "#ffefc1" // lighter gray for light pieces
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

fn make_hash(input_string: &str, timestamp_int: u128, iterate_hash_x_times: i32) -> u128 {

    let timestamp_string = timestamp_int.to_string();

    // first set the string-to-hash to be the input_string
    let mut string_to_hash = String::from(input_string);

    // then add the timestamp to the string
    string_to_hash.push_str(&timestamp_string);

    // Before starting: set hash to value of 1
    let mut hash: u128 = 1;

    for _ in 0..iterate_hash_x_times {

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
            hash = match hash_str[4..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }
        
        // remove 3 front characters from long hashes
        if hash_str.len() > 30 {
            hash = match hash_str[8..].parse() {
                Ok(parsed_hash) => parsed_hash,
                Err(_) => {
                    eprintln!("Failed to parse hash: {}", hash_str);
                    0 // Set a default value or take appropriate action on parsing failure
                }
            };
        }

    }
    
    } // finished hash 10x loop
    
    // return hash
    hash
}


// fn scrambled_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
//     // use with: let scrambled = scrambled_eggs(array, "SEED_HERE");

//     // Convert seed into a sequence of operations
//     let operations: Vec<(usize, bool)> = seed.chars().map(|c| ((c as usize) % array.len(), c.is_ascii_uppercase())).collect();

//     // Apply operations
//     for (i, &(index, direction)) in operations.iter().enumerate() {
//         if i % 2 == 0 {
//             // Apply operation to rows
//             if direction {
//                 array[index].rotate_left(1);
//             } else {
//                 array[index].rotate_right(1);
//             }
//         } else {
//             // Apply operation to columns
//             let mut column: Vec<_> = array.iter().map(|row| row[index]).collect();
//             if direction {
//                 column.rotate_left(1);
//             } else {
//                 column.rotate_right(1);
//             }
//             for (row, &value) in array.iter_mut().zip(column.iter()) {
//                 row[index] = value;
//             }
//         }
//     }

//     array
// }

// fn unscrambled_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
//     // use with: let unscrambled = unscrambled_eggs(array, "SEED_HERE");

//     // Convert seed into a sequence of operations
//     let operations: Vec<(usize, bool)> = seed.chars().map(|c| ((c as usize) % array.len(), c.is_ascii_uppercase())).collect();

//     // Apply operations in reverse
//     for (i, &(index, direction)) in operations.iter().enumerate().rev() {
//         if i % 2 == 0 {
//             // Apply operation to rows
//             if direction {
//                 array[index].rotate_right(1);
//             } else {
//                 array[index].rotate_left(1);
//             }
//         } else {
//             // Apply operation to columns
//             let mut column: Vec<_> = array.iter().map(|row| row[index]).collect();
//             if direction {
//                 column.rotate_right(1);
//             } else {
//                 column.rotate_left(1);
//             }
//             for (row, &value) in array.iter_mut().zip(column.iter()) {
//                 row[index] = value;
//             }
//         }
//     }

//     array
// }

/*
chess 960 board heuristic

rules:
- no unwrap
- as few extra packages as possible

1. time-seed a random number generator

2. piece_array

make an array 0-7

all set to none/null

or emtpy strings


3. available_position_array

make another array to hold all available position index numbers


3. bishop_1_number

pick any available random even number for first bishop_1_number

put the bishop_1_number in that piece_array spot 'b'

remove that number from the available_position_array



4. bishop_2_number

pick any available random even number for bishop_2_number

put the bishop_2_number in that piece_array spot 'b'

remove that number from the available_position_array


5. king_number
pick a number for king between 1-6 (cannot be at the end of a row)

put the king in that spot 'k'

remove that number from the available_position_array


6. rook1_number
pick any number < the king_number for the first rook
(in range)

put 'r' in that piece_array spot

remove that number from the available_position_array


7. rook2_number
pick any number > the king_number for the 2nd rook

put 'r' in that piece_array spot

remove that number from the available_position_array


8. two 'n' knight numbers

pick any two available numbers, set those

put 'n' in those piece_array spots

remove those numbers from the available_position_array


9. queen

there should be one remaining available_position_array number

put 'q' in that piece_array spot


10. white and black

set white and black pieces mirroring each-other

in this format:
let chessboard_state: [[char; 8]; 8] = [
['r', 'n', 'b', 'q', ' ', 'b', 'n', 'r'],
['p', 'p', 'p', 'p', 'p', ' ', 'p', 'p'],
[' ', ' ', ' ', ' ', ' ', 'k', ' ', ' '],
[' ', ' ', ' ', ' ', ' ', 'p', ' ', ' '],
[' ', ' ', 'P', ' ', ' ', ' ', ' ', ' '],
[' ', ' ', ' ', ' ', ' ', ' ', 'P', 'N'],
['P', 'P', 'P', 'P', 'P', 'P', ' ', 'P'],
['R', 'N', 'B', 'Q', 'K', 'B', ' ', 'R'],
];

fn main() -> Result<(), &'static str> {
    let chessboard = generate_chess960()?;
    for row in chessboard.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    Ok(())
}


*/

use rand::prelude::*;
use std::convert::TryInto;

fn generate_chess960() -> Result<[[char; 8]; 8], &'static str> {
    let mut rng = rand::thread_rng();
    let mut piece_array = [' '; 8];
    let mut available_position_array: Vec<u8> = (0..8).collect();

    // Pick and remove the first bishop's number, then determine its parity
    let bishop_1_idx = rng.gen_range(0..available_position_array.len());
    let bishop_1_number = available_position_array.remove(bishop_1_idx);
    let bishop_1_parity = bishop_1_number % 2;
    piece_array[bishop_1_number as usize] = 'b';

    // Pick and remove the second bishop's number, ensuring that its parity is the opposite of the first
    let mut bishop_2_number;
    let mut bishop_2_idx;
    loop {
        bishop_2_idx = rng.gen_range(0..available_position_array.len());
        bishop_2_number = available_position_array[bishop_2_idx];
        if bishop_2_number % 2 != bishop_1_parity {
            break;
        }
    }
    available_position_array.remove(bishop_2_idx);
    piece_array[bishop_2_number as usize] = 'b';

    // king_number
    let king_idx = rng.gen_range(1..available_position_array.len() - 1);
    let king_number = available_position_array.remove(king_idx);
    piece_array[king_number as usize] = 'k';

    // rook1_number
    let rook1_number = available_position_array.iter().take_while(|&&x| x < king_number).last().unwrap().clone();
    available_position_array.retain(|&x| x != rook1_number);
    piece_array[rook1_number as usize] = 'r';

    // rook2_number
    let rook2_number = available_position_array.iter().skip_while(|&&x| x <= king_number).next().unwrap().clone();
    available_position_array.retain(|&x| x != rook2_number);
    piece_array[rook2_number as usize] = 'r';

    // knights
    for _ in 0..2 {
        let knight_number = available_position_array.remove(0);
        piece_array[knight_number as usize] = 'n';
    }

    // queen
    let queen_number = available_position_array.remove(0);
    piece_array[queen_number as usize] = 'q';

    let mut chessboard_state: [[char; 8]; 8] = [[' '; 8]; 8];
    chessboard_state[0] = piece_array.iter().map(|&p| p.to_ascii_uppercase()).collect::<Vec<_>>().try_into().unwrap();
    chessboard_state[1] = ['P'; 8];
    chessboard_state[6] = ['p'; 8];
    chessboard_state[7] = piece_array;

    Ok(chessboard_state)
}





