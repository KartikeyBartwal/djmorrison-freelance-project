use std::io::{self, Write};
use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba, ImageError};
use rand::{self, Rng};

fn iterator(k: i32) -> Result<(), ImageError> {
    let img = image::io::Reader::open(r"F:\Freelancing\I will do python javascript cpp and rust programs\djohnmorrison\myproject\generate images\target indian image.png")?
        .decode()?;

    println!("Read the image");

    let (w, h) = img.dimensions();

    println!("Obtained the dimensions of the image");

    let mut output = ImageBuffer::new(w, h);

    println!("Created output buffer");

    let mut rng = rand::thread_rng();
    let l: i32 = rng.gen_range(25..255);
    let m: i32 = rng.gen_range(25..255);
    let n: i32 = rng.gen_range(25..255);

    println!("Genereated thread_rng for each layer RGB");

    for (x, y, pixel) in img.pixels() {
        if pixel[2] == 255 && pixel[1] == 255 && pixel[0] == 255 {
            output.put_pixel(x, y, Rgba::from([pixel[2], pixel[1], pixel[0], pixel[3]]));
        } else {
            println!("Processing pixel at ({}, {})", x, y); // Logging the pixel being processed
            io::stdout().flush().unwrap();

            let red = ((pixel[2] as i32 + l) % 255) as u8;
            let green = ((pixel[1] as i32 + m) % 255) as u8;
            let blue = ((pixel[0] as i32 + n) % 255) as u8;
            output.put_pixel(x, y, Rgba::from([red, green, blue, pixel[3]]));
        }
    }

    let output_file_path = format!(r"F:\Freelancing\I will do python javascript cpp and rust programs\djohnmorrison\myproject\generate images\indians{}.png", k);
    output.save(output_file_path)?;

    // println!("Image processing completed for {}", output_file_path); // Logging completion

    Ok(())
}


fn main() {
    let mut iterations = 3;
    println!("Number of iterations... {}" , iterations);

    let mut image_number = 1;
    while iterations > 0 {
        println!("Running iteration number {}" , iterations);
        let _ = iterator(iterations);
        iterations -= 1;
        println!("number of images created {}" , image_number);
        image_number += 1;
        
        for _ in 0..50 {
            print!("=");
        }
    }
}