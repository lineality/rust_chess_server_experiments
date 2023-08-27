

Let's make a conversion system in Rust and test the output to make sure it is working!

  /*
    Convert type and convert coordinates
    chess-coordinates (column, row) to png pixel (0,0 top left row, colum)
    (char, u8) -> Option<(usize, usize)> 
    */

    // inspect Before conversion
    println!("inspect Before conversion");
    dbg!("from_char_u8 -> ", from_char_u8);
    dbg!("to_char_u8 -> ", to_char_u8);


    // convert number scale... 1-8 to 0-7
    // conver letter to number... a = 1 (or 0)
    // convert chess coordinates to png coordinates... 1,8 *or 0,7 -> 0,0

    // Incomplete Code here!
    let from: Option<(usize, usize)> = ... from_char_u8
    let to: Option<(usize, usize)> = ... to_char_u8


    // inspect After conversion
    println!("inspect After conversion");
    dbg!("from -> ", from);
    dbg!("to -> ", to);

    // manually entering what the results should be: 
    let from = Some((7, 1)); // Replace with your desired values
    let to = Some((5, 2));   // Replace with your desired values

    // inspect After conversion
    println!("should be! -> ");
    dbg!("from -> ", from);
    dbg!("to -> ", to);
