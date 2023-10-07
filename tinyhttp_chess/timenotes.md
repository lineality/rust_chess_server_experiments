todo: 



in pub fn generate_html_with_time_data()
you need to update the displays for incriment and time control...because
they are now split into black and white...





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