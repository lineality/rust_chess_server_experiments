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

$ sudo lsof -n -i :8000 | grep LISTEN
$ sudo lsof -n -i :80 | grep LISTEN
$ kill -9 <PID #####>

for just game_name
show two svg images...black and white...

/*
    let board_orientation: bool = true; // 
    // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
    generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;

    let board_orientation: bool = false; // 
    // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
    generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;
*/

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
use tiny_http::{Server, Request, Response, Method, Header, StatusCode}; 
use std::path::Path;
// use rand::prelude::*;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write, Read, Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::node::element::Image;
use base64::Engine; // Bring the Engine trait into scope
use base64::engine::general_purpose::STANDARD;


use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::Duration;

extern crate image;

use image::{Rgba, ImageBuffer, GenericImageView};
use rand::Rng;
use std::fmt::Debug;
// use std::any::type_name;


use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
// use std::thread;

const RAM_QUEUE_THRESHOLD: usize = 100000;


// use zeroize::Zeroize;

// Variables
type Board = [[char; 8]; 8];

struct GameData {
    game_name: String,
    hashed_gamephrase: u128,
    game_type: String,
    game_timestamp: u128,
    activity_timestamp: u128,
    ip_hash_list: Vec<u128>,
    game_board_state: [[char; 8]; 8],
}

pub struct CleanerState {
    next_check_time: SystemTime, // This is a variable of type `SystemTime`
    expiration_by_name: HashMap<String, SystemTime>,
    names_by_expiration: BTreeMap<SystemTime, Vec<String>>,
}

fn main() {
    /*
    Automated cleaning system
    */
    // Use a constant for the filename
    const NEXT_CHECK_TIME_FILE: &str = "next_check_time.txt";

    // Instantiate CleanerState
    let mut cleaner_state = match CleanerState::new(NEXT_CHECK_TIME_FILE) {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to initialize CleanerState: {}", e);
            return;
        }
    };

    let server = match Server::http("0.0.0.0:8000") {
        Ok(server) => server,
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            return;
        }
    };

    println!("Server >*< trench runnnnnnning at http://0.0.0.0:8000 |o| |o| ");


    /*
    Hybrid Queue for Managing Requests
    */

    // Create the in-memory queue as a thread-safe data structure using Arc and Mutex
    let in_memory_queue: Arc<Mutex<VecDeque<Request>>> = Arc::new(Mutex::new(VecDeque::new()));
    loop {
        // Handle incoming requests
        let incoming_request = match server.recv() {
            Ok(request) => request,
            Err(e) => {
                eprintln!("Failed to receive request: {}", e);
                continue;
            }
        };

        // Push the incoming request to the in-memory queue
        {
            let mut queue = in_memory_queue.lock().unwrap();
            queue.push_back(incoming_request);
        }
        

        // Check if the in-memory queue size exceeds the threshold, and if so, write a batch to disk
        if let Ok(queue) = in_memory_queue.lock() {
            if queue.len() >= RAM_QUEUE_THRESHOLD {
                // Start a new thread to write the batch to disk asynchronously
                // let in_memory_queue_clone = in_memory_queue.clone();
                // if let Err(e) = thread::spawn(move || {
                //     write_batch_to_disk(in_memory_queue_clone);
                // }).join() {
                //     eprintln!("Failed to write batch to disk: {:?}", e);
                // }
                if let Err(e) = write_batch_to_disk(in_memory_queue.clone()) {
                    eprintln!("Error writing batch to disk: {:?}", e);
                }
                // Clear the in-memory queue after writing to disk
                if let Ok(mut queue) = in_memory_queue.lock() {
                    queue.clear();
                } else {
                    eprintln!("Failed to clear in-memory queue.");
                }
            }
        } else {
            eprintln!("Failed to lock in-memory queue.");
        }

        // Process requests in the in-memory queue
        process_in_memory_requests(&in_memory_queue, &mut cleaner_state);
    }
}




// fn process_in_memory_requests(in_memory_queue: &Arc<Mutex<VecDeque<Request>>>) {
fn process_in_memory_requests(in_memory_queue: &Arc<Mutex<VecDeque<Request>>>, cleaner_state: &mut CleanerState) {

    let mut queue = in_memory_queue.lock().unwrap();
    // Process requests in the in-memory queue
    while let Some(request) = queue.pop_front() {


        // Implement your request processing logic here
        if request.method() == &Method::Get {
            // let url_parts: Vec<&str> = request.url().split('/').collect();

            let url = request.url();
            let sanitized_url = sanitize_url(url);
            let url_parts: Vec<&str> = sanitized_url.split('/').collect();

            let is_favicon_request = url_parts.get(1).map_or(false, |part| *part == "favicon.ico");
            
            if is_favicon_request {
                let path = "favicon.ico";
                let response = match File::open(&path) {
                    Ok(mut file) => {
                        let mut content = Vec::new();
                        if file.read_to_end(&mut content).is_err() {
                            Response::from_data(Vec::new()).with_status_code(StatusCode(500))
                        } else {
                            let mut response = Response::from_data(content);
                            response.add_header(Header::from_bytes(&b"Content-Type"[..], &b"image/x-icon"[..]).unwrap());
                            response
                        }
                    }
                    Err(_) => Response::from_data(Vec::new()).with_status_code(StatusCode(404)),
                };
                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
                return; // Return early to prevent further processing for this request.
            }


            /*
            Server Here
            for request in server.incoming_requests() {
            */ 

            // Terminal Inspection of Request
            println!("url_parts.len: {}",url_parts.len());

            // Print the whole url_parts list as a single string
            let url_parts_string = url_parts.join(", ");
            println!("url_parts: [{}]", url_parts_string);

            // Iterate over url_parts and print each item
            for (i, part) in url_parts.iter().enumerate() {
                println!("url_parts[{}]: {}", i, part);
            }

            // process update expiration dates of projects
            process_url_and_update_expiration(&url_parts, cleaner_state);


            // get reduced ip_stamp rather than whole ip
            let ip_stamp = match request.remote_addr() {
                Some(socket_addr) => {
                    
                    let ip_string = socket_addr.ip().to_string();
                    println!("ip_string: {}", ip_string);
                    let alternating_stamp = str_filter_alternating(&ip_string);
                    let reduced_ip_stamp = remove_duplicates(&alternating_stamp);

                    reduced_ip_stamp

                },
                None => {
                    // println!("Could not retrieve client IP");
                    continue;
                },
                 
            };

            // Testing Only
            println!("ip_stamp: {}", ip_stamp);
            



            
            // Continue processing other requests...
            
            



            /////////////////////
            // site landing page
            /////////////////////

            if url_parts.len() == 1 || url_parts[1] == "" {

                // inspection
                println!{" starting ->  site landing pageif url_parts.len() == 1 || url_parts[0] "};

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


            ////////////////////////
            // game landing pages
            // and meta-tag images
            ///////////////////////
            else if url_parts.len() == 2 {

                // inspection
                println!(" starting ->   game landing pages pages url_parts.len == 2  ");

                let game_name = url_parts[1].to_string();
                println!("{}",url_parts[0].to_string());
                println!("{}",url_parts[1].to_string());
                println!("{}",game_name);


                // Docs
                if game_name == "docs" {
                    // inspection
                    // println!{" docs ->  "};

                    let response = std::fs::read_to_string("docs.txt").map_or_else(
                        |e| {
                            eprintln!("Failed to read docs: {}", e);
                            Response::from_string(format!("Failed to read docs: {}", e)).with_status_code(500)
                        },
                        |docs_string| {
                            Response::from_string(docs_string).with_status_code(200)
                        },
                    );

                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // No need to run the rest of the loop for the docs page
                }



                // 960
                if game_name == "960" {
                    let result = generate_new_chess960_board_only();
                    let header_result = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..]);
                
                    let response = match result {
                        Ok(document) => {
                            // Convert the SVG Document into a String
                            let svg_content = document.to_string();
                
                            match header_result {
                                Ok(header) => Response::from_string(svg_content).with_header(header).with_status_code(200),
                                Err(_) => {
                                    eprintln!("Invalid header!");
                                    Response::from_string("Internal Server Error").with_status_code(500)
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to handle move: {}", e);
                            Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                        }
                    };
                
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // No need to run the rest of the loop for the docs page
                }



            


                /* 
                note: separate game vs. meta-tag request
                */

                // if let Some(stripped_name) = game_name.strip_prefix("metatag_") {
                //     let new_game_name = &stripped_name[..stripped_name.len() - 4];
                //     // then proceed with the rest of your logic using new_game_name
                // }



                /* 
                If url is image_gamename
                */
                if let Some(stripped_name) = game_name.strip_prefix("image_") {
                    let new_game_name = &stripped_name[..stripped_name.len() - 4];

                    let new_game_name = new_game_name.to_string();
                    // then proceed with the rest of your logic using new_game_name
                
                    // // // declare response outside match blocks so we can assign it in each match block
                    // let response = Response::from_string(response_string);
                    if is_existing_game_name(&new_game_name) {
                        // println!("Game exists, proceed with the game logic.");
                    } else {
                        println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
                    }

                    // // // call game move function
                    // let response = match show_board(new_game_name) {
                    //     Ok(svg_content) => {
                    //         let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                    //             .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                    
                    //         Response::from_string(svg_content).with_header(header).with_status_code(200)
                    //     },
                    //     Err(e) => {
                    //         eprintln!("Failed to handle move: {}", e);
                    //         Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                    //     }
                    // };

                    let response = match show_board_png(&new_game_name) { // Assuming this function returns the PNG content
                        Ok(png_content) => {
                            // Create a Header for the PNG content type
                            let header = Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..])
                                .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                            
                            // Create the Response with the PNG content, header, and status code
                            Response::from_data(png_content).with_header(header).with_status_code(200)
                        },
                        Err(e) => {
                            eprintln!("Failed to handle move: {}", e);
                            Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                        }
                    };


                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // No need to run the rest of the loop;
                }
                    


                /* 
                If url is metatag_gamename:
                if game_name.starts_with("metatag_") {
                    let new_game_name = game_name.trim_start_matches("metatag_").to_string();

                if game_name.starts_with("metatag_") {
                    let new_game_name = game_name.trim_start_matches("metatag_").to_string();


                */
                if let Some(stripped_name) = game_name.strip_prefix("metatag_") {
                    let new_game_name = &stripped_name[..stripped_name.len() - 4];
                    let new_game_name = new_game_name.to_string();
                    // then proceed with the rest of your logic using new_game_name
                
                    // // // declare response outside match blocks so we can assign it in each match block
                    // let response = Response::from_string(response_string);
                    if is_existing_game_name(&new_game_name) {
                        // println!("Game exists, proceed with the game logic.");
                    } else {
                        println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
                    }

                    // // // svg call game move function
                    // let response = match show_board(new_game_name) {
                    //     Ok(svg_content) => {
                    //         let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                    //             .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                    
                    //         Response::from_string(svg_content).with_header(header).with_status_code(200)
                    //     },
                    //     Err(e) => {
                    //         eprintln!("Failed to handle move: {}", e);
                    //         Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                    //     }
                    // };


                    let response = match show_board_png(&new_game_name) { // Assuming this function returns the PNG content
                        Ok(png_content) => {
                            // Create a Header for the PNG content type
                            let header = Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..])
                                .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                            
                            // Create the Response with the PNG content, header, and status code
                            Response::from_data(png_content).with_header(header).with_status_code(200)
                        },
                        Err(e) => {
                            eprintln!("Failed to handle move: {}", e);
                            Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                        }
                    };
                    
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // No need to run the rest of the loop;
                }


                /* 
                If just game_name:
                */

                else {
                    if !is_existing_game_name(&game_name) {
                        println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
                        // Handle error or continue
                    }
                
                    let response = match show_board(game_name.clone()) {
                        Ok(svg_content) => {
                            let html_content = format!(r#"
                            <!DOCTYPE html>
                            <head>
                            <meta property="og:title" content="Current Game Board" />
                            <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
                            </head>
                            <html>
                                <body style="background-color:black;">
                                    <br>
                                    <div>{}</div>
                                </body>
                            </html>
                            "#, game_name, svg_content);
                
                            match Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]) {
                                Ok(header) => Response::from_string(html_content).with_header(header).with_status_code(200),
                                Err(_) => Response::from_string("Failed to create header").with_status_code(500),
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to handle move: {}", e);
                            Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                        }
                    };
                
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    // continue; // No need to run the rest of the loop;
                }
                
                    
                    // return an html response with with 
                // { // Clone game_name


                //     the new version needs to use this html AND the svg from a file

                //     Ok(_) => { // Changed svg_content to _
                //         let html_content = format!(r#"
                //         <!DOCTYPE html>
                //         <head>
                //         <meta property="og:title" content="Current Game Board" />
                //         <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
                //         </head>
                //         <html>
                //             <body style="background-color:black;">
                //                 <br>
                //                 <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
                //             </body>
                //         </html>
                //         "#, game_name, game_name);
                
                //         match Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]) {
                //             Ok(header) => Response::from_string(html_content).with_header(header).with_status_code(200),
                //             Err(_) => Response::from_string("Failed to create header").with_status_code(500),
                //         }
                //     },
                //     // Err(e) => {
                //     //     eprintln!("Failed to handle move: {}", e);
                //     //     Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                //     // }
                // };
                
            
                // )
                // (old old version with no html)
                // // // // declare response outside match blocks so we can assign it in each match block
                // // let response = Response::from_string(response_string);
                // if is_existing_game_name(&game_name) {
                //     // println!("Game exists, proceed with the game logic.");
                // } else {
                //     println!("none such games: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
                // }

                // // // call game move function
                // let response = match show_board(game_name) {
                //     Ok(svg_content) => {
                //         let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                //             .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                
                //         Response::from_string(svg_content).with_header(header).with_status_code(200)
                //     },
                //     Err(e) => {
                //         eprintln!("Failed to handle move: {}", e);
                //         Response::from_string(format!("Failed to show_board: {}", e)).with_status_code(500)
                //     }
                // };
                

                // if let Err(e) = request.respond(response) {
                //     eprintln!("Failed to respond to request: {}", e);
                // }

            
                continue; // No need to run the rest of the loop;
                }




            /////////
            // m0ve
            ////////
            else if url_parts.len() == 3 {

                // inspection
                println!(" starting ->   m0ve else if url_parts.len() == 3 ");

                // if 'start' reset and return blank board
                // if erase delete game folder


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


                // // // raw image call game move function
                // let response = match handle_chess_move(game_name, move_data) {
                //     Ok(svg_content) => {
                //         let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                //             .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                
                //         Response::from_string(svg_content).with_header(header).with_status_code(200)
                //     },
                //     Err(e) => {
                //         eprintln!("Failed to handle move: {}", e);
                //         Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                //     }
                // };
                

                // if let Err(e) = request.respond(response) {
                //     eprintln!("Failed to respond to request: {}", e);
                // }


                let response = match handle_chess_move(game_name.clone(), move_data) { // Clone game_name
                    Ok(_) => { // Changed svg_content to _
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
                            </body>
                        </html>
                        "#, game_name, game_name);
                
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
                
                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request: {}", e);
                }
                

                // let response = match handle_chess_move(game_name.clone(), move_data) { // Clone game_name
                //     Ok(svg_content) => { // Change back to svg_content
                //         let html_content = format!(r#"
                //         <!DOCTYPE html>
                //         <html>
                //             <body style="background-color:black;">
                //                 <br>
                //                 <div style="width:50px; height:50px;">{}</div>
                //             </body>
                //         </html>
                //         "#, svg_content); // Insert SVG content directly
                
                //         match Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]) {
                //             Ok(header) => Response::from_string(html_content).with_header(header).with_status_code(200),
                //             Err(_) => Response::from_string("Failed to create header").with_status_code(500),
                //         }
                //     },
                //     Err(e) => {
                //         eprintln!("Failed to handle move: {}", e);
                //         Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                //     }
                // };
                
                // if let Err(e) = request.respond(response) {
                //     eprintln!("Failed to respond to request: {}", e);
                // }
                




            } 


            //////////////
            // login m0ve
            //////////////
            else if url_parts.len() == 4 {

                // inspection
                println!(" starting ->  // login m0ve else if url_parts.len() == 4");

                let game_name = url_parts[1].to_string();
                let move_data = url_parts[2].to_string();  
                let game_phrase = url_parts[3].to_string();  

                // if is_existing_game_name(&game_name) {
                //     // println!("Game exists, proceed with the game logic.");
                // } else {
                //     println!("none such game_name and or game_phrase: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE");
                // }

                let is_existing_game = is_existing_game_name(&game_name);

                if is_existing_game {
                    println!("Game name is valid.");
                    // add ip_hash to list...
                } else {
                    let err_msg = "none such game_name and or game_phrase: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE";
                    println!("{}", err_msg);
                    let response = Response::from_string(err_msg).with_status_code(400);
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // Skip the rest of the loop for this request
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
                    let err_msg = "none such game_name and or game_phrase: y0urm0ve.com/setup/chess/YOUR_GAME_NAME/YOUR_GAME_PHRASE";
                    println!("{}", err_msg);
                    let response = Response::from_string(err_msg).with_status_code(400);
                    if let Err(e) = request.respond(response) {
                        eprintln!("Failed to respond to request: {}", e);
                    }
                    continue; // Skip the rest of the loop for this request
                }

                // if is_valid {
                //     println!("Game phrase is valid.");
                //     // add ip_hash to list...


                // } else {
                //     println!("Game phrase is not valid.");
                // }

                println!("move_data -> {}", move_data);
                if move_data == "start" {
                    match restore_original_game_board_state(&game_name) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Failed to restore game board state: {}", e),
                    }
                    continue;
                }
                
                if move_data == "erase" {
                    match remove_directory(&game_name) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Failed to remove directory: {}", e),
                    }
                    continue;
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
                // let response = match handle_chess_move(game_name, move_data) {
                //     Ok(svg_content) => {
                //         let header = Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..])
                //             .unwrap_or_else(|_| panic!("Invalid header!")); // This is a placeholder; replace it with an appropriate error handling.
                
                //         Response::from_string(svg_content).with_header(header).with_status_code(200)
                //     },
                //     Err(e) => {
                //         eprintln!("Failed to handle move: {}", e);
                //         Response::from_string(format!("Failed to handle move: {}", e)).with_status_code(500)
                //     }
                // };
                

                // if let Err(e) = request.respond(response) {
                //     eprintln!("Failed to respond to request: {}", e);
                // }
                


                // html call game move function
                let response = match handle_chess_move(game_name.clone(), move_data) { // Clone game_name
                    Ok(_) => { // Changed svg_content to _
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
                            </body>
                        </html>
                        "#, game_name, game_name);
                
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
                    continue;    
                    }
                } else {
                    let game_type = url_parts[2].to_string();
                    let game_name = url_parts[3].to_string();
                    let game_phrase = url_parts[4].to_string();

                    // Call e.g. setup_new_png_chess_game here

                    // Check the game type and call the corresponding function

                    // Check the game type and call the corresponding function
                    let response = if game_type == "chess960" {
                        match setup_new_chess960_game(&game_type, &game_name, &game_phrase, &ip_stamp) {
                            Ok(_) => {
                                // Load SVG content from file
                                match std::fs::read_to_string(format!("games/{}/board.svg", game_name)) {
                                    Ok(svg_content) => {
                                        // Create header
                                        match Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..]) {
                                            Ok(header) => Response::from_string(svg_content).with_header(header).with_status_code(200),
                                            Err(_) => Response::from_string("Failed to create response header.").with_status_code(500),
                                        }
                                    },
                                    Err(_) => Response::from_string("Failed to read SVG file.").with_status_code(500),
                                }
                            },
                            Err(e) => Response::from_string(format!("Failed to set up Chess960 game: {}", e)).with_status_code(500),
                        }
                    } else if game_type == "chess" {
                        match setup_new_png_chess_game(&game_type, &game_name, &game_phrase, &ip_stamp) {
                            Ok(_) => {
                                // Load SVG content from file
                                match std::fs::read_to_string(format!("games/{}/board.svg", game_name)) {
                                    Ok(svg_content) => {
                                        // Create headerfsetu
                                        match Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..]) {
                                            Ok(header) => Response::from_string(svg_content).with_header(header).with_status_code(200),
                                            Err(_) => Response::from_string("Failed to create response header.").with_status_code(500),
                                        }
                                    },
                                    Err(_) => Response::from_string("Failed to read SVG file.").with_status_code(500),
                                }
                            },
                            Err(e) => Response::from_string(format!("Failed to set up Chess game: {}", e)).with_status_code(500),
                        }
                    } else if game_type == "chesssvg" {
                        match setup_new_svg_chess_game(&game_type, &game_name, &game_phrase, &ip_stamp) {
                            Ok(_) => {
                                // Load SVG content from file
                                match std::fs::read_to_string(format!("games/{}/board.svg", game_name)) {
                                    Ok(svg_content) => {
                                        // Create headerfsetu
                                        match Header::from_bytes(&b"Content-Type"[..], &b"image/svg+xml"[..]) {
                                            Ok(header) => Response::from_string(svg_content).with_header(header).with_status_code(200),
                                            Err(_) => Response::from_string("Failed to create response header.").with_status_code(500),
                                        }
                                    },
                                    Err(_) => Response::from_string("Failed to read SVG file.").with_status_code(500),
                                }
                            },
                            Err(e) => Response::from_string(format!("Failed to set up Chess game: {}", e)).with_status_code(500),
                        }
                    } else {
                        Response::from_string("Invalid game type.").with_status_code(400)
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

            y0urm0ve.com/
             setup/
              chess/
               GAME_NAME/
                GAME_PHRASE 

            y0ur
             m0ve.com/
              GAME_NAME/
               Pc2c4

            y0ur
               m0ve.
                   com/docs
               

            "#.to_string();

            // return response string
            Ok(response_string)
        }

    fn handle_chess_move(game_name: String, move_data: String) -> Result<String, Box<dyn std::error::Error>> {

        let mut response_string = String::new();


        // File Setup
        let dir_path = format!("./games/{}", game_name);

        std::fs::create_dir_all(&dir_path).expect("Failed to create directory");

        // let file_path = format!("{}/moves.csv", dir_path);

        // let file = match OpenOptions::new()
        // .write(true)
        // .create(true)
        // .append(true)
        // .open(&file_path) {
        // Ok(file) => file,
        // Err(e) => {
        //     return Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::Other, 
        //         format!("Failed to open file: {}", e))));
        // }};

        // let mut wtr = csv::Writer::from_writer(file);


        // // Write new move to CSV file
        // if let Err(e) = wtr.write_record(&[move_data.clone()]) {
        //     return Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::Other, format!("Failed to write to file: {}", e))));
        // }

        // if let Err(e) = wtr.flush() {
        //     return Err(Box::new(std::io::Error::new(
        //         std::io::ErrorKind::Other, format!("Failed to flush writer: {}", e))));
        // }

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
                let from_coords_result = to_coords_chess(&format!("{}{}", from.0, from.1));
                let from_coords = from_coords_result?;
                let (x, y) = from_coords;
                board[x][y] = ' ';
                from_x_y_coordinates = from_coords;


                // "TO" moves
                let to_coords_chess_result = to_coords_chess(&format!("{}{}", to.0, to.1));
                let to_coords_chess = to_coords_chess_result?;
                let (x, y) = to_coords_chess;
                board[x][y] = piece;
                to_x_y_coordinates = to_coords_chess;


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

                // generate png
                // Assuming from and to are already defined as (char, u8)
                // let from_option: Option<(usize, usize)> = Some((from.1 as usize - 1, (from.0 as u8 - b'a') as usize));
                // let to_option: Option<(usize, usize)> = Some((to.1 as usize - 1, (to.0 as u8 - b'a') as usize));


                // let from_option: Option<(usize, usize)> = Some(((from.0 as u8 - b'a') as usize, from.1 as usize - 1));
                // let to_option: Option<(usize, usize)> = Some(((to.0 as u8 - b'a') as usize, to.1 as usize - 1));
                
                dbg!("from -> ", from);
                dbg!("to -> ", to);

                // // // Flip the row for the PNG coordinate system
                // let from_png_row = 7 - (from.1 as usize - 1);
                // let to_png_row = 7 - (to.1 as usize - 1);

                // let from_option: Option<(usize, usize)> = Some((from_png_row, (from.0 as u8 - b'a') as usize));
                // let to_option: Option<(usize, usize)> = Some((to_png_row, (to.0 as u8 - b'a') as usize));


                // Now pass these converted options to your function
                generate_png_chess_board(&board, &game_name, from, to, !is_black)?;


                // Generate png with board_string
                // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
                // generate_png_chess_board(&board, &game_name, from, to, !is_black)?;




                // Generate SVG with these coordinates
                // Inverting the coordinates for black's perspective
                let from_black_oriented = (7 - from_x_y_coordinates.0, 7 - from_x_y_coordinates.1);
                let to_black_oriented = (7 - to_x_y_coordinates.0, 7 - to_x_y_coordinates.1);

                let doc = if is_black {
                    generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented))
                } else {
                    generate_white_oriented_chessboard(&board, Some(from_x_y_coordinates), Some(to_x_y_coordinates))  
                };

                // Generate SVG with these coordinates
                // let doc = generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented));

                // Define the file name
                // let file_name = "board.svg", game_name;
                let file_path = format!("games/{}/board.svg", game_name);


                // Write the svg code to the file
                svg::save(file_path, &doc).expect("Unable to write to file");

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



    fn show_board(game_name: String) -> Result<String, Box<dyn std::error::Error>> {

        // File Setup
        let dir_path = format!("./games/{}", game_name);

        std::fs::create_dir_all(&dir_path).expect("Failed to create directory");

                // Generate SVG with these coordinates
                // let doc = generate_black_oriented_chessboard(&board, Some(from_black_oriented), Some(to_black_oriented));

                // Define the file name
                // let file_name = "board.svg", game_name;
                let file_path = format!("games/{}/board.svg", game_name);


                // Read the content of the SVG file
                let svg_content = std::fs::read_to_string(&file_path)?;

                Ok(svg_content)

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


    fn to_coords_chess(chess_notation: &str) -> Result<(usize, usize), String> {
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

    fn save_original_game_board_state(game_name: &str, board: [[char; 8]; 8]) -> io::Result<()> {
        let dir_path = format!("./games/{}", game_name);
        std::fs::create_dir_all(&dir_path).expect("Failed to create directory");

        let file_path = format!("{}/original_state.txt", dir_path);
        let mut file = File::create(&file_path).expect("Failed to create file");

        for row in &board {
            let line: String = row.iter().collect();
            writeln!(file, "{}", line).expect("Failed to write to file");
        }
        
        Ok(())
    }


    // fn restore_original_game_board_state(game_name: &str) -> io::Result<()> {
    //     // Define the file paths
    //     let dir_path = format!("./games/{}", game_name);
    //     let original_state_file_path = format!("{}/original_state.txt", dir_path);
    //     let game_board_state_file_path = format!("{}/game_board_state.txt", dir_path);
    
    //     // Read the original state file
    //     let original_state = std::fs::read_to_string(&original_state_file_path)?;
    
    //     // Write the contents of the original state file to the game board state file
    //     std::fs::write(game_board_state_file_path, original_state)?;


    //     // Convert the original_state string into a board
    //     let board = string_to_board(&original_state);

    //     // Generate and save SVG using the board
    //     generate_and_save_svg(&board, game_name)?;

    //     println!("Done! restore_original_game_board_state");
    //     Ok(())
    // }
    
    fn remove_directory(game_name: &str) -> io::Result<()> {
        
        // Define the directory path
        let dir_path = format!("./games/{}", game_name);
    
        // Remove the directory and all its contents
        std::fs::remove_dir_all(dir_path)?;
    
        println!("Done! remove_directory");

        Ok(())
    }
    
    fn restore_original_game_board_state(game_name: &str) -> io::Result<()> {
        // Define the file paths
        let dir_path = format!("./games/{}", game_name);
        let original_state_file_path = format!("{}/original_state.txt", dir_path);
        let game_board_state_file_path = format!("{}/game_board_state.txt", dir_path);
    
        // Read the original state file
        let original_state = std::fs::read_to_string(&original_state_file_path)?;
    
        // Write the contents of the original state file to the game board state file
        std::fs::write(game_board_state_file_path.clone(), original_state.clone())?;
    
        // Convert the original_state string into a board
        let board_result = load_game_board_state(&game_name);
        match board_result {
            Ok(board) => {
                // svg in game_name directory
                generate_and_save_svg(&board, &game_name)?;
                println!("Done! restore_original_game_board_state");
            },
            Err(e) => {
                return Err(e)
            }
        };
        Ok(())
    }



    // fn restore_original_game_board_state(game_name: &str) -> io::Result<()> {
    //     // Define the file paths
    //     let dir_path = format!("./games/{}", game_name);
    //     let original_state_file_path = format!("{}/original_state.txt", dir_path);
    //     let game_board_state_file_path = format!("{}/game_board_state.txt", dir_path);
    
    //     // Read the original state file
    //     let original_state = std::fs::read_to_string(&original_state_file_path)?;
    
    //     // Convert the original_state string into a board
    //     let board = load_game_board_state(&original_state);
        
    //     // Write the contents of the original state file to the game board state file
    //     std::fs::write(game_board_state_file_path, &original_state)?;
    
    //     // Generate and save SVG using the board
    //     generate_and_save_svg(&board, game_name)?;
    
    //     println!("Done! restore_original_game_board_state");
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




    // fn generate_new_chess960_board_only(game_type: &str, game_name: &str, game_phrase: &str, ip_stamp: &str) -> std::io::Result<Document> {

    //     /*
    //     make and return new svg chess960 board
    //     */


    //     // Set up board
    //     let game_board_state_result = generate_chess960();
    //     let game_board_state = match game_board_state_result {
    //         Ok(board) => board,
    //         Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    //     };

    //     // generate board
    //     let doc = generate_white_oriented_chessboard(&game_board_state, None, None);
    

    //     Ok(doc)
    // }

    fn generate_new_chess960_board_only() -> Result<Document, &'static str> {
        // Generate the Chess960 board
        let chessboard_result = generate_chess960();
        let chessboard = match chessboard_result {
            Ok(board) => board,
            Err(err) => return Err(err),
        };
    
        // Generate the SVG representation of the board
        let doc = generate_white_oriented_chessboard(&chessboard, None, None);
    
        Ok(doc)
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

        // svg in game_name directory
        generate_and_save_svg(&game_board_state, game_name)?;

        let board_orientation: bool = true; // 
        generate_chessboard_backboards(game_name, board_orientation)?;
    
        let board_orientation: bool = false; // 
        generate_chessboard_backboards(game_name, board_orientation)?;
    
        let board_orientation: bool = true; // 
        // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
        no_move_create_chessboard_with_pieces(&game_board_state, game_name, board_orientation)?;


        // // This is now done with game_data struct/impl
        // // Save game (save game_board_state to .txt file)
        if let Err(e) = save_original_game_board_state(&game_name, game_board_state) {
            eprintln!("Failed to save game state: {}", e);
        }

        Ok(())
    }


    fn setup_new_png_chess_game(game_type: &str, game_name: &str, game_phrase: &str, ip_stamp: &str) -> std::io::Result<()> {

        // Validate game_name: novel, permitted, ascii etc.
        if !is_valid_game_name(game_name) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                "Invalid game name: ascii abc_123 novel names, try again. "));
            }
        /*

        make files and folders...
        set up and save initial game board
        data in struct, separate files
        
        */

        let board_orientation: bool = true; // 
        generate_chessboard_backboards(game_name, board_orientation)?;
    
        let board_orientation: bool = false; // 
        generate_chessboard_backboards(game_name, board_orientation)?;
    

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
        if let Err(e) = save_original_game_board_state(&game_name, board) {
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


        // generate png first board

        // Set up board
        // let board: [[char; 8]; 8] = [
        //     ['r', 'n', 'b', 'q', 'k', 'b', ' ', 'r'],
        //     ['p', 'p', 'p', 'p', ' ', 'p', 'p', 'p'],
        //     [' ', ' ', ' ', ' ', ' ', 'n', ' ', ' '],
        //     [' ', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
        //     [' ', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
        //     [' ', ' ', 'N', ' ', ' ', ' ', ' ', ' '],
        //     ['P', ' ', 'P', 'P', 'P', 'P', 'P', 'P'],
        //     ['R', ' ', 'B', 'Q', 'K', 'B', 'N', 'R']
        // ];
        // let game_board_state:[[char; 8]; 8] = board;

        let board_orientation: bool = true; // 
        // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
        no_move_create_chessboard_with_pieces(&game_board_state, game_name, board_orientation)?;

        // // Generate SVG
        // let doc = generate_white_oriented_chessboard(&game_board_state, None, None);

        // // Define the file path
        // let svg_file_path = format!("games/{}/board.svg", game_name);

        // // Write the SVG code to the file
        // svg::save(svg_file_path, &doc).expect("Unable to write SVG to file");

        // Generate and save SVG
        generate_and_save_svg(&game_board_state, game_name)?;

        Ok(())
    }



    fn setup_new_svg_chess_game(game_type: &str, game_name: &str, game_phrase: &str, ip_stamp: &str) -> std::io::Result<()> {

        // Validate game_name: novel, permitted, ascii etc.
        if !is_valid_game_name(game_name) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                "Invalid game name: ascii abc_123 novel names, try again. "));
            }
        /*

        make files and folders...
        set up and save initial game board
        data in struct, separate files
        
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
        if let Err(e) = save_original_game_board_state(&game_name, board) {
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

        // // Generate SVG
        // let doc = generate_white_oriented_chessboard(&game_board_state, None, None);

        // // Define the file path
        // let svg_file_path = format!("games/{}/board.svg", game_name);

        // // Write the SVG code to the file
        // svg::save(svg_file_path, &doc).expect("Unable to write SVG to file");

        // Generate and save SVG
        generate_and_save_svg(&game_board_state, game_name)?;

        Ok(())
    }


    // fn generate_and_save_svg(board: &[[char; 8]; 8], game_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     // Generate SVG
    //     let doc = generate_white_oriented_chessboard(board, None, None);
    
    //     // Define the file path
    //     let svg_file_path = format!("games/{}/board.svg", game_name);
    
    //     // Write the SVG code to the file
    //     svg::save(svg_file_path, &doc).map_err(|e| e.into())
    // }
    
    fn generate_and_save_svg(board: &[[char; 8]; 8], game_name: &str) -> std::io::Result<()> {
        // Generate SVG
        let doc = generate_white_oriented_chessboard(board, None, None);
    
        // Define the file path
        let svg_file_path = format!("games/{}/board.svg", game_name);
    
        // Write the SVG code to the file
        svg::save(svg_file_path, &doc).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
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
            "settings",
            "security",
            "docs",
            "y0ur_m0ve",
            "start",
            "erase",
            "metatag"
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

    fn generate_white_oriented_chessboard(
        chessboard: &[[char; 8]; 8], 
        from: Option<(usize, usize)>, 
        to: Option<(usize, usize)>
    ) -> Document {
    
        let mut doc = Document::new()
            .set("width", "1000")  // Adjusting the width to account for labels
            .set("height", "1000")  // Adjusting the height to account for labels
            .set("viewBox", (0, 0, 1000, 1000))
            .set("style", "background-color: #000;");  // Set background to black
    
        // Define labels
        let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top
    
        // Add column labels
        for (idx, label) in column_labels.iter().enumerate() {
            let label_text = Text::new()
                .set("x", 100 + idx * 100 + 50)  // Adjusting the x-coordinate to account for labels
                .set("y", 944)  // Positioning the label slightly above the bottom edge
                .set("text-anchor", "middle")
                .set("font-size", 40)
                .set("fill", "#757575")  // Set text color to white
                .add(svg::node::Text::new(label.to_string()));
            doc = doc.add(label_text);
        }
    
        // Add row labels
        for (idx, label) in row_labels.iter().enumerate() {
            let label_text = Text::new()
                .set("x", 64)  // Positioning the label slightly to the right of the left edge
                .set("y", 100 + idx * 100 + 70)  // Adjusting the y-coordinate to account for labels
                .set("text-anchor", "middle")
                .set("font-size", 40)
                .set("fill", "#757575")  // Set text color to white
                .add(svg::node::Text::new(label.to_string()));
            doc = doc.add(label_text);
        }
    
        for (row, row_pieces) in chessboard.iter().enumerate() {
            for (col, &piece) in row_pieces.iter().enumerate() {
                let x = 100 + col * 100;  // Adjusting the x-coordinate to account for labels
                let y = 100 + row * 100;  // Adjusting the y-coordinate to account for labels
    
                let square_color = if (row + col) % 2 == 0 {
                    "#ccc"
                } else {
                    "#666"
                };
    
                let square = Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", 100)
                    .set("height", 100)
                    .set("fill", square_color);
    
                doc = doc.add(square);
    
                if piece != ' ' {
    
                    // setting from an to color
                    if let Some(from_coords) = from {
                        let (row, col) = from_coords;
                        let x = 100 + col * 100;
                        let y = 100 + row * 100;
    
                        let highlight = Rectangle::new()
                            .set("x", x)
                            .set("y", y)
                            .set("width", 100)
                            .set("height", 100)
                            .set("fill", "none") // Transparent fill
                            .set("stroke", "#3189D9")
                            .set("stroke-width", 6);
    
                        doc = doc.add(highlight);
                    }
    
                    if let Some(to_coords_chess) = to {
                        let (row, col) = to_coords_chess;
                        let x = 100 + col * 100;
                        let y = 100 + row * 100;
    
                        let highlight = Rectangle::new()
                            .set("x", x)
                            .set("y", y)
                            .set("width", 100)
                            .set("height", 100)
                            .set("fill", "none") // Transparent fill
                            .set("stroke", "#3189D9")
                            .set("stroke-width", 6);
    
                        doc = doc.add(highlight);
                    }
    
                    // map character to piece name and background
                    let (piece_name, background) = match piece {
                        'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        _   => panic!("Unknown piece"),
                    };
    
                    // Load SVG chess piece based on piece name and background
                    let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
                    let data_url = load_image_as_data_url(&file_path)
                        .expect("Failed to load image as data URL");
    
                    let piece_image = Image::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 100)
                        .set("height", 100)
                        .set("href", data_url);
    
                    doc = doc.add(piece_image);
                }
            }
        }
    
        doc
    }
    

    // // Function to generate the SVG chessboard
    // fn generate_white_oriented_chessboard_small(
    //     chessboard: &[[char; 8]; 8], 
    //     from: Option<(usize, usize)>, 
    //         to: Option<(usize, usize)>
    //     ) -> Document {

    //     let mut doc = Document::new()
    //         .set("width", "500")  // Adjusting the width to account for labels
    //         .set("height", "500")  // Adjusting the height to account for labels
    //         .set("viewBox", (0, 0, 500, 500))
    //         .set("style", "background-color: #000;");  // Set background to black

    //     // Define labels
    //     let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    //     let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

    //     // Add column labels
    //     for (idx, label) in column_labels.iter().enumerate() {
    //         let label_text = Text::new()
    //             .set("x", 50 + idx * 50 + 25)  // Adjusting the x-coordinate to account for labels
    //             .set("y", 472)  // Positioning the label slightly above the bottom edge
    //             .set("text-anchor", "middle")
    //             .set("font-size", 20)
    //             .set("fill", "#757575")  // Set text color to white
    //             .add(svg::node::Text::new(label.to_string()));
    //         doc = doc.add(label_text);
    //     }

    //     // Add row labels
    //     for (idx, label) in row_labels.iter().enumerate() {
    //         let label_text = Text::new()
    //             .set("x", 32)  // Positioning the label slightly to the right of the left edge
    //             .set("y", 50 + idx * 50 + 35)  // Adjusting the y-coordinate to account for labels
    //             .set("text-anchor", "middle")
    //             .set("font-size", 20)
    //             .set("fill", "#757575")  // Set text color to white
    //             .add(svg::node::Text::new(label.to_string()));
    //         doc = doc.add(label_text);
    //     }

    //     for (row, row_pieces) in chessboard.iter().enumerate() {
    //         for (col, &piece) in row_pieces.iter().enumerate() {
    //             let x = 50 + col * 50;  // Adjusting the x-coordinate to account for labels
    //             let y = 50 + row * 50;  // Adjusting the y-coordinate to account for labels

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
                    
    //                 if let Some(to_coords_chess) = to {
    //                     let (row, col) = to_coords_chess;
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


    //                 // map character to piece name and background
    //                 let (piece_name, background) = match piece {
    //                     'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     _   => panic!("Unknown piece"),
    //                 };

    //                 // // Load SVG chess piece based on piece name and background
    //                 let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
    //                 let data_url = load_image_as_data_url(&file_path)
    //                     .expect("Failed to load image as data URL");

    //                 let piece_image = Image::new()
    //                     .set("x", x)
    //                     .set("y", y)
    //                     .set("width", 50)
    //                     .set("height", 50)
    //                     .set("href", data_url);

    //                 doc = doc.add(piece_image);
    //             }
    //         }
    //     }

    // doc
    // }



    // // Function to generate the SVG chessboard
    // fn generate_white_oriented_chessboard_letters(
    //     chessboard: &[[char; 8]; 8], 
    //     from: Option<(usize, usize)>, 
    //     to: Option<(usize, usize)>
    // ) -> Document {
    // let mut doc = Document::new()
    //     .set("width", "500")  // Adjusting the width to account for labels
    //     .set("height", "500")  // Adjusting the height to account for labels
    //     .set("viewBox", (0, 0, 500, 500))
    //     .set("style", "background-color: #000;");  // Set background to black

    // // Define labels
    // let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    // let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

    // // Add column labels
    // for (idx, label) in column_labels.iter().enumerate() {
    //     let label_text = Text::new()
    //         .set("x", 50 + idx * 50 + 25)  // Adjusting the x-coordinate to account for labels
    //         .set("y", 472)  // Positioning the label slightly above the bottom edge
    //         .set("text-anchor", "middle")
    //         .set("font-size", 20)
    //         .set("fill", "#757575")  // Set text color to white
    //         .add(svg::node::Text::new(label.to_string()));
    //     doc = doc.add(label_text);
    // }

    // // Add row labels
    // for (idx, label) in row_labels.iter().enumerate() {
    //     let label_text = Text::new()
    //         .set("x", 32)  // Positioning the label slightly to the right of the left edge
    //         .set("y", 50 + idx * 50 + 35)  // Adjusting the y-coordinate to account for labels
    //         .set("text-anchor", "middle")
    //         .set("font-size", 20)
    //         .set("fill", "#757575")  // Set text color to white
    //         .add(svg::node::Text::new(label.to_string()));
    //     doc = doc.add(label_text);
    // }

    // for (row, row_pieces) in chessboard.iter().enumerate() {
    //     for (col, &piece) in row_pieces.iter().enumerate() {
    //         let x = 50 + col * 50;  // Adjusting the x-coordinate to account for labels
    //         let y = 50 + row * 50;  // Adjusting the y-coordinate to account for labels

    //         let square_color = if (row + col) % 2 == 0 {
    //             "#ccc"
    //         } else {
    //             "#666"
    //         };

    //         let square = Rectangle::new()
    //             .set("x", x)
    //             .set("y", y)
    //             .set("width", 50)
    //             .set("height", 50)
    //             .set("fill", square_color);

    //         doc = doc.add(square);

    //         if piece != ' ' {


    //             // setting from an to color
    //             if let Some(from_coords) = from {
    //                 let (row, col) = from_coords;
    //                 let x = 50 + col * 50;
    //                 let y = 50 + row * 50;
                
    //                 let highlight = Rectangle::new()
    //                     .set("x", x)
    //                     .set("y", y)
    //                     .set("width", 50)
    //                     .set("height", 50)
    //                     .set("fill", "none") // Transparent fill
    //                     .set("stroke", "#3189D9")
    //                     .set("stroke-width", 3);
                
    //                 doc = doc.add(highlight);
    //             }
                
    //             if let Some(to_coords_chess) = to {
    //                 let (row, col) = to_coords_chess;
    //                 let x = 50 + col * 50;
    //                 let y = 50 + row * 50;
                
    //                 let highlight = Rectangle::new()
    //                     .set("x", x)
    //                     .set("y", y)
    //                     .set("width", 50)
    //                     .set("height", 50)
    //                     .set("fill", "none") // Transparent fill
    //                     .set("stroke", "#3189D9")
    //                     .set("stroke-width", 3);
                
    //                 doc = doc.add(highlight);
    //             }


    //             let piece_color = if square_color == "#ccc" { // for lighter background
    //                 if piece.is_lowercase() {
    //                     "#9e0b00" // darker red for dark pieces
    //                 } else {
    //                     "#665628" // darker gray for light pieces
    //                 }
    //             } else { // for darker background
    //                 if piece.is_lowercase() {
    //                     "#ff8e8e" // lighter red for dark pieces
    //                 } else {
    //                     "#ffefc1" // lighter gray for light pieces
    //                 }
    //             };


    //             let mut text = Text::new()
    //                 .set("x", x + 25)
    //                 .set("y", y + 35)
    //                 .set("text-anchor", "middle")
    //                 .set("font-size", 30)
    //                 .set("fill", piece_color);

    //             if piece.is_uppercase() {
    //                 text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
    //             } else {
    //                 text = text.add(svg::node::Text::new(piece.to_string()));
    //             }

    //             doc = doc.add(text);
    //         }
    //     }
    // }

    // doc
    // }



    // Function to generate the SVG chessboard with black orientation
    fn generate_black_oriented_chessboard(
        chessboard: &[[char; 8]; 8], 
        from: Option<(usize, usize)>, 
        to: Option<(usize, usize)>
        ) -> Document {

        let mut doc = Document::new()
            .set("width", "1000")  
            .set("height", "1000")  
            .set("viewBox", (0, 0, 1000, 1000))
            .set("style", "background-color: #2f0300;");  // Set background to dark red

        // Define labels, reversed for black piece orientation
        let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
        let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

        // Add column labels
        for (idx, label) in column_labels.iter().enumerate() {
            let label_text = Text::new()
                .set("x", 100 + idx * 100 + 50)  
                .set("y", 944)  
                .set("text-anchor", "middle")
                .set("font-size", 40)
                .set("fill", "#757575")  // Set text color to dark grey
                .add(svg::node::Text::new(label.to_string()));
            doc = doc.add(label_text);
        }

        // Add row labels
        for (idx, label) in row_labels.iter().enumerate() {
            let label_text = Text::new()
                .set("x", 64)  
                .set("y", 100 + idx * 100 + 70)  
                .set("text-anchor", "middle")
                .set("font-size", 40)
                .set("fill", "#757575")  
                .add(svg::node::Text::new(label.to_string()));
            doc = doc.add(label_text);
        }

        for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
            for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
                let x = 100 + col * 100;  
                let y = 100 + row * 100;  

                // Set Square Colours
                let square_color = if (row + col) % 2 == 0 {
                    "#ccc"
                } else {
                    "#666"
                };
                
                let square = Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", 100)
                    .set("height", 100)
                    .set("fill", square_color);

                doc = doc.add(square);

                if piece != ' ' {

                    if let Some(from_coords) = from {
                        let (row, col) = from_coords;
                        let x = 100 + col * 100;
                        let y = 100 + row * 100;
                    
                        let highlight = Rectangle::new()
                            .set("x", x)
                            .set("y", y)
                            .set("width", 100)
                            .set("height", 100)
                            .set("fill", "none") // Transparent fill
                            .set("stroke", "#3189D9")
                            .set("stroke-width", 3);
                    
                        doc = doc.add(highlight);
                    }
                    
                    if let Some(to_coords_chess) = to {
                        let (row, col) = to_coords_chess;
                        let x = 100 + col * 100;
                        let y = 100 + row * 100;
                    
                        let highlight = Rectangle::new()
                            .set("x", x)
                            .set("y", y)
                            .set("width", 100)
                            .set("height", 100)
                            .set("fill", "none") // Transparent fill
                            .set("stroke", "#3189D9")
                            .set("stroke-width", 6);
                    
                        doc = doc.add(highlight);
                    }


                    // map character to piece name and background
                    let (piece_name, background) = match piece {
                        'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                        _   => panic!("Unknown piece"),
                    };

                    // // Load SVG chess piece based on piece name and background
                    let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
                    let panic_message = format!("Failed to load image as data URL from path: {}", &file_path);
                    let data_url = load_image_as_data_url(&file_path).expect(&panic_message);
                    

                    let piece_image = Image::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 100)
                        .set("height", 100)
                        .set("href", data_url);

                    doc = doc.add(piece_image);
                }
            }
        }

        doc
    }


    // // Function to generate the SVG chessboard with black orientation
    // fn generate_black_oriented_chessboard_small(
    //     chessboard: &[[char; 8]; 8], 
    //     from: Option<(usize, usize)>, 
    //     to: Option<(usize, usize)>
    //     ) -> Document {

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

    //             // Set Square Colours
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
                    
    //                 if let Some(to_coords_chess) = to {
    //                     let (row, col) = to_coords_chess;
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


    //                 // map character to piece name and background
    //                 let (piece_name, background) = match piece {
    //                     'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
    //                     _   => panic!("Unknown piece"),
    //                 };

    //                 // // Load SVG chess piece based on piece name and background
    //                 let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
    //                 let panic_message = format!("Failed to load image as data URL from path: {}", &file_path);
    //                 let data_url = load_image_as_data_url(&file_path).expect(&panic_message);
                    

    //                 let piece_image = Image::new()
    //                     .set("x", x)
    //                     .set("y", y)
    //                     .set("width", 50)
    //                     .set("height", 50)
    //                     .set("href", data_url);

    //                 doc = doc.add(piece_image);
    //             }
    //         }
    //     }

    //     doc
    // }


    fn load_image_as_data_url(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let encoded = STANDARD.encode(&buffer); // Now works since Engine is in scope
        let mime_type = "image/svg+xml"; // Adjust if not using SVG images

        Ok(format!("data:{};base64,{}", mime_type, encoded))
    }


//     // Function to generate the SVG chessboard with black orientation
//     fn generate_black_oriented_chessboard_letters(
//         chessboard: &[[char; 8]; 8], 
//         from: Option<(usize, usize)>, 
//         to: Option<(usize, usize)>
//         ) -> Document {

//         let mut doc = Document::new()
//             .set("width", "500")  
//             .set("height", "500")  
//             .set("viewBox", (0, 0, 500, 500))
//             .set("style", "background-color: #2f0300;");  // Set background to dark red

//         // Define labels, reversed for black piece orientation
//         let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
//         let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

//         // Add column labels
//         for (idx, label) in column_labels.iter().enumerate() {
//             let label_text = Text::new()
//                 .set("x", 50 + idx * 50 + 25)  
//                 .set("y", 472)  
//                 .set("text-anchor", "middle")
//                 .set("font-size", 20)
//                 .set("fill", "#757575")  // Set text color to dark grey
//                 .add(svg::node::Text::new(label.to_string()));
//             doc = doc.add(label_text);
//         }

//         // Add row labels
//         for (idx, label) in row_labels.iter().enumerate() {
//             let label_text = Text::new()
//                 .set("x", 32)  
//                 .set("y", 50 + idx * 50 + 35)  
//                 .set("text-anchor", "middle")
//                 .set("font-size", 20)
//                 .set("fill", "#757575")  
//                 .add(svg::node::Text::new(label.to_string()));
//             doc = doc.add(label_text);
//         }

//         for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
//             for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
//                 let x = 50 + col * 50;  
//                 let y = 50 + row * 50;  

//                 let square_color = if (row + col) % 2 == 0 {
//                     "#ccc"
//                 } else {
//                     "#666"
//                 };
                
//                 let square = Rectangle::new()
//                     .set("x", x)
//                     .set("y", y)
//                     .set("width", 50)
//                     .set("height", 50)
//                     .set("fill", square_color);

//                 doc = doc.add(square);

//                 if piece != ' ' {

//                     if let Some(from_coords) = from {
//                         let (row, col) = from_coords;
//                         let x = 50 + col * 50;
//                         let y = 50 + row * 50;
                    
//                         let highlight = Rectangle::new()
//                             .set("x", x)
//                             .set("y", y)
//                             .set("width", 50)
//                             .set("height", 50)
//                             .set("fill", "none") // Transparent fill
//                             .set("stroke", "#3189D9")
//                             .set("stroke-width", 3);
                    
//                         doc = doc.add(highlight);
//                     }
                    
//                     if let Some(to_coords_chess) = to {
//                         let (row, col) = to_coords_chess;
//                         let x = 50 + col * 50;
//                         let y = 50 + row * 50;
                    
//                         let highlight = Rectangle::new()
//                             .set("x", x)
//                             .set("y", y)
//                             .set("width", 50)
//                             .set("height", 50)
//                             .set("fill", "none") // Transparent fill
//                             .set("stroke", "#3189D9")
//                             .set("stroke-width", 3);
                    
//                         doc = doc.add(highlight);
//                     }

                        
//                     let piece_color = if square_color == "#666" { // for darker background
//                         if piece.is_uppercase() {
//                             "#ffefc1" // lighter gray for light pieces
//                         } else {
//                             "#ff8e8e" // lighter red for dark pieces
//                         }
//                     } else { // for lighter background
//                         if piece.is_uppercase() {
//                             "#665628" // darker gray for light pieces
//                         } else {
//                             "#9e0b00" // darker red for dark pieces
//                         }
//                     };

//                     let mut text = Text::new()
//                         .set("x", x + 25)
//                         .set("y", y + 35)
//                         .set("text-anchor", "middle")
//                         .set("font-size", 30)
//                         .set("fill", piece_color);

//                     if piece.is_uppercase() {
//                         text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
//                     } else {
//                         text = text.add(svg::node::Text::new(piece.to_string()));
//                     }

//                     doc = doc.add(text);
//                 }
//             }
//     }

//     doc
// }




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
    chessboard_state[7] = piece_array.iter().map(|&p| p.to_ascii_uppercase()).collect::<Vec<_>>().try_into().unwrap();
    chessboard_state[6] = ['P'; 8];
    chessboard_state[1] = ['p'; 8];
    chessboard_state[0] = piece_array;

    Ok(chessboard_state)
}


impl CleanerState {


    pub fn new(next_check_time_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Ensure the file exists, if not, create it
        if !Path::new(next_check_time_file).exists() {
            let default_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 7);
            write_next_check_time_to_file(next_check_time_file, default_time)?;
        }
        
        let next_check_time = read_next_check_time_from_file(next_check_time_file)?;
        Ok(Self {
            next_check_time,
            expiration_by_name: HashMap::new(),
            names_by_expiration: BTreeMap::new(),
        })
    }


    pub fn update_directory_expiration_if_exists(&mut self, name: &str) {
        if self.expiration_by_name.contains_key(name) {
            let new_expiration_date = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 30); // 30 days from now
            self.add_directory(name.to_string(), new_expiration_date);
        }
    }
    


    pub fn add_directory(&mut self, name: String, expiration_date: SystemTime) {
        self.remove_directory(&name);
        self.expiration_by_name.insert(name.clone(), expiration_date);
        self.names_by_expiration
            .entry(expiration_date)
            .or_insert_with(Vec::new)
            .push(name);
    }

    pub fn remove_directory(&mut self, name: &str) {
        if let Some(expiration_date) = self.expiration_by_name.remove(name) {
            if let Some(names) = self.names_by_expiration.get_mut(&expiration_date) {
                names.retain(|n| n != name);
                if names.is_empty() {
                    self.names_by_expiration.remove(&expiration_date);
                }
            }
        }
    }

    pub fn update_next_check_time(&mut self, next_check_time_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.next_check_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 7);  // one week from now
        write_next_check_time_to_file(next_check_time_file, self.next_check_time)
    }
    
    pub fn check_and_remove_expired(&mut self, next_check_time_file: &str) {
        let current_time = SystemTime::now();
        if let Ok(duration) = self.next_check_time.duration_since(current_time) {
            if duration.as_secs() == 0 {

                match self.update_next_check_time(next_check_time_file) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Failed to update next check time: {}", e),
                }

                let now = SystemTime::now();
                let expired_keys: Vec<SystemTime> = self.names_by_expiration
                    .range(..now)
                    .map(|(&key, _)| key)
                    .collect();
    
                for key in expired_keys {
                    if let Some(names) = self.names_by_expiration.remove(&key) {
                        for name in names {
                            match remove_directory(&name) {
                                Ok(_) => (),
                                Err(e) => println!("Failed to remove directory '{}': {}", name, e),
                            }
                        }
                    }
                }
            }
        }
    }
    
    
    
}
    



// ...

fn read_next_check_time_from_file(filename: &str) -> Result<SystemTime, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let secs = contents.trim().parse::<u64>()?;
    Ok(UNIX_EPOCH + Duration::from_secs(secs))
}

fn write_next_check_time_to_file(filename: &str, next_check_time: SystemTime) -> Result<(), Box<dyn std::error::Error>> {
    let secs = next_check_time.duration_since(UNIX_EPOCH)?.as_secs();
    let contents = secs.to_string();

    let mut file = fs::File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


fn check_url_parts_against_directory(url_parts: &[&str]) -> std::io::Result<Option<String>> {
    // Read all directory names from games/
    let directory_names: Vec<_> = fs::read_dir("games/")?
        .filter_map(|entry| {
            entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(String::from)))
        })
        .collect();

    let indices_to_check = [1, 3];

    for &index in &indices_to_check {
        if index < url_parts.len() && directory_names.contains(&url_parts[index].to_string()) {
            return Ok(Some(url_parts[index].to_string()));
        }
    }

    Ok(None)
}

fn update_directory_expiration_from_url(url_parts: &[&str], cleaner: &mut CleanerState) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(matched_directory) = check_url_parts_against_directory(url_parts)? {
        // Access the directory to update its expiration time
        cleaner.update_directory_expiration_if_exists(&matched_directory);
    }
    Ok(())
}

// fn process_url_and_update_expiration2(url: &str) {
//     let matched_dir = check_url_parts_against_directory(url);
//     if let Some(dir_name) = matched_dir {
//         update_directory_expiration_from_url(&dir_name);
//     }
// }


// fn process_url_and_update_expiration(url_parts: &[&str]) {
//     // Note: Assuming check_url_parts_against_directory returns an Option<String>
//     match check_url_parts_against_directory(url_parts) {
//         Ok(Some(dir_name)) => {
//             // Assuming update_directory_expiration_from_url takes a directory name
//             // and returns a Result<(), ErrorType>
//             if let Err(e) = update_directory_expiration_from_url(&dir_name) {
//                 eprintln!("Error updating directory expiration: {}", e);
//             }
//         },
//         Ok(None) => println!("No matching directory found."),
//         Err(e) => println!("Error checking input against directory: {}", e),
//     }
// }


// fn process_url_and_update_expiration(url_parts: &[&str], cleaner_state: &mut CleanerState) {
fn process_url_and_update_expiration(url_parts: &[&str], cleaner_state: &mut CleanerState) {

    // Note: Assuming check_url_parts_against_directory returns an Option<String>
    match check_url_parts_against_directory(url_parts) {
        Ok(Some(dir_name)) => {
            // Convert dir_name to slice
            let dir_name_slice = [&dir_name[..]];

            // Now pass both arguments to update_directory_expiration_from_url
            if let Err(e) = update_directory_expiration_from_url(&dir_name_slice, cleaner_state) {
                eprintln!("Error updating directory expiration: {}", e);
            }
        },
        Ok(None) => println!("No directory names fournd to update expiration dates."),
        Err(e) => println!("Error checking input against directory: {}", e),
    }
}


// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     // Lock the in-memory queue for writing to disk
//     let queue = in_memory_queue.lock().unwrap();
    
//     // Implement the logic to write each request to disk asynchronously
//     for request in queue.iter() {
//         if let Some(body) = request.as_reader() {
//             let mut buffer = Vec::new();
//             if let Ok(_) = body.read_to_end(&mut buffer) {
//                 if let Ok(mut file) = File::create("requests.txt") {
//                     if let Ok(_) = file.write_all(&buffer) {
//                         println!("Successfully wrote request to disk.");
//                     } else {
//                         println!("Failed to write request to disk.");
//                     }
//                 } else {
//                     println!("Failed to create the file.");
//                 }
//             } else {
//                 println!("Failed to read request body.");
//             }
//         }
//     }
// }




// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     // Lock the in-memory queue for writing to disk
//     let queue = in_memory_queue.lock().unwrap();

//     // Create a buffer to store all requests' contents
//     let mut buffer = Vec::new();
    

//     // Collect all requests' contents into the buffer
//     for request in queue.iter() {
//         if request.method() == &Method::Get {
//             let url_parts: Vec<&str> = request.url().split('/').collect();
//                 let mut temp_buffer = Vec::new();
//                 if let Ok(_) = (body as &dyn std::io::Read).read_to_end(&mut temp_buffer) {
//                     buffer.extend_from_slice(&temp_buffer);
//             }
//         } else {
//             // For other types of requests (e.g., POST, PUT, DELETE), read the body if present
//             if let Some(body) = request.as_reader() {
//                 let mut temp_buffer = Vec::new();
//                 if let Ok(_) = (body as &dyn std::io::Read).read_to_end(&mut temp_buffer) {
//                     buffer.extend_from_slice(&temp_buffer);
//                 }
//             }
//         }
//     }
    
    
    

//     // Implement the logic to write the batch to disk asynchronously
//     if !buffer.is_empty() {
//         if let Ok(mut file) = File::create("requests.txt") {
//             if let Ok(mut writer) = BufWriter::new(&mut file).write_all(&buffer) {
//                 println!("Successfully wrote batch of requests to disk.");
//             } else {
//                 println!("Failed to write batch of requests to disk.");
//             }
//         } else {
//             println!("Failed to create the file.");
//         }
//     } else {
//         println!("No requests to write to disk.");
//     }
// }



// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     // Lock the in-memory queue for writing to disk
//     let queue = in_memory_queue.lock().unwrap();

//     // Create a buffer to store all requests' contents
//     let mut buffer = Vec::new();

//     // Collect all requests' contents into the buffer
//     for request in queue.iter() {
//         if request.method() == &Method::Get {
//             let url_parts: Vec<&str> = request.url().split('/').collect();
//             // Process the URL parts for GET requests (as you have already implemented)
//         } else {
//             // For other types of requests (e.g., POST, PUT, DELETE), read the body if present
//             if let Some(body) = request.body() {
//                 let mut temp_buffer = Vec::new();
//                 if let Ok(_) = body.read_to_end(&mut temp_buffer) {
//                     buffer.extend_from_slice(&temp_buffer);
//                 }
//             }
//         }
//     }

//     // Implement the logic to write the batch to disk asynchronously
//     if !buffer.is_empty() {
//         if let Ok(mut file) = File::create("requests.txt") {
//             if let Ok(mut writer) = BufWriter::new(&mut file).write_all(&buffer) {
//                 println!("Successfully wrote batch of requests to disk.");
//             } else {
//                 println!("Failed to write batch of requests to disk.");
//             }
//         } else {
//             println!("Failed to create the file.");
//         }
//     } else {
//         println!("No requests to write to disk.");
//     }
// }



// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     // Lock the in-memory queue for writing to disk
//     let queue = in_memory_queue.lock().unwrap();

//     // Create a buffer to store all requests' contents
//     let mut buffer = Vec::new();

//     // Collect all requests' contents into the buffer
//     for request in queue.iter() {
//         if request.method() == &Method::Get {
//             let url_parts: Vec<&str> = request.url().split('/').collect();
//             // Process the URL parts for GET requests (as you have already implemented)
//         } else {
//             // For other types of requests (e.g., POST, PUT, DELETE), read the body if present
//             if let Some(body) = request.body() {
//                 let mut temp_buffer = Vec::new();
//                 if let Ok(_) = body.read_to_end(&mut temp_buffer) {
//                     buffer.extend_from_slice(&temp_buffer);
//                 }
//             }
//         }
//     }

//     // Implement the logic to write the batch to disk asynchronously
//     if !buffer.is_empty() {
//         if let Ok(mut file) = File::create("requests.txt") {
//             if let Ok(mut writer) = BufWriter::new(&mut file).write_all(&buffer) {
//                 println!("Successfully wrote batch of requests to disk.");
//             } else {
//                 println!("Failed to write batch of requests to disk.");
//             }
//         } else {
//             println!("Failed to create the file.");
//         }
//     } else {
//         println!("No requests to write to disk.");
//     }
// }


// use std::io::Read;

// // Function to write a batch of requests to disk asynchronously
// fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) {
//     // Lock the in-memory queue for writing to disk
//     let queue = in_memory_queue.lock().unwrap();

//     // Create a buffer to store all requests' contents
//     let buffer = Vec::new();

//     // Collect all requests' contents into the buffer
//     for request in queue.iter() {
//         if request.method() == &Method::Get {
//             let url_parts: Vec<&str> = request.url().split('/').collect();
//             // Process the URL parts for GET requests (as you have already implemented)
//         } else {
//         }
//     }

//     // Implement the logic to write the batch to disk asynchronously
//     if !buffer.is_empty() {
//         if let Ok(mut file) = File::create("requests.txt") {
//             if let Ok(writer) = BufWriter::new(&mut file).write_all(&buffer) {
//                 println!("Successfully wrote batch of requests to disk.");
//             } else {
//                 println!("Failed to write batch of requests to disk.");
//             }
//         } else {
//             println!("Failed to create the file.");
//         }
//     } else {
//         println!("No requests to write to disk.");
//     }
// }


fn write_batch_to_disk(in_memory_queue: Arc<Mutex<VecDeque<Request>>>) -> Result<(), Error> {
    // Lock the in-memory queue for writing to disk
    let mut queue = in_memory_queue.lock().unwrap();

    // Create a buffer to store all requests' contents as Strings
    let mut buffer = String::new();

    // Collect all requests' contents into the buffer
    for request in queue.drain(..) {
        // Here, for simplicity, we are adding request methods and URLs to buffer
        // Adjust as necessary
        buffer.push_str(&format!("{} {}\n", request.method(), request.url()));
    }

    // If there's data to write
    if !buffer.is_empty() {
        // Append to the file to ensure previous data isn't overwritten
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("requests.txt")?;
        
        let mut writer = BufWriter::new(&mut file);
        writer.write_all(buffer.as_bytes())?;
        println!("Successfully wrote batch of requests to disk.");
    } else {
        println!("No requests to write to disk.");
    }

    Ok(())
}


// fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
//     /*
//     extern crate image;
//     use image::ImageBuffer;
//     use std::path::Path;

//     combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "output.png")?;
//     Ok(())
//      */

//     // Load the images
//     let image1 = image::open(image_path1)?;
//     let image2 = image::open(image_path2)?;

//     // Check the height of the images and make them the same if necessary, or handle differently as needed.
//     let height = std::cmp::max(image1.height(), image2.height());

//     // Create a new image with the combined width of both images and the maximum height
//     let mut combined_image = ImageBuffer::new(image1.width() + image2.width(), height);

//     // Copy pixels from image1 into the new image
//     for (x, y, pixel) in image1.to_rgba8().enumerate_pixels() {
//         combined_image.put_pixel(x, y, *pixel);
//     }

//     // Copy pixels from image2 into the new image, offsetting by the width of image1
//     for (x, y, pixel) in image2.to_rgba8().enumerate_pixels() {
//         combined_image.put_pixel(x + image1.width(), y, *pixel);
//     }

//     // Save the new image
//     combined_image.save(output_path)?;

//     Ok(())
// }


fn sanitize_url(input: &str) -> String {
    let without_trailing_slashes_and_spaces = input.trim_end_matches(|c| c == '/' || c == ' ');
    without_trailing_slashes_and_spaces
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() 
        || c == '_' 
        || c == '/'
        || c == '.'
        || c == '='
        || c == '&'
        || c == '^'
        || c == '@'
        || c == '-'
        || c == '+'
        || c == '*'
        || c == '!'
        || c == '~'
        || c == '`'
        || c == ';'
        || c == ','
    )
        .collect()
}

// Define the show_board_png function to get the PNG content from the file or other sources
fn show_board_png(new_game_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(format!("games/{}/board.png", new_game_name))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}





/*
png image functions

extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer, GenericImageView};
use rand::Rng;
use std::{fs, io};
use std::io::{ErrorKind, Error};
use std::fmt::Debug;
*/

// fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
fn combine_side_by_side<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // println!(
    //     "\ncombine_side_by_side...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
    //     &image_path1,
    //     &image_path2,
    //     &output_path
    // );

    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_side_by_side Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_side_by_side Failed to load image at {:?}", &image_path2))))?;
    


    /*
    extern crate image;
    use image::ImageBuffer;
    use std::path::Path;

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "output.png")?;
    Ok(())
     */

    // Load the images
    // let image1 = image::open(image_path1)?;
    // let image2 = image::open(image_path2)?;


    // Check the height of the images and make them the same if necessary, or handle differently as needed.
    let height = std::cmp::max(image1.height(), image2.height());

    // Create a new image with the combined width of both images and the maximum height
    let mut combined_image = ImageBuffer::new(image1.width() + image2.width(), height);

    // Copy pixels from image1 into the new image
    for (x, y, pixel) in image1.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y, *pixel);
    }

    // Copy pixels from image2 into the new image, offsetting by the width of image1
    for (x, y, pixel) in image2.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x + image1.width(), y, *pixel);
    }

    // Save the new image
    combined_image.save(output_path)?;

    Ok(())
}



fn combine_top_to_bottom<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // println!(
    //     "\ncombine_top_to_bottom...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
    //     &image_path1,
    //     &image_path2,
    //     &output_path
    // );
    // Load the images
    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_top_to_bottom Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_top_to_bottom Failed to load image at {:?}", &image_path2))))?;
    

    // Check the width of the images and make them the same if necessary, or handle differently as needed.
    let width = std::cmp::max(image1.width(), image2.width());

    // Create a new image with the combined height of both images and the maximum width
    let mut combined_image = ImageBuffer::new(width, image1.height() + image2.height());

    // Copy pixels from image1 into the new image
    for (x, y, pixel) in image1.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y, *pixel);
    }

    // Copy pixels from image2 into the new image, offsetting by the height of image1
    for (x, y, pixel) in image2.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y + image1.height(), *pixel);
    }

    // Save the new image
    combined_image.save(output_path)?;

    Ok(())
}



fn overlay_images<P: AsRef<Path> + std::fmt::Debug>(
    image_path1: P,
    image_path2: P,
    output_path: P,
) -> Result<(), image::ImageError> {
    println!(
        "\nOverlaying images...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
        &image_path1,
        &image_path2,
        &output_path
    );

    // Load the images with custom error handling
    let mut image1 = image::open(&image_path1)
        .map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path1))))?
        .to_rgba8();
    let image2 = image::open(&image_path2)
        .map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path2))))?
        .to_rgba8();


    // Ensure the images are the same size, or handle differently as needed.
    assert_eq!(image1.dimensions(), image2.dimensions());

    // Iterate over the coordinates and pixels of image2.
    for (x, y, pixel) in image2.enumerate_pixels() {
        let pixel1 = image1.get_pixel_mut(x, y);
        // Perform blending here. You could write your own blending logic or use a pre-existing function.
        blend_pixels(pixel1, *pixel);
    }

    // Save the result.
    image1.save(output_path)?;

    Ok(())
}


// A simple alpha blending function. You might want to use a more sophisticated blending function depending on your needs.
fn blend_pixels(bottom: &mut Rgba<u8>, top: Rgba<u8>) {
    // println!(
    //     "\nblend_pixels...\nbottom: {:?}\ntop: {:?}",
    //     bottom,
    //     top,
    // );

    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 * (1.0 - alpha_top) / 255.0;
    let alpha_combined = alpha_top + alpha_bottom;

    for i in 0..3 {
        bottom[i] = ((top[i] as f32 * alpha_top + bottom[i] as f32 * alpha_bottom) / alpha_combined) as u8;
    }
    bottom[3] = (alpha_combined * 255.0) as u8;
}


fn random_image_from_directory(directory: &str) -> Result<String, std::io::Error> {
    // println!(
    //     "\nrandom_image_from_directory...\ndirectory: {:?}",
    //     &directory,
    // );
    let paths: Vec<_> = fs::read_dir(directory)?
        .filter_map(Result::ok) // Only keep the Ok values
        .collect();

    if paths.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No images found"));
    }

    let random_entry = &paths[rand::thread_rng().gen_range(0..paths.len())];
    let random_file = random_entry.file_name();
    let file_path_str = random_file.to_str().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Invalid file name"))?;
    let file_path = format!("{}/{}", directory, file_path_str);

    Ok(file_path)
}


fn make_board_core(sandbox_path: &str, orientation_white: bool) -> Result<(), io::Error> {
    // println!("\nmake_board_core()...");
    let mut row_images = Vec::new();

    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Make the 8 Rows
    for row in 0..8 {
        let mut row_image_path = String::new();

        // for col in 0..8 {
        //     let texture_directory = if (row + col) % 2 == 0 {
        //         if orientation_white {
        //             "image_files/lightsquares"
        //         } else {
        //             "image_files/darksquares"
        //         }
        //     } else {
        //         if orientation_white {
        //             "image_files/darksquares"
        //         } else {
        //             "image_files/lightsquares"
        //         }
        //     };

        for col in 0..8 {
            let texture_directory = if (row + col) % 2 == 0 {
                "image_files/lightsquares"
            } else {
                "image_files/darksquares"
            };

            let random_image_path = random_image_from_directory(texture_directory)?;

            if row_image_path.is_empty() {
                row_image_path = random_image_path;
            } else {
                let output_path = format!("{}/row_{}_col_{}.png", sandbox_path, row, col);
            combine_side_by_side(row_image_path, random_image_path, output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

                row_image_path = output_path;
            }
        }

        row_images.push(row_image_path);
    }

    // connect the 8 rows
    let mut final_board_image_path = row_images[0].clone();
    for i in 1..8 {
        let output_path = format!("{}/final_row_{}.png", sandbox_path, i);
        combine_top_to_bottom(final_board_image_path, row_images[i].clone(), output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    
        final_board_image_path = output_path;
    }


    // Move (create?) the final image inside the sandbox
    let sandbox_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    fs::rename(final_board_image_path, sandbox_path)?;

    Ok(())
}


fn make_and_attach_letter_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // println!(
    //     "make_and_attach_letter_bar images...\nsandbox_path: {:?}\norientation_white: {:?}\nboard_image_path: {:?}",
    //     &sandbox_path,
    //     &orientation_white,
    //     &board_image_path
    // );

    // ONLY HERE
    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Determine the order of letters
    let letters_order = if orientation_white {
        ["a.png", "b.png", "c.png", "d.png", "e.png", "f.png", "g.png", "h.png"]
    } else {
        ["h.png", "g.png", "f.png", "e.png", "d.png", "c.png", "b.png", "a.png"]
    };

    // Create the letter bar by combining individual letters
    let mut letter_bar_path = String::new();
    for letter in &letters_order {
        let path = format!("image_files/legend_alpha_num/{}", letter);
        if letter_bar_path.is_empty() {
            letter_bar_path = path;
        } else {
            let new_output_path = format!("{}/tmp_{}.png", sandbox_path, letter); // Prepend sandbox_path
            combine_side_by_side(letter_bar_path, path, new_output_path.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            letter_bar_path = new_output_path;
        }
    }


    // Combine the letter bar with the board image
    let final_image_with_letters_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    combine_top_to_bottom(board_image_path, &letter_bar_path, &final_image_with_letters_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // Clean up temporary files created during the process
    for letter in &letters_order {
        let tmp_file_path = format!("{}/tmp_{}.png", sandbox_path, letter);
        if std::path::Path::new(&tmp_file_path).exists() {
            let _ = std::fs::remove_file(tmp_file_path);
        }
    }

    Ok(())
}


fn make_and_attach_number_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // println!(
    //     "make_and_attach_number_bar images...\nsandbox_path: {:?}\norientation_white: {:?}\nboard_image_path: {:?}",
    //     &sandbox_path,
    //     &orientation_white,
    //     &board_image_path
    // );

    // ONLY HERE
    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Determine the order of numbers
    let numbers_order = if orientation_white {
        ["8.png", "7.png", "6.png", "5.png", "4.png", "3.png", "2.png", "1.png"]
    } else {
        ["1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png"]
    };

    // Create a temporary image for the vertical number bar
    let mut number_bar_path = String::new(); // Start without a path, as we will build it dynamically

    for number in &numbers_order {
        let path = format!("image_files/legend_alpha_num/{}", number);
        if number_bar_path.is_empty() {
            number_bar_path = path;
        } else {
            let new_output_path = format!("{}/tmp_{}.png", sandbox_path, number);
            combine_top_to_bottom(number_bar_path, path, new_output_path.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            number_bar_path = new_output_path;
        }
    }

    // Now combine the final number bar with a blank image at the bottom
    let blank_image_path: String = "image_files/legend_alpha_num/blank.png".to_string();
    let new_output_path = format!("{}/tmp_blank.png", sandbox_path);
    combine_top_to_bottom(number_bar_path, blank_image_path, new_output_path.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    number_bar_path = new_output_path.clone();

    // Combine the number bar with the board image
    let final_image_with_numbers_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    combine_side_by_side(&number_bar_path, &board_image_path.to_string(), &final_image_with_numbers_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // Optional: Clean up temporary files created during the process
    for number in &numbers_order {
        let tmp_file_path = format!("{}/tmp_{}.png", sandbox_path, number);
        if std::path::Path::new(&tmp_file_path).exists() {
            let _ = std::fs::remove_file(tmp_file_path);
        }
    }

    // Remove the temporary blank file
    if std::path::Path::new(&new_output_path).exists() {
        let _ = std::fs::remove_file(new_output_path);
    }

    Ok(())
}


// fn generate_chessboard_backboards(game_name: &str, orientation_white: bool) -> Result<(), Error> {
fn generate_chessboard_backboards(game_name: &str, orientation_white: bool) -> Result<(), Error> {

    // println!(
    //     "generate_chessboard_backboards images...\ngame_name: {:?}\norientation_white: {:?}",
    //     &game_name,
    //     &orientation_white,
    // );

    let orientation_string: String = if orientation_white {
            "white".to_string()
        } else {
            "black".to_string()
        };

    let sandbox_path: String = format!("games/{}/sandboxes/sandbox_backboard", game_name);

    // Check if the sandbox_backboard folder exists
    if fs::metadata(&sandbox_path).is_ok() {
        // If it exists, delete it
        fs::remove_dir_all(&sandbox_path)?;
    }

    // Create the new directory
    fs::create_dir_all(&sandbox_path)?;

    // check if sandbox_backboard exists, if so, delete it

    let final_image_path: String = format!("games/{}", game_name);


    // Create the temporary directory
    fs::create_dir_all(&sandbox_path)?;

    // Generate the chessboard
    let result = make_board_core(&sandbox_path, orientation_white);
    
    // Add letter bar
    let board_image_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    make_and_attach_letter_bar(&sandbox_path, orientation_white, &board_image_path)?;

    // Add number bar
    make_and_attach_number_bar(&sandbox_path, orientation_white, &board_image_path)?;

    // Assuming the final image is first created inside sandbox as final_image.png
    // then moved to the game folder
    let final_image_source = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    let final_image_destination = format!("{}/back_board_{}.png", final_image_path, orientation_string);

    // Move the final image to the desired location
    if result.is_ok() {
        fs::rename(final_image_source, final_image_destination)?;
    }

    // Clean up by deleting the temporary directory
    let _ = fs::remove_dir_all(&sandbox_path);

    result
}



// fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8], white_orientation: bool) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
//     let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels

//     for (row, row_pieces) in chessboard.iter().enumerate() {
//         for (col, &piece) in row_pieces.iter().enumerate() {
//             let (actual_row, actual_col) = if white_orientation {
//                 (row, col)
//             } else {
//                 (7 - row, 7 - col)
//             };
            
//             let square_color = if (actual_row + actual_col) % 2 != 0 { "darksquare" } else { "lightsquare" };
//             let (piece_prefix, piece_suffix) = match piece {

//                 'p' => ("p_", square_color),
//                 'r' => ("r_", square_color),
//                 'n' => ("n_", square_color),
//                 'b' => ("b_", square_color),
//                 'q' => ("q_", square_color),
//                 'k' => ("k_", square_color),
//                 'P' => ("P_", square_color),
//                 'R' => ("R_", square_color),
//                 'N' => ("N_", square_color),
//                 'B' => ("B_", square_color),
//                 'Q' => ("Q_", square_color),
//                 'K' => ("K_", square_color),
//                 _ => continue,
            
//             };

//             let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
//             let piece_image_path = random_image_from_directory(&texture_directory)?;
//             let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//             let (x, y) = (col * 75, row * 75);

//             // Overlay the piece image on the correct square
//             for (i, j, pixel) in piece_image.pixels() {
//                 let (x_new, y_new) = (x + i as usize, y + j as usize);
//                 img.put_pixel(x_new as u32, y_new as u32, pixel);
//             }
//         }
//     }

//     Ok(img)
// }


fn format_directory_name(dir_name: &str) -> String {
    let formatted = dir_name
        .replace(" ", "")
        .replace("(", "")
        .replace(")", "")
        .replace(",", "_");
    formatted
}


fn create_chess_pieces_layer(
    chessboard: &[[char; 8]; 8],
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>,
    white_orientation: bool,
    game_name: &str,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
    let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels
    let mut mutable_board = *chessboard;

    // save move-from location
    if let Some((from_row, from_col)) = from {
        mutable_board[from_row][from_col] = '>';
    }

    /*
    Make/check directory:
    1. get the piece locations as the board game-state
    2. get the from-to move data
    3. add from-box as addition playing piece
    4. add to-box to image and overlay
    Use sandbox for temp files.
    */


    /*
    Section: Make sure directory exists:
    */
    // Create a string that uniquely identifies the from-to move
    let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>
    // Set alpha directory-path
    let original_dir_name = format!("games/{}/sandboxes/temp_{}", game_name, from_to_name);
    // format directory path
    let formatted_dir_name = format_directory_name(&original_dir_name);
    // if directory does not exist, created it.
    if !formatted_dir_name.is_empty() && !std::path::Path::new(&formatted_dir_name).exists() {
        fs::create_dir_all(&formatted_dir_name)?;
        println!("Directory '{}' created successfully!", formatted_dir_name);
    } else {
        println!("Directory '{}' already exists.", formatted_dir_name);
    }

    // Make a unique temp directory based on game_name and the move
    // let dir_name = "my_directory";

    // if !dir_name.is_empty() && !std::path::Path::new(&dir_name).exists() {
    //     fs::create_dir(&dir_name)?;
    //     println!("Directory '{}' created successfully!", dir_name);
    // } else {
    //     println!("Directory '{}' already exists.", dir_name);
    // }


    if white_orientation {
        for (row, row_pieces) in mutable_board.iter().enumerate() {
            for (col, &piece) in row_pieces.iter().enumerate() {
                // Process the piece, (row, col) will be the actual coordinates for white orientation
                let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
            let (piece_prefix, piece_suffix) = match piece {

                'p' => ("p_", square_color),
                'r' => ("r_", square_color),
                'n' => ("n_", square_color),
                'b' => ("b_", square_color),
                'q' => ("q_", square_color),
                'k' => ("k_", square_color),
                'P' => ("P_", square_color),
                'R' => ("R_", square_color),
                'N' => ("N_", square_color),
                'B' => ("B_", square_color),
                'Q' => ("Q_", square_color),
                'K' => ("K_", square_color),
                '>' => ("from_to_box_", square_color),
                _ => continue,
            };

            let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
            let piece_image_path = random_image_from_directory(&texture_directory)?;
            let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

            let (x, y) = (col * 75, row * 75);

            // Overlay the piece image on the correct square
            for (i, j, pixel) in piece_image.pixels() {
                let (x_new, y_new) = (x + i as usize, y + j as usize);
                img.put_pixel(x_new as u32, y_new as u32, pixel);
            }

            /* To->Here border-box overlay: 
            this whole huge next section 
            for just that little line 
            face palm*/

            // if to-square is the same as current row and col:
            // if let Some((to_row, to_col)) = adjust_to_png_coords(to) {

            if let Some((to_row, to_col)) = to {
                if to_row == row && to_col == col {

                    // println!("WHITE inspect before more_to png ovelay");
                    // dbg!("this piece, to_row -> ", row);
                    // dbg!("this piece, to_col -> ", col);
                    // dbg!("(input) to -> ", to);


                    // if let Some((to_row, to_col)) = adjust_to_png_coords(to) {
                        // if let Some((to_row, to_col)) = to {

                        let to_piece_char = chessboard[to_row][to_col];
                    
                        let to_square_color = if (to_row + to_col) % 2 != 0 { "darksquare" } else { "lightsquare" };
    
                        // Determine the directory where the "to" border image is stored
                        let border_directory = format!("image_files/chess_pieces/{}_{}", "from_to_box", to_square_color);

                        // Get a random image path from the directory
                        println!("About to select random image from directory: {}", border_directory);
                        let random_border_path = random_image_from_directory(&border_directory)?;
        
                        // Determine the directory where the "to" border image is stored
                        let texture_directory = format!("image_files/chess_pieces/{}_{}", to_piece_char, to_square_color);
                        // image_files/chess_pieces/from_to_box_lightsquare

                        // Create a string that uniquely identifies the from-to move
                        let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>

                        let original_dir_name = format!("games/{}/sandboxes/temp_{}/to_here_overlay.png", game_name, from_to_name);
                        let temp_image_path = format_directory_name(&original_dir_name); 

                        // // Make a unique temp directory based on game_name and the move
                        // let temp_image_path = format!("games/{}/sandboxes/temp_{}/to_here_overlay.png", game_name, from_to_name);
                
                        // Get a random image path from the directory
                        println!("About to select random image from directory: {}", texture_directory);
                        let random_piece_image_path = random_image_from_directory(&texture_directory)?;
                
                        // overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

                        // Perform the overlay
                        println!("About to overlay image. Temp directory: {}", temp_image_path);
                        overlay_images(random_piece_image_path, random_border_path, temp_image_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                       // Again: Make a unique temp directory based on game_name and the move
                       let original_dir_name = format!("games/{}/sandboxes/temp_{}/to_here_overlay.png", game_name, from_to_name);
                       let temp_image_path = format_directory_name(&original_dir_name); 

                        // Load the image from the temporary directory
                        let piece_image = image::open(Path::new(&temp_image_path))
                            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
                
                        let (x, y) = (col * 75, row * 75);
                
                        // Overlay the piece image on the correct square
                        for (i, j, pixel) in piece_image.pixels() {
                            let (x_new, y_new) = (x + i as usize, y + j as usize);
                            img.put_pixel(x_new as u32, y_new as u32, pixel);
                        }
                }
                // }
            }
            }
        }
    } else {  // .rev().enumerate()
        for (row, row_pieces) in mutable_board.iter().rev().enumerate() {
            for (col, &piece) in row_pieces.iter().rev().enumerate() {

                // Process the piece, (row, col) will be the actual coordinates for white orientation
                let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
            let (piece_prefix, piece_suffix) = match piece {

                'p' => ("p_", square_color),
                'r' => ("r_", square_color),
                'n' => ("n_", square_color),
                'b' => ("b_", square_color),
                'q' => ("q_", square_color),
                'k' => ("k_", square_color),
                'P' => ("P_", square_color),
                'R' => ("R_", square_color),
                'N' => ("N_", square_color),
                'B' => ("B_", square_color),
                'Q' => ("Q_", square_color),
                'K' => ("K_", square_color),
                '>' => ("from_to_box_", square_color),
                _ => continue,
            
            };

            let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
            let piece_image_path = random_image_from_directory(&texture_directory)?;
            let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

            let (x, y) = (col * 75, row * 75);

            // Overlay the piece image on the correct square
            for (i, j, pixel) in piece_image.pixels() {
                let (x_new, y_new) = (x + i as usize, y + j as usize);
                img.put_pixel(x_new as u32, y_new as u32, pixel);
            }

            // dbg!("this piece, piece -> ", piece);
            // dbg!("this piece, to_row -> ", row);
            // dbg!("this piece, to_col -> ", col);
            // dbg!("(input) to -> ", to);
            // dbg!("(input) adjust_to_png_coords(to) -> ", adjust_to_png_coords(to));


            /* To->Here border-box overlay: 
            this whole huge next section 
            for just that little line 
            face palm*/

            // if to_square is the same as current iteration row and col:
            if let Some((to_row, to_col)) = adjust_to_png_coords(to) {

                if to_row == row && to_col == col {

                    // println!("black inspect before more_to png ovelay");
                    // dbg!("this piece, to_row -> ", row);
                    // dbg!("this piece, to_col -> ", col);
                    // dbg!("(input) to -> ", to);
                    // dbg!("(input) adjust_to_png_coords(to) -> ", adjust_to_png_coords(to));



                    if let Some((to_row, to_col)) = to {

                        // println!("black inspect before more_to png ovelay");
                        // dbg!("this piece, to_row -> ", row);
                        // dbg!("this piece, to_col -> ", col);
                        // dbg!("(input) to -> ", to);
                        // dbg!("(input) adjust_to_png_coords(to) -> ", adjust_to_png_coords(to));

                        let to_piece_char = chessboard[to_row][to_col];
                    
                        let to_square_color = if (to_row + to_col) % 2 != 0 { "darksquare" } else { "lightsquare" };

                        // Determine the directory where the "to" border image is stored
                        let border_directory = format!("image_files/chess_pieces/{}_{}", "from_to_box", to_square_color);

                        // Get a random image path from the directory
                        println!("About to select random image from directory: {}", border_directory);
                        let random_border_path = random_image_from_directory(&border_directory)?;
        
                        // Determine the directory where the "to" border image is stored
                        let texture_directory = format!("image_files/chess_pieces/{}_{}", to_piece_char, to_square_color);
                        // image_files/chess_pieces/from_to_box_lightsquare

                        // Create a string that uniquely identifies the from-to move
                        let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>
                
                        // Make a unique temp directory based on game_name and the move
                        let original_dir_name = format!("games/{}/sandboxes/temp_{}/to_here_overlay.png", game_name, from_to_name);
                        let temp_image_path = format_directory_name(&original_dir_name); 
 
                        // Get a random image path from the directory
                        println!("About to select random image from directory: {}", texture_directory);
                        let random_piece_image_path = random_image_from_directory(&texture_directory)?;
                
                        // overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

                        // Perform the overlay
                        println!("About to overlay image. Temp directory: {}", temp_image_path);
                        overlay_images(random_piece_image_path, random_border_path, temp_image_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

                        // Again: Make a unique temp directory based on game_name and the move
                        let original_dir_name = format!("games/{}/sandboxes/temp_{}/to_here_overlay.png", game_name, from_to_name);
                        let temp_image_path = format_directory_name(&original_dir_name); 
 
                        // Load the image from the temporary directory
                        let piece_image = image::open(Path::new(&temp_image_path))
                            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
                
                        // let (x, y) = if !white_orientation {
                        //     ((7 - col) * 75, (7 - row) * 75)
                        // } else {
                        //     (col * 75, row * 75)
                        // };
                        

                        let (x, y) = (col * 75, row * 75);
                
                        // Overlay the piece image on the correct square
                        for (i, j, pixel) in piece_image.pixels() {
                            let (x_new, y_new) = (x + i as usize, y + j as usize);
                            img.put_pixel(x_new as u32, y_new as u32, pixel);
                        }
                }

                }
            }
        }
    }
}

    Ok(img)
}



// fn no_move_create_chess_pieces_layer(
//     chessboard: &[[char; 8]; 8],
//     white_orientation: bool,
// ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
//     let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels

//     /*
//     1. get the piece locations as the board game-state
//     2. get the from-to move data
//     */

//     if white_orientation {
//         for (row, row_pieces) in chessboard.iter().enumerate() {
//             for (col, &piece) in row_pieces.iter().enumerate() {
//                 // Process the piece, (row, col) will be the actual coordinates for white orientation
//                 let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
//             let (piece_prefix, piece_suffix) = match piece {

//                 'p' => ("p_", square_color),
//                 'r' => ("r_", square_color),
//                 'n' => ("n_", square_color),
//                 'b' => ("b_", square_color),
//                 'q' => ("q_", square_color),
//                 'k' => ("k_", square_color),
//                 'P' => ("P_", square_color),
//                 'R' => ("R_", square_color),
//                 'N' => ("N_", square_color),
//                 'B' => ("B_", square_color),
//                 'Q' => ("Q_", square_color),
//                 'K' => ("K_", square_color),
//                 _ => continue,
            
//             };

//             let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
//             let piece_image_path = random_image_from_directory(&texture_directory)?;
//             let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//             let (x, y) = (col * 75, row * 75);

//             // Overlay the piece image on the correct square
//             for (i, j, pixel) in piece_image.pixels() {
//                 let (x_new, y_new) = (x + i as usize, y + j as usize);
//                 img.put_pixel(x_new as u32, y_new as u32, pixel);
//             }
//             }
//         }


//     } else {
//         for (row, row_pieces) in chessboard.iter().rev().enumerate() {
//             for (col, &piece) in row_pieces.iter().rev().enumerate() {
//                 // Process the piece, (7 - row, 7 - col) will be the actual coordinates for black orientation
//                 let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
//             let (piece_prefix, piece_suffix) = match piece {

//                 'p' => ("p_", square_color),
//                 'r' => ("r_", square_color),
//                 'n' => ("n_", square_color),
//                 'b' => ("b_", square_color),
//                 'q' => ("q_", square_color),
//                 'k' => ("k_", square_color),
//                 'P' => ("P_", square_color),
//                 'R' => ("R_", square_color),
//                 'N' => ("N_", square_color),
//                 'B' => ("B_", square_color),
//                 'Q' => ("Q_", square_color),
//                 'K' => ("K_", square_color),
//                 _ => continue,
            
//             };

//             let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
//             let piece_image_path = random_image_from_directory(&texture_directory)?;
//             let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//             let (x, y) = (col * 75, row * 75);

//             // Overlay the piece image on the correct square
//             for (i, j, pixel) in piece_image.pixels() {
//                 let (x_new, y_new) = (x + i as usize, y + j as usize);
//                 img.put_pixel(x_new as u32, y_new as u32, pixel);
//             }
//             }
//         }    
//     }

//     Ok(img)
// }



fn create_chessboard_with_pieces(
    game_board_state: &[[char; 8]; 8],
    game_name: &str,
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>,
    orientation_white: bool,
) -> Result<(), io::Error> {
    println!(
        "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
        &game_board_state
    );

    /*
    TODO set up temp file sandbox:

    // 1. Create a string that uniquely identifies the from-to move
    let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>


    // 2. If directory already exists, delete the old one.



    // 3. Make a unique temp directory based on game_name and the move
    let temp_directory = format!("games/{}/sandboxes/temp_{}.png", game_name, from_to_name);

    */

    /*
    Section: Make sure fresh directory exists:
    */
    // Create a string that uniquely identifies the from-to move
    let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>
    // Set alpha directory-path
    let original_dir_name = format!("games/{}/sandboxes/temp_{}", game_name, from_to_name);
    // format directory path
    let formatted_dir_name = format_directory_name(&original_dir_name);



    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    let pieces_image = create_chess_pieces_layer(game_board_state, from, to, orientation_white, game_name)?;
    let pieces_image_path = format!("{}/chess_pieces.png", formatted_dir_name);
    pieces_image
        .save(Path::new(&pieces_image_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    // Open the bottom and top blank images
    let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
    let side_vertical_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
    // let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    // let top_blank_image = image::open(Path::new(side_vertical_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    // ? TODO maybe add sandbox destination here:
    // Combine pieces with bottom blank
    let vertical_combined_path = format!("{}/vertical_combined.png", formatted_dir_name);
    combine_top_to_bottom(pieces_image_path, bottom_blank_path.to_string(), vertical_combined_path.to_string())
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // ? TODO maybe add sandbox destination here:
    // Combine vertical combined with top blank
    let final_pieces_image_path = format!("{}/final_pieces.png", formatted_dir_name);
    combine_side_by_side(side_vertical_blank_path.to_string(), vertical_combined_path.to_string(), final_pieces_image_path.to_string())
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let back_board_path = format!("games/{}/back_board_{}.png", game_name, orientation_string);
    let output_path = format!("games/{}/board.png", game_name);

    // Overlay the backboard with final pieces image
    overlay_images(Path::new(&back_board_path), Path::new(&final_pieces_image_path), Path::new(&output_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error

    }



    fn no_move_create_chess_pieces_layer(
        chessboard: &[[char; 8]; 8],
        white_orientation: bool,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
        let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels
    
        /*
        1. get the piece locations as the board game-state
        2. get the from-to move data
        */
    
        if white_orientation {
            for (row, row_pieces) in chessboard.iter().enumerate() {
                for (col, &piece) in row_pieces.iter().enumerate() {
                    // Process the piece, (row, col) will be the actual coordinates for white orientation
                    let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
                let (piece_prefix, piece_suffix) = match piece {
    
                    'p' => ("p_", square_color),
                    'r' => ("r_", square_color),
                    'n' => ("n_", square_color),
                    'b' => ("b_", square_color),
                    'q' => ("q_", square_color),
                    'k' => ("k_", square_color),
                    'P' => ("P_", square_color),
                    'R' => ("R_", square_color),
                    'N' => ("N_", square_color),
                    'B' => ("B_", square_color),
                    'Q' => ("Q_", square_color),
                    'K' => ("K_", square_color),
                    _ => continue,
                
                };
    
                let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
                let piece_image_path = random_image_from_directory(&texture_directory)?;
                let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
                let (x, y) = (col * 75, row * 75);
    
                // Overlay the piece image on the correct square
                for (i, j, pixel) in piece_image.pixels() {
                    let (x_new, y_new) = (x + i as usize, y + j as usize);
                    img.put_pixel(x_new as u32, y_new as u32, pixel);
                }
                }
            }
    
    
        } else {
            for (row, row_pieces) in chessboard.iter().rev().enumerate() {
                for (col, &piece) in row_pieces.iter().rev().enumerate() {
                    // Process the piece, (7 - row, 7 - col) will be the actual coordinates for black orientation
                    let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
                let (piece_prefix, piece_suffix) = match piece {
    
                    'p' => ("p_", square_color),
                    'r' => ("r_", square_color),
                    'n' => ("n_", square_color),
                    'b' => ("b_", square_color),
                    'q' => ("q_", square_color),
                    'k' => ("k_", square_color),
                    'P' => ("P_", square_color),
                    'R' => ("R_", square_color),
                    'N' => ("N_", square_color),
                    'B' => ("B_", square_color),
                    'Q' => ("Q_", square_color),
                    'K' => ("K_", square_color),
                    _ => continue,
                
                };
    
                let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
                let piece_image_path = random_image_from_directory(&texture_directory)?;
                let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
                let (x, y) = (col * 75, row * 75);
    
                // Overlay the piece image on the correct square
                for (i, j, pixel) in piece_image.pixels() {
                    let (x_new, y_new) = (x + i as usize, y + j as usize);
                    img.put_pixel(x_new as u32, y_new as u32, pixel);
                }
                }
            }    
        }
    
        Ok(img)
    }
    
    
    
    fn no_move_create_chessboard_with_pieces(
        game_board_state: &[[char; 8]; 8],
        game_name: &str,
        orientation_white: bool,
    ) -> Result<(), io::Error> {
        println!(
            "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
            &game_board_state
        );
    
        let orientation_string: String = if orientation_white {
            "white".to_string()
        } else {
            "black".to_string()
        };
    
        let pieces_image = no_move_create_chess_pieces_layer(game_board_state, orientation_white)?;
        let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
        pieces_image
            .save(Path::new(&pieces_image_path))
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
        // Open the bottom and top blank images
        let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
        let side_vertical_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
        // let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        // let top_blank_image = image::open(Path::new(side_vertical_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
        // Combine pieces with bottom blank
        let vertical_combined_path = format!("games/{}/vertical_combined.png", game_name);
        combine_top_to_bottom(pieces_image_path, bottom_blank_path.to_string(), vertical_combined_path.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
        // Combine vertical combined with top blank
        let final_pieces_image_path = format!("games/{}/final_pieces.png", game_name);
        combine_side_by_side(side_vertical_blank_path.to_string(), vertical_combined_path.to_string(), final_pieces_image_path.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
        let back_board_path = format!("games/{}/back_board_{}.png", game_name, orientation_string);
        let output_path = format!("games/{}/board.png", game_name);
    
        // Overlay the backboard with final pieces image
        overlay_images(Path::new(&back_board_path), Path::new(&final_pieces_image_path), Path::new(&output_path))
            .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
    }
    

// fn old_create_chessboard_with_pieces(
//     game_board_state: &[[char; 8]; 8],
//     game_name: &str,
//     from: Option<(usize, usize)>, 
//     to: Option<(usize, usize)>,
//     orientation_white: bool,
// ) -> Result<(), io::Error> {
//     println!(
//         "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
//         &game_board_state
//     );

//     /*
//     TODO set up temp file sandbox:

//     // 1. Create a string that uniquely identifies the from-to move
//     let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>


//     // 2. If directory already exists, delete the old one.



//     // 3. Make a unique temp directory based on game_name and the move
//     let temp_directory = format!("games/{}/sandboxes/temp_{}.png", game_name, from_to_name);

//     */

//     let orientation_string: String = if orientation_white {
//         "white".to_string()
//     } else {
//         "black".to_string()
//     };

//     let pieces_image = create_chess_pieces_layer(game_board_state, from, to, orientation_white, game_name)?;
//     let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
//     pieces_image
//         .save(Path::new(&pieces_image_path))
//         .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//     // Open the bottom and top blank images
//     let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
//     let side_vertical_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
//     // let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
//     // let top_blank_image = image::open(Path::new(side_vertical_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//     // Combine pieces with bottom blank
//     let vertical_combined_path = format!("games/{}/vertical_combined.png", game_name);
//     combine_top_to_bottom(pieces_image_path, bottom_blank_path.to_string(), vertical_combined_path.to_string())
//     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

//     // Combine vertical combined with top blank
//     let final_pieces_image_path = format!("games/{}/final_pieces.png", game_name);
//     combine_side_by_side(side_vertical_blank_path.to_string(), vertical_combined_path.to_string(), final_pieces_image_path.to_string())
//     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

//     let back_board_path = format!("games/{}/back_board_{}.png", game_name, orientation_string);
//     let output_path = format!("games/{}/board.png", game_name);

//     // Overlay the backboard with final pieces image
//     overlay_images(Path::new(&back_board_path), Path::new(&final_pieces_image_path), Path::new(&output_path))
//         .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error


//     /*
//     TODO clean up temp file sandbox:

//     // Recoursively delete sandbox direcory.

// // 1. Create a string that uniquely identifies the from-to move


//     // Clean up temporary files created during the process
//     for letter in &letters_order {
//         let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>
//         let temp_directory = format!("games/{}/sandboxes/temp_{}.png", game_name, from_to_name);
//         if std::path::Path::new(&temp_directory).exists() {
//             let _ = std::fs::remove_file(temp_directory);
//         }
//     }


//     */

//     // TODO Clean up files here

//     }



// fn no_move_create_chessboard_with_pieces(
//     game_board_state: &[[char; 8]; 8],
//     game_name: &str,
//     orientation_white: bool,
// ) -> Result<(), io::Error> {
//     println!(
//         "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
//         &game_board_state
//     );

//     let orientation_string: String = if orientation_white {
//         "white".to_string()
//     } else {
//         "black".to_string()
//     };

//     let pieces_image = no_move_create_chess_pieces_layer(game_board_state, orientation_white)?;
//     let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
//     pieces_image
//         .save(Path::new(&pieces_image_path))
//         .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//     // Open the bottom and top blank images
//     let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
//     let side_vertical_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
//     // let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
//     // let top_blank_image = image::open(Path::new(side_vertical_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//     // Combine pieces with bottom blank
//     let vertical_combined_path = format!("games/{}/vertical_combined.png", game_name);
//     combine_top_to_bottom(pieces_image_path, bottom_blank_path.to_string(), vertical_combined_path.to_string())
//     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

//     // Combine vertical combined with top blank
//     let final_pieces_image_path = format!("games/{}/final_pieces.png", game_name);
//     combine_side_by_side(side_vertical_blank_path.to_string(), vertical_combined_path.to_string(), final_pieces_image_path.to_string())
//     .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

//     let back_board_path = format!("games/{}/back_board_{}.png", game_name, orientation_string);
//     let output_path = format!("games/{}/board.png", game_name);

//     // Overlay the backboard with final pieces image
//     overlay_images(Path::new(&back_board_path), Path::new(&final_pieces_image_path), Path::new(&output_path))
//         .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
// }


// // Function to adjust coordinates to png pixel coordinates (0,0 top left row, column)
// // Takes the output of to_coords_chess function
// fn adjust_to_png_coords(chess_coords: (usize, usize)) -> Option<(usize, usize)> {
//     // Extract row and column from the input tuple
//     let (row, column) = chess_coords;

//     // Validate column index
//     if column > 7 {
//         return None; // Invalid column index
//     }

//     // Validate and reverse row index
//     let row_index = match row {
//         0..=7 => 7 - row, // Reverse the row index (chess starts from bottom-left)
//         _ => return None,  // Invalid row number
//     };

//     Some((row_index, column))
// }




// Function to adjust coordinates to png pixel coordinates (0,0 top left row, column)
// Takes an Option of a tuple as input
fn adjust_to_png_coords(chess_coords_option: Option<(usize, usize)>) -> Option<(usize, usize)> {
    if let Some((row, column)) = chess_coords_option {
        // Validate column index
        if column > 7 {
            return None; // Invalid column index
        }
        let column_index = match column {
            0..=7 => 7 - column, // Reverse the row index (chess starts from bottom-left)
            _ => return None,  // Invalid row number
        };


        // Validate and reverse row index
        let row_index = match row {
            0..=7 => 7 - row, // Reverse the row index (chess starts from bottom-left)
            _ => return None,  // Invalid row number
        };

        return Some((row_index, column_index));
    }
    None
}

fn generate_png_chess_board(
    game_board_state: &[[char; 8]; 8],
    game_name: &str,
    // from: Option<(usize, usize)>, 
    // to: Option<(usize, usize)>,
    from_char_u8: (char, u8),
    to_char_u8: (char, u8),
    orientation_white: bool,
) -> Result<(), io::Error> {


    // Flip the row for the PNG coordinate system
    /*
    Convert type and convert coordinates
    chess-coordinates (column, row) to png pixel (0,0 top left row, colum)
    (char, u8) -> Option<(usize, usize)> 
    */

    // inspect Before conversion
    println!("inspect Before conversion");
    dbg!("from_char_u8 -> ", from_char_u8);
    dbg!("to_char_u8 -> ", to_char_u8);

    // // convert number scale
    // let from_png_row = 7 - (from_char_u8.1 as usize - 1);
    // let to_png_row = 7 - (to_char_u8.1 as usize - 1);

    // // letter and number to number and number
    // let from: Option<(usize, usize)> = Some((from_png_row, (from_char_u8.0 as u8 - b'a') as usize));
    // let to: Option<(usize, usize)> = Some((to_png_row, (to_char_u8.0 as u8 - b'a') as usize));

    // // Convert chess coordinates to png with a function
    // let from: Option<(usize, usize)> = chess_to_png_coords(from_char_u8);
    // let to: Option<(usize, usize)> = chess_to_png_coords(to_char_u8);

    // Convert chess coordinates to 0,0 array coordinates (bottom right is 0,0)
    let from_char_u8_str = format!("{}{}", from_char_u8.0, from_char_u8.1);
    let to_char_u8_str = format!("{}{}", to_char_u8.0, to_char_u8.1);
    let from: Option<(usize, usize)> = to_coords_chess(&from_char_u8_str).ok();
    let to: Option<(usize, usize)> = to_coords_chess(&to_char_u8_str).ok();
    
    

    // inspect After conversion
    println!("inspect After conversion");
    dbg!("from -> ", from);
    dbg!("to -> ", to);


    // // manually entering test: 
    // let from = Some((7, 1)); // Replace with your desired values
    // let to = Some((5, 2));   // Replace with your desired values
    // // inspect After conversion
    // println!("should be! -> ");
    // dbg!("from -> ", from);
    // dbg!("to -> ", to);


    /*
    Section: Make sure fresh directory exists:
    */
    // Create a string that uniquely identifies the from-to move
    let from_to_name = format!("{:?}_{:?}", from, to);  // Using debug format for Option<(usize, usize)>
    // Set alpha directory-path
    let original_dir_name = format!("games/{}/sandboxes/temp_{}", game_name, from_to_name);
    // format directory path
    let formatted_dir_name = format_directory_name(&original_dir_name);
    // Check if the directory exists
    if std::path::Path::new(&formatted_dir_name).exists() {
        // Remove the old directory if it already exists
        match fs::remove_dir_all(&formatted_dir_name) {
            Ok(_) => println!("Successfully deleted older directory '{}'", formatted_dir_name),
            Err(e) => eprintln!("Failed to delete older directory: {}", e),
        }
    }
    // Create the new directory
    match fs::create_dir_all(&formatted_dir_name) {
        Ok(_) => println!("Directory '{}' created successfully!", formatted_dir_name),
        Err(e) => eprintln!("Failed to create directory: {}", e),
    }

    // Make Board
    create_chessboard_with_pieces(&game_board_state, game_name, from, to, orientation_white)?;


    /*
    Turn Off for Debugging 
    (turn on (uncomment) the "Ok(())" below
    */
    // // Perform the cleanup at the end
    // // Perform the cleanup at the end
    match clean_up_directory(&formatted_dir_name) {
        Ok(_) => {
            // Successfully cleaned up
            println!("Cleanup was successful.");
            Ok(())
        }
        Err(e) => {
            // Failed to clean up
            eprintln!("Cleanup failed: {}", e);
            Err(e)
        }
    }

    // Ok(())
}

 // Cleanup function that deletes the directory and returns Result
fn clean_up_directory(formatted_dir_name: &str) -> Result<(), io::Error> {
    match fs::remove_dir_all(&formatted_dir_name) {
        Ok(_) => {
            println!(
                "Successfully deleted the sandbox directory '{}'",
                formatted_dir_name
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to delete the sandbox directory: {}", e);
            Err(io::Error::new(
                ErrorKind::Other,
                format!("Failed to delete the sandbox directory: {}", e),
            ))
        }
    }
}
   
/*
    let board_orientation: bool = true; // 
    // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
    generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;

    let board_orientation: bool = false; // 
    // create_chessboard_with_pieces(&game_board_state, game_name, from, to, board_orientation)?;
    generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;
*/

// fn main() -> Result<(), std::io::Error> {
//     let game_name = "game";

//     // let board_orientation: bool = true; // 
//     // generate_chessboard_backboards(game_name, board_orientation)?;

//     // let board_orientation: bool = false; // 
//     // generate_chessboard_backboards(game_name, board_orientation)?;

//     /*
//     [src/main.rs:1533] "from -> " = "from -> "
//     [src/main.rs:1533] from = (
//     'c',
//     2,
//     )
//     [src/main.rs:1534] "to -> " = "to -> "
//     [src/main.rs:1534] to = (
//     'c',
//     4,
//     )
//     from: (char, u8),
//     to: (char, u8),
//     */


//     // Manual test
//     // let from = Some((7, 1)); // Replace with your desired values
//     // let to = Some((5, 2));   // Replace with your desired values

//     let from: (char, u8) = (
//         'b',
//         1,
//         );
//     let to: (char, u8) = (
//         'c',
//         3,
//         );


//     let from: (char, u8) = (
//         'b',
//         2,
//         );
//     let to: (char, u8) = (
//         'b',
//         4,
//         );

//     let from: (char, u8) = (
//         'f',
//         8,
//         );
//     let to: (char, u8) = (
//         'd',
//         6,
//         );

//     // Set up board
//     let board: [[char; 8]; 8] = [
//         ['r', 'n', 'b', 'q', 'k', ' ', ' ', 'r'],
//         ['p', 'p', 'p', 'p', ' ', 'p', 'p', 'p'],
//         [' ', ' ', ' ', 'b', ' ', 'n', ' ', ' '],
//         [' ', ' ', ' ', ' ', 'p', ' ', ' ', ' '],
//         [' ', 'P', ' ', ' ', ' ', ' ', ' ', ' '],
//         [' ', ' ', 'N', ' ', ' ', ' ', ' ', 'N'],
//         ['P', ' ', 'P', 'P', 'P', 'P', 'P', 'P'],
//         ['R', ' ', 'B', 'Q', 'K', 'B', 'n', 'R']
//     ];
//     let game_board_state:[[char; 8]; 8] = board;


//     let board_orientation: bool = false; // Black Pieces Orientation
//     // let board_orientation: bool = true; // White
//     generate_png_chess_board(&game_board_state, game_name, from, to, board_orientation)?;

    


//     Ok(())
// }



/* Timed Projects */
#[derive(Debug)]
// struct TimedGame {
//     game_name: String,
//     start_time: u64,  // POSIX timestamp
//     increments_tuple_list: Vec<(i32, i32)>,  // (on_move, time_added) tuples
//     timecontrolminutes_tuple_list: Vec<(i32, i32)>,  // (on_move, time_added) tuples

//     last_move_time: u64,
//     player_white: bool,
//     game_move_number: u64,

// }

struct TimedGame {
    game_name: String,
    start_time: u64,
    increments_tuple_list: Vec<(i32, i32)>,
    timecontrolminutes_tuple_list: Vec<(i32, i32)>,
    last_move_time: u64,
    player_white: bool,
    game_move_number: usize,
}

impl TimedGame {

    // let mut time_struct = TimedGame {
    //     game_name: "".to_string(),
    //     start_time: 0,
    //     increments_tuple_list: vec![],
    //     timecontrolminutes_tuple_list: vec![],
    //     last_move_time: 0,
    //     player_white: true,
    //     game_move_number: 0,
    // };

    fn from_str(input: &str) -> Option<TimedGame> {
        let segments: Vec<&str> = input.split('_').collect();
        if segments.len() < 2 {
            return None;
        }
    
        // Declare variables here
        let mut start_time: u64 = 0;
        let mut increments_tuple_list: Vec<(i32, i32)> = Vec::new();
        let mut timecontrolminutes_tuple_list: Vec<(i32, i32)> = Vec::new();
    
        for segment in &segments[1..] {
            let mut iter = segment.split('(');
    
            let control_type = iter.next()?;
            let tuple_str = iter.next()?;
    
            let tuple_str = &tuple_str[0..tuple_str.len() - 1];
            
            match control_type {
                "timed" => {
                    start_time = tuple_str.parse().ok()?;
                },
                "incrementseconds" => {
                    let tuple_elements: Vec<i32> = tuple_str.split(',')
                        .filter_map(|x| x.parse().ok())
                        .collect();
                    if tuple_elements.len() % 2 == 0 {
                        for i in 0..(tuple_elements.len() / 2) {
                            increments_tuple_list.push((tuple_elements[i * 2], tuple_elements[i * 2 + 1]));
                        }
                    } else {
                        return None;
                    }
                },
                "timecontrolmin" => {
                    let tuple_elements: Vec<i32> = tuple_str.split(',')
                        .filter_map(|x| x.parse().ok())
                        .collect();
                    if tuple_elements.len() % 2 == 0 {
                        for i in 0..(tuple_elements.len() / 2) {
                            timecontrolminutes_tuple_list.push((tuple_elements[i * 2], tuple_elements[i * 2 + 1]));
                        }
                    } else {
                        return None;
                    }
                },
                _ => return None,
            }
        }
    
        Some(TimedGame {
            game_name: "".to_string(), // Default to an empty string for now
            start_time,
            increments_tuple_list,
            timecontrolminutes_tuple_list,
            last_move_time: 0,
            player_white: true,
            game_move_number: 0,
        })
    }
    
    fn to_html(&self, white_time: i32, black_time: i32, game_move: i32) -> String {
        format!(
            "<p>White Time: {}</p>
            <p>Black Time: {}</p>
            <p>Move Number: {}</p>", 
            white_time, black_time, game_move
        )
    }

    // fn process_chess_time_file(game_name: &str) -> String {
    //     // ... rest of the code remains the same
    //     // Initialize time_struct here, if you need it
    // }


    fn save_to_txt(&self) -> std::io::Result<()> {
        let path = format!("games/{}.txt", self.game_name);
        let mut file = fs::File::create(path)?;

        writeln!(file, "Game Name: {}", self.game_name)?;
        writeln!(file, "Start Time: {}", self.start_time)?;

        writeln!(file, "Increments:")?;
        for (on_move, time_added) in &self.increments_tuple_list {
            writeln!(file, "{},{}", on_move, time_added)?;
        }

        writeln!(file, "Time Controls:")?;
        for (on_move, time_added) in &self.timecontrolminutes_tuple_list {
            writeln!(file, "{},{}", on_move, time_added)?;
        }
        
        Ok(())
    }


    fn parse_get_request(url: &str) -> Option<TimedGame> {
        // ... your code
        // Make sure to return a value at the end
        TimedGame::from_str("") // This is a placeholder; replace it with real data
    }

    fn load_from_txt(game_name: &str) -> io::Result<TimedGame> {
        let path = format!("games/{}/time_data.txt", game_name);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut start_time = 0;
        let mut increments_tuple_list = Vec::new();
        let mut timecontrolminutes_tuple_list = Vec::new();
        let mut last_move_time = 0;  // initialize a default value
        let mut player_white = true; // initialize a default value
        let mut game_move_number = 0; // initialize a default value
    
        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            match index {
                // 0 => game_name = &line.replace("Game Name: ", ""),
                1 => start_time = line.replace("Start Time: ", "").parse().unwrap(),
                2 => continue, // This is "Increments:"
                _ => {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() == 2 {
                        let on_move: i32 = parts[0].parse().unwrap();
                        let time_added: i32 = parts[1].parse().unwrap();
                        if line.starts_with("Time Controls:") {
                            timecontrolminutes_tuple_list.push((on_move, time_added));
                        } else {
                            increments_tuple_list.push((on_move, time_added));
                        }
                    }
                }
            }
        }

        Ok(TimedGame {
            game_name: game_name.to_string(),
            start_time,
            increments_tuple_list,
            timecontrolminutes_tuple_list,
            last_move_time,
            player_white,
            game_move_number,
        })
    }


/// Create a TimedGame from a known preset
pub fn from_preset(preset: &str) -> Option<TimedGame> {
    match preset {
        "norway120" => Some(TimedGame {
            game_name: "Norway 120 Minutes".to_string(),
            start_time: 7200, // 120 minutes in seconds
            increments_tuple_list: vec![(40, 1800)], // Add 30 minutes after move 40
            timecontrolminutes_tuple_list: vec![],
            last_move_time: 0,  // Initialize missing field
            player_white: true, // Initialize missing field
            game_move_number: 0, // Initialize missing field
        }),
        "norwayarmageddon" => Some(TimedGame {
            game_name: "Norway Armageddon".to_string(),
            start_time: 300, // 5 minutes in seconds
            increments_tuple_list: vec![], // No increment
            timecontrolminutes_tuple_list: vec![],
            last_move_time: 0,  // Initialize missing field
            player_white: true, // Initialize missing field
            game_move_number: 0, // Initialize missing field
        }),
        _ => None, // Unknown preset
    }
}



    fn process_chess_time_file(game_name: &str) -> String {
        let path = format!("games/{}/time_data.txt", game_name);
    
        if !Path::new(&path).exists() {
            return "".to_string();
        }
    
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return "".to_string(),
        };
    
        let mut reader = BufReader::new(file);
        let mut time_struct = TimedGame {
            game_name: "".to_string(),
            start_time: 0,
            increments_tuple_list: vec![],
            timecontrolminutes_tuple_list: vec![],
            last_move_time: 0,
            player_white: true,
            game_move_number: 0,
        };
    
        // Initialize `new_posix` and `current_increment` to remove "cannot find value" errors
        let new_posix: u64 = 0; // Initialize properly
        let current_increment: u64 = 0; // Initialize properly
    
        let mut this_player_time_spent = new_posix.saturating_sub(time_struct.last_move_time);
    
        // ... (rest of your code, make sure to handle errors without unwrap)
    
        "Some HTML or response".to_string() // Return value
    }




}


fn time_data_parse_get_request(url: &str) -> Option<TimedGame> {
    /*
        if let Some(game) = parse_get_request(url) {
        // Save the game
        if let Err(e) = save_game(&game) {
            println!("Failed to save game: {:?}", e);
        }
        // Output HTML
        println!("{}", game.to_html(100, 100, 1));
    } else {
        println!("Failed to parse game from URL.");
    }
     */
    // Split URL into segments
    let segments: Vec<&str> = url.split('/').collect();
    
    // Validate segment count
    if segments.len() < 4 {
        return None;
    }
    
    // Extract game data segment
    let game_data_segment = segments[3];
    
    // Pass it to TimedGame::from_str for parsing
    TimedGame::from_str(game_data_segment);


    let game_data_segment = segments[3];

    if game_data_segment == "norway120" || game_data_segment == "norwayarmageddon" {
        // Handle these special cases
        Some(TimedGame::from_preset(game_data_segment)?)
    } else {
        // Pass it to TimedGame::from_str for parsing
        TimedGame::from_str(game_data_segment)
    }

}