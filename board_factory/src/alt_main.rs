extern crate image;
extern crate rand;

use std::io;
use std::fs;
use std::path::{PathBuf, Path};
use image::{ImageBuffer, GenericImageView, Rgba, DynamicImage};
use rand::Rng;



fn main() -> Result<(), image::ImageError> {
    // let chessboard = generate_chessboard("white", "lightsquares", "darksquares")?;
    // chessboard.save("chessboard.png")?;

    // let light_square_path = Path::new("lightsquare");
    // let dark_square_path = Path::new("darksquare");
    // let numbers_and_letters_path = Path::new("number_and_letters");
    // let output_path = Path::new("chessboard.png");

    // create_chessboard(&light_square_path, &dark_square_path, &numbers_and_letters_path, &output_path)?;

    // println!("Chessboard created successfully!");

    // Test: make sample
    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;

    Ok(())
}

/* 
Helper Functions 
*/




// works
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



// works
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


// fn generate_chessboard(orientation: &str, light_squares_dir: &str, dark_squares_dir: &str) -> Result<DynamicImage, image::ImageError> {
//     // Define the chessboard dimensions
//     let square_size = 64; // Assuming 64 pixels for each square
//     let board_size = 8 * square_size;

//     // Define the chessboard layout
//     let layout = match orientation {
//         "white" => vec!['l', 'd', 'l', 'd', 'l', 'd', 'l', 'd'],
//         _ => vec!['d', 'l', 'd', 'l', 'd', 'l', 'd', 'l'],
//     };

//     // Create an empty image buffer to represent the chessboard
//     let mut chessboard = ImageBuffer::new(board_size, board_size);

//     // Loop through the rows and columns, assigning either a light or dark square
//     for row in 0..8 {
//         for col in 0..8 {
//             let square_type = layout[(row + col) % 2];
//             let square_path = match square_type {
//                 'l' => choose_random_image(light_squares_dir)?,
//                 _ => choose_random_image(dark_squares_dir)?,
//             };
//             let square_image = image::open(square_path)?.resize(square_size, square_size, image::imageops::FilterType::Nearest);
//             image::imageops::overlay(&mut chessboard, &square_image, (col * square_size) as u32, (row * square_size) as u32);
//         }
//     }

//     // Add the legends to the board
//     for i in 0..8 {
//         let number_path = PathBuf::from(format!("legend_alpha_num/{}.png", i + 1));
//         let letter_path = PathBuf::from(format!("legend_alpha_num/{}.png", (b'a' + i) as char));
//         let number_image = image::open(number_path)?;
//         let letter_image = image::open(letter_path)?;

//         // Calculate positions based on orientation
//         let (x_num, y_num, x_letter, y_letter) = match orientation {
//             "white" => (i * square_size, board_size, board_size, i * square_size),
//             _ => (board_size, i * square_size, i * square_size, board_size),
//         };

//         image::imageops::overlay(&mut chessboard, &number_image, x_num as u32, y_num as u32);
//         image::imageops::overlay(&mut chessboard, &letter_image, x_letter as u32, y_letter as u32);
//     }


//     Ok(DynamicImage::ImageRgba8(chessboard))
// }

// fn create_chessboard(
//     light_square_path: &Path,
//     dark_square_path: &Path,
//     numbers_and_letters_path: &Path,
//     output_path: &Path,
// ) -> Result<(), image::ImageError> {
//     let light_files = fs::read_dir(light_square_path).expect("Unable to read light square directory");
//     let dark_files = fs::read_dir(dark_square_path).expect("Unable to read dark square directory");
//     let light_images: Vec<_> = light_files.filter_map(Result::ok).map(|f| f.path()).collect();
//     let dark_images: Vec<_> = dark_files.filter_map(Result::ok).map(|f| f.path()).collect();

//     let square_size = image::open(&light_images[0])?.dimensions();
//     let mut board_image = ImageBuffer::new(8 * square_size.0 + 20, 8 * square_size.1 + 20);

//     let mut rng = rand::thread_rng();

//     // The rest of the code remains the same

//     Ok(())
// }



// untested
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




// works
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
