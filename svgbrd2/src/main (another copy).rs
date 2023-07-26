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
    let column_labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 50 + idx * 50 + 25)  // Adjusting the x-coordinate to account for labels
            .set("y", 465)  // Positioning the label slightly above the bottom edge
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#fff")  // Set text color to white
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 25)  // Positioning the label slightly to the right of the left edge
            .set("y", 50 + idx * 50 + 35)  // Adjusting the y-coordinate to account for labels
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#fff")  // Set text color to white
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

