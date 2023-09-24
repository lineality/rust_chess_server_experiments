
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


Our task here is to 


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