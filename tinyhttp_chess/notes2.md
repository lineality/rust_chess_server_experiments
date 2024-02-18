
rust question: please advise on how to format

pub fn load_timedata_from_txt(game_name: &str) -> io::Result<TimedProject> {
    println!("\n=== start load_timedata_from_txt() ===");
    
    let path = format!("games/{}/time_data.txt", game_name);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Initialize variables
    let mut project_start_time_timestamp: u64 = 0;
    let mut white_time_remaining_sec: u32 = 0;
    let mut black_time_remaining_sec: u32 = 0;
    let mut white_current_time_increment: u32 = 0;
    let mut black_current_time_increment: u32 = 0;
    let mut white_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32> = HashMap::new();
    let mut black_time_timecontrolmin_incrsec_key_values_list: HashMap<u32, u32> = HashMap::new();
    let mut white_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut black_move_timecontrolmin_incrsec_key_values_list: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut current_move_timestamp: u64 = 0;
    let mut previous_move_timestamp: u64 = 0;
    let mut player_white: bool = true;
    let mut game_move_number: u32 = 0;
    let mut endpoint_return_mode: &str = "";

    // remove whitespace
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, ": ").collect();

        if parts.len() != 2 {
            println!("Skipping malformed line: {}", line);
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "project_start_time_timestamp" => {
                project_start_time_timestamp = parts[1].parse().unwrap_or(0);
                println!("project_start_time_timestamp: {}", project_start_time_timestamp);
            },
            "game_name" => {
                // Do nothing for game_name since it's the input to the function
            },
            "white_time_remaining_sec" => {
                white_time_remaining_sec = parts[1].parse().unwrap_or(0);
                println!("white_time_remaining_sec: {}", white_time_remaining_sec);
            },
            "black_time_remaining_sec" => {
                black_time_remaining_sec = parts[1].parse().unwrap_or(0);
                println!("black_time_remaining_sec: {}", black_time_remaining_sec);
            },
            "white_time_remaining_sec" => {
                white_current_time_increment = parts[1].parse().unwrap_or(0);
                println!("white_time_remaining_sec: {}", white_current_time_increment);
            },
            "black_time_remaining_sec" => {
                black_current_time_increment = parts[1].parse().unwrap_or(0);
                println!("black_time_remaining_sec: {}", black_current_time_increment);
            },
            "white_time_timecontrolmin_incrsec_key_values_list" => {
                white_time_timecontrolmin_incrsec_key_values_list = string_to_hashmap_timedata(parts[1]);
                println!("white_time_timecontrolmin_incrsec_key_values_list: {:?}", white_time_timecontrolmin_incrsec_key_values_list);
            },
            "black_time_timecontrolmin_incrsec_key_values_list" => {
                black_time_timecontrolmin_incrsec_key_values_list = string_to_hashmap_timedata(parts[1]);
                println!("black_time_timecontrolmin_incrsec_key_values_list: {:?}", black_time_timecontrolmin_incrsec_key_values_list);
            },
            "white_move_timecontrolmin_incrsec_key_values_list" => {
                white_move_timecontrolmin_incrsec_key_values_list = string_to_tuple_hashmap_timedata(parts[1]);
                println!("white_move_timecontrolmin_incrsec_key_values_list: {:?}", white_move_timecontrolmin_incrsec_key_values_list);
            },
            "black_move_timecontrolmin_incrsec_key_values_list" => {
                black_move_timecontrolmin_incrsec_key_values_list = string_to_tuple_hashmap_timedata(parts[1]);
                println!("black_move_timecontrolmin_incrsec_key_values_list: {:?}", black_move_timecontrolmin_incrsec_key_values_list);
            },
            "current_move_timestamp" => {
                current_move_timestamp = parts[1].parse().unwrap_or(0);
                println!("current_move_timestamp: {}", current_move_timestamp);
            },
            "previous_move_timestamp" => {
                previous_move_timestamp = parts[1].parse().unwrap_or(0);
                println!("previous_move_timestamp: {}", previous_move_timestamp);
            },
            "player_white" => {
                player_white = parts[1].parse().unwrap_or(true);
                println!("player_white: {}", player_white);
            },
            "game_move_number" => {
                game_move_number = parts[1].parse().unwrap_or(0);
                println!("game_move_number: {}", game_move_number);
            },
            "endpoint_return_mode" => {
                endpoint_return_mode = parts[1].parse().unwrap_or(0);
                println!("endpoint_return_mode: {}", endpoint_return_mode);
            },
            _ => println!("Unknown key encountered: {}\n\n", parts[0]),
        }
        
    println!("\n--- end load_timedata_from_txt() ---");
        
    }

    Ok(TimedProject {
        game_name: game_name.to_string(),
        project_start_time_timestamp,
        white_time_remaining_sec,
        black_time_remaining_sec,
        white_current_time_increment,
        black_current_time_increment,
        white_time_timecontrolmin_incrsec_key_values_list,
        black_time_timecontrolmin_incrsec_key_values_list,
        white_move_timecontrolmin_incrsec_key_values_list,
        black_move_timecontrolmin_incrsec_key_values_list,
        current_move_timestamp,
        previous_move_timestamp,
        player_white,
        game_move_number,
        endpoint_return_mode,
    })
 
       
    }

