use hieroglyph::{DrawingContext, Segments};
use image::{Pixel, Rgba, RgbaImage};

fn main() {
    let name = "🅱 Jäne H🅱oe Bib 🦆 🇩🇪 🗕 🗗 🗙 𓁹‿𓁹 ♛";
    let segments = Segments::new(name);
    let mut ctx = DrawingContext::new();
    ctx.rgb(255, 255, 255);
    ctx.font_size(50.0);

    let mut canvas = RgbaImage::new(ctx.width(&segments), 55);
    ctx.draw(&segments, |(x, y), rgba| {
        let color = Rgba(rgba);
        let Ok(x) = x.try_into() else { return };
        let Ok(y) = y.try_into() else { return };
        let Some(pixel) = canvas.get_pixel_checked(x, y) else {
            return;
        };
        let mut pixel = pixel.clone();
        pixel.blend(&color);
        canvas.put_pixel(x, y, pixel);
    });

    canvas.save("examples/example.png").unwrap();
}
