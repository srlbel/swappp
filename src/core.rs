use image::{ImageBuffer, Rgb};
use std::path::Path;

pub fn find_closest_palette_color(
    original_pixel: Rgb<u8>,
    palette: &[Rgb<u8>],
) -> Rgb<u8> {
    let Rgb([r1, g1, b1]) = original_pixel;
    let mut min_dist_sq = u32::MAX;
    let mut closest_color = Rgb([0, 0, 0]);

    for &Rgb([r2, g2, b2]) in palette.iter() {
        let dr = r1 as i32 - r2 as i32;
        let dg = g1 as i32 - g2 as i32;
        let db = b1 as i32 - b2 as i32;
        let dist_sq = (dr * dr + dg * dg + db * db) as u32;

        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            closest_color = Rgb([r2, g2, b2]);
        }
    }

    closest_color
}

pub fn change_color_scheme(
    input_path: &Path,
    output_path: &Path,
    palette: &[Rgb<u8>],
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path).unwrap().to_rgb8();
    println!("Dimensions: {:?}", img.dimensions());

    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let new_pixel = find_closest_palette_color(pixel.clone(), palette);
            new_img.put_pixel(x, y, new_pixel);
        }
    }

    new_img.save(output_path)?;
    Ok(())
}