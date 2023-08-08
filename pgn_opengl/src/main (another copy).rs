use image::ImageBuffer;
use resvg::prelude::*;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let svg_data = include_str!("board.svg");
    let tree = usvg::Tree::from_str(svg_data, &usvg::Options::default()).unwrap();

    let (width, height) = (1000, 1000);
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();

    resvg::render(&tree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

    // Save the pixmap as a PNG image
    let image = ImageBuffer::from_fn(width, height, |x, y| {
        let p = pixmap.pixel(x as _, y as _);
        image::Rgba([p.r(), p.g(), p.b(), p.a()])
    });
    image.save("output.png").unwrap();

    // Create OpenGL texture
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // ... OpenGL setup code to create texture from pixmap ...

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        // ... Event handling code ...
    });
}
