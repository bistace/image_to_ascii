use anyhow::{Context, Result};
use image::{imageops::resize, io::Reader, DynamicImage};

// Characters ordered from low-intensity to high-intensity
// Credit: https://paulbourke.net/dataformats/asciiart/
const CHARS: &[u8] = " .:-=+*#%@".as_bytes();

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let image_input = args
        .next()
        .context("Please enter the path to an image file as the first argument")?;
    let new_width = args
        .next()
        .context("Please enter the new image width in characters as the second argument")?
        .parse::<u32>()?;

    let reader = Reader::open(image_input)?.decode()?;
    let aspect_ratio = reader.width() as f32 / reader.height() as f32;

    let new_height = new_width as f32 * aspect_ratio;
    let resized = resize(
        &reader,
        new_width,
        new_height as u32,
        image::imageops::FilterType::Lanczos3,
    );

    let gray_scale = DynamicImage::ImageRgba8(resized).to_luma8();

    // Ascii table should contain as much characters as there are pixels blus
    // a new line character for each pixel line in the image
    let mut ascii = Vec::with_capacity(
        (gray_scale.width() * gray_scale.height() + gray_scale.height()) as usize,
    );
    for (index, px) in gray_scale.iter().enumerate() {
        let index_in_chars: usize = (*px as f32 / 255_f32 * CHARS.len() as f32) as usize;
        ascii.push(CHARS[index_in_chars]);
        if index % new_width as usize == 0 {
            ascii.push(b'\n');
        }
    }

    let ascii_str = std::str::from_utf8(&ascii)?.to_string();
    println!("{}", ascii_str);

    Ok(())
}
