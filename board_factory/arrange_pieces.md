
# to do

and make another wrapper...
mkdir p_darksquare p_lightsquare P_darksquare P_lightsquare r_darksquare r_lightsquare R_darksquare R_lightsquare n_darksquare n_lightsquare N_darksquare N_lightsquare b_darksquare b_lightsquare B_darksquare B_lightsquare q_darksquare q_lightsquare Q_darksquare Q_lightsquare k_darksquare k_lightsquare K_darksquare K_lightsquare 


# two layers, two functions
board_layer
gamepiece_layer

The board_layer can be automatically generated with no inputs
because there is no board state.

two function: make white board, make black oriented board
with the legend reversed.

## Goals:
There is already a set of functions to make the back_board for the chess board,
in directory games/{game_name}/back_board.png

functions exist to added png images vertically and horizontally

there is a game-state

Task: 
1. make a function (or set of functions) to create a png image file
that places black and white pieces (selected from directories of variant images,
depending on light or dark squares: so each piece-type has four directies)
2. overlay the chess_pieces.png with the board_back.png, 
and save that file as 
games/{game_name}/board.png



## File structure:
..home_directory]$ ls
 arrange_pieces.md               Cargo.lock   games         junk   target
 Cargo.toml                      image_files  src


image_files/gamepieces]$ ls
p_darksquare
p_lightsquare
P_darksquare
P_lightsquare

r_darksquare
r_lightsquare
R_darksquare
R_lightsquare

n_darksquare
n_lightsquare
N_darksquare
N_lightsquare

b_darksquare
b_lightsquare
B_darksquare
B_lightsquare

q_darksquare
q_lightsquare
Q_darksquare
Q_lightsquare

k_darksquare
k_lightsquare
K_darksquare
K_lightsquare




## Factors:
- white piece or black piece orientation
- light and dark background squares randomly selected from directories of variation textures (.png files)
- bottom and side are blank files
image_files/legend_alpha_num/blank.png



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




these functions are maybe a start but they malfunction: 


fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, io::Error> {
    
    let mut img = ImageBuffer::new(800, 800);
    // fn create_chess_pieces_layer(chessboard: &[[char; 8]; 8]) -> Result<ImageBuffer<Rgba<u8>>, io::Error> {
    //     let mut img = ImageBuffer::new(800, 800); // Assuming 100x100 pixels for each square

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

            // let piece_image_path = format!("image_files/gamepieces/{}{}", piece_prefix, piece_suffix);
            // let piece_image = image::open(Path::new(&piece_image_path))?;


            let piece_image_path = format!("image_files/gamepieces/{}{}", piece_prefix, piece_suffix);
            let piece_image = image::open(Path::new(&piece_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;
            // let piece_image: DynamicImage = piece_image; // Ensure the correct type for piece_image
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

use std::io::ErrorKind;

fn create_chessboard_with_pieces(game_board_state: &[[char; 8]; 8], game_name: &str) -> Result<(), io::Error> {
    let pieces_image = create_chess_pieces_layer(game_board_state)?;
    let pieces_image_path = format!("games/{}/chess_pieces.png", game_name);
    pieces_image.save(Path::new(&pieces_image_path)).map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    let back_board_path = format!("games/{}/back_board.png", game_name);
    let output_path = format!("games/{}/board.png", game_name);

    overlay_images(Path::new(&back_board_path), Path::new(&pieces_image_path), Path::new(&output_path))
        .map_err(|e| io::Error::new(ErrorKind::Other, e)) // Convert the error to io::Error
}

e.g. see this output:
combine_top_to_bottom...
image_path1: "games/game/sandboxes/sandbox_backboard/tmp_1.png.png"
image_path2: "image_files/legend_alpha_num/blank.png"
output_path: "games/game/sandboxes/sandbox_backboard/tmp_blank.png"
combine_side_by_side...
image_path1: "games/game/sandboxes/sandbox_backboard/tmp_blank.png"
image_path2: "games/game/sandboxes/sandbox_backboard/back_board.png"
output_path: "games/game/sandboxes/sandbox_backboard/back_board.png"
Error: Custom { kind: Other, error: IoError(Os { code: 2, kind: NotFound, message: "No such file or directory" }) }



the original idea is to
step 1: make a png set of pieces
step 2: overlay that onto the back_board

but the above code appears to try to put each pieces over the backboard in one for-loop,
and the error says a temp is being merged side by side with the backboard which makes no sense.








