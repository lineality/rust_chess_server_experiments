


// Variables

use std::fs::File;
use std::io::{self, Read};
use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::node::element::Image;
use base64::Engine; // Bring the Engine trait into scope
use base64::engine::general_purpose::STANDARD;


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

    let doc = generate_white_oriented_chessboard(&chessboard_state, from, to);
    
    // Define the file name
    let file_name = "chessboard_black_oriented.svg";

    // Write the svg code to the file
    svg::save(file_name, &doc).expect("Unable to write to file");

    println!("SVG file has been created successfully.");
}


fn generate_white_oriented_chessboard(
    chessboard: &[[char; 8]; 8], 
    from: Option<(usize, usize)>, 
    to: Option<(usize, usize)>
) -> Document {

    let mut doc = Document::new()
        .set("width", "1000")  // Adjusting the width to account for labels
        .set("height", "1000")  // Adjusting the height to account for labels
        .set("viewBox", (0, 0, 1000, 1000))
        .set("style", "background-color: #000;");  // Set background to black

    // Define labels
    let column_labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let row_labels = ['8', '7', '6', '5', '4', '3', '2', '1'];  // Chessboard starts with 8 from the top

    // Add column labels
    for (idx, label) in column_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 100 + idx * 100 + 50)  // Adjusting the x-coordinate to account for labels
            .set("y", 944)  // Positioning the label slightly above the bottom edge
            .set("text-anchor", "middle")
            .set("font-size", 40)
            .set("fill", "#757575");  // Set text color to white
        doc = doc.add(label_text);
    }

    // Add row labels
    for (idx, label) in row_labels.iter().enumerate() {
        let label_text = Text::new()
            .set("x", 64)  // Positioning the label slightly to the right of the left edge
            .set("y", 100 + idx * 100 + 70)  // Adjusting the y-coordinate to account for labels
            .set("text-anchor", "middle")
            .set("font-size", 40)
            .set("fill", "#757575");  // Set text color to white
        doc = doc.add(label_text);
    }

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let x = 100 + col * 100;  // Adjusting the x-coordinate to account for labels
            let y = 100 + row * 100;  // Adjusting the y-coordinate to account for labels

            let square_color = if (row + col) % 2 == 0 {
                "#ccc"
            } else {
                "#666"
            };

            let square = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", 100)
                .set("height", 100)
                .set("fill", square_color);

            doc = doc.add(square);

            if piece != ' ' {

                // setting from an to color
                if let Some(from_coords) = from {
                    let (row, col) = from_coords;
                    let x = 100 + col * 100;
                    let y = 100 + row * 100;

                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 100)
                        .set("height", 100)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 6);

                    doc = doc.add(highlight);
                }

                if let Some(to_coords) = to {
                    let (row, col) = to_coords;
                    let x = 100 + col * 100;
                    let y = 100 + row * 100;

                    let highlight = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", 100)
                        .set("height", 100)
                        .set("fill", "none") // Transparent fill
                        .set("stroke", "#3189D9")
                        .set("stroke-width", 6);

                    doc = doc.add(highlight);
                }

                // map character to piece name and background
                let (piece_name, background) = match piece {
                    'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
                    _   => panic!("Unknown piece"),
                };

                // Load SVG chess piece based on piece name and background
                let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
                let data_url = load_image_as_data_url(&file_path)
                    .expect("Failed to load image as data URL");

                let piece_image = Image::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", 100)
                    .set("height", 100)
                    .set("href", data_url);

                doc = doc.add(piece_image);
            }
        }
    }

    doc
}


// // Function to generate the SVG chessboard with black orientation
// fn generate_black_oriented_chessboard(
//     chessboard: &[[char; 8]; 8], 
//     from: Option<(usize, usize)>, 
//     to: Option<(usize, usize)>
//     ) -> Document {

//     let mut doc = Document::new()
//         .set("width", "500")  
//         .set("height", "500")  
//         .set("viewBox", (0, 0, 500, 500))
//         .set("style", "background-color: #2f0300;");  // Set background to dark red

//     // Define labels, reversed for black piece orientation
//     let column_labels = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
//     let row_labels = ['1', '2', '3', '4', '5', '6', '7', '8'];

//     // Add column labels
//     for (idx, label) in column_labels.iter().enumerate() {
//         let label_text = Text::new()
//             .set("x", 50 + idx * 50 + 25)  
//             .set("y", 472)  
//             .set("text-anchor", "middle")
//             .set("font-size", 20)
//             .set("fill", "#757575")  // Set text color to dark grey
//             .add(svg::node::Text::new(label.to_string()));
//         doc = doc.add(label_text);
//     }

//     // Add row labels
//     for (idx, label) in row_labels.iter().enumerate() {
//         let label_text = Text::new()
//             .set("x", 32)  
//             .set("y", 50 + idx * 50 + 35)  
//             .set("text-anchor", "middle")
//             .set("font-size", 20)
//             .set("fill", "#757575")  
//             .add(svg::node::Text::new(label.to_string()));
//         doc = doc.add(label_text);
//     }

//     for (row, row_pieces) in chessboard.iter().rev().enumerate() {  // Reverse rows for black piece orientation
//         for (col, &piece) in row_pieces.iter().rev().enumerate() {  // Reverse columns for black piece orientation
//             let x = 50 + col * 50;  
//             let y = 50 + row * 50;  

//             // Set Square Colours
//             let square_color = if (row + col) % 2 == 0 {
//                 "#ccc"
//             } else {
//                 "#666"
//             };
            
//             let square = Rectangle::new()
//                 .set("x", x)
//                 .set("y", y)
//                 .set("width", 50)
//                 .set("height", 50)
//                 .set("fill", square_color);

//             doc = doc.add(square);

//             if piece != ' ' {

//                 if let Some(from_coords) = from {
//                     let (row, col) = from_coords;
//                     let x = 50 + col * 50;
//                     let y = 50 + row * 50;
                
//                     let highlight = Rectangle::new()
//                         .set("x", x)
//                         .set("y", y)
//                         .set("width", 50)
//                         .set("height", 50)
//                         .set("fill", "none") // Transparent fill
//                         .set("stroke", "#3189D9")
//                         .set("stroke-width", 3);
                
//                     doc = doc.add(highlight);
//                 }
                
//                 if let Some(to_coords) = to {
//                     let (row, col) = to_coords;
//                     let x = 50 + col * 50;
//                     let y = 50 + row * 50;
                
//                     let highlight = Rectangle::new()
//                         .set("x", x)
//                         .set("y", y)
//                         .set("width", 50)
//                         .set("height", 50)
//                         .set("fill", "none") // Transparent fill
//                         .set("stroke", "#3189D9")
//                         .set("stroke-width", 3);
                
//                     doc = doc.add(highlight);
//                 }


//                 // map character to piece name and background
//                 let (piece_name, background) = match piece {
//                     'p' => ("black_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'r' => ("black_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'n' => ("black_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'b' => ("black_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'q' => ("black_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'k' => ("black_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'P' => ("white_pawn", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'R' => ("white_rook", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'N' => ("white_night", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'B' => ("white_bishop", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'Q' => ("white_queen", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     'K' => ("white_king", if square_color == "#666" {"darksquare"} else {"lightsquare"}),
//                     _   => panic!("Unknown piece"),
//                 };

//                 // // Load SVG chess piece based on piece name and background
//                 let file_path = format!("pieces_svg/{}_{}.svg", piece_name, background);
//                 let data_url = load_image_as_data_url(&file_path)
//                     .expect("Failed to load image as data URL");

//                 let piece_image = Image::new()
//                     .set("x", x)
//                     .set("y", y)
//                     .set("width", 50)
//                     .set("height", 50)
//                     .set("href", data_url);

//                 doc = doc.add(piece_image);
//             }
//         }
//     }

//     doc
// }


fn load_image_as_data_url(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let encoded = STANDARD.encode(&buffer); // Now works since Engine is in scope
    let mime_type = "image/svg+xml"; // Adjust if not using SVG images

    Ok(format!("data:{};base64,{}", mime_type, encoded))
}
