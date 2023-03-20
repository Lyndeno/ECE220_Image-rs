use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};

#[derive(Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn make_red(&mut self) {
        self.g = 0;
        self.b = 0;
    }

    pub fn make_green(&mut self) {
        self.r = 0;
        self.b = 0;
    }

    pub fn make_blue(&mut self) {
        self.g = 0;
        self.r = 0;
    }
}

pub struct PixelArray {
    data: Vec<Pixel>,
    pub width: usize,
    pub height: usize,
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
        f: &mut BufReader<File>,
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
        f: &mut BufWriter<File>,
        px_offset: usize,
        padding: usize,
    ) -> Result<(), std::io::Error> {
        f.seek(SeekFrom::Start(px_offset as u64))?;

        let pad: u8 = 0x00;

        for y in 0..self.height {
            for x in 0..self.width {
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

    pub fn modify(mut self, f: &dyn Fn(&mut Pixel)) -> Self {
        for y in 0..self.height {
            for x in 0..self.width {
                f(&mut self[(x, y)]);
            }
        }
        self
    }

    pub fn make_red(mut self) -> Self {
        self.modify(&Pixel::make_red)
    }

    pub fn make_green(mut self) -> Self {
        self.modify(&Pixel::make_green)
    }

    pub fn make_blue(mut self) -> Self {
        self.modify(&Pixel::make_blue)
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
