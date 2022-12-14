#![allow(unused)]
use array2::Array2;
use csc411_image::{RgbImage, Read, Write};
use clap::Parser;
use std::time::{Instant};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Row Major
    #[clap(long = "row-major", required = false)]
    row_major: bool,
    // Col Major
    #[clap(long = "col-major", required = false)]
    col_major: bool,
    // Transposw
    #[clap(long = "transpose", required = false)]
    transpose: bool,
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Rotation
    #[clap(short = 'r', long = "rotate", required = false)]
    rotate: Option<u32>,
    // Path
    #[clap(required = false)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let rotate = args.rotate;
    let flip = args.flip;
    let transpose = args.transpose;

    if rotate != None && rotate != Some(90) && rotate != Some(180){
        eprintln!("This rotation has not been implemented");
        std::process::exit(1);
    }

    if flip != None{
        eprintln!("flip has not been implemented");
        std::process::exit(1);
    }

    if transpose{
        eprintln!("transpose has not been implemented");
        std::process::exit(1);
    }

    let image;
    if args.path != None {
        let input = args.path;
        image = RgbImage::read(input.as_deref()).unwrap();
    }
    else{
        //read from stdin
        image = RgbImage::read(None).unwrap();
    }

    let pre_image = Array2::from_row_major(
        image.width as usize,
        image.height as usize,
        image.pixels.clone(),
    );

    let pre_image_clone = &pre_image.unwrap().clone();
    let denom = image.denominator;

    rotate_by(pre_image_clone, denom, rotate, args.col_major)
}

fn rotate_by(pre_image: &Array2<csc411_image::Rgb>, denom: u16, rotate: Option<u32>, col_major: bool) {
    let width = pre_image.width() as usize;
    let height = pre_image.height() as usize;

    let init_value = pre_image.get(0, 0).unwrap().clone();

    let mut final_image_pixels = vec![init_value; width * height];

    let mut final_image = csc411_image::RgbImage {
        pixels: final_image_pixels.clone(),
        width: height as u32,
        height: width as u32,
        denominator: denom,
    };

    if col_major {

        let col_major_iter = pre_image.iter_col_major();

        //starting with a blank pixel vector, for each iteration, the pixel at i,j in the original image is copied
        //the corresponding location in the rotated image vector, terminates once original image vector has been iterated over
        for (i,j,_val) in col_major_iter{
            let original_pixel = pre_image.get(i, j).unwrap().clone();
            modify_pixels(i, j, width, height, rotate, original_pixel, &mut final_image_pixels);
        }
    }
    else{

        let row_major_iter = pre_image.iter_row_major();

        for (i,j,_val) in row_major_iter{
            let original_pixel = pre_image.get(i, j).unwrap().clone();
            modify_pixels(i, j, width, height, rotate, original_pixel, &mut final_image_pixels);
        }
    }

    if rotate == Some(90) {
        //create final image, height and width are flipped for 90-deg rotation
        final_image = csc411_image::RgbImage {
            pixels: final_image_pixels.clone(),
            width: height as u32,
            height: width as u32,
            denominator: denom,
        };
    }

    if rotate == Some(180) {
        //create final image
        final_image = csc411_image::RgbImage {
            pixels: final_image_pixels.clone(),
            width: width as u32,
            height: height as u32,
            denominator: denom,
        };

    }
    final_image.write(None);
}

fn get_index(c: usize, r: usize, width: usize, height: usize) -> Option<usize> {
    if c < width && r < height {
        Some(r * width + c)
    } else {
        None
    }
}

fn modify_pixels(i: usize, j: usize, width: usize, height: usize, rotate: Option<u32>, original_pixel: csc411_image::Rgb, final_image_pixels: &mut Vec<csc411_image::Rgb>){

    let mut index = None;
    if rotate == Some(90) {
        index = get_index(height - j - 1, i, height, width);
    }
    if rotate == Some(180) {
        index = get_index(width - i - 1, height - j - 1, width, height);
    }

    final_image_pixels[index.unwrap()] = original_pixel;
}