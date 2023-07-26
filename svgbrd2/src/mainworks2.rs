use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::Document;
use std::fs;

// Function to generate the SVG chessboard
fn generate_chessboard(chessboard: &[[char; 8]; 8]) -> String {
    let mut doc = Document::new()
        .set("width", "400")
        .set("height", "400");

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let x = col * 50;
            let y = row * 50;

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
                let mut text = Text::new()
                    .set("x", x + 25)
                    .set("y", y + 35)
                    .set("text-anchor", "middle")
                    .set("font-size", 30)
                    .set("fill", if piece.is_lowercase() { "#cc0000" } else { "#ff9999" });

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
        [' ', ' ', ' ', ' ', ' ', 'K', ' ', ' '],
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

