use std::fs::File;

use byteorder::ReadBytesExt;
use clap::Parser;

mod fileinfo;
mod pixel;

use crate::fileinfo::FileInfo;
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

    let file_info = FileInfo::from_file(&mut inputfile)?;

    let pixels = PixelArray::from_bm(
        &mut inputfile,
        file_info.px_width as usize,
        file_info.px_height as usize,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;

    let mut output_file = File::create(args.output)?;
    file_info.write_file(&mut output_file)?;
    pixels.write_bm(
        &mut output_file,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;

    Ok(())
}
