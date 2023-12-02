use image::{DynamicImage, ImageRgba};
use std::fs;
use std::path::Path;



fn place_image(collage: &mut DynamicImage, img: &ImageRgba, x_offset: u32, y_offset: u32) {
    let img_width: u32 = img.width();
    let img_height: u32 = img.height();

    // Iterate over pixels of the 'img' image
    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = img.get_pixel(x, y);
            let (r, g, b, a) = pixel.channels();

            // Calculate destination coordinates within the 'collage'
            let dst_x: <u32 as Add<u32>>::Output = x + x_offset;
            let dst_y: <u32 as Add<u32>>::Output = y + y_offset;

            // Check if the destination coordinate is within the bounds of the 'collage'
            if dst_x < collage.width() && dst_y < collage.height() {
                let dst_pixel = collage.get_pixel_mut(dst_x, dst_y);
                let (dst_r, dst_g, dst_b, dst_a) = dst_pixel.channels();

                // Blend the pixels using alpha values
                let blended_r = (a * r + (1.0 - a) * dst_r) as u8;
                let blended_g = (a * g + (1.0 - a) * dst_g) as u8;
                let blended_b = (a * b + (1.0 - a) * dst_b) as u8;

                // Set the blended pixel value in the 'collage'
                dst_pixel.set_channels(blended_r, blended_g, blended_b, a);
            }
        }
    }
}



fn main() {
    let folder_path = r"F:\Freelancing\I will do python javascript cpp and rust programs\djohnmorrison\myproject\generate images\indians";

    if let Ok(entries) = fs::read_dir(folder_path) {
        let mut images: Vec<DynamicImage> = vec![];

        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "png"{
                    if let Ok(image) = image::open(&path) {
                        let resized_image = image.resize(100, 100, image::imageops::FilterType::Lanczos3);
                        images.push(resized_image);
                    }
                }
            }
        }

        let mut collage = DynamicImage::new_rgb8(100000, 100000);
        let mut x_offset: u32 = 0;
        let mut y_offset: u32 = 0;

        for img in &images {
            if x_offset + img.width() > 100000 {
                x_offset = 0;
                y_offset += img.height();
            }

            place_image(&mut collage, img, x_offset, y_offset);
            x_offset += img.width();
        }

        collage.save("path/to/save/collage.png").unwrap();
    }
}