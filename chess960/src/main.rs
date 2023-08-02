/*
chess 960 board heuristic

rules:
- no unwrap
- as few extra packages as possible

1. time-seed a random number generator

2. piece_array

make an array 0-7

all set to none/null

or emtpy strings


3. available_position_array

make another array to hold all available position index numbers


3. bishop_1_number

pick any available random even number for first bishop_1_number

put the bishop_1_number in that piece_array spot 'b'

remove that number from the available_position_array



4. bishop_2_number

pick any available random even number for bishop_2_number

put the bishop_2_number in that piece_array spot 'b'

remove that number from the available_position_array


5. king_number
pick a number for king between 1-6 (cannot be at the end of a row)

put the king in that spot 'k'

remove that number from the available_position_array


6. rook1_number
pick any number < the king_number for the first rook
(in range)

put 'r' in that piece_array spot

remove that number from the available_position_array


7. rook2_number
pick any number > the king_number for the 2nd rook

put 'r' in that piece_array spot

remove that number from the available_position_array


8. two 'n' knight numbers

pick any two available numbers, set those

put 'n' in those piece_array spots

remove those numbers from the available_position_array


9. queen

there should be one remaining available_position_array number

put 'q' in that piece_array spot


10. white and black

set white and black pieces mirroring each-other

in this format:
let chessboard_state: [[char; 8]; 8] = [
['r', 'n', 'b', 'q', ' ', 'b', 'n', 'r'],
['p', 'p', 'p', 'p', 'p', ' ', 'p', 'p'],
[' ', ' ', ' ', ' ', ' ', 'k', ' ', ' '],
[' ', ' ', ' ', ' ', ' ', 'p', ' ', ' '],
[' ', ' ', 'P', ' ', ' ', ' ', ' ', ' '],
[' ', ' ', ' ', ' ', ' ', ' ', 'P', 'N'],
['P', 'P', 'P', 'P', 'P', 'P', ' ', 'P'],
['R', 'N', 'B', 'Q', 'K', 'B', ' ', 'R'],
];

*/

use rand::prelude::*;
use std::convert::TryInto;

fn generate_chess960() -> Result<[[char; 8]; 8], &'static str> {
    let mut rng = rand::thread_rng();
    let mut piece_array = [' '; 8];
    let mut available_position_array: Vec<u8> = (0..8).collect();

    // Pick and remove the first bishop's number, then determine its parity
    let bishop_1_idx = rng.gen_range(0..available_position_array.len());
    let bishop_1_number = available_position_array.remove(bishop_1_idx);
    let bishop_1_parity = bishop_1_number % 2;
    piece_array[bishop_1_number as usize] = 'b';

    // Pick and remove the second bishop's number, ensuring that its parity is the opposite of the first
    let mut bishop_2_number;
    let mut bishop_2_idx;
    loop {
        bishop_2_idx = rng.gen_range(0..available_position_array.len());
        bishop_2_number = available_position_array[bishop_2_idx];
        if bishop_2_number % 2 != bishop_1_parity {
            break;
        }
    }
    available_position_array.remove(bishop_2_idx);
    piece_array[bishop_2_number as usize] = 'b';

    // king_number
    let king_idx = rng.gen_range(1..available_position_array.len() - 1);
    let king_number = available_position_array.remove(king_idx);
    piece_array[king_number as usize] = 'k';

    // rook1_number
    let rook1_number = available_position_array.iter().take_while(|&&x| x < king_number).last().unwrap().clone();
    available_position_array.retain(|&x| x != rook1_number);
    piece_array[rook1_number as usize] = 'r';

    // rook2_number
    let rook2_number = available_position_array.iter().skip_while(|&&x| x <= king_number).next().unwrap().clone();
    available_position_array.retain(|&x| x != rook2_number);
    piece_array[rook2_number as usize] = 'r';

    // knights
    for _ in 0..2 {
        let knight_number = available_position_array.remove(0);
        piece_array[knight_number as usize] = 'n';
    }

    // queen
    let queen_number = available_position_array.remove(0);
    piece_array[queen_number as usize] = 'q';

    let mut chessboard_state: [[char; 8]; 8] = [[' '; 8]; 8];
    chessboard_state[0] = piece_array.iter().map(|&p| p.to_ascii_uppercase()).collect::<Vec<_>>().try_into().unwrap();
    chessboard_state[1] = ['P'; 8];
    chessboard_state[6] = ['p'; 8];
    chessboard_state[7] = piece_array;

    Ok(chessboard_state)
}

fn main() -> Result<(), &'static str> {
    let chessboard = generate_chess960()?;
    for row in chessboard.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    Ok(())
}




