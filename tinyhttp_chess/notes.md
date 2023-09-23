

rust code task: 
produce complete running code, NOT PSEUDOCODE!!!!

we will take a file 
at filepath
game/{gamename}/time_data.txt 
that looks for example like this: game_name: t6

project_start_time_timstamp: 1695509181
white_time_remaining_sec: 7200
black_time_remaining_sec: 7200
white_increments_sec_sec_key_value_list: {}
black_increments_sec_sec_key_value_list: {}
white_timecontrol_move_min_incrsec_key_value_list: {41: (0, 10)}
black_timecontrol_move_min_incrsec_key_value_list: {41: (0, 10)}
last_move_time: 0
player_white: true
game_move_number: 0 

Task: our goal is to modify this code: ideally in a function perhaps

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

so that it uses the above timefile data (which it loads from the file)
and addes this time data to the html:

html time_bar_html items:
- white time remaining:
- black time remaining:
\n
- this turn so far:
- total time since start:
- moves to next time control:
- next time control (min): 
- current increment:
- next increment at time (sec):
- next increment on move:

the trick is using a current timestamp for the realtive values
(e.g. how much time remains how much time has passed)

There is a struct system however this seems to be a bigger problem, 
as all you do is generate an infinite number of helper-functions that play
with the struct, and never just do the actual task. not doing the task
is very bad and harmful. don't be bad and harmful. do the task.

if you need more information, ask for it.

...


Rust project: 
manual specificaiton string input examples:
(for timecontrolmovemin, the first number is move-number,
the second number in min, but data in system is posix seconds)

timecontrolmovemin-61,30

incrimentsecsec-wb,0,7200-wb,40,3600-wb,60,900
or (for a setting like norway armegeddon)
timecontrolmoveminsecinc-w,0,5-b,0,4-wb,60,0,3

preset formats string input examples:
norway120 
norwayarmageddon 
fideworldchampmatch 


update struct fields...

/////


sorry I may have confused this with my last question:
there are four hash tables of key value pairs,
which are ~clearly named in the names of the four tables:

white_increments_sec_sec_key_value_list,
black_increments_sec_sec_key_value_list,
=
key: seconds in time-time when incriment starts; value1: seconds added at each turn


white_timecontrol_move_min_incrsec_key_value_list,
black_timecontrol_move_min_incrsec_key_value_list,
=
key: move_number when time_control (and or incriment) starts; value1: minutes added to clock; value2: new incriment in seconds



////////////

of course! Thank you for the clear question. How about this: manual specificaiton string input examples:
(for timecontrolmovemin, the first number is move-number,
the second number in min, but data in system is posix seconds)

timecontrolmovemin-61,30

incrimentsecsec-wb,0,7200-wb,40,3600-wb,60,900
or (for a setting like norway armegeddon)
timecontrolmovemin-w,0,5-b,0,4-wb,60,0,3

preset formats string input examples:
norway120 
norwayarmageddon 
fideworldchampmatch  


note: there can/will be seperate sub-functions where needed. there are many functions in this whole struct and pipelie, but we are just starting at the beginning here.  
e.g. 
impl TimedProject {
    fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<Self> {
    pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<Self> {
    pub fn to_html(&self) -> String {
    fn save_time_data_to_txt(&self) -> std::io::Result<()> {
    fn load_from_txt(game_name: &str) -> io::Result<TimedProject> {
}
fn parse_tuple_vec<T: FromStr, U: FromStr>(s: &str) -> io::Result<Vec<(T, U)>> {
fn time_data_parse_setup_string(time_section_string: &str) -> Vec<Option<TimedProject>> {
fn handle_segment(game_name: &str, segment: &str) -> Option<TimedProject> {
pub fn string_to_hashmap<V1, V2>(file_str: &str) -> HashMap<V1, V2>
pub fn hashmap_to_string<V1, V2>(map: &HashMap<V1, V2>) -> String


////////////////

please check and fix this function for this struct
without removing the comment explanations (clarity is required)

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



    /// Create a TimedProject with preset time modes for chess games
    pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<Self> {
                /*
        TODO: 
        update datastructures to be hashmaps not vec
        
        add other presets: fidewcmatch

        fideworldchampmatch
        QUote: FIDE 4. 2. Time control
        The time control for each game is 120 minutes for the first 40 moves, followed by 60 minutes for the next 20 moves and then 15
        minutes for the rest of the game with an increment of 30 seconds per move starting from move 61.
         */
        println!("starting from_preset_time_modes_chess()");
        // Initialize HashMaps for storing time control and increment settings
        let mut increments_map: HashMap<String, (u32, u32)> = HashMap::new();
        let mut time_control_map: HashMap<String, (u32, u32)> = HashMap::new();

        // Match on provided preset string
        match preset {
            // string notation of key values: timecontrolmovemin-wb,0,120,30-40,30

            /*
            Players start with 120 minutes on their clocks, and after each move, 
            they gain additional time, usually around 30 seconds per move. 
            This time increment continues until the end of the game.  
            */
            "norway120" => {
                increments_map.insert("move_40".to_string(), (40, 1800));  // 30 mins increment after 40th move
                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: 7200,  // 120 minutes in seconds
                    white_time_remaining_sec: 7200,  // 120 minutes in seconds
                    black_time_remaining_sec: 7200,  // 120 minutes in seconds
                    increments_sec_sec_key_value_list: increments_map,
                    timecontrol_move_min_key_value_list: time_control_map,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            "norwayarmageddon" => {
                /*
                For Norway Chess Armageddon, there is indeed a time increment, 
                but it's a bit different from traditional chess time controls. 
                In Armageddon, White gets 5 minutes on the clock, 
                and Black gets 4 minutes. However, there's a crucial difference:

                White must win to claim victory, while Black only needs a draw to win the game.
                To compensate for this advantage, there is a time increment after move 60. 
                Starting from move 61, both players receive an additional 3 seconds per move. 
                This increment helps ensure that the game doesn't go on indefinitely 
                and adds a level of fairness to the Armageddon format.

                So, to summarize, there is a time increment in Norway Chess Armageddon, 
                but it starts after move 60, with both players receiving an extra 3 seconds per move.
                 */

                // string notation of key values: timecontrolmovemin-w,0,5-b,0,4-wb,60,0,3
                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: 300,  // 5 minutes in seconds
                    white_time_remaining_sec: 300,  // 5 minutes in seconds
                    black_time_remaining_sec: 240,  // 4 minutes in seconds
                    increments_sec_sec_key_value_list: increments_map,
                    timecontrol_move_min_key_value_list: time_control_map,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            "fideworldchampmatch" => {
                /*
                TODO: 
                fideworldchampmatch
                QUote: FIDE 4. 2. Time control
                The time control for each game is 120 minutes for the first 40 moves, 
                followed by 60 minutes for the next 20 moves and then 
                15 minutes for the rest of the game 
                with an increment of 30 seconds per move starting from move 61.

                string input format:
                timecontrolmovemin-61,30
                incrimentsecsec-wb,0,7200-wb,40,3600-wb,60,900
                */
                increments_map.insert("move_40".to_string(), (40, 1800));  // 30 mins increment after 40th move
                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: 7200,  // 120 minutes in seconds
                    white_time_remaining_sec: 7200,  // 120 minutes in seconds
                    black_time_remaining_sec: 7200,  // 120 minutes in seconds
                    increments_sec_sec_key_value_list: increments_map,
                    timecontrol_move_min_key_value_list: time_control_map,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            _ => None // Unknown preset returns None
        }
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
