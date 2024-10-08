use hieroglyph::{DrawingContext, Segments};
use image::{Pixel, Rgba, RgbaImage};

fn main() {
    // let name = "Jäne 🅱oe Bib 🦆 🇩🇪 🗕 🗗 🗙 𓁹‿𓁹";
    let name = "Jäne Bib 🗕 🗗 🗙 𓁹‿𓁹 a̐éö̲";
    let segments = Segments::new(name);
    let mut ctx = DrawingContext::new();
    ctx.rgb(255, 255, 255);
    ctx.font_size(50.0);

    let mut canvas = RgbaImage::new(2000, 500);
    ctx.draw(&segments, |(x, y), rgba| {
        let color = Rgba(rgba);
        let Ok(x) = (x + 50).try_into() else { return };
        let Ok(y) = (y + 50).try_into() else { return };
        let Some(pixel) = canvas.get_pixel_checked(x, y) else {
            return;
        };
        let mut pixel = pixel.clone();
        pixel.blend(&color);
        canvas.put_pixel(x, y, pixel);
    });

    canvas.save("img.png").unwrap();
}
