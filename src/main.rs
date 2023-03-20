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
    Blur,
}

struct BitMap {
    meta: FileInfo,
    image: PixelArray,
}

impl BitMap {
    pub fn from_buf(f: &mut BufReader<File>) -> Result<Self, std::io::Error> {
        let meta = FileInfo::from_file(f)?;
        let image = PixelArray::from_bm(
            f,
            meta.px_width as usize,
            meta.px_height as usize,
            meta.pix_offset as usize,
            meta.get_padding(),
        )?;
        Ok(Self { meta, image })
    }

    pub fn write_buf(&self, f: &mut BufWriter<File>) -> Result<(), std::io::Error> {
        self.meta.write_file(f)?;
        self.image
            .write_bm(f, self.meta.pix_offset as usize, self.meta.get_padding())?;
        f.flush()?;
        Ok(())
    }

    pub fn make_red(self) -> Self {
        Self {
            meta: self.meta,
            image: self.image.make_red(),
        }
    }

    pub fn make_green(self) -> Self {
        Self {
            meta: self.meta,
            image: self.image.make_green(),
        }
    }

    pub fn make_blue(self) -> Self {
        Self {
            meta: self.meta,
            image: self.image.make_blue(),
        }
    }

    pub fn make_blur(self, blur_x: usize, blur_y: usize) -> Self {
        Self {
            meta: self.meta,
            image: self.image.make_blur(blur_x, blur_y),
        }
    }
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

    let in_file = File::open(args.input)?;
    let mut in_buf = BufReader::new(in_file);

    let mut id = [0, 0];
    id[0] = in_buf.read_u8()?;
    id[1] = in_buf.read_u8()?;

    if String::from_utf8_lossy(&id) == *"BM" {
        println!("Valid bitmap");
    } else {
        panic!("Not a bitmap");
    }

    let file_info = FileInfo::from_file(&mut in_buf)?;
    let in_bm = BitMap::from_buf(&mut in_buf)?;

    let out_bm = match args.operation {
        ImgOp::Red => in_bm.make_red(),
        ImgOp::Green => in_bm.make_green(),
        ImgOp::Blue => in_bm.make_blue(),
        ImgOp::Blur => in_bm.make_blur(7, 7),
    };

    let out_file = File::create(args.output)?;
    let mut out_buf = BufWriter::new(out_file);
    file_info.write_file(&mut out_buf)?;
    out_bm.write_buf(&mut out_buf)?;

    Ok(())
}
