extern crate svgtypes;
extern crate tiny_skia as skia;
extern crate glow;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;

fn convert_svg_to_png(svg_path: &str, png_path: &str, width: u32, height: u32) {
    // Load SVG file
    let mut svg_content = String::new();
    File::open(svg_path)
        .expect("Failed to open SVG file")
        .read_to_string(&mut svg_content)
        .expect("Failed to read SVG content");

    // Parse SVG and convert to PNG
    let svg_data = svgtypes::PathData::from_svg(&svg_content).expect("Failed to parse SVG");
    let mut pixmap = skia::Pixmap::new(width, height).expect("Failed to create pixmap");
    let mut canvas = skia::Canvas::new(pixmap.canvas());
    canvas.clear(skia::Color::WHITE);
    svg_data.draw(&canvas).expect("Failed to draw SVG");

    // Save pixmap as PNG
    pixmap
        .save_png(png_path)
        .expect("Failed to save PNG image");
}

fn setup_opengl_texture(png_data: &[u8], width: i32, height: i32, gl: &glow::Context) -> glow::Texture {
    // Create OpenGL texture
    let texture = unsafe {
        let texture_id = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture_id));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width,
            height,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(png_data),
        );
        texture_id
    };

    texture
}

fn main() {

    let svg_path = "board.svg";
    let png_path = "board.png";
    let width = 1000; // Set your desired width
    let height = 1000; // Set your desired height

    convert_svg_to_png(svg_path, png_path, width, height);


    let png_path = "board.png";
    let mut png_data = Vec::new();
    File::open(png_path)
        .expect("Failed to open PNG file")
        .read_to_end(&mut png_data)
        .expect("Failed to read PNG content");

    let width = 100; // Replace with actual dimensions
    let height = 100; // Replace with actual dimensions

    // Set up OpenGL
    let gl = glow::Context::from_loader_function(|s| {
        ptr::null()
    });

    // Create OpenGL texture
    let texture = setup_opengl_texture(&png_data, width, height, &gl);

    // ... Additional OpenGL setup and rendering code here ...
}
