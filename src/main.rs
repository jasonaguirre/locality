use array2::Array2;
use csc411_image::{RgbImage, Read};
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Row Major
    #[clap(long = "row-major", required = false)]
    row_major: bool,
    // Col Major
    #[clap(long = "col-major", required = false)]
    col_major: bool,
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,
    // Path
    #[clap(required = false)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let rotate = args.rotate;
    let image;
    if args.path != None {
        let input = args.path;
        image = RgbImage::read(input.as_deref()).unwrap();
    }
    else{
        //get standard input
        assert!(env::args().len() < 2, "Too many arguments!");
        image = RgbImage::read(None).unwrap();
    }

    let pre_image = Array2::from_row_major(
        image.width as usize,
        image.height as usize,
        image.pixels,
    );

    if args.col_major{

        //let col_image_pixels = pre_image.expect("REASON").iter_col_major().collect();

        let pre_image = Array2::from_row_major(
            image.width as usize,
            image.height as usize,
            pre_image.expect("REASON").iter_col_major().collect(),
        );
    }
}

fn rotate_90(pre_image: &Array2<csc411_image::Rgb>) {
    let w = pre_image.width();
    let h = pre_image.height();

    let final_image = Array2::new(0,h,w);

    // for row in 0..h{
    //     for col in 0..w{
    //
    //     }
    // }


}
fn rotate_180(pre_image: &Array2<csc411_image::Rgb>) {

}