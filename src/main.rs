use std::fs::File;

use byteorder::ReadBytesExt;
use clap::Parser;

mod fileinfo;
mod pixel;

use crate::fileinfo::{read_file_info, write_file_info};
use crate::pixel::PixelArray;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of input file
    input: String,

    /// Name of output file
    output: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    println!("Hello, world! {}", args.input);

    let mut inputfile = File::open(args.input)?;
    let mut id = [0, 0];
    //inputfile.read_exact(&mut id)?;
    id[0] = inputfile.read_u8()?;
    id[1] = inputfile.read_u8()?;

    if String::from_utf8_lossy(&id) == String::from("BM") {
        println!("Valid bitmap");
    } else {
        panic!("Not a bitmap");
    }

    // fileinfo
    //let mut test = inputfile.read_u16::<NativeEndian>()?;

    let file_info = read_file_info(&mut inputfile)?;

    println!("{:?} {:?}", String::from_utf8_lossy(&id), file_info);

    let pixels = PixelArray::from_bm(
        &mut inputfile,
        file_info.px_width as usize,
        file_info.px_height as usize,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;

    let mut output_file = File::create(args.output)?;
    write_file_info(file_info.clone(), &mut output_file)?;
    pixels.write_bm(
        &mut output_file,
        pixels.width,
        pixels.height,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;

    Ok(())
}
