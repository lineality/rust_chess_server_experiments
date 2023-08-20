extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer};
use rand::Rng;
use std::{fs, io};
use std::fs::File;

use std::io::Error;
use std::io::ErrorKind;
use std::fmt::Debug;

// fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
fn combine_side_by_side<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {

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
    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path2))))?;
    


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

    // Load the images
    let image1 = image::open(&image_path1).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path1))))?;
    let image2 = image::open(&image_path2).map_err(|_| image::ImageError::from(io::Error::new(io::ErrorKind::Other, format!("Failed to load image at {:?}", &image_path2))))?;
    

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

fn make_board_core(sandbox_path: &str, orientation_white: bool) -> Result<(), io::Error> {
    let mut row_images = Vec::new();

    // Make the 8 Rows
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
    let sandbox_path = format!("{}/back_board.png", sandbox_path);
    fs::rename(final_board_image_path, sandbox_path)?;

    Ok(())
}






fn make_and_attach_letter_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // Determine the order of letters
    let letters_order = if orientation_white {
        ["a.png", "b.png", "c.png", "d.png", "e.png", "f.png", "g.png", "h.png"]
    } else {
        ["h.png", "g.png", "f.png", "e.png", "d.png", "c.png", "b.png", "a.png"]
    };

    // Create the letter bar by combining individual letters
    let mut letter_bar_path = String::new();
    for letter in &letters_order {
        let path = format!("legend_alpha_num/{}", letter);
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
    let final_image_with_letters_path = format!("{}/back_board.png", sandbox_path);
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




// fn make_and_attach_number_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
//     // Determine the order of numbers
//     let numbers_order = if orientation_white {
//         ["8.png", "7.png", "6.png", "5.png", "4.png", "3.png", "2.png", "1.png"]
//     } else {
//         ["1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png"]
//     };

//     // Create a temporary image for the vertical number bar
//     let mut number_bar_path = "legend_alpha_num/blank.png".to_string(); // Start with a blank image

//     for number in &numbers_order {
//         let path = format!("legend_alpha_num/{}", number);
//         let new_output_path = format!("{}/tmp_{}.png", sandbox_path, number); // Prepend sandbox_path
//         combine_top_to_bottom(number_bar_path, path, new_output_path.clone())
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
//         number_bar_path = new_output_path;
//     }

//     // Combine the number bar with the board image
//     let final_image_with_numbers_path = format!("{}/back_board.png", sandbox_path);
//     combine_side_by_side(&number_bar_path, &board_image_path.to_string(), &final_image_with_numbers_path)
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//     // Optional: Clean up temporary files created during the process
//     for number in &numbers_order {
//         let tmp_file_path = format!("{}/tmp_{}.png", sandbox_path, number);
//         if std::path::Path::new(&tmp_file_path).exists() {
//             let _ = std::fs::remove_file(tmp_file_path);
//         }
//     }

//     Ok(())
// }


fn make_and_attach_number_bar(sandbox_path: &str, orientation_white: bool, board_image_path: &str) -> Result<(), io::Error> {
    // Determine the order of numbers
    let numbers_order = if orientation_white {
        ["8.png", "7.png", "6.png", "5.png", "4.png", "3.png", "2.png", "1.png"]
    } else {
        ["1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png"]
    };

    // Create a temporary image for the vertical number bar
    let mut number_bar_path = String::new(); // Start without a path, as we will build it dynamically

    for number in &numbers_order {
        let path = format!("legend_alpha_num/{}", number);
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
    let blank_image_path: String = "legend_alpha_num/blank.png".to_string();
    let new_output_path = format!("{}/tmp_blank.png", sandbox_path);
    combine_top_to_bottom(number_bar_path, blank_image_path, new_output_path.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    number_bar_path = new_output_path.clone();

    // Combine the number bar with the board image
    let final_image_with_numbers_path = format!("{}/back_board.png", sandbox_path);
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



fn generate_chessboard_backboard_wrapper(game_name: &str, orientation_white: bool) -> Result<(), Error> {
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
    let result = make_board_core(&sandbox_path, orientation_white);
    
    // Add letter bar
    let board_image_path = format!("{}/back_board.png", sandbox_path);
    make_and_attach_letter_bar(&sandbox_path, orientation_white, &board_image_path)?;


    // Add number bar
    make_and_attach_number_bar(&sandbox_path, orientation_white, &board_image_path)?;



    // Assuming the final image is first created inside sandbox as final_image.png
    // then moved to the game folder
    let final_image_source = format!("{}/back_board.png", sandbox_path);
    let final_image_destination = format!("{}/back_board.png", final_image_path);

    // Move the final image to the desired location
    if result.is_ok() {
        fs::rename(final_image_source, final_image_destination)?;
    }

    // Clean up by deleting the temporary directory
    let _ = fs::remove_dir_all(&sandbox_path);

    result
}


// use image::{GenericImageView, ImageError};



fn main() -> Result<(), std::io::Error> {
    let game_name = "game";
    let white = true;

    generate_chessboard_backboard_wrapper(game_name, white)?;


    // Addit
    // combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    // overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    // overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;



    Ok(())
}



// Define the show_board_png function to get the PNG content from the file or other sources
fn show_board_png(new_game_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(format!("games/{}/board.png", new_game_name))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

