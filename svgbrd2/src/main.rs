
/*
TODO
https://www.color-hex.com/color/9e0b00

1. make an alternate black-piece oriented version
mostly the same, but the pieces are inverted
the background is shifted
the labels are reversed
*/

use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::Document;
use std::fs;


// Function to generate the SVG chessboard
fn generate_chessboard(chessboard: &[[char; 8]; 8]) -> String {
    let mut doc = Document::new()
        .set("width", "500")  // Adjusting the width to account for labels
        .set("height", "500")  // Adjusting the height to account for labels
        .set("viewBox", (0, 0, 500, 500))
        .set("style", "background-color: #000;");  // Set background to black

    // Define labels
    let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 50 + idx * 50 + 25)  // Adjusting the x-coordinate to account for labels
            .set("y", 472)  // Positioning the label slightly above the bottom edge
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  // Set text color to white
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 32)  // Positioning the label slightly to the right of the left edge
            .set("y", 50 + idx * 50 + 35)  // Adjusting the y-coordinate to account for labels
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#757575")  // Set text color to white
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let x = 50 + col * 50;  // Adjusting the x-coordinate to account for labels
            let y = 50 + row * 50;  // Adjusting the y-coordinate to account for labels

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


                let piece_color = if square_color == "#ccc" { // for lighter background
                    if piece.is_lowercase() {
                        "#9e0b00" // darker red for dark pieces
                    } else {
                        "#665628" // darker gray for light pieces
                    }
                } else { // for darker background
                    if piece.is_lowercase() {
                        "#ff8e8e" // lighter red for dark pieces
                    } else {
                        "#ffefc1" // lighter gray for light pieces
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

    doc.to_string()
}


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

    let svg_code = generate_chessboard(&chessboard_state);
    
    println!("{}", svg_code);
    // Define the file name
    let file_name = "chessboard.svg";

    // Write the svg code to the file
    fs::write(file_name, svg_code).expect("Unable to write to file");

    println!("SVG file has been created successfully.");
    

}

