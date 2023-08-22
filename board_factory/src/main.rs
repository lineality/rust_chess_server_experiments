extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer, GenericImageView};
use rand::Rng;
use std::{fs, io};
use std::io::{ErrorKind, Error};
use std::fmt::Debug;


// fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
fn combine_side_by_side<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // println!(
    //     "\ncombine_side_by_side...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
    //     &image_path1,
    //     &image_path2,
    //     &output_path
    // );

    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_side_by_side Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_side_by_side Failed to load image at {:?}", &image_path2))))?;
    


    /*
    extern crate image;
    use image::ImageBuffer;
    use std::path::Path;

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "output.png")?;
    Ok(())
     */

    // Load the images
    // let image1 = image::open(image_path1)?;
    // let image2 = image::open(image_path2)?;


    // Check the height of the images and make them the same if necessary, or handle differently as needed.
    let height = std::cmp::max(image1.height(), image2.height());

    // Create a new image with the combined width of both images and the maximum height
    let mut combined_image = ImageBuffer::new(image1.width() + image2.width(), height);

    // Copy pixels from image1 into the new image
    for (x, y, pixel) in image1.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y, *pixel);
    }

    // Copy pixels from image2 into the new image, offsetting by the width of image1
    for (x, y, pixel) in image2.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x + image1.width(), y, *pixel);
    }

    // Save the new image
    combined_image.save(output_path)?;

    Ok(())
}



fn combine_top_to_bottom<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // println!(
    //     "\ncombine_top_to_bottom...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
    //     &image_path1,
    //     &image_path2,
    //     &output_path
    // );
    // Load the images
    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_top_to_bottom Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("combine_top_to_bottom Failed to load image at {:?}", &image_path2))))?;
    

    // Check the width of the images and make them the same if necessary, or handle differently as needed.
    let width = std::cmp::max(image1.width(), image2.width());

    // Create a new image with the combined height of both images and the maximum width
    let mut combined_image = ImageBuffer::new(width, image1.height() + image2.height());

    // Copy pixels from image1 into the new image
    for (x, y, pixel) in image1.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y, *pixel);
    }

    // Copy pixels from image2 into the new image, offsetting by the height of image1
    for (x, y, pixel) in image2.to_rgba8().enumerate_pixels() {
        combined_image.put_pixel(x, y + image1.height(), *pixel);
    }

    // Save the new image
    combined_image.save(output_path)?;

    Ok(())
}



fn overlay_images<P: AsRef<Path> + std::fmt::Debug>(
    image_path1: P,
    image_path2: P,
    output_path: P,
) -> Result<(), image::ImageError> {
    println!(
        "\nOverlaying images...\nimage_path1: {:?}\nimage_path2: {:?}\noutput_path: {:?}",
        &image_path1,
        &image_path2,
        &output_path
    );

    // Load the images with custom error handling
    let mut image1 = image::open(&image_path1)
        .map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path1))))?
        .to_rgba8();
    let image2 = image::open(&image_path2)
        .map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path2))))?
        .to_rgba8();


    // Ensure the images are the same size, or handle differently as needed.
    assert_eq!(image1.dimensions(), image2.dimensions());

    // Iterate over the coordinates and pixels of image2.
    for (x, y, pixel) in image2.enumerate_pixels() {
        let pixel1 = image1.get_pixel_mut(x, y);
        // Perform blending here. You could write your own blending logic or use a pre-existing function.
        blend_pixels(pixel1, *pixel);
    }

    // Save the result.
    image1.save(output_path)?;

    Ok(())
}


// A simple alpha blending function. You might want to use a more sophisticated blending function depending on your needs.
fn blend_pixels(bottom: &mut Rgba<u8>, top: Rgba<u8>) {
    // println!(
    //     "\nblend_pixels...\nbottom: {:?}\ntop: {:?}",
    //     bottom,
    //     top,
    // );

    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 * (1.0 - alpha_top) / 255.0;
    let alpha_combined = alpha_top + alpha_bottom;

    for i in 0..3 {
        bottom[i] = ((top[i] as f32 * alpha_top + bottom[i] as f32 * alpha_bottom) / alpha_combined) as u8;
    }
    bottom[3] = (alpha_combined * 255.0) as u8;
}


fn random_image_from_directory(directory: &str) -> Result<String, std::io::Error> {
    // println!(
    //     "\nrandom_image_from_directory...\ndirectory: {:?}",
    //     &directory,
    // );
    let paths: Vec<_> = fs::read_dir(directory)?
        .filter_map(Result::ok) // Only keep the Ok values
        .collect();

    if paths.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No images found"));
    }

    let random_entry = &paths[rand::thread_rng().gen_range(0..paths.len())];
    let random_file = random_entry.file_name();
    let file_path_str = random_file.to_str().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Invalid file name"))?;
    let file_path = format!("{}/{}", directory, file_path_str);

    Ok(file_path)
}


fn make_board_core(sandbox_path: &str, orientation_white: bool) -> Result<(), io::Error> {
    // println!("\nmake_board_core()...");
    let mut row_images = Vec::new();

    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Make the 8 Rows
    for row in 0..8 {
        let mut row_image_path = String::new();

        for col in 0..8 {
            let texture_directory = if (row + col) % 2 == 0 {
                if orientation_white {
                    "image_files/lightsquares"
                } else {
                    "image_files/darksquares"
                }
            } else {
                if orientation_white {
                    "image_files/darksquares"
                } else {
                    "image_files/lightsquares"
                }
            };

            let random_image_path = random_image_from_directory(texture_directory)?;

            if row_image_path.is_empty() {
                row_image_path = random_image_path;
            } else {
                let output_path = format!("{}/row_{}_col_{}.png", sandbox_path, row, col);
            combine_side_by_side(row_image_path, random_image_path, output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

                row_image_path = output_path;
            }
        }

        row_images.push(row_image_path);
    }

    // connect the 8 rows
    let mut final_board_image_path = row_images[0].clone();
    for i in 1..8 {
        let output_path = format!("{}/final_row_{}.png", sandbox_path, i);
        combine_top_to_bottom(final_board_image_path, row_images[i].clone(), output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    
        final_board_image_path = output_path;
    }


    // Move (create?) the final image inside the sandbox
    let sandbox_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    fs::rename(final_board_image_path, sandbox_path)?;

    Ok(())
}


fn make_and_attach_letter_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // println!(
    //     "make_and_attach_letter_bar images...\nsandbox_path: {:?}\norientation_white: {:?}\nboard_image_path: {:?}",
    //     &sandbox_path,
    //     &orientation_white,
    //     &board_image_path
    // );

    // ONLY HERE
    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Determine the order of letters
    let letters_order = if orientation_white {
        ["a.png", "b.png", "c.png", "d.png", "e.png", "f.png", "g.png", "h.png"]
    } else {
        ["h.png", "g.png", "f.png", "e.png", "d.png", "c.png", "b.png", "a.png"]
    };

    // Create the letter bar by combining individual letters
    let mut letter_bar_path = String::new();
    for letter in &letters_order {
        let path = format!("image_files/legend_alpha_num/{}", letter);
        if letter_bar_path.is_empty() {
            letter_bar_path = path;
        } else {
            let new_output_path = format!("{}/tmp_{}.png", sandbox_path, letter); // Prepend sandbox_path
            combine_side_by_side(letter_bar_path, path, new_output_path.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            letter_bar_path = new_output_path;
        }
    }


    // Combine the letter bar with the board image
    let final_image_with_letters_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    combine_top_to_bottom(board_image_path, &letter_bar_path, &final_image_with_letters_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // Optional: Clean up temporary files created during the process
    for letter in &letters_order {
        let tmp_file_path = format!("{}/tmp_{}.png", sandbox_path, letter);
        if std::path::Path::new(&tmp_file_path).exists() {
            let _ = std::fs::remove_file(tmp_file_path);
        }
    }

    Ok(())
}


fn make_and_attach_number_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // println!(
    //     "make_and_attach_number_bar images...\nsandbox_path: {:?}\norientation_white: {:?}\nboard_image_path: {:?}",
    //     &sandbox_path,
    //     &orientation_white,
    //     &board_image_path
    // );

    // ONLY HERE
    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    // Determine the order of numbers
    let numbers_order = if orientation_white {
        ["8.png", "7.png", "6.png", "5.png", "4.png", "3.png", "2.png", "1.png"]
    } else {
        ["1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png"]
    };

    // Create a temporary image for the vertical number bar
    let mut number_bar_path = String::new(); // Start without a path, as we will build it dynamically

    for number in &numbers_order {
        let path = format!("image_files/legend_alpha_num/{}", number);
        if number_bar_path.is_empty() {
            number_bar_path = path;
        } else {
            let new_output_path = format!("{}/tmp_{}.png", sandbox_path, number);
            combine_top_to_bottom(number_bar_path, path, new_output_path.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            number_bar_path = new_output_path;
        }
    }

    // Now combine the final number bar with a blank image at the bottom
    let blank_image_path: String = "image_files/legend_alpha_num/blank.png".to_string();
    let new_output_path = format!("{}/tmp_blank.png", sandbox_path);
    combine_top_to_bottom(number_bar_path, blank_image_path, new_output_path.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    number_bar_path = new_output_path.clone();

    // Combine the number bar with the board image
    let final_image_with_numbers_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    combine_side_by_side(&number_bar_path, &board_image_path.to_string(), &final_image_with_numbers_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // Optional: Clean up temporary files created during the process
    for number in &numbers_order {
        let tmp_file_path = format!("{}/tmp_{}.png", sandbox_path, number);
        if std::path::Path::new(&tmp_file_path).exists() {
            let _ = std::fs::remove_file(tmp_file_path);
        }
    }
    // Remove the temporary blank file
    if std::path::Path::new(&new_output_path).exists() {
        let _ = std::fs::remove_file(new_output_path);
    }

    Ok(())
}


fn generate_chessboard_backboards(game_name: &str, orientation_white: bool) -> Result<(), Error> {
    // println!(
    //     "generate_chessboard_backboards images...\ngame_name: {:?}\norientation_white: {:?}",
    //     &game_name,
    //     &orientation_white,
    // );

    let orientation_string: String = if orientation_white {
            "white".to_string()
        } else {
            "black".to_string()
        };

    let sandbox_path: String = format!("games/{}/sandboxes/sandbox_backboard", game_name);

    // Check if the sandbox_backboard folder exists
    if fs::metadata(&sandbox_path).is_ok() {
        // If it exists, delete it
        fs::remove_dir_all(&sandbox_path)?;
    }

    // Create the new directory
    fs::create_dir_all(&sandbox_path)?;

    // check if sandbox_backboard exists, if so, delete it

    let final_image_path: String = format!("games/{}", game_name);


    // Create the temporary directory
    fs::create_dir_all(&sandbox_path)?;

    // Generate the chessboard
    let result = make_board_core(&sandbox_path, orientation_white);
    
    // Add letter bar
    let board_image_path = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    make_and_attach_letter_bar(&sandbox_path, orientation_white, &board_image_path)?;

    // Add number bar
    make_and_attach_number_bar(&sandbox_path, orientation_white, &board_image_path)?;

    // Assuming the final image is first created inside sandbox as final_image.png
    // then moved to the game folder
    let final_image_source = format!("{}/back_board_{}.png", sandbox_path, orientation_string);
    let final_image_destination = format!("{}/back_board_{}.png", final_image_path, orientation_string);

    // Move the final image to the desired location
    if result.is_ok() {
        fs::rename(final_image_source, final_image_destination)?;
    }

    // Clean up by deleting the temporary directory
    let _ = fs::remove_dir_all(&sandbox_path);

    result
}



// fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8], white_orientation: bool) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
//     let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels

//     for (row, row_pieces) in chessboard.iter().enumerate() {
//         for (col, &piece) in row_pieces.iter().enumerate() {
//             let (actual_row, actual_col) = if white_orientation {
//                 (row, col)
//             } else {
//                 (7 - row, 7 - col)
//             };
            
//             let square_color = if (actual_row + actual_col) % 2 != 0 { "darksquare" } else { "lightsquare" };
//             let (piece_prefix, piece_suffix) = match piece {

//                 'p' => ("p_", square_color),
//                 'r' => ("r_", square_color),
//                 'n' => ("n_", square_color),
//                 'b' => ("b_", square_color),
//                 'q' => ("q_", square_color),
//                 'k' => ("k_", square_color),
//                 'P' => ("P_", square_color),
//                 'R' => ("R_", square_color),
//                 'N' => ("N_", square_color),
//                 'B' => ("B_", square_color),
//                 'Q' => ("Q_", square_color),
//                 'K' => ("K_", square_color),
//                 _ => continue,
            
//             };

//             let texture_directory = format!("image_files/chess_pieces/{}{}", piece_prefix, piece_suffix);
//             let piece_image_path = random_image_from_directory(&texture_directory)?;
//             let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//             let (x, y) = (col * 75, row * 75);

//             // Overlay the piece image on the correct square
//             for (i, j, pixel) in piece_image.pixels() {
//                 let (x_new, y_new) = (x + i as usize, y + j as usize);
//                 img.put_pixel(x_new as u32, y_new as u32, pixel);
//             }
//         }
//     }

//     Ok(img)
// }


fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8], white_orientation: bool) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
    let mut img = ImageBuffer::new(600, 600); // 8x8 squares at 75 pixels


    if white_orientation {
        for (row, row_pieces) in chessboard.iter().enumerate() {
            for (col, &piece) in row_pieces.iter().enumerate() {
                // Process the piece, (row, col) will be the actual coordinates for white orientation
                let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
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


    } else {
        for (row, row_pieces) in chessboard.iter().rev().enumerate() {
            for (col, &piece) in row_pieces.iter().rev().enumerate() {
                // Process the piece, (7 - row, 7 - col) will be the actual coordinates for black orientation
                let square_color = if (row + col) % 2 != 0 { "darksquare" } else { "lightsquare" };
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
    }

    Ok(img)
}


fn create_chessboard_with_pieces(
    game_board_state: &[[char; 8]; 8],
    game_name: &str,
    orientation_white: bool,
) -> Result<(), io::Error> {
    println!(
        "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
        &game_board_state
    );

    let orientation_string: String = if orientation_white {
        "white".to_string()
    } else {
        "black".to_string()
    };

    let pieces_image = create_chess_pieces_layer(game_board_state, orientation_white)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image
        .save(Path::new(&pieces_image_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    // Open the bottom and top blank images
    let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
    let side_vertical_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
    // let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
    // let top_blank_image = image::open(Path::new(side_vertical_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    // Combine pieces with bottom blank
    let vertical_combined_path = format!("games/{}/vertical_combined.png", game_name);
    combine_top_to_bottom(pieces_image_path, bottom_blank_path.to_string(), vertical_combined_path.to_string())
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Combine vertical combined with top blank
    let final_pieces_image_path = format!("games/{}/final_pieces.png", game_name);
    combine_side_by_side(side_vertical_blank_path.to_string(), vertical_combined_path.to_string(), final_pieces_image_path.to_string())
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let back_board_path = format!("games/{}/back_board_{}.png", game_name, orientation_string);
    let output_path = format!("games/{}/board.png", game_name);

    // Overlay the backboard with final pieces image
    overlay_images(Path::new(&back_board_path), Path::new(&final_pieces_image_path), Path::new(&output_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
}



fn main() -> Result<(), std::io::Error> {
    let game_name = "game";

    let board_orientation: bool = true; // 
    generate_chessboard_backboards(game_name, board_orientation)?;

    let board_orientation: bool = false; // 
    generate_chessboard_backboards(game_name, board_orientation)?;


    // Set up board
    let board: [[char; 8]; 8] = [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
    ];
    let game_board_state:[[char; 8]; 8] = board;


    let board_orientation: bool = true; // 
    create_chessboard_with_pieces(&game_board_state, game_name, board_orientation)?;

    let board_orientation: bool = false; // 
    create_chessboard_with_pieces(&game_board_state, game_name, board_orientation)?;

    Ok(())
}
