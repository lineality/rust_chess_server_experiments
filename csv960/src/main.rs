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

fn get_value_from_csv(row_index: usize, file_path: &str) -> Result<(String, String), io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Enumerate lines, skip the header, and find the one matching the row_index
    for (index, line) in reader.lines().enumerate() {
        let line = line?; // Early return error if line reading fails
        if index == row_index {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                return Ok((parts[0].to_string(), parts[1].to_string()));
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid CSV format"));
            }
        }
    }

    // If no line is found at the specified index, return an error
    Err(io::Error::new(io::ErrorKind::NotFound, "Row not found"))
}

fn lookup_960(row_index: usize) {
    let file_path = "chess960_positions_simple.csv";

    match get_value_from_csv(row_index, file_path) {
        Ok((white, black)) => println!("Found: {}, {}", white, black),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let row_index = 1; // Assuming you want the first position after the header
    lookup_960(row_index); // This function does not return a value but prints the result
}

