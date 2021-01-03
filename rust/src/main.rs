use std::{fs, path::Path};

use anyhow::Result;
use image::{self, imageops, ImageBuffer};
use rayon::prelude::*;

mod config;

fn main() -> Result<()> {
    use config::{NX, NY};

    let mut img = ImageBuffer::new(NX as u32, config::NY as u32);

    let scn = config::create();

    const TOTAL: usize = NX * NY;
    let img_pixels: Vec<[u8; 3]> = (0..TOTAL)
        .into_par_iter()
        .map(|i: usize| -> [u8; 3] {
            let trng = &mut rand::thread_rng();
            let position = (i / NY, i % NY);
            scn.color(
                position,
                (config::NX as usize, config::NY as usize, config::NS),
                config::DEP,
                trng,
            )
        })
        .collect();

    for x in 0..NX {
        for y in 0..NY {
            *img.get_pixel_mut(x as u32, y as u32) = image::Rgb(img_pixels[x * NY + y]);
        }
    }

    imageops::flip_vertical_in_place(&mut img);

    let folder = "images";
    let fname = "image.png";

    fs::create_dir(folder)?;
    let full_path = Path::new(folder).join(fname);

    img.save(full_path)?;

    Ok(())
}
