
# to do
sandbox folder needs to have a timestamp to avoid collisions,
make two timestamps after waiting a random time between, and use that
as the sandbox name add on.
if a sandbox folder already exists, wait a random number of seconds over 20 seconds and try again
with a new timestamp

add a sandbox_name path to the main maker function

and make another wrapper...


# two layers, two functions
board_layer

gamepiece_layer

The board_layer can be automatically generated with no inputs
because there is no board state.

two function: make white board, make black oriented board
with the legend reversed.

## Goal:
make the game-peiece layer given the game-state input:

show example svg function,



## File structure:
..board_factory]$ ls
board_factory_readme.md  Cargo.toml   legend_alpha_num  src
Cargo.lock               darksquares  lightsquares      target

..legend_alpha_num]$ ls
1.png  3.png  5.png  7.png  a.png  c.png  e.png  g.png
2.png  4.png  6.png  8.png  b.png  d.png  f.png  h.png


## Factors:
- white piece or black piece orientation
- light and dark background squares randomly selected from directories of variation textures (.png files)
- blank top border (to be used later for time)


#### Code Below works for side-join:
```
extern crate image;
use std::path::Path;
use image::{Rgba, ImageBuffer};
```


# working assemble-image functions with these headlines.

fn combine_top_to_bottom<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {


fn combine_side_by_side<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {
  
fn random_image_from_directory(directory: &str) -> Result<String, std::io::Error> {



// working step 3 solution, perhaps template for step 4?
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



// run with
fn main() -> Result<(), std::io::Error> {
    let game_name = "game";
    let white = true;
    generate_chessboard_backboard(game_name, white)?;
    Ok(())
}


fn main() -> Result<(), image::ImageError> {

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

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
    attach_letter_bar(&sandbox_path, orientation_white, &board_image_path)?;


    // Add number bar



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


















# save and show

the imaged should be saved to a directory, just as this function retrieve it:

// Define the show_board_png function to get the PNG content from the file or other sources
fn show_board_png(new_game_name: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(format!("games/{}/board.png", new_game_name))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

























# later



# Game state data
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


### and calls for board state like this:
        // Set up board
        let game_board_state_result = generate_chess960();
        let game_board_state = match game_board_state_result {
            Ok(board) => board,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        };

## svg board generators have this header
    fn generate_black_oriented_chessboard(
        chessboard: &[[char; 8]; 8], 
        from: Option<(usize, usize)>, 
        to: Option<(usize, usize)>
        ) -> Document {











# functions

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

fn main() -> Result<(), image::ImageError> {

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle.png")?;

    overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;

    Ok(())
}







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


//works
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
