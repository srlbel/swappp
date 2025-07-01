use image::{GenericImageView, ImageBuffer, Rgba};
use clap::Parser;
use std::{path::Path, u32};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

pub const CATPPUCCIN_MOCHA: &[Rgba<u8>] = &[
    Rgba([0xF5, 0xE0, 0xDC, 255]),
    Rgba([0xF2, 0xCD, 0xCD, 255]),
    Rgba([0xF5, 0xC2, 0xE7, 255]),
    Rgba([0xCB, 0xA6, 0xF7, 255]),
    Rgba([0xF3, 0x8B, 0xA8, 255]),
    Rgba([0xEB, 0xA0, 0xAC, 255]),
    Rgba([0xFA, 0xB3, 0x87, 255]),
    Rgba([0xF9, 0xE2, 0xAF, 255]),
    Rgba([0xA6, 0xE3, 0xA1, 255]),
    Rgba([0x94, 0xE2, 0xD5, 255]),
    Rgba([0x89, 0xDC, 0xEB, 255]),
    Rgba([0x74, 0xC7, 0xEC, 255]),
    Rgba([0x89, 0xB4, 0xFA, 255]),
    Rgba([0xB4, 0xBE, 0xFE, 255]),
    Rgba([0xCD, 0xD6, 0xF4, 255]),
    Rgba([0xBA, 0xC2, 0xDE, 255]),
    Rgba([0xA6, 0xAD, 0xC8, 255]),
    Rgba([0x93, 0x99, 0xB2, 255]),
    Rgba([0x7F, 0x84, 0x9C, 255]),
    Rgba([0x6C, 0x70, 0x86, 255]),
    Rgba([0x58, 0x5B, 0x70, 255]),
    Rgba([0x45, 0x47, 0x5A, 255]),
    Rgba([0x31, 0x32, 0x44, 255]),
    Rgba([0x1E, 0x1E, 0x2E, 255]),
    Rgba([0x18, 0x18, 0x25, 255]),
    Rgba([0x11, 0x11, 0x1B, 255]),
];

fn find_closest_palette_color(original_pixel: Rgba<u8>, palette: &[Rgba<u8>]) -> Rgba<u8> {
    let Rgba([r1, g1, b1, _a1]) = original_pixel;
    let mut min_dist_sq = u32::MAX;
    let mut closest_color = Rgba([0, 0, 0, 255]);

    for &Rgba([r2, g2, b2, _a2]) in palette.iter() {
        let dr = r1 as i32 - r2 as i32;
        let dg = g1 as i32 - g2 as i32;
        let db = b1 as i32 - b2 as i32;
        let dist_sq = (dr * dr + dg * dg + db * db) as u32;

        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            closest_color = Rgba([r2, g2, b2, _a1]);
        }
    }

    closest_color
}

fn change_color_scheme(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    println!("Dimensions: {:?}", img.dimensions());

    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let new_pixel = find_closest_palette_color(pixel, CATPPUCCIN_MOCHA);
            new_img.put_pixel(x, y, new_pixel);
        }
    }

    new_img.save(output_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_file = Path::new(&args.input);
    let output_file = Path::new(&args.output);

    match change_color_scheme(input_file, output_file) {
        Ok(_) => println!("Ok"),
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
