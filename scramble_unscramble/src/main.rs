fn scramble_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
    // Convert seed into a sequence of operations
    let operations: Vec<(usize, bool)> = seed.chars().map(|c| ((c as usize) % array.len(), c.is_ascii_uppercase())).collect();

    // Apply operations
    for (i, &(index, direction)) in operations.iter().enumerate() {
        if i % 2 == 0 {
            // Apply operation to rows
            if direction {
                array[index].rotate_left(1);
            } else {
                array[index].rotate_right(1);
            }
        } else {
            // Apply operation to columns
            let mut column: Vec<_> = array.iter().map(|row| row[index]).collect();
            if direction {
                column.rotate_left(1);
            } else {
                column.rotate_right(1);
            }
            for (row, &value) in array.iter_mut().zip(column.iter()) {
                row[index] = value;
            }
        }
    }

    array
}

fn unscramble_eggs(mut array: Vec<Vec<char>>, seed: &str) -> Vec<Vec<char>> {
    // Convert seed into a sequence of operations
    let operations: Vec<(usize, bool)> = seed.chars().map(|c| ((c as usize) % array.len(), c.is_ascii_uppercase())).collect();

    // Apply operations in reverse
    for (i, &(index, direction)) in operations.iter().enumerate().rev() {
        if i % 2 == 0 {
            // Apply operation to rows
            if direction {
                array[index].rotate_right(1);
            } else {
                array[index].rotate_left(1);
            }
        } else {
            // Apply operation to columns
            let mut column: Vec<_> = array.iter().map(|row| row[index]).collect();
            if direction {
                column.rotate_right(1);
            } else {
                column.rotate_left(1);
            }
            for (row, &value) in array.iter_mut().zip(column.iter()) {
                row[index] = value;
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
