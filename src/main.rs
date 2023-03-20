use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use clap::Parser;

#[derive(Debug, Clone)]
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

impl FileInfo {
    pub fn new() -> Self {
        Self {
            id1: 0,
            id2: 0,
            size_file: 0,
            unused1: 0,
            unused2: 0,
            pix_offset: 0,
            //DIB Header
            dib_size: 0,
            px_width: 0,
            px_height: 0,
            cplane: 0,
            bit_px: 0,
            px_compress: 0,
            raw_size: 0,
            dpi_h: 0,
            dpi_v: 0,
            colors: 0,
            imp_colors: 0,
        }
    }

    pub fn get_padding(&self) -> usize {
        (self.raw_size as usize / self.px_height as usize) - (self.px_width as usize * 3)
    }
}

#[derive(Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

struct PixelArray {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl PixelArray {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Pixel::new(); width * height],
            width,
            height,
        }
    }

    pub fn from_bm(
        f: &mut File,
        w: usize,
        h: usize,
        px_offset: usize,
        padding: usize,
    ) -> Result<Self, std::io::Error> {
        let mut array = PixelArray::new(w, h);
        f.seek(SeekFrom::Start(px_offset as u64))?;

        for y in 0..h {
            for x in 0..w {
                array[(x, y)].b = f.read_u8()?;
                array[(x, y)].g = f.read_u8()?;
                array[(x, y)].r = f.read_u8()?;
            }
            f.seek(SeekFrom::Current(padding as i64))?;
        }

        Ok(array)
    }

    pub fn write_bm(
        &self,
        f: &mut File,
        w: usize,
        h: usize,
        px_offset: usize,
        padding: usize,
    ) -> Result<(), std::io::Error> {
        f.seek(SeekFrom::Start(px_offset as u64))?;

        let pad: u8 = 0x00;

        for y in 0..h {
            for x in 0..w {
                f.write_u8(self[(x, y)].b)?;
                f.write_u8(self[(x, y)].g)?;
                f.write_u8(self[(x, y)].r)?;
            }
            //f.seek(SeekFrom::Current(padding as i64))?;
            for i in 0..padding {
                f.write_u8(pad)?;
            }
        }

        Ok(())
    }
}

type XyPos = (usize, usize);
impl std::ops::Index<XyPos> for PixelArray {
    type Output = Pixel;
    fn index(&self, (x, y): XyPos) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}
impl std::ops::IndexMut<XyPos> for PixelArray {
    fn index_mut(&mut self, (x, y): XyPos) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of input file
    input: String,

    /// Name of output file
    output: String,
}

fn read_file_info(f: &mut File) -> Result<FileInfo, std::io::Error> {
    let mut i = FileInfo::new();
    f.rewind()?;

    i.id1 = f.read_u8()?;
    i.id2 = f.read_u8()?;

    i.size_file = f.read_u32::<NativeEndian>()?;

    i.unused1 = f.read_u16::<NativeEndian>()?;
    i.unused2 = f.read_u16::<NativeEndian>()?;

    i.pix_offset = f.read_u32::<NativeEndian>()?;

    i.dib_size = f.read_u32::<NativeEndian>()?;
    i.px_width = f.read_u32::<NativeEndian>()?;
    i.px_height = f.read_u32::<NativeEndian>()?;

    i.cplane = f.read_u16::<NativeEndian>()?;
    i.bit_px = f.read_u16::<NativeEndian>()?;

    i.px_compress = f.read_u32::<NativeEndian>()?;
    i.raw_size = f.read_u32::<NativeEndian>()?;

    i.dpi_h = f.read_u32::<NativeEndian>()?;
    i.dpi_v = f.read_u32::<NativeEndian>()?;

    i.colors = f.read_u32::<NativeEndian>()?;
    i.imp_colors = f.read_u32::<NativeEndian>()?;

    Ok(i)
}

fn write_file_info(i: FileInfo, f: &mut File) -> Result<FileInfo, std::io::Error> {
    f.rewind()?;

    f.write_u8(i.id1)?;
    f.write_u8(i.id2)?;

    f.write_u32::<NativeEndian>(i.size_file)?;

    f.write_u16::<NativeEndian>(i.unused1)?;
    f.write_u16::<NativeEndian>(i.unused2)?;

    f.write_u32::<NativeEndian>(i.pix_offset)?;

    f.write_u32::<NativeEndian>(i.dib_size)?;
    f.write_u32::<NativeEndian>(i.px_width)?;
    f.write_u32::<NativeEndian>(i.px_height)?;

    f.write_u16::<NativeEndian>(i.cplane)?;
    f.write_u16::<NativeEndian>(i.bit_px)?;

    f.write_u32::<NativeEndian>(i.px_compress)?;
    f.write_u32::<NativeEndian>(i.raw_size)?;

    f.write_u32::<NativeEndian>(i.dpi_h)?;
    f.write_u32::<NativeEndian>(i.dpi_v)?;

    f.write_u32::<NativeEndian>(i.colors)?;
    f.write_u32::<NativeEndian>(i.imp_colors)?;

    Ok(i)
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
