use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind, Write, Read};
use std::fmt;

// GameData struct
struct GameData {
    game_timestamp: i64,
    hashed_gamephrase: i128,
    activity_timestamp: i64,
    game_type: String,
    ip_hash_list: Vec<i128>,
    game_board_state: [[char; 8]; 8],
    game_piece_type: String,
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
            self.game_piece_type,
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
            self.game_piece_type,
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
        let mut game_piece_type = String::new();

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
            game_piece_type,
        })
    }

    // Save GameData struct to a JSON file
    fn save_game_data(&self, game_name: &str) -> Result<(), Error> {
        let game_dir = format!("./{}", game_name);
        create_dir_all(&game_dir)?;

