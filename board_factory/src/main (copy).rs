extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer};
use rand::Rng;
use std::{fs, io};
use std::fs::File;


fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    /*
    extern crate image;
    use image::ImageBuffer;
    use std::path::Path;

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "output.png")?;
    Ok(())
     */

    // Load the images
    let image1 = image::open(image_path1)?;
    let image2 = image::open(image_path2)?;

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



fn combine_top_to_bottom<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // Load the images
    let image1 = image::open(image_path1)?;
    let image2 = image::open(image_path2)?;

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




fn overlay_images<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
    // Load the images.
    let mut image1 = image::open(image_path1)?.to_rgba8();
    let image2 = image::open(image_path2)?.to_rgba8();

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
    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 * (1.0 - alpha_top) / 255.0;
    let alpha_combined = alpha_top + alpha_bottom;

    for i in 0..3 {
        bottom[i] = ((top[i] as f32 * alpha_top + bottom[i] as f32 * alpha_bottom) / alpha_combined) as u8;
    }
    bottom[3] = (alpha_combined * 255.0) as u8;
}




fn random_image_from_directory(directory: &str) -> Result<String, std::io::Error> {
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

use std::path::PathBuf;
fn choose_random_image(dir_path: &str) -> Result<PathBuf, io::Error> {
    // Read directory contents
    let paths: Vec<_> = std::fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // Check if there are any paths available
    if paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "No images found in the directory"));
    }

    // Randomly choose a path by generating a random index
    let random_index = rand::random::<usize>() % paths.len();
    let random_path = paths[random_index].clone();

    Ok(random_path)
}



use std::io::Read; 

fn engine_generate_chessboard_backboard(sandbox_path: &str, orientation_white: bool) -> Result<(), io::Error> {
    let mut row_images = Vec::new();

    for row in 0..8 {
        let mut row_image_path = String::new();

        for col in 0..8 {
            let texture_directory = if (row + col) % 2 == 0 {
                if orientation_white {
                    "lightsquares"
                } else {
                    "darksquares"
                }
            } else {
                if orientation_white {
                    "darksquares"
                } else {
                    "lightsquares"
                }
            };

            let random_image_path = random_image_from_directory(texture_directory)?;

            if row_image_path.is_empty() {
                row_image_path = random_image_path;
            } else {
                let output_path = format!("{}/row_{}_col_{}.png", sandbox_path, row, col);
            combine_side_by_side(row_image_path, random_image_path, output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;


    //             let output_path = format!("row_{}_col_{}.png", row, col);
    //             combine_side_by_side(row_image_path, random_image_path, output_path.clone())
    // .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;


                row_image_path = output_path;
            }
        }

        row_images.push(row_image_path);
    }

    let mut final_board_image_path = row_images[0].clone();

    for i in 1..8 {
        let output_path = format!("{}/final_row_{}.png", sandbox_path, i);
        combine_top_to_bottom(final_board_image_path, row_images[i].clone(), output_path.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    
        final_board_image_path = output_path;
    
    

    // let mut final_board_image_path = row_images[0].clone();

    // for i in 1..8 {
    //     let output_path = format!("final_row_{}.png", i);
    //     combine_top_to_bottom(final_board_image_path, row_images[i].clone(), output_path.clone())
    // .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;


    //     final_board_image_path = output_path;
    }

    // // Create the game directory if it does not exist
    // let game_directory = format!("games/{}", gamename);
    // fs::create_dir_all(&game_directory)?;

    // // Move the final image to the game directory
    // let final_path = format!("{}/board.png", game_directory);
    // fs::rename(final_board_image_path, final_path)?;

    // Move the final image inside the sandbox
    let final_path_in_sandbox = format!("{}/back_board.png", sandbox_path);
    fs::rename(final_board_image_path, final_path_in_sandbox)?;

    Ok(())
}




// Define the show_board_png function to get the PNG content from the file or other sources
fn show_board_png(new_game_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(format!("games/{}/board.png", new_game_name))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}


// fn generate_chessboard_backboard(game_name: &str, orientation_white: bool) -> Result<(), std::io::Error> {
//     let sandbox_path = format!("games/{}/board_sandbox", game_name);

//     // Create the temporary directory
//     fs::create_dir_all(&sandbox_path)?;

//     // Generate the chessboard
//     let result = engine_generate_chessboard_backboard(&sandbox_path, orientation_white);

//     // Clean up by deleting the temporary directory
//     fs::remove_dir_all(&sandbox_path)?;

//     result
// }


use std::io::Error;
use std::io::ErrorKind;

fn generate_chessboard_backboard(game_name: &str, orientation_white: bool) -> Result<(), Error> {
    let sandbox_path: String = format!("games/{}/sandbox", game_name);
    let final_image_path: String = format!("games/{}", game_name);

    // Check if the sandbox already exists
    if fs::metadata(&sandbox_path).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Sandbox already exists; please try again later.",
        ));
    }

    // Create the temporary directory
    fs::create_dir_all(&sandbox_path)?;

    // Generate the chessboard
    let result = engine_generate_chessboard_backboard(&sandbox_path, orientation_white);

    // Assuming the final image is created inside sandbox as final_image.png
    // then moved to the game folder
    let final_image_source = format!("{}/back_board.png", sandbox_path);
    let final_image_destination = format!("{}/back_board.png", final_image_path);

    // Move the final image to the desired location
    if result.is_ok() {
        fs::rename(final_image_source, final_image_destination)?;
    }

    // // Clean up by deleting the temporary directory
    // fs::remove_dir_all(&sandbox_path)?;

    // Clean up by deleting the temporary directory
    let _ = fs::remove_dir_all(&sandbox_path);

    result
}


// // Creates a blank row
// fn create_blank_row() -> Result<String, io::Error> {
//     let blank_image_path = "legend_alpha_num/blank.png"; // Define the path to your blank image
//     let blank_row_paths = vec![blank_image_path; 9]; // 9 blank images to create the row
//     combine_side_by_side(blank_row_paths)
// }

// // Creates a letter row based on orientation
// fn create_letter_row_image(orientation_white: bool) -> Result<String, io::Error> {
//     let letters = if orientation_white {
//         vec!["a", "b", "c", "d", "e", "f", "g", "h"]
//     } else {
//         vec!["h", "g", "f", "e", "d", "c", "b", "a"]
//     };

//     let blank_image_path = "legend_alpha_num/blank.png"; // Define the path to your blank image
//     let mut paths = vec![blank_image_path];
//     for letter in letters {
//         paths.push(format!("legend_alpha_num/{}.png", letter)); // Path to each letter image
//     }
//     paths.push(blank_image_path);

//     combine_side_by_side(paths)
// }

// // Creates a number row image based on the given row and orientation
// fn create_number_row_image(row: usize, orientation_white: bool) -> Result<String, io::Error> {
//     let number = if orientation_white {
//         row + 1
//     } else {
//         8 - row
//     };

//     let blank_image_path = "legend_alpha_num/blank.png"; // Define the path to your blank image
//     let number_image_path = format!("legend_alpha_num/{}.png", number);

//     let paths = vec![blank_image_path, number_image_path];
//     combine_side_by_side(paths)
// }

fn main() -> Result<(), std::io::Error> {
    let game_name = "game";
    let white = true;

    generate_chessboard_backboard(game_name, white)?;


    // Addit
    // combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    // overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    // overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;



    Ok(())
}
