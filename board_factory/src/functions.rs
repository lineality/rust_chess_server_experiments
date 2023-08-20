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

