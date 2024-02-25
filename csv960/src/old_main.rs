/*
Code to look up a value in a simple csv file.
The index is the lookup key directly,
the return includes the two string fields in the csv
perhaps simply returned as a tuple of two strings
to be loaded into two varibles

in this format:
```
White,Black
BBQNNRKR,bbqnnrkr
BQNBNRKR,bqnbnrkr
BQNNRBKR,bqnnrbkr
BQNNRKRB,bqnnrkrb
```

e.g.
lookup_960(1)
-> ('BBQNNRKR','bbqnnrkr')


*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_value_from_csv(row_index: u16, file_path: &str) -> Result<String, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Enumerate lines and find the one matching the row_index
    for (index, line) in reader.lines().enumerate() {
        if index == row_index {
            return line.map_err(Into::into); // Convert Result<String, io::Error> to match function signature
        }
    }

    // If no line is found at the specified index, return an error
    Err(io::Error::new(io::ErrorKind::NotFound, "Row not found"))
}

fn lookup_960(look_this_up: u16) {
    let file_path = "chess960_positions_simple.csv";
    let row_index = 0; // Example row index, adjust as needed

    match get_value_from_csv(row_index, file_path) {
        Ok(line) => println!("Found line: {}", line),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {

    let look_this_up = 960

    match lookup_960(row_index, file_path) {
        Ok(line) => println!("Found line: {}", line),
        Err(e) => println!("Error: {}", e),
    }
}


