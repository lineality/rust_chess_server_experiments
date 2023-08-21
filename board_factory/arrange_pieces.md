

## Goals:
There is already a set of functions to make the back_board for the chess board,
in directory games/{game_name}/back_board.png

9x9 squares, each 75pixcels*75, so 675*675 overall. 
that is an 8x8 plus a legend, so 9x9.

functions exist to added png images vertically and horizontally

a blank 8x1 and 9x1 exist to add 

there is a game-state for input.

there is a function that makes the pieces layer

Task: 

1 this includes making the 8x8 arrangment of pieces 9x9, by adding 
8x_blank_bottom.png, 9x_blank_top.png

2. overlay the chess_pieces.png with the board_back.png, 
and save that file as 

games/{game_name}/board.png



## File structure:
..home_directory]$ ls
 arrange_pieces.md               Cargo.lock   games         junk   target
 Cargo.toml                      image_files  src




# working assemble-image functions with these headlines.

fn overlay_images<P: AsRef<Path>>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {

fn combine_top_to_bottom<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {

fn combine_side_by_side<P: AsRef<Path> + Debug>(image_path1: P, image_path2: P, output_path: P) -> Result<(), image::ImageError> {

  
fn random_image_from_directory(directory: &str) -> Result<String, std::io::Error> {


Would you like to see a working function that makes an .svg image,
though most logic details are different, the overall process and outcome are the same
the header may be similar (the parameters)

    fn generate_white_oriented_chessboard(
        chessboard: &[[char; 8]; 8], 
        from: Option<(usize, usize)>, 
        to: Option<(usize, usize)>
    ) -> Document {




// run with

fn main() -> Result<(), std::io::Error> {
    let game_name = "game";
    let white = true;

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


    generate_chessboard_backboard_wrapper(game_name, white)?;


    create_chessboard_with_pieces(&game_board_state, game_name)?;
 

    Ok(())
}


The task and problem are with this function, which does not correctly
make the 9x9 or overlay it.

the above simple function ALREADY EXIST to do these operations,
e.g.
fn main() -> Result<(), image::ImageError> {

    combine_side_by_side("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle_side_by_side.png")?;

    combine_top_to_bottom("white_pawn_darksquare.png", "white_pawn_lightsquare.png", "rectangle_top_to_bottom.png")?;

    overlay_images("light_wood_square.png", "white_pawn_lightsquare.png", "light_overlay.png")?;

    overlay_images("dark_wood_square.png", "white_pawn_darksquare.png", "dark_overlay.png")?;

    Ok(())
}

these need to be fixed: 

use std::io::ErrorKind;

fn create_chessboard_with_pieces(game_board_state: &[[char; 8]; 8], game_name: &str) -> Result<(), io::Error> {
    println!(
        "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
        &game_board_state,
    );

    let pieces_image = create_chess_pieces_layer(game_board_state)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image.save(Path::new(&pieces_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    let back_board_path = format!("games/{}/back_board.png", game_name);
    let output_path = format!("games/{}/board.png", game_name);

    overlay_images(Path::new(&back_board_path), Path::new(&pieces_image_path), Path::new(&output_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
}



use std::io::ErrorKind;
fn create_chessboard_with_pieces(game_board_state: &[[char; 8]; 8], game_name: &str,orientation_white: bool) -> Result<(), io::Error> {
    println!(
        "\ncreate_chessboard_with_pieces images...\ngame_board_state: {:?}",
        &game_board_state,
    );

    let pieces_image = create_chess_pieces_layer(game_board_state, orientation_white)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image.save(Path::new(&pieces_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    let back_board_path = format!("games/{}/back_board.png", game_name);
    let output_path = format!("games/{}/board.png", game_name);

    overlay_images(Path::new(&back_board_path), Path::new(&pieces_image_path), Path::new(&output_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
}


// fn create_chessboard_with_pieces(chessboard: &[[char; 8]; 8], game_name: &str, white_orientation: bool) -> Result<(), io::Error> {
//     let pieces_image = create_chess_pieces_layer(chessboard, white_orientation)?;

//     let bottom_blank_path = "image_files/legend_alpha_num/8x_blank_bottom.png";
//     let top_blank_path = "image_files/legend_alpha_num/9x_blank_top.png";
//     let bottom_blank_image = image::open(Path::new(bottom_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
//     let top_blank_image = image::open(Path::new(top_blank_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

//     let mut vertical_combined = ImageBuffer::new(675, 675); // 9x9 squares at 75 pixels
//     combine_top_to_bottom(&pieces_image, &bottom_blank_image, &mut vertical_combined)?;
    
//     let mut final_image = ImageBuffer::new(675, 675); // 9x9 squares at 75 pixels
//     combine_side_by_side(&vertical_combined, &top_blank_image, &mut final_image)?;

//     let board_back_path = format!("games/{}/back_board.png", game_name);
//     overlay_images(&board_back_path, &final_image, format!("games/{}/board.png", game_name))?;

//     Ok(())
// }










