Todo: 


maybe problem in 
    fn from_increment_and_time_control()


timedata file is being erased for
white_time_timecontrolmin_incrsec_key_values_list: {}
black_time_timecontrolmin_incrsec_key_values_list: {}
white_move_timecontrolmin_incrsec_key_values_list: {}
black_move_timecontrolmin_incrsec_key_values_list: {}

    // Print struct
    println!("Print Struct (debug) {:?}", project); );


    // Print struct for debug purposes
    println!("Print Struct (debug) {:?}", self);

find out where this is happening

2nd update time based tuples to be just like move based tuples

1st
given this time struct:
struct TimedProject {
    game_name: String, // The name of the game
    project_start_time_timestamp: u64, // Timestamp when the project started
    
    // current remaining time
    white_time_remaining_sec: u32,
    black_time_remaining_sec: u32,

    // current time increment
    white_current_time_increment: u32,
    black_current_time_increment: u32,

    // HashMap containing time-based time-control and increment settings
    white_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32>,
    black_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32>,

    // HashMap containing move-based time-control and increment settings
    white_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)>,

    current_move_timestamp: u64, // Timestamp of this move
    previous_move_timestamp: u64, // Timestamp of the last move
    player_white: bool, // Indicates if the player is white
    game_move_number: u32, // Current move number in the game
}

to produce html lines with this format:
	html_timedata_string.push_str(&format!(" White Time Increment starts on move {}: {} min {} sec\n", move_num, min, sec));
        html_timedata_string.push_str(&format!(" Black Time Increment starts on move {}: {} min {} sec\n", move_num, min, sec));
        
        html_timedata_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", project.white_time_remaining_sec, project.black_time_remaining_sec));
        html_timedata_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn, time_since_start));
        
        html_timedata_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
        html_timedata_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

        html_timedata_string.push_str(&format!("- White Time Remaining: {}\n- Black Time Remaining: {}\n", white_time_str, black_time_str));
        html_timedata_string.push_str(&format!("- Time Spent This Turn so Far: {}\n- Total Time Since Start of Game: {}\n", time_this_turn_str, time_since_start_str));

        html_timedata_string.push_str(&format!("- Next Time-Control at Move: {}\n- Next Time-Control (in minutes): {}\n", moves_to_next_time_control, next_time_control_min));
        html_timedata_string.push_str(&format!("- Current Increment: {}\n- Next Increment at time (sec): {}\n- Next Increment on Move: {}\n", current_increment, next_increment_time, next_increment_move));

        html_timedata_string.push_str(&format!("<li>White Time Remaining: {}</li><li>Black Time Remaining: {}</li>", white_time_str, black_time_str));
        html_timedata_string.push_str(&format!("<li>This Game Move: {}</li>", project.game_move_number));

To do this: Make boolean-flag variables, based on the results of checking two sets of tuples in a hash-table:

1. are the two current time controls the same? same_time_controls_flag = ?
if so, print one line
if not, print separate

2. is everything empty (if so, nothing to do) everything_empty_flag = ?

3. are all the keys in the hash table lower number than current_move (if so, nothing to do) all_before_current_move_flag = ?
4. are both key-value sets in the hash-table (for black-player, white player) identical? white_black_tables_idential_flag = ?
5. is there any future time-incerement? (if not, do nothing) time_incriment_exists_flag = ?

6. is there any future time-control? (if not, do nothing) future_time_control_exists_flag = ?
7. is there any time based (as opposed to move based) event? time_based_events_exist = ?

Based on these flags print only the lines that are required to print.
1. if black and white date are the same, do not print separate black and white data redundantly
2. if a field is empty, do not print anything for that field


Note: turn seconds into minutes for time-control only (NOT increments)




//////////////////////////

todo:
like this whole function
    fn from_increment_and_time_control(game_name: &str, input: &str) -> Option<Self> {

for custom inputs for time based and move
based will need to be changed


check custom many rules
(any rules)

simplify print...
or...for api mode...keep all print?


The next step is refining how these data are printed to HTML:

These printed lines could be made more clear (and less buggy):
White Time Increment starts on move 41: adding 0 sec per move.
White Time Control starts on move 41: adding 3600 min.
White Time Increment starts on move 61: adding 30 sec per move.
White Time Control starts on move 61: adding 900 min.
Black Time Increment starts on move 41: adding 0 sec per move.
White Time Control starts on move 41: adding 3600 min.
Black Time Increment starts on move 61: adding 30 sec per move.
White Time Control starts on move 61: adding 900 min.


Eight rows can be reduced to three (in this case):
    Time Control on move 41, adds 60 min.
    Time Control on move 61, adds 15 min.
    Increment starts on move 61, adding 30 sec per move.

e.g. if a value is zero, print nothing.
If white and black share the same rule, print one line.
(if separate, print separate.)




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
        // Loop through white_timecontrol_move_min_incrsec_key_values_list to dynamically include information
        for (move_num, (min, sec)) in &project.white_timecontrol_move_min_incrsec_key_values_list {
            html_timedata_string.push_str(&format!("<li>White Time Increment starts on move {}: adding {} sec per move.</li>", move_num, sec));
            html_timedata_string.push_str(&format!("<li>White Time Control starts on move {}: adding {} min.</li>", move_num, min));
        }

        // Loop through black_timecontrol_move_min_incrsec_key_values_list to dynamically include information
        for (move_num, (min, sec)) in &project.black_timecontrol_move_min_incrsec_key_values_list {
            html_timedata_string.push_str(&format!("<li>Black Time Increment starts on move {}: adding {} sec per move.</li>", move_num, sec));
            html_timedata_string.push_str(&format!("<li>Black Time Control starts on move {}: adding {} min.</li>", move_num, min));
        }




struct TimedProject {
    game_name: String, // The name of the game
    project_start_time_timestamp: u64, // Timestamp when the project started
    white_time_remaining_sec: u32, // Remaining time for white player in seconds
    black_time_remaining_sec: u32, // Remaining time for black player in seconds

    // HashMap containing increment settings
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,

    // HashMap containing time control settings
    white_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,

    current_move_timestamp: u64, // Timestamp of this move
    previous_move_timestamp: u64, // Timestamp of the last move
    player_white: bool, // Indicates if the player is white
    game_move_number: u32, // Current move number in the game
}


in pub fn generate_html_with_time_data()
you need to update the displays for incriment and time control...because
they are now split into black and white...

generate_html_with_time_data(
        // TODO this needs to be updated for black and white separate settings
        let (moves_to_next_time_control, next_time_control_min, current_increment, next_increment_time, next_increment_move) = calculate_time_control_and_increment_details(project);
        



adding time incriment

also: maybe set buffer overflow guard

also check custom time-controls setup...

note: how do time incriments get calculated...from the tuples?
current inciment stat needed?

game_name: t2
project_start_time_timestamp: 1696645932
white_time_remaining_sec: 6937
black_time_remaining_sec: 7044
white_increments_sec_sec_key_value_list: {}
black_increments_sec_sec_key_value_list: {}
white_timecontrol_move_min_incrsec_key_values_list: {41: (0, 10)}
black_timecontrol_move_min_incrsec_key_values_list: {41: (0, 10)}
current_move_timestamp: 1696646281
previous_move_timestamp: 1696646154
player_white: false
game_move_number: 41



add: next turn black/white
updating time etc not happening:
update_timedata_after_move(
wrapper_move_update_and_make_html(


print material in datt writer



ok?
string_to_hashmap_timedata and 
string_to_tuple_hashmap_timedata 


white_timecontrol_move_min_incrsec_key_values_list: {41: (0, 10)}
black_timecontrol_move_min_incrsec_key_values_list: {41: (0, 10)}



data is not being read (better)
data is not being updated
new time_data_file is not being written





rust code task: 
produce complete running code, NOT PSEUDOCODE!!!!

we will take a struct

struct TimedProject {
    game_name: String, // The name of the game
    project_start_time_timestamp: u64, // Timestamp when the project started
    white_time_remaining_sec: u32, // Remaining time for white player in seconds
    black_time_remaining_sec: u32, // Remaining time for black player in seconds

    // HashMap containing increment settings
    white_increments_sec_sec_key_value_list: HashMap<u32, u32>,
    black_increments_sec_sec_key_value_list: HashMap<u32, u32>,

    // HashMap containing time control settings
    white_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,
    black_timecontrol_move_min_incrsec_key_values_list: HashMap<u32, (u32, u32)>,

    last_move_time: u64, // Timestamp of the last move
    player_white: bool, // Indicates if the player is white
    game_move_number: u16, // Current move number in the game
}


values make look like this:

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


Our task here is to complete the top part of this function to replace the placeholder pseudo-code with actaul production code.


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
    


        // TODO: These are placeholder variables. You will need to calculate them based on your game logic.
        let moves_to_next_time_control = 0;
        let next_time_control_min = 0;
        let current_increment = 0;
        let next_increment_time = 0;
        let next_increment_move = 0;


...


rust error: I am in the process of trying to add another type of mode to be handled in this input.
time control is already handled.
now output format needs to be handled.
please suggest code/logic for a second search of the segments for the relevant terms.

code:

  fn handling_modes(game_name: &str, input: &str) -> Option<Self> {
    // Get the current timestamp
    let current_timestamp = timestamp_64();
    // Split the input string by '-'
    let segments: Vec<&str> = input.split('-').collect();
    if segments.len() < 2 {
      return None;
    }

    // Initialize empty HashMaps for storing increment and time control settings
    let mut white_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32> = HashMap::new();
    let mut black_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32> = HashMap::new();
    let mut white_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut black_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)> = HashMap::new();
     
    // Initialize remaining time variables
    let mut white_time_remaining_sec: u32 = 0;
    let mut black_time_remaining_sec: u32 = 0;

    // TODO set based on tuple for time/move zero
    // TODO mutibple?
    let white_current_time_increment: u32 = 0;
    let black_current_time_increment: u32 = 0;
     
    // Skip the first segment and loop over the rest
    for segment in segments.iter().skip(1) {
      // Split each segment by ',' and collect into a vector
      let elements: Vec<&str> = segment.split(',').collect();

      // Check for preset formats
      if ["norway120", 
        "norwayarmageddon",
        "fideworldchampmatch",
        "bypost",
        "bullet1",
        "bullet2",
        "bliiz5",                                                                                                              
        ].contains(segment) {
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

      // output mode
      let endpoint_return_mode = "api_json";
       
      // Handle segments based on the first element in the segments list
      match segments[0] {
        "incrimentsecsec" => {
          // Insert increments for white and black
          white_time_timecontrolmin_incrsec_key_values_list.insert(key, value1);
          black_time_timecontrolmin_incrsec_key_values_list.insert(key, value1);
        }
        "timecontrolmovemin" => {
          // If key is zero, set the initial time in seconds
          if key == 0 {
            white_time_remaining_sec = value1 * 60;
            black_time_remaining_sec = value1 * 60;
          }
          // Insert time controls for white and black
          white_move_timecontrolmin_incrsec_key_values_list.insert(key, (value1, value2));
          black_move_timecontrolmin_incrsec_key_values_list.insert(key, (value1, value2));
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
      white_current_time_increment,
      black_current_time_increment,
      white_time_timecontrolmin_incrsec_key_values_list,
      black_time_timecontrolmin_incrsec_key_values_list,
      white_move_timecontrolmin_incrsec_key_values_list,
      black_move_timecontrolmin_incrsec_key_values_list,
      current_move_timestamp: 0,
      previous_move_timestamp: 0,
      player_white: true,
      game_move_number: 0,
      endpoint_return_mode: endpoint_return_mode.to_string(),
    })
  }



error[E0425]: cannot find value `endpoint_return_mode` in this scope
  --> src/main.rs:6159:35
   |
6061 |   endpoint_return_mode: String, // api_json, .png, .html, .svg, ascii, txt, TUI, etc.
   |   ---------------------------- a field by that name exists in `Self`
...
6159 |       endpoint_return_mode: endpoint_return_mode.to_string(),


this value is inside 'segments' so may require a new section
endpoint_return_mode is NOT a game time control, a separate task,
likely requiring another search:

    // Skip the first segment and loop over the rest
    for segment in segments.iter().skip(1) {
      // Split each segment by ',' and collect into a vector
      let elements: Vec<&str> = segment.split(',').collect();

      // Check for preset formats
      if ["api_json", 
		"png",
		"svg",
		"html",                                                                                            
        ].contains(segment) {
        return TimedProject::from_preset_time_modes_chess(segment, game_name);
      }