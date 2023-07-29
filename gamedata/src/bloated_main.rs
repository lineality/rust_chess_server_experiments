use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind, Write, Read};
use std::fmt;


/*
Areas in code where this functionality will be used:
1. setup: initial setup where all game resources are created (mostly blank) for the first time.
2. (re)start, where a game's state and settings are effectively 'setup' from scratch (reset)
3. game_move: update the game_data after making a move
    - 
    - 

*/

// GameData struct
struct GameData {
    hashed_gamephrase: i128,
    game_type: String,
    game_timestamp: i64,
    activity_timestamp: i64,
    ip_hash_list: Vec<i128>,
    game_board_state: [[char; 8]; 8],
}

impl fmt::Debug for GameData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameData {{ game_timestamp: {}, hashed_gamephrase: {}, activity_timestamp: {}, game_type: {}, ip_hash_list: {:?}, game_board_state: {:?}, game_piece_type: {} }}",
            self.game_timestamp,
            self.hashed_gamephrase,
            self.activity_timestamp,
            self.game_type,
            self.ip_hash_list,
            self.game_board_state, 
        )
    }
}

impl GameData {
    // Convert GameData struct to JSON string
    fn to_json(&self) -> String {
        format!(
            r#"{{"activity_timestamp":{},"game_type":"{}","ip_hash_list":[{}],"game_board_state":[{}] ,"game_timestamp":{},"hashed_gamephrase":{},"game_piece_type":"{}"}}"#,
            self.activity_timestamp,
            self.game_type,
            self.ip_hash_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            self.game_board_state
                .iter()
                .map(|row| format!("[{}]", row.iter().map(|&ch| format!("'{}'", ch)).collect::<Vec<String>>().join(",")))
                .collect::<Vec<String>>()
                .join(","),
            self.game_timestamp,
            self.hashed_gamephrase,
        )
    }

    // Parse JSON string to GameData struct
    fn from_json(json_data: &str) -> Result<Self, Error> {
        let json_data = json_data.trim();
        if !json_data.starts_with('{') || !json_data.ends_with('}') {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid JSON format"));
        }

        let mut activity_timestamp = 0;
        let mut game_type = String::new();
        let mut ip_hash_list = Vec::new();
        let mut game_board_state = [[char::default(); 8]; 8];
        let mut game_timestamp = 0;
        let mut hashed_gamephrase = 0;

        let fields = json_data[1..json_data.len() - 1].split(',');
        for field in fields {
            let parts: Vec<&str> = field.trim().splitn(2, ':').collect();  // splitn(2, ':') to ensure we only split on the first colon
            if parts.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid JSON field"));
            }
            let key = parts[0].trim_matches('"');
            let value = parts[1].trim();

            match key {
                "activity_timestamp" => activity_timestamp = value.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid activity_timestamp"))?,
                "game_type" => game_type = value.trim_matches('"').to_string(),
                "ip_hash_list" => {
                    let hash_values = value.trim_matches('[').trim_matches(']');
                    for val in hash_values.split(',') {
                        ip_hash_list.push(val.trim().parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid ip_hash_list"))?);
                    }
                }
                "game_board_state" => {
                    let rows = value.trim_matches('[').trim_matches(']');
                    let rows = rows.split("],[").collect::<Vec<&str>>();
                    for (i, row) in rows.iter().enumerate() {
                        let chars: Vec<char> = row.trim_matches('[').trim_matches(']').split(',').map(|ch| ch.trim_matches('\'').chars().next().unwrap_or_default()).collect();
                        for (j, &ch) in chars.iter().enumerate() {
                            game_board_state[i][j] = ch;
                        }
                    }
                }
                "game_timestamp" => game_timestamp = value.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid game_timestamp"))?,
                "hashed_gamephrase" => hashed_gamephrase = value.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid hashed_gamephrase"))?,
                "game_piece_type" => game_piece_type = value.trim_matches('"').to_string(),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown JSON field")),
            }
        }

        Ok(GameData {
            activity_timestamp,
            game_type,
            ip_hash_list,
            game_board_state,
            game_timestamp,
            hashed_gamephrase,
        })
    }


    fn save_game_data(&self, game_name: &str) -> Result<(), Error> {
        let game_dir = format!("./{}", game_name);
        create_dir_all(&game_dir)?;

        let json_data = self.to_json();

        let file_path = format!("{}/game_data.json", game_dir);
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    fn load_game_data(game_name: &str) -> Result<GameData, Error> {
        let file_path = format!("./{}/game_data.json", game_name);
        let mut file = File::open(file_path)?;

        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let game_data = GameData::from_json(&json_data)?;
        Ok(game_data)
    }
}

fn main() {
    let game_data = GameData {
        activity_timestamp: 0,
        game_type: String::from("chess"),
        ip_hash_list: Vec::new(),
        game_board_state: [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ],
        game_timestamp: 0,
        hashed_gamephrase: String::new(),
    };

    let game_name = "my_game";
    match game_data.save_game_data(game_name) {
        Ok(_) => println!("Game data saved successfully."),
        Err(e) => println!("Error while saving game data: {}", e),
    }

    match GameData::load_game_data(game_name) {
        Ok(data) => println!("Retrieved game data: {:?}", data),
        Err(e) => println!("Error while loading game data: {}", e),
    }
}




fn setup_new_game(game_type: &str, game_name: &str, game_phrase: &str, ip_hash: i128) -> std::io::Result<()> {

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

    // organized game_data
    let timestamp = get_current_timestamp(); // Assuming this is a function to get current timestamp
    let hashed_gamephrase = hash_game_phrase(game_phrase); // Assuming this is a function to hash game phrase

    // Initialize the GameData
    let game_data = GameData {
        activity_timestamp: timestamp,
        game_type: game_type.to_string(),
        ip_hash_list: Vec::new(),
        game_board_state: board,
        game_timestamp: timestamp,
        hashed_gamephrase: hashed_gamephrase,
    };



    // save svg in game diretory

    // Save game_data
    match game_data.save_game_data(game_name) {
        Ok(_) => println!("Game data saved successfully."),
        Err(e) => println!("Error while saving game data: {}", e),
    }


    // Posix Timestamp in gamedata json file:

    // append ip hash to list, in this case it is the first and only item (initial setup)
    // but in general the impl for game_data should allow for appending to ip_hash_list




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

