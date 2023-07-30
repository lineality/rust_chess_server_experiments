fn scramble_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
    // Convert seed into a sequence of numbers
    let numbers: Vec<usize> = seed.chars().map(|c| c as usize).collect();

    // Apply transformations
    for (i, &num) in numbers.iter().enumerate() {
        let num = num % array.len(); // ensure num is within valid range
        if i % 2 == 0 {
            // Apply transformation to rows
            array.rotate_left(num);
        } else {
            // Apply transformation to columns
            for row in array.iter_mut() {
                let num = num % row.len(); // ensure num is within valid range
                row.rotate_left(num);
            }
        }
    }

    array
}

fn unscramble_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
    // Convert seed into a sequence of numbers
    let numbers: Vec<usize> = seed.chars().map(|c| c as usize).collect();

    // Apply transformations
    for (i, &num) in numbers.iter().enumerate() {
        let num = num % array.len(); // ensure num is within valid range
        if i % 2 == 0 {
            // Apply transformation to rows
            array.rotate_right(num);
        } else {
            // Apply transformation to columns
            for row in array.iter_mut() {
                let num = num % row.len(); // ensure num is within valid range
                row.rotate_right(num);
            }
        }
    }

    array
}

fn main() {
    // Your original 2D array (Vec of Vec)
    let array = vec![
        vec!['a', 'b', 'c', 'd'],
        vec!['e', 'f', 'g', 'h'],
        vec!['i', 'j', 'k', 'l'],
    ];

    for row in array.iter() {
        println!("{:?}", row);
    }

    let scrambled = scramble_eggs(array, "hello");

    for row in scrambled.iter() {
        println!("{:?}", row);
    }

    let unscrambled = unscramble_eggs(scrambled, "hello");

    for row in unscrambled.iter() {
        println!("{:?}", row);
    }

}
