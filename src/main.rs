use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use byteorder::ReadBytesExt;
use clap::{Parser, ValueEnum};

mod fileinfo;
mod pixel;

use crate::fileinfo::FileInfo;
use crate::pixel::PixelArray;

#[derive(ValueEnum, Clone, Debug)]
enum ImgOp {
    Red,
    Green,
    Blue,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of input file
    input: String,

    /// Name of output file
    output: String,

    /// Operation to apply
    #[arg(value_enum)]
    operation: ImgOp,
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

    let mut in_buf = BufReader::new(inputfile);
    let file_info = FileInfo::from_file(&mut in_buf)?;

    let pixels = PixelArray::from_bm(
        &mut in_buf,
        file_info.px_width as usize,
        file_info.px_height as usize,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;

    let out_image = match args.operation {
        ImgOp::Red => pixels.make_red(),
        ImgOp::Green => pixels.make_green(),
        ImgOp::Blue => pixels.make_blue(),
    };

    let mut output_file = File::create(args.output)?;
    let mut out_buf = BufWriter::new(output_file);
    file_info.write_file(&mut out_buf)?;
    out_image.write_bm(
        &mut out_buf,
        file_info.pix_offset as usize,
        file_info.get_padding(),
    )?;
    out_buf.flush()?;

    Ok(())
}
