
Rust project: 

for this struct:
struct TimedProject {
    game_name: String, // The name of the game
    project_start_time_timestamp: u64, // Timestamp when the project started
    white_time_remaining_sec: u32, // Remaining time for white player in seconds
    black_time_remaining_sec: u32, // Remaining time for black player in seconds
    // HashMap containing increment settings
    increments_sec_sec_key_value_list: HashMap<String, (u32, u32)>,
    // HashMap containing time control settings
    timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)>,
    last_move_time: u64, // Timestamp of the last move
    player_white: bool, // Indicates if the player is white
    game_move_number: usize, // Current move number in the game
}


especially for these hash tables:
    // HashMap containing increment settings
    increments_sec_sec_key_value_list: HashMap<String, (u32, u32)>,
    // HashMap containing time control settings
    timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)>,


for this string input format:
incrimentsecsec-0,30-300,10-30,5
timecontrolmovemin-0,240-40,30-60,15

please help with functions to extract the key-value pairs
(pairs is - delimited)
(key-value are , delimited)
note, a second value for timecontrolmovemin-0,240-40,30-60,15
e.g.
timecontrolmovemin-0,240,5-40,30,30-60,15,10
would set the time-incriment at that move

the not-working function is here:

    fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<TimedProject> {

        println!("starting from_increment_and_time_control() input: {}", input);
        println!("starting from_increment_and_time_control() game_name: {}", game_name);


        // Determine the current POSIX timestamp in seconds
        let current_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => {
                println!("An error occurred while obtaining system time");
                return None; // Return None to align with function's return type
            }
        };

        // Split the input into segments by underscores
        let segments: Vec<&str> = input.split('_').collect();
        
        if segments.len() < 2 {
            return None;
        }

        let project_start_time_timestamp: u64 = current_timestamp;
        let mut increments_sec_sec_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
        let mut timecontrol_move_min_key_value_list: HashMap<String, (u32, u32)> = HashMap::new();
        let mut white_time_remaining_sec: u32 = 0;
        let mut black_time_remaining_sec: u32 = 0;

        // Parse the remaining segments
        for segment in &segments[1..] {
            println!("in from_increment_and_time_control() this segment: {}", segment);

            if *segment == "norway120" || *segment == "norwayarmageddon" {
                return TimedProject::from_preset_time_modes_chess(segment, &game_name);
            }

            let mut iter = segment.split('(');
            let control_type = iter.next()?;
            
            // Gather all tuples
            let joined_tuples: String = iter.collect::<Vec<_>>().join("(");
            let tuple_strs: Vec<&str> = joined_tuples.split(')').collect();
    
            for tuple_str in tuple_strs {
                if tuple_str.is_empty() {
                    continue;
                }
                
                let elements: Vec<u32> = tuple_str.split(',')
                    .filter_map(|x| x.parse().ok())
                    .collect();
                
                if elements.len() != 2 {
                    return None;
                }

                // Match the control type and process accordingly
                match control_type {
                    "incrementseconds" => {
                        increments_sec_sec_key_value_list.push((elements[0], elements[1]));
                    },
                    "timecontrolmin" => {
                        if elements[0] == 0 {
                            white_time_remaining_sec = elements[1] * 60;  // Convert minutes to seconds
                            black_time_remaining_sec = elements[1] * 60;  // Convert minutes to seconds
                        }
                        timecontrol_move_min_key_value_list.push((elements[0] as u32, elements[1] as u32));
                    },
                    _ => return None,
                }
            }
        }

        // Create and return the TimedProject struct
        Some(TimedProject {
            game_name: game_name.to_string(),
            project_start_time_timestamp,
            white_time_remaining_sec,
            black_time_remaining_sec,
            increments_sec_sec_key_value_list,
            timecontrol_move_min_key_value_list,
            last_move_time: 0,
            player_white: true,
            game_move_number: 0,
        })
    }

note: please do not talk about other sujects such as the whole impl, I am NOT asking about that.

//////////

Three functionst that work together related to time based around a struct:

tasks:
A. improve system to find a good minimal format for recording a dynamic number of tuple or dictionary settings in two areas (incriments and time controls)
perhaps as a dictionary in a struct text file, or perhaps separate files, perhaps tuples, perhaps dictionaries. NO SERD!
B. impliment that for how tuple data is saves, loaded, and used to generate html

For the struct:
Let's use a hashmap (~dictionary) use std::collections::HashMap;
to store the time_control and time_incriment information.

For the file:
increments_sec_sec_tuple_list:0,30-300,10-30,5
timecontrol_move_min_tuple_list:40,60-100,15


3 functions:
1. load data from file into a struct
2. update data and turn data from struct into html
3. save data back to file. 
there is no system-state memory other than that file.

here is an example of how it is saved now:

game_name: t5
project_start_time_timstamp: 7200
white_time_remaining_sec: 7200
black_time_remaining_sec: 7200
increments_sec_sec_tuple_list: [(40, 1800)]
timecontrol_move_min_tuple_list: []
last_move_time: 0
player_white: true
game_move_number: 0


Tasks:

1. maybe need to change input format...basically dropping the parenthesis...


1. update the struct so that it uses a hashmap and not a list of tuples
Let's use a hashmap (~dictionary) use std::collections::HashMap;
to store the time_control and time_incriment information.

2. update how data is saved in the file 
(format: )
For the file:
increments_sec_sec_tuple_list:0,30-300,10-30,5
timecontrol_move_min_tuple_list:40,60-100,15

note: fix how tuples are written, this never worked correctly (only pre-set worked)

3. update how data is loaded from the file 

4. update how data is updated
update_and_html() function

5. update how html is written
update_and_html() function



////


The task here is to fix how game_name is handled in the last two functions time_data_parse_setup_string and handle_segment. time_data_parse_setup_string should send two inputs to handle_segment, sending the game_name separately. somehow, various functions need to send that back to the impl of the struct, and I am not sure the calling of a single struct to store all the data is being done correctly.



Q: can gamephrase be optional?

Game Logic:

1. don't use "wrap"

2. get input via 'get' url direct text input
game_name/move/gamephrase

get IP of user (to not require gamephrase again)

3. check that game is set up: if not, then set game up.
- make game folder

4. check if move is same as last move
if same, do nothing

5. check move input is legal syntax

6. make "to" coordinates more concrete

7. make move

8. record move in game_log (whatever the name is) file (in taht game's directory)

9. record move in last-move file (in that game's directory)

10. record time-stamp of last move (in that game's directory)

(game-data json?)

11. export as html, not raw text...




fix how games are set up.

maybe 
setup/game_name/start
setup/game_name/report
setup/game_name/game_phrase

if blank...show current game board...
print last move from log...
or print log below board...


.................


todo:
- save inputs to use them in various ways

1. check format
2. update board

- check input

filter out incorrect input format
- show html unicode
- 

output html unicode

createimage overlay system, or composite image system,


new game check

reload game progress

chess 960 starting position generator

Below are some suggestions to improve the provided Rust code:

Error handling: Instead of using unwrap(), which could potentially cause a panic if the Result or Option is Err or None respectively, it would be more robust to handle the errors gracefully with match or if let constructs. However, since this is a bit more complex, I'll use the ? operator for brevity.

Separation of concerns: Consider separating server initialization, request handling, and game logic into different functions. This not only improves readability, but also helps in testing individual components of the codebase.

Improve logging and reporting: Use the log crate for better logging of different events in the application.

Efficiency: Instead of reading the file and updating the board with every move, keep track of the state of the game board in memory and update the file and the memory state when a new move is made.

Cleaner error messages: When errors happen, give the user more context and details about what went wrong. This includes when you fail to parse a move or when a move is not valid according to the rules of the game.

Input validation: Make sure to validate user input. For example, currently, a user could send a request with invalid game name or move data, causing the program to crash or behave unexpectedly.


The existing code already looks quite efficient and well-structured. However, here are some possible improvements:

Error Handling:
Instead of using .unwrap(), proper error handling should be used. This is more robust and provides more information about potential issues.

Modularize the code:
The main function is quite long and does a lot of different things. It can be broken down into smaller functions for better readability and maintainability. For instance, the request handling part inside the for loop can be moved to a separate function.

Use Constants:
Repeated string literals like "0.0.0.0:8000" and file paths could be replaced with constants.

Avoid excessive file operations:
Each time a move is made, the program reads all moves from the file and updates the board accordingly. This could be optimized by directly updating the board with each move, and using the file just as a record for the moves.

Logging and Debugging:
Consider implementing a proper logging mechanism to record different levels of details for production and debugging scenarios.
