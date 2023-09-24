
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

Here is more information about the hashmaps of data on incriments and time controls:

// Time Section
/* 
Timed Projects 

e.g. string 
thisgamename_incrimentseconds-(0,30)-(300,10)-(30,5)_timecontrolmin-(0,240)-(40,30)-(60,15)

thisgamename_incrimentseconds-0,30-300,10-30,5_timecontrolmin-0,240-40,30-60,15

or just strip

*/

#[derive(Debug)]
/*
Time Modes 
*/

struct TimedProject {
    game_name: String, // The name of the game
    project_start_time_timestamp: u64, // Timestamp when the project started
    white_time_remaining_sec: u32, // Remaining time for white player in seconds
    black_time_remaining_sec: u32, // Remaining time for black player in seconds

    // HashMap containing increment settings
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,

    // HashMap containing time control settings
    white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)>,

    last_move_time: u64, // Timestamp of the last move
    player_white: bool, // Indicates if the player is white
    game_move_number: u16, // Current move number in the game
}

impl TimedProject {
// Modified `from_str` function to handle the described format

    fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<Self> {
        // Get the current timestamp
        let current_timestamp = timestamp_64();
        // Split the input string by '-'
        let segments: Vec<&str> = input.split('-').collect();
        if segments.len() < 2 {
            return None;
        }

        // Initialize empty HashMaps for storing increment and time control settings
        let mut white_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();
        let mut black_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();
        let mut white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();
        let mut black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();
        
        // Initialize remaining time variables
        let mut white_time_remaining_sec: u32 = 0;
        let mut black_time_remaining_sec: u32 = 0;

        // Skip the first segment and loop over the rest
        for segment in segments.iter().skip(1) {
            // Split each segment by ',' and collect into a vector
            let elements: Vec<&str> = segment.split(',').collect();

            // Check for preset formats
            if ["norway120", "norwayarmageddon"].contains(segment) {
                return TimedProject::from_preset_time_modes_chess(segment, game_name);
            }

            // Minimum two elements should be there in each segment
            if elements.len() < 2 {
                return None;
            }

            // Parse key, value1, and optional value2
            let key = elements[0].parse::<u32>().ok()?;
            let value1 = elements[1].parse::<u32>().ok()?;
            let value2 = elements.get(2).and_then(|x| x.parse().ok())?;

            // Handle segments based on the first element in the segments list
            match segments[0] {
                "incrimentsecsec" => {
                    // Insert increments for white and black
                    white_increments_sec_sec_key_value_list.insert(key, value1);
                    black_increments_sec_sec_key_value_list.insert(key, value1);
                }
                "timecontrolmovemin" => {
                    // If key is zero, set the initial time in seconds
                    if key == 0 {
                        white_time_remaining_sec = value1 * 60;
                        black_time_remaining_sec = value1 * 60;
                    }
                    // Insert time controls for white and black
                    white_timecontrol_move_min_incrsec_key_value_list.insert(key, (value1, value2));
                    black_timecontrol_move_min_incrsec_key_value_list.insert(key, (value1, value2));
                }
                _ => return None,
            }
        }

        // Construct and return the TimedProject object
        Some(TimedProject {
            game_name: game_name.to_string(),
            project_start_time_timestamp: current_timestamp,
            white_time_remaining_sec,
            black_time_remaining_sec,
            white_increments_sec_sec_key_value_list,
            black_increments_sec_sec_key_value_list,
            white_timecontrol_move_min_incrsec_key_value_list,
            black_timecontrol_move_min_incrsec_key_value_list,
            last_move_time: 0,
            player_white: true,
            game_move_number: 0,
        })
    }


    // Updates fields related to the last move
    fn update_after_move(&mut self, move_data: &str) {
        /* 
        update timestamp
        if letter in move data is capital that's white
        if white moves, player_white: false
        if black moves, player_white: true
        if player_white changes, THEN game_move_number += 1
        */

        // Update last move time
        self.last_move_time = timestamp_64();
        
        // Store the old player color
        let old_is_white = self.player_white;

        // Check if the move is by white or black based on the case of the first two letters
        let is_white_move = move_data.chars().take(2).any(|c| c.is_uppercase());

        // Logic to toggle player_white and update game_move_number
        if is_white_move {
            self.player_white = false;
        } else {
            self.player_white = true;
        }

        // Last, increment game move number only if player color changed
        if old_is_white != self.player_white {
            self.game_move_number += 1;
        }
    }

    

    /// Create a TimedProject with preset time modes for chess games
    pub fn from_preset_time_modes_chess(preset: &str, game_name: &str) -> Option<Self> {

        /*
        Fide World Championship Match 

        fidewcmatch
        QUote: FIDE 4. 2. Time control
        The time control for each game is 120 minutes for the first 40 moves, 
        followed by 60 minutes for the next 20 moves and 
        then 15 minutes for the rest of the game 
        with an increment of 30 seconds per move starting from move 61.

        move_41 + 60 min
        move_61 + 15 min + 30sec incriment
        */

        /*
        Norway Chess Armageddon

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

        /*
        Norway "120" Classical Game 
        The time control is 120 minutes for the entire game, 
        with a 10-second increment per move starting on move 41.

        https://www.chess.com/events/2023-norway-chess#format
         */


        // Initialize HashMaps for time control and increment settings

        // White Incriments
        // Key: seconds in time when increment starts
        // Value: (seconds added at each turn)
        let mut white_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();

        // Black Incriments
        // Key: seconds in time when increment starts
        // Value: (seconds added at each turn)
        let mut black_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();

        // White Incriments
        // Key: move_number when time_control starts (total time on clock) / new incriment starts (time added with each move)
        // Value_1: (minutes added to clock, new increment in seconds)
        // Value_2: (seconds incriment started at that move-number)
        let mut white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();

        // Black Incriments
        // Key: move_number when time_control starts (total time on clock) / new incriment starts (time added with each move)
        // Value_1: (minutes added to clock, new increment in seconds)
        // Value_2: (seconds incriment started at that move-number)
        let mut black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();



        // Match on provided preset string
        match preset {
            "norway120" => {
                // there is no time-based incriment rule
                // 10-second increment per move starting on move 41.
                white_timecontrol_move_min_incrsec_key_value_list.insert(41, (0, 10)); // 10-second increment per move starting on move 41.
                black_timecontrol_move_min_incrsec_key_value_list.insert(41, (0, 10)); // 10-second increment per move starting on move 41. 

                
                
                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: timestamp_64(), // TODO: This is a TIMESTAMP!!!! not a starting time amoount!!!!
                    white_time_remaining_sec: 7200,
                    black_time_remaining_sec: 7200,
                    white_increments_sec_sec_key_value_list,
                    black_increments_sec_sec_key_value_list,
                    white_timecontrol_move_min_incrsec_key_value_list,
                    black_timecontrol_move_min_incrsec_key_value_list,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            "norwayarmageddon" => {
                // there is no time-based incriment rule
                // 3 secs increment after 61st move
                white_timecontrol_move_min_incrsec_key_value_list.insert(61, (0, 3)); // 3 secs increment after 61st move
                black_timecontrol_move_min_incrsec_key_value_list.insert(61, (0, 3)); // 3 secs increment after 61st move

                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: timestamp_64(), // TODO: This is a TIMESTAMP!!!! not a starting time amoount!!!!
                    white_time_remaining_sec: 300,  // 5 min for white
                    black_time_remaining_sec: 240,  // four mins for black
                    white_increments_sec_sec_key_value_list,
                    black_increments_sec_sec_key_value_list,
                    white_timecontrol_move_min_incrsec_key_value_list,
                    black_timecontrol_move_min_incrsec_key_value_list,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            "fideworldchampmatch" => {
                // Initialize special rules for FIDE World Championship match
                // increments_map.insert("move_40".to_string(), (40, 1800));  // 30 mins increment after 40th move
                // increments_map.insert("move_61".to_string(), (61, 30));  // 30 secs increment after 61st move

                // Rule 1
                // move_41 + 60 min
                // 60 min = 3600 sec
                white_timecontrol_move_min_incrsec_key_value_list.insert(41, (3600, 0)); // 30 mins increment after 40th move
                black_timecontrol_move_min_incrsec_key_value_list.insert(41, (3600, 0)); // 30 mins increment after 40th move                

                // Rule 2
                // move_61 + 15 min + 30sec incriment
                // 15 min = 900 sec
                white_timecontrol_move_min_incrsec_key_value_list.insert(61, (900, 30)); // 30 mins increment after 40th move
                black_timecontrol_move_min_incrsec_key_value_list.insert(61, (900, 30)); // 30 mins increment after 40th move                

        

                Some(Self {
                    game_name: game_name.to_string(),
                    project_start_time_timestamp: timestamp_64(), // TODO: This is a TIMESTAMP!!!! not a starting time amoount!!!!
                    white_time_remaining_sec: 7200,
                    black_time_remaining_sec: 7200,
                    white_increments_sec_sec_key_value_list,
                    black_increments_sec_sec_key_value_list,
                    white_timecontrol_move_min_incrsec_key_value_list,
                    black_timecontrol_move_min_incrsec_key_value_list,
                    last_move_time: 0,
                    player_white: true,
                    game_move_number: 0,
                })
            },
            _ => None // Unknown preset
        }
    }


            
    
    fn save_time_data_to_txt(&self) -> std::io::Result<()> {
        // Generate the intended path for debugging purposes
        let path = format!("games/{}/time_data.txt", self.game_name);
        println!("Attempting to save at path: {}", path);

        // Try to create the file
        match fs::File::create(&path) {
            Ok(mut file) => {
                writeln!(file, "game_name: {}", self.game_name)?;
                writeln!(file, "project_start_time_timstamp: {}", self.project_start_time_timestamp)?;
                writeln!(file, "white_time_remaining_sec: {}", self.white_time_remaining_sec)?;
                writeln!(file, "black_time_remaining_sec: {}", self.black_time_remaining_sec)?;
                writeln!(file, "white_increments_sec_sec_key_value_list: {:?}", self.white_increments_sec_sec_key_value_list)?;
                writeln!(file, "black_increments_sec_sec_key_value_list: {:?}", self.black_increments_sec_sec_key_value_list)?;
                writeln!(file, "white_timecontrol_move_min_incrsec_key_value_list: {:?}", self.white_timecontrol_move_min_incrsec_key_value_list)?;
                writeln!(file, "black_timecontrol_move_min_incrsec_key_value_list: {:?}", self.black_timecontrol_move_min_incrsec_key_value_list)?;
                writeln!(file, "last_move_time: {}", self.last_move_time)?;
                writeln!(file, "player_white: {}", self.player_white)?;
                writeln!(file, "game_move_number: {}", self.game_move_number)?;

                println!("Successfully saved time_data.txt at path: {}", path);
                Ok(())
            }
            Err(e) => {
                // Provide more contextual information about where it failed
                println!("Failed to save project at path: {}. Error is: {}", path, e);
                Err(e)
            }
        }
    }


    fn load_timedata_from_txt(game_name: &str) -> io::Result<TimedProject> {
        let path = format!("games/{}/time_data.txt", game_name);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
    
        let mut project_start_time_timestamp: u64 = 0;
        let mut white_time_remaining_sec: u32 = 0;
        let mut black_time_remaining_sec: u32 = 0;
        let mut white_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();
        let mut black_increments_sec_sec_key_value_list: HashMap<u32, u32> = HashMap::new();
        let mut white_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();
        let mut black_timecontrol_move_min_incrsec_key_value_list: HashMap<u32, (u32, u32)> = HashMap::new();
        let mut last_move_time: u64 = 0;
        let mut player_white: bool = true;
        let mut game_move_number: u16 = 0;


        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(": ").collect();
    
            match parts[0] {
                "project_start_time_timestamp" => {
                    project_start_time_timestamp = parts[1].parse().unwrap();
                }
                "white_time_remaining_sec" => {
                    white_time_remaining_sec = parts[1].parse().unwrap();
                }
                "black_time_remaining_sec" => {
                    black_time_remaining_sec = parts[1].parse().unwrap();
                }
                "white_increments_sec_sec_key_value_list" => {
                    white_increments_sec_sec_key_value_list = string_to_hashmap(parts[1]);
                }
                "black_increments_sec_sec_key_value_list" => {
                    black_increments_sec_sec_key_value_list = string_to_hashmap(parts[1]);
                }
                "white_timecontrol_move_min_incrsec_key_value_list" => {
                    white_timecontrol_move_min_incrsec_key_value_list = string_to_tuple_hashmap(parts[1]);
                }
                "black_timecontrol_move_min_incrsec_key_value_list" => {
                    black_timecontrol_move_min_incrsec_key_value_list = string_to_tuple_hashmap(parts[1]);
                }
                "last_move_time" => {
                    last_move_time = parts[1].parse().unwrap();
                }
                "player_white" => {
                    player_white = parts[1].parse().unwrap();
                }
                "game_move_number" => {
                    game_move_number = parts[1].parse().unwrap();
                }
                _ => {}
            }
        }
    
        Ok(TimedProject {
            game_name: game_name.to_string(),
            project_start_time_timestamp,
            white_time_remaining_sec,
            black_time_remaining_sec,
            white_increments_sec_sec_key_value_list,
            black_increments_sec_sec_key_value_list,
            white_timecontrol_move_min_incrsec_key_value_list,
            black_timecontrol_move_min_incrsec_key_value_list,
            last_move_time,
            player_white,
            game_move_number,
        })
    }
    

// End of struct implimentation: TimedProject
}

