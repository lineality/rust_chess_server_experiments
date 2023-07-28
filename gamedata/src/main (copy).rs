use serde::{Serialize, Deserialize};
use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind};
use std::path::Path;

// Implement Serialize and Deserialize for GameData
#[derive(Serialize, Deserialize)]
struct GameData {
    // ... (rest of the struct definition)
}

impl GameData {
    // Function to save game data as a JSON file
    fn save_game_data(&self, game_name: &str) -> Result<(), Error> {
        // Create the game directory if it doesn't exist
        let game_dir = format!("./{}", game_name);
        create_dir_all(&game_dir)?;

        // Serialize GameData to JSON
        let json_data = serde_json::to_string(&self)?;

        // Write JSON data to the file
        let file_path = format!("{}/game_data.json", game_dir);
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    // Function to retrieve game data from a JSON file
    fn load_game_data(game_name: &str) -> Result<GameData, Error> {
        // Read the JSON data from the file
        let file_path = format!("./{}/game_data.json", game_name);
        let file = File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let game_data: GameData = serde_json::from_reader(reader)?;

        Ok(game_data)
    }
}
fn main() {
    // Example data to save
    let game_data = GameData {
        activity_timestamp: 0,
        game_type: String::from("chess"),
        ip_hash_list: Vec::new(),
        game_board_state: [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            // ... (rest of the board state)
        ],
        game_timestamp: 0,
        hashed_gamephrase: String::new(),
        svg_string: String::new(),
    };

    // Save the game data
    let game_name = "my_game";
    match game_data.save_game_data(game_name) {
        Ok(_) => println!("Game data saved successfully."),
        Err(e) => println!("Error while saving game data: {}", e),
    }

    // Retrieve the game data
    match GameData::load_game_data(game_name) {
        Ok(data) => println!("Retrieved game data: {:?}", data),
        Err(e) => println!("Error while loading game data: {}", e),
    }
}

use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind, Write, Read};
use std::path::Path;

struct GameData {
    activity_timestamp: i64,
    game_type: String,
    ip_hash_list: Vec<i32>,
    game_board_state: [[char; 8]; 8], // Assuming 8x8 2D array of characters
    game_timestamp: i64,
    hashed_gamephrase: String,
    svg_string: String,
}

impl GameData {
    // Function to save game data as a JSON-like file
    fn save_game_data(&self, game_name: &str) -> Result<(), Error> {
        // Create the game directory if it doesn't exist
        let game_dir = format!("./{}", game_name);
        create_dir_all(&game_dir)?;

        // Serialize GameData to a JSON-like string
        let json_data = format!(
            r#"{{
                "activity_timestamp": {},
                "game_type": "{}",
                "ip_hash_list": {:?},
                "game_board_state": {:?},
                "game_timestamp": {},
                "hashed_gamephrase": "{}",
                "svg_string": "{}"
            }}"#,
            self.activity_timestamp,
            self.game_type,
            self.ip_hash_list,
            self.game_board_state,
            self.game_timestamp,
            self.hashed_gamephrase,
            self.svg_string
        );

        // Write JSON-like data to the file
        let file_path = format!("{}/game_data.json", game_dir);
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    // Function to retrieve game data from a JSON-like file
    fn load_game_data(game_name: &str) -> Result<GameData, Error> {
        // Read the JSON-like data from the file
        let file_path = format!("./{}/game_data.json", game_name);
        let mut file = File::open(file_path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        // Parse JSON-like data into GameData instance
        let game_data: GameData = json_data.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        Ok(game_data)
    }
}

// Example usage:
fn main() {
    // Example data to save
    let game_data = GameData {
        activity_timestamp: 0,
        game_type: String::from("chess"),
        ip_hash_list: Vec::new(),
        game_board_state: [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            // ... (rest of the board state)
        ],
        game_timestamp: 0,
        hashed_gamephrase: String::new(),
        svg_string: String::new(),
    };

    // Save the game data
    let game_name = "my_game";
    match game_data.save_game_data(game_name) {
        Ok(_) => println!("Game data saved successfully."),
        Err(e) => println!("Error while saving game data: {}", e),
    }

    // Retrieve the game data
    match GameData::load_game_data(game_name) {
        Ok(data) => println!("Retrieved game data: {:?}", data),
        Err(e) => println!("Error while loading game data: {}", e),
    }
}

