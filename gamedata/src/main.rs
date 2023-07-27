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

...

import json
import os

class GameData:
    def __init__(self):
        self.activity_timestamp = 0
        self.game_type = ""
        self.ip_hash_list = []
        self.game_board_state = [[' ' for _ in range(8)] for _ in range(8)]
        self.game_timestamp = 0
        self.hashed_gamephrase = ""
        self.svg_string = ""

    def save_game_data(self, game_name):
        # Create the game directory if it doesn't exist
        game_dir = f"./{game_name}"
        os.makedirs(game_dir, exist_ok=True)

        # Serialize GameData to JSON
        json_data = json.dumps(self.__dict__)

        # Write JSON data to the file
        file_path = f"{game_dir}/game_data.json"
        with open(file_path, 'w') as file:
            file.write(json_data)

    @classmethod
    def load_game_data(cls, game_name):
        # Read the JSON data from the file
        file_path = f"./{game_name}/game_data.json"
        with open(file_path, 'r') as file:
            json_data = file.read()

        # Deserialize JSON data into GameData instance
        game_data = cls()
        game_data.__dict__ = json.loads(json_data)

        return game_data

# Usage example:
def main():
    # Example data to save
    game_data = GameData()
    game_data.game_type = "chess"
    game_data.game_board_state = [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        # ... (rest of the board state)
    ]

    # Save the game data
    game_name = "my_game"
    game_data.save_game_data(game_name)
    print("Game data saved successfully.")

    # Retrieve the game data
    loaded_game_data = GameData.load_game_data(game_name)
    print("Retrieved game data:", loaded_game_data.__dict__)

if __name__ == "__main__":
    main()

