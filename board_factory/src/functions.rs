use image::{ImageBuffer, Rgba};
use std::path::Path;
use std::io;

fn create_piece_image(chessboard: &[[char; 8]; 8]) -> Result<ImageBuffer<Rgba<u8>>, io::Error> {
    let mut img = ImageBuffer::new(800, 800); // Assuming 100x100 pixels for each square

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let square_color = if (row + col) % 2 == 0 { "lightsquare" } else { "darksquare" };
            let (piece_prefix, piece_suffix) = match piece {
                'p' => ("p_", square_color),
                'r' => ("r_", square_color),
                'n' => ("n_", square_color),
                'b' => ("b_", square_color),
                'q' => ("q_", square_color),
                'k' => ("k_", square_color),
                'P' => ("P_", square_color),
                'R' => ("R_", square_color),
                'N' => ("N_", square_color),
                'B' => ("B_", square_color),
                'Q' => ("Q_", square_color),
                'K' => ("K_", square_color),
                _ => continue,
            };

            let piece_image_path = format!("image_files/gamepieces/{}{}", piece_prefix, piece_suffix);
            let piece_image = image::open(Path::new(&piece_image_path))?;
            let (x, y) = (col * 100, row * 100);

            // Overlay the piece image on the correct square
            for (i, j, pixel) in piece_image.pixels() {
                let (x_new, y_new) = (x + i as usize, y + j as usize);
                img.put_pixel(x_new as u32, y_new as u32, pixel);
            }
        }
    }

    Ok(img)
}

fn create_chessboard(chessboard: &[[char; 8]; 8], game_name: &str) -> Result<(), io::Error> {
    let pieces_image = create_piece_image(chessboard)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image.save(Path::new(&pieces_image_path))?;

    let back_board_path = format!("games/{}/back_board.png", game_name);
    let output_path = format!("games/{}/board.png", game_name);
    
    overlay_images(Path::new(&back_board_path), Path::new(&pieces_image_path), Path::new(&output_path))
}


use image::RgbaImage;
use std::path::Path;

fn create_chessboard_image(chessboard: &[[char; 8]; 8], game_name: &str) -> Result<(), image::ImageError> {
    let mut board_back = image::open(format!("games/{}/back_board.png", game_name))?;
    let mut chess_pieces = RgbaImage::new(800, 800); // Assuming 8x8 board with 100x100 squares

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let square_type = if (row + col) % 2 == 0 { "lightsquare" } else { "darksquare" };
            let piece_path = match piece {
                'p' => format!("image_files/gamepieces/p_{}.png", square_type),
                'P' => format!("image_files/gamepieces/P_{}.png", square_type),
                'r' => format!("image_files/gamepieces/r_{}.png", square_type),
                'R' => format!("image_files/gamepieces/R_{}.png", square_type),
                // ... Add other pieces here
                ' ' => continue,
                _ => panic!("Unknown piece"),
            };

            let piece_image = image::open(&piece_path)?;
            image::imageops::overlay(&mut chess_pieces, &piece_image, col * 100, row * 100);
        }
    }

    overlay_images(board_back, chess_pieces, format!("games/{}/board.png", game_name))
}

// Existing function
fn overlay_images<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // Your existing implementation
    // ...
}


use image::{ImageBuffer, Rgba};
use std::path::Path;
use std::io;

fn create_piece_image(chessboard: &[[char; 8]; 8]) -> Result<ImageBuffer<Rgba<u8>>, io::Error> {
    let mut img = ImageBuffer::new(800, 800); // Assuming 100x100 pixels for each square

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let square_color = if (row + col) % 2 == 0 { "lightsquare" } else { "darksquare" };
            let (piece_prefix, piece_suffix) = match piece {
                'p' => ("p_", square_color),
                'r' => ("r_", square_color),
                'n' => ("n_", square_color),
                'b' => ("b_", square_color),
                'q' => ("q_", square_color),
                'k' => ("k_", square_color),
                'P' => ("P_", square_color),
                'R' => ("R_", square_color),
                'N' => ("N_", square_color),
                'B' => ("B_", square_color),
                'Q' => ("Q_", square_color),
                'K' => ("K_", square_color),
                _ => continue,
            };

            let piece_image_path = format!("image_files/gamepieces/{}{}", piece_prefix, piece_suffix);
            let piece_image = image::open(Path::new(&piece_image_path))?;
            let (x, y) = (col * 100, row * 100);

            // Overlay the piece image on the correct square
            for (i, j, pixel) in piece_image.pixels() {
                let (x_new, y_new) = (x + i as usize, y + j as usize);
                img.put_pixel(x_new as u32, y_new as u32, pixel);
            }
        }
    }

    Ok(img)
}

fn create_chessboard(chessboard: &[[char; 8]; 8], game_name: &str) -> Result<(), io::Error> {
    let pieces_image = create_piece_image(chessboard)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image.save(Path::new(&pieces_image_path))?;

    let back_board_path = format!("games/{}/back_board.png", game_name);
    let output_path = format!("games/{}/board.png", game_name);
    
    overlay_images(Path::new(&back_board_path), Path::new(&pieces_image_path), Path::new(&output_path))
}
    // Addit
    // combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    // overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    // overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;


    fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
        println!(
            "\ncreate_chess_pieces_layer images...\nchessboard: {:?}",
            &chessboard,
        );
    
        let mut img = ImageBuffer::new(600, 600); // Assuming 75x75 pixels for each square
        for (row, row_pieces) in chessboard.iter().enumerate() {
            for (col, &piece) in row_pieces.iter().enumerate() {
                let square_color = if (row + col) % 2 == 0 { "darksquare" } else { "lightsquare" }; // Fixed logic
                let (piece_prefix, piece_suffix) = match piece {
    
                    'p' => ("p_", square_color),
                    'r' => ("r_", square_color),
                    'n' => ("n_", square_color),
                    'b' => ("b_", square_color),
                    'q' => ("q_", square_color),
                    'k' => ("k_", square_color),
                    'P' => ("P_", square_color),
                    'R' => ("R_", square_color),
                    'N' => ("N_", square_color),
                    'B' => ("B_", square_color),
                    'Q' => ("Q_", square_color),
                    'K' => ("K_", square_color),
                    _ => continue,
                };
    
                // let piece_image_path = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
                // let piece_image = image::open(Path::new(&piece_image_path))?;
    
    
    
                // let piece_image_path = format!("image_files/chess_pieces/{}{}/image.png", piece_prefix, piece_suffix); // Added a slash and image filename
    
                let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
                let piece_image_path = random_image_from_directory(&texture_directory)?;
    
                println!("Piece: {}", piece);
                println!("piece_prefix: {}", piece_prefix);
                println!("piece_suffix: {}", piece_suffix);
                println!("piece_image_path: {}", piece_image_path);
    
                let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
                let piece_image_path = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
                let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
             
                let piece_image: DynamicImage = piece_image; // Ensure the correct type for piece_image
                let (x, y) = (col * 100, row * 100);
    
                // Overlay the piece image on the correct square
                for (i, j, pixel) in piece_image.pixels() {
                    let (x_new, y_new) = (x + i as usize, y + j as usize);
                    img.put_pixel(x_new as u32, y_new as u32, pixel);
                }
            }
        }
    
    
        //  append the blank rows and columns
        let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
        let top_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
    
        let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let top_blank_image = image::open(Path::new(top_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
                
    
    
    
        // let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    
        // Concatenate the images
        let mut final_image = ImageBuffer::new(675, 675); // 9x9 squares at 75 pixels
        image::imageops::overlay(&mut final_image, &img, 0, 0);
        image::imageops::overlay(&mut final_image, &bottom_blank_image, 0, 600); // Append to the bottom
        image::imageops::overlay(&mut final_image, &top_blank_image, 600, 0); // Append to the right side
    
        Ok(final_image)
    
    }
    
    



fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8], white_orientation: bool) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
    let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels

    for (row, row_pieces) in chessboard.iter().enumerate() {
        for (col, &piece) in row_pieces.iter().enumerate() {
            let (actual_row, actual_col) = if white_orientation {
                (row, col)
            } else {
                (7 - row, 7 - col)
            };
            
            let square_color = if (actual_row + actual_col) % 2 == 0 { "darksquare" } else { "lightsquare" };
            let (piece_prefix, piece_suffix) = match piece {

                'p' => ("p_", square_color),
                'r' => ("r_", square_color),
                'n' => ("n_", square_color),
                'b' => ("b_", square_color),
                'q' => ("q_", square_color),
                'k' => ("k_", square_color),
                'P' => ("P_", square_color),
                'R' => ("R_", square_color),
                'N' => ("N_", square_color),
                'B' => ("B_", square_color),
                'Q' => ("Q_", square_color),
                'K' => ("K_", square_color),
                _ => continue,
            
            };

            let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
            let piece_image_path = random_image_from_directory(&texture_directory)?;
            let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

            let (x, y) = (col * 75, row * 75);

            // Overlay the piece image on the correct square
            for (i, j, pixel) in piece_image.pixels() {
                let (x_new, y_new) = (x + i as usize, y + j as usize);
                img.put_pixel(x_new as u32, y_new as u32, pixel);
            }
        }
    }

    Ok(img)
}

