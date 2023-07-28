use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind, Write, Read};
use std::path::Path;
use std::fmt;

struct GameData {
    activity_timestamp: i64,
    game_type: String,
    ip_hash_list: Vec<i32>,
    game_board_state: [[char; 8]; 8], // Assuming 8x8 2D array of characters
    game_timestamp: i64,
    hashed_gamephrase: String,
    svg_string: String,
}

impl fmt::Debug for GameData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameData {{ activity_timestamp: {}, game_type: {}, ip_hash_list: {:?}, game_board_state: {:?}, game_timestamp: {}, hashed_gamephrase: {}, svg_string: {} }}",
               self.activity_timestamp,
               self.game_type,
               self.ip_hash_list,
               self.game_board_state,
               self.game_timestamp,
               self.hashed_gamephrase,
               self.svg_string
        )
    }
}

impl GameData {
    fn to_json(&self) -> String {
        format!(
            r#"{{"activity_timestamp":{},"game_type":"{}","ip_hash_list":[{}],"game_board_state":[{}] ,"game_timestamp":{},"hashed_gamephrase":"{}","svg_string":"{}"}}"#,
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
            self.svg_string
        )
    }

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
        let mut hashed_gamephrase = String::new();
        let mut svg_string = String::new();

        let fields = json_data[1..json_data.len() - 1].split(',');
        for field in fields {
            let parts: Vec<&str> = field.trim().split(':').collect();
            if parts.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid JSON field"));
            }
            let key = parts[0].trim_matches('"');
            let value = parts[1].trim();

            match key {
                "activity_timestamp" => activity_timestamp = value.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid activity_timestamp"))?,
                "game_type" => game_type = value.trim_matches('"').to_string(),
                "ip_hash_list" => {
                    for val in value.split(',') {
                        ip_hash_list.push(val.trim().parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid ip_hash_list"))?);
                    }
                }
                "game_board_state" => {
                    let board_state = value.trim_matches('[').trim_matches(']').split(']');
                    let mut i = 0;
                    for row in board_state {
                        let chars: Vec<char> = row.trim_matches('[').trim_matches(']').split(',').map(|ch| ch.trim_matches('\'').chars().next().unwrap_or_default()).collect();
                        for (j, &ch) in chars.iter().enumerate() {
                            game_board_state[i][j] = ch;
                        }
                        i += 1;
                    }
                }
                "game_timestamp" => game_timestamp = value.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid game_timestamp"))?,
                "hashed_gamephrase" => hashed_gamephrase = value.trim_matches('"').to_string(),
                "svg_string" => svg_string = value.trim_matches('"').to_string(),
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
            svg_string,
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
        svg_string: String::new(),
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

