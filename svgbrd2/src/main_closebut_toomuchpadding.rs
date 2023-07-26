use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::Document;
use std::fs;


fn generate_chessboard(chessboard: &[[char; 8]; 8]) -> String {
    let mut doc = Document::new()
        .set("width", "550")  
        .set("height", "550")  
        .set("viewBox", (0, 0, 550, 550))
        .set("style", "background-color: #000;");  

    let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  

    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 75 + idx * 50)  
            .set("y", 525)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#aaa")  
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 25)  
            .set("y", 75 + idx * 50)  
            .set("text-anchor", "middle")
            .set("font-size", 20)
            .set("fill", "#aaa")  
            .add(svg::node::Text::new(label.to_string()));
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let x = 50 + col * 50;  
            let y = 50 + row * 50;  

            let square_color = if (row + col) % 2 == 0 {
                "#999"
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
                    .set("fill", if piece.is_lowercase() { "#ff0000" } else { "#99ff99" });

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
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ];

    let svg_code = generate_chessboard(&chessboard_state);
    
    println!("{}", svg_code);

    let file_name = "chessboard.svg";

    fs::write(file_name, svg_code).expect("Unable to write to file");

    println!("SVG file has been created successfully.");
    

}
