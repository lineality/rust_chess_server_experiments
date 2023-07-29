

/*
Areas in code where this functionality will be used:
1. setup: initial setup where all game resources are created (mostly blank) for the first time.
2. (re)start, where a game's state and settings are effectively 'setup' from scratch (reset)
3. game_move: update the game_data after making a move
    - 
    - 


security and what data is pulled at what time:
two structs:
game_login
vs.
game_data...
- board

for security, should the board be separate?


*/

// GameData struct
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

struct GameData {
    hashed_gamephrase: i128,
    game_type: String,
    game_timestamp: i64,
    activity_timestamp: i64,
    ip_hash_list: Vec<i128>,
    game_board_state: [[char; 8]; 8],
}

impl GameData {
    fn new(hashed_gamephrase: i128, game_type: &str, ip_hash_list: Vec<i128>, game_board_state: [[char; 8]; 8]) -> io::Result<Self> {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Error while getting duration: {}", err),
            )
        })?;
        let timestamp_secs = duration.as_secs() as i64;

        Ok(Self {
            hashed_gamephrase,
            game_type: game_type.to_string(),
            game_timestamp: timestamp_secs,
            activity_timestamp: timestamp_secs,
            ip_hash_list,
            game_board_state,
        })
    }

    fn to_json(&self, dir_path: &str) -> io::Result<()> {
        let ip_hash_list = self.ip_hash_list.iter()
            .map(|&n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let game_board_state = self.game_board_state.iter()
            .map(|row| {
                row.iter()
                    .map(|&c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join(", ");

        let json_data = format!(
            r#"{{
                "hashed_gamephrase": {},
                "game_type": "{}",
                "game_timestamp": {},
                "activity_timestamp": {},
                "ip_hash_list": [{}],
                "game_board_state": ["{}"]
            }}"#,
            self.hashed_gamephrase,
            self.game_type,
            self.game_timestamp,
            self.activity_timestamp,
            ip_hash_list,
            game_board_state
        );

        let json_path = format!("{}/game_data.json", dir_path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&json_path)?;

        writeln!(file, "{}", json_data)?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let hashed_gamephrase = 1234567890123456789;  // Example value
    let game_type = "Game Type";
    let ip_hash_list = vec![123, 456, 789];
    let game_board_state = [['a'; 8]; 8];
    
    let game_data = GameData::new(hashed_gamephrase, game_type, ip_hash_list, game_board_state)?;
    game_data.to_json("")?;
    
    Ok(())
}
