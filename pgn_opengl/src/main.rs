use resvg::*;
use tiny_skia as skia;
use std::fs::File;
use std::io::Read;
use image::png::PngEncoder;
use image::ColorType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read SVG from file
    let mut file = File::open("input.svg")?;
    let mut svg_content = String::new();
    file.read_to_string(&mut svg_content)?;

    // Parse the SVG
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_str(&svg_content, &opt)?;

    // Calculate dimensions
    let size = rtree.svg_node().size.to_screen_size();

    // Create a pixmap
    let mut pixmap = skia::Pixmap::new(size.width(), size.height()).unwrap();

    // Render the SVG to the pixmap
    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut());

    // Convert pixmap into Vec<u8> for image crate
    let buffer: Vec<u8> = pixmap.data().to_vec();

    // Create a PNG file
    let ref mut fout = File::create("output.png")?;
    let encoder = PngEncoder::new(fout);
    encoder.encode(&buffer, size.width() as u32, size.height() as u32, ColorType::Rgba8)?;

    println!("SVG has been successfully converted to PNG!");
    Ok(())
}

