use std::{fs::File, io::Read};

use byteorder::{NativeEndian, ReadBytesExt};
use clap::Parser;

struct FileInfo {
    id1: u8,
    id2: u8,
    size_file: u32,
    unused1: u16,
    unused2: u16,
    pix_offset: u32,
    //DIB Header
    dib_size: u32,
    px_width: u32,
    px_height: u32,
    cplane: u16,
    bit_px: u16,
    px_compress: u32,
    raw_size: u32,
    dpi_h: u32,
    dpi_v: u32,
    colors: u32,
    imp_colors: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of input file
    file: String,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    println!("Hello, world! {}", args.file);

    let mut inputfile = File::open(args.file)?;
    let mut id = [0, 0];
    //inputfile.read_exact(&mut id)?;
    id[0] = inputfile.read_u8()?;
    id[1] = inputfile.read_u8()?;
    let mut test = inputfile.read_u16::<NativeEndian>()?;

    println!("{:?} {:?}", String::from_utf8_lossy(&id), test);

    Ok(())
}
