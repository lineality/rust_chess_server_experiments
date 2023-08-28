use std::fs::File;
use std::io::Write;

fn write_chunk(file: &mut File, type_code: &[u8; 4], data: &[u8]) {
    let length = (data.len() as u32).to_be_bytes();
    let mut crc_data = Vec::new();
    crc_data.extend_from_slice(type_code);
    crc_data.extend_from_slice(data);
    let crc = crc::crc32::checksum_ieee(&crc_data).to_be_bytes();

    file.write_all(&length).unwrap();
    file.write_all(type_code).unwrap();
    file.write_all(data).unwrap();
    file.write_all(&crc).unwrap();
}

fn main() {
    let mut file = File::create("4x4.png").unwrap();

    // PNG signature
    file.write_all(&[137, 80, 78, 71, 13, 10, 26, 10]).unwrap();

    // IHDR chunk
    let ihdr: [u8; 13] = [
        0, 0, 0, 4, // Width: 4
        0, 0, 0, 4, // Height: 4
        8,          // Bit depth
        2,          // Color type: RGB
        0,          // Compression method
        0,          // Filter method
        0,          // Interlace method
    ];
    write_chunk(&mut file, b"IHDR", &ihdr);

    // IDAT chunk: minimal raw image data for a 4x4 black and white image
    // No compression is used for simplicity (not recommended for larger images).
    let idat: [u8; 48] = [
        // Row 1: white, black, white, black
        255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 0,
        // Row 2: black, black, black, black
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // Row 3: black, black, black, black
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // Row 4: black, black, black, black
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    write_chunk(&mut file, b"IDAT", &idat);

    // IEND chunk
    write_chunk(&mut file, b"IEND", &[]);

    println!("4x4.png has been created.");
}
