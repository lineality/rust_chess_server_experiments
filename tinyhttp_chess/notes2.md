use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn read_time_data(game_name: &str) -> io::Result<HashMap<String, String>> {
    let path = format!("game/{}/time_data.txt", game_name);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            data.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Ok(data)
}

fn generate_html(game_name: &str, data: HashMap<String, String>) -> String {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    let project_start_time: u64 = data["project_start_time_timstamp"].parse().expect("Invalid timestamp");
    let white_time_remaining: u64 = data["white_time_remaining_sec"].parse().expect("Invalid time");
    let black_time_remaining: u64 = data["black_time_remaining_sec"].parse().expect("Invalid time");
    let last_move_time: u64 = data["last_move_time"].parse().expect("Invalid last move time");

    let total_time_since_start = current_time - project_start_time;
    let this_turn_time = current_time - last_move_time;

    // More advanced time calculations can be done here based on the game rules

    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <br>
            <!-- Time Data -->
            <p>white time remaining: {}</p>
            <p>black time remaining: {}</p>
            <p>this turn so far: {}</p>
            <p>total time since start: {}</p>
            <p>moves to next time control: Placeholder</p>
            <p>next time control (min): Placeholder</p>
            <p>current increment: Placeholder</p>
            <p>next increment at time (sec): Placeholder</p>
            <p>next increment on move: Placeholder</p>
        </body>
    </html>
    "#, game_name, game_name, white_time_remaining, black_time_remaining, this_turn_time, total_time_since_start);

    html_content
}

fn main() -> io::Result<()> {
    let game_name = "t6";
    let data = read_time_data(game_name)?;
    let html_content = generate_html(game_name, data);
    println!("{}", html_content);

    Ok(())
}


use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn read_time_data(game_name: &str) -> io::Result<HashMap<String, String>> {
    let path = format!("game/{}/time_data.txt", game_name);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            data.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Ok(data)
}

fn generate_html(game_name: &str, data: HashMap<String, String>) -> String {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    let project_start_time: u64 = data["project_start_time_timstamp"].parse().expect("Invalid timestamp");
    let white_time_remaining: u64 = data["white_time_remaining_sec"].parse().expect("Invalid time");
    let black_time_remaining: u64 = data["black_time_remaining_sec"].parse().expect("Invalid time");

    let total_time_since_start = current_time - project_start_time;

    // Additional time calculations can be done here...

    let html_content = format!(r#"
    <!DOCTYPE html>
    <head>
    <meta property="og:title" content="Current Game Board" />
    <meta property="og:image" content="https://y0urm0ve.com/metatag_{}.png" />
    </head>
    <html>
        <body style="background-color:black;">
            <br>
            <img src="https://y0urm0ve.com/image_{}.png" alt="chess board" height="850px" width="850px" />
            <br>
            <!-- Time Data -->
            <p>white time remaining: {}</p>
            <p>black time remaining: {}</p>
            <p>this turn so far: TBD</p>
            <p>total time since start: {}</p>
            <p>moves to next time control: TBD</p>
            <p>next time control (min): TBD</p>
            <p>current increment: TBD</p>
            <p>next increment at time (sec): TBD</p>
            <p>next increment on move: TBD</p>
        </body>
    </html>
    "#, game_name, game_name, white_time_remaining, black_time_remaining, total_time_since_start);

    html_content
}

fn main() -> io::Result<()> {
    let game_name = "t6";
    let data = read_time_data(game_name)?;
    let html_content = generate_html(game_name, data);
    println!("{}", html_content);

    Ok(())
}
