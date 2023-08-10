extern crate image;
extern crate rand;
extern crate glob;

use image::RgbaImage;
use std::path::Path;
use rand::seq::SliceRandom;
use glob::glob;

fn build_chess_board<P: AsRef<Path>>(output_path: P) -> Result<(), image::ImageError> {
    // Collect paths for light and dark square textures
    let light_squares: Vec<_> = glob("./light_textures/*.png")?.filter_map(Result::ok).collect();
    let dark_squares: Vec<_> = glob("./dark_textures/*.png")?.filter_map(Result::ok).collect();

    // Load an example image to get dimensions
    let example_image = image::open(&light_squares[0])?.to_rgba8();
    let square_size = example_image.dimensions();

    // Create a blank 9x9 grid with space for labels
    let mut board_image = RgbaImage::new(square_size.0 * 9, square_size.1 * 9);

    // Random number generator
    let mut rng = rand::thread_rng();

    // Iterate through the 8x8 grid
    for y in 0..8 {
        for x in 0..8 {
            let is_light_square = (x + y) % 2 == 0;
            let texture_paths = if is_light_square { &light_squares } else { &dark_squares };

            // Choose a random texture
            let texture_path = texture_paths.choose(&mut rng).unwrap();

            // Load the texture
            let texture = image::open(texture_path)?.to_rgba8();

            // Paste the texture onto the board
            let x_pos = (x + 1) * square_size.0; // Add offset for label
            let y_pos = (y + 1) * square_size.1; // Add offset for label
            image::imageops::replace(&mut board_image, &texture, x_pos, y_pos);
        }
    }

    // TODO: Add code to place the 1-8 and A-H labels at the borders

    // Save the result
    board_image.save(output_path)?;

    Ok(())
}


extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer};


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

/*
fn main() -> Result<(), image::ImageError> {

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;

    Ok(())
}
*/
