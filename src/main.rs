use locality::TwoDArray;
use csc411_image::{Read, GrayImage};
use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    // Row Major
    #[clap(long = "row-major", required = false)]
    row_major: Boolean,
    // Col Major
    #[clap(long = "col-major", required = false)]
    col_major: Boolean,
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let rotate = args.rotate;
    let image;
    if env::args().len() == 2 {
        let input = env::args().nth(1);
        image = GrayImage::read(input.as_deref()).unwrap();
    }
    else{
        //get standard input
        assert!(env::args().len() < 2, "Too many arguments!");
        image = GrayImage::read(None).unwrap();
    }



}
