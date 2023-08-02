use rand::prelude::*;
use std::convert::TryInto;

fn generate_chess960() -> Result<[[char; 8]; 8], &'static str> {
    let mut rng = rand::thread_rng();
    let mut piece_array = [' '; 8];
    let mut available_position_array: Vec<u8> = (0..8).collect();
    let mut available_even_numbers: Vec<u8> = (0..4).map(|x| x * 2).collect();

    // bishop_1_number
    let bishop_1_idx = rng.gen_range(0..available_even_numbers.len());
    let bishop_1_number = available_even_numbers.remove(bishop_1_idx);
    available_position_array.retain(|&x| x != bishop_1_number);
    piece_array[bishop_1_number as usize] = 'b';

    // bishop_2_number
    let bishop_2_idx = rng.gen_range(0..available_even_numbers.len());
    let bishop_2_number = available_even_numbers.remove(bishop_2_idx);
    available_position_array.retain(|&x| x != bishop_2_number);
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

