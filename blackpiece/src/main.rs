
use std::path::Path;

// Variables
type Board = [[char; 8]; 8];
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::Text;

fn main() {
    let chessboard_state: [[char; 8]; 8] = [
        ['r', 'n', 'b', 'q', ' ', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', ' ', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', 'k', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', 'p', ' ', ' '],
        [' ', ' ', 'P', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', 'P', 'N'],
        ['P', 'P', 'P', 'P', 'P', 'P', ' ', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', ' ', 'R'],
    ];

    let from = Some((6, 4)); // e2
    let to = Some((4, 4)); // e4

    let doc = generate_black_oriented_chessboard(&chessboard_state, from, to);
    
    // Define the file name
    let file_name = "chessboard_black_oriented.svg";

    // Write the svg code to the file
    svg::save(file_name, &doc).expect("Unable to write to file");

    println!("SVG file has been created successfully.");
}


// Function to generate the SVG chessboard with black orientation
fn generate_black_oriented_chessboard(
    chessboard: &[[char; 8]; 8], 
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>
    ) -> Document {

    let mut doc = Document::new()
        .set("width", "500")  
        .set("height", "500")  
        .set("viewBox", (0, 0, 500, 500))
        .set("style", "background-color: #2f0300;");  // Set background to dark red

    // Define labels, reversed for black piece orientation
    let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
    let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 50 + idx * 50 + 25)  
            .set("y", 472)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  // Set text color to dark grey
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 32)  
            .set("y", 50 + idx * 50 + 35)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
        for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
            let x = 50 + col * 50;  
            let y = 50 + row * 50;  

            let square_color = if (row + col) % 2 == 0 {
                "#ccc"
            } else {
                "#666"
            };
            
            let square = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", 50)
                .set("height", 50)
                .set("fill", square_color);

            doc = doc.add(square);

            if piece != ' ' {

                if let Some(from_coords) = from {
                    let (row, col) = from_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }
                
                if let Some(to_coords) = to {
                    let (row, col) = to_coords;
                    let x = 50 + col * 50;
                    let y = 50 + row * 50;
                
                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 50)
                        .set("height", 50)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 3);
                
                    doc = doc.add(highlight);
                }

                    
                let piece_color = if square_color == "#666" { // for darker background
                    if piece.is_uppercase() {
                        "#ffefc1" // lighter gray for light pieces
                    } else {
                        "#ff8e8e" // lighter red for dark pieces
                    }
                } else { // for lighter background
                    if piece.is_uppercase() {
                        "#665628" // darker gray for light pieces
                    } else {
                        "#9e0b00" // darker red for dark pieces
                    }
                };

                let mut text = Text::new()
                    .set("x", x + 25)
                    .set("y", y + 35)
                    .set("text-anchor", "middle")
                    .set("font-size", 30)
                    .set("fill", piece_color);

                if piece.is_uppercase() {
                    text = text.add(svg::node::Text::new(piece.to_uppercase().to_string()));
                } else {
                    text = text.add(svg::node::Text::new(piece.to_string()));
                }

                doc = doc.add(text);
            }
        }
    }

    doc
}


