use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::num::Wrapping;

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
        for p in &mut self.data {
            f(p);
        }
        self
    }

    pub fn make_red(self) -> Self {
        self.modify(&Pixel::make_red)
    }

    pub fn make_green(self) -> Self {
        self.modify(&Pixel::make_green)
    }

    pub fn make_blue(self) -> Self {
        self.modify(&Pixel::make_blue)
    }

    pub fn make_blur(&self, blur_y: usize, blur_x: usize) -> Self {
        let mut new = Self::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let x_offset = (blur_x - 1) / 2;
                let y_offset = (blur_y - 1) / 2;

                // Use Wrapping() so that if we subtract and go into the negatives we wrap around.
                // Then we only check that we are not exceeding the maximum height/width.
                let mut h_low_bound = Wrapping(x) - Wrapping(x_offset);
                let mut h_high_bound = Wrapping(x) + Wrapping(x_offset);
                let mut v_low_bound = Wrapping(y) - Wrapping(y_offset);
                let mut v_high_bound = Wrapping(y) + Wrapping(y_offset);

                let mut pix_count = 0;

                let mut r_tot = 0;
                let mut g_tot = 0;
                let mut b_tot = 0;

                if h_low_bound.0 >= self.width {
                    h_low_bound.0 = 0;
                }
                if h_high_bound.0 >= self.width {
                    h_high_bound.0 = self.width - 1;
                }
                if v_low_bound.0 >= self.height {
                    v_low_bound.0 = 0;
                }
                if v_high_bound.0 >= self.height {
                    v_high_bound.0 = self.height - 1;
                }

                for i in h_low_bound.0..=h_high_bound.0 {
                    for j in v_low_bound.0..=v_high_bound.0 {
                        r_tot += self[(i, j)].r as usize;
                        g_tot += self[(i, j)].g as usize;
                        b_tot += self[(i, j)].b as usize;

                        pix_count += 1;
                    }
                }

                let r_avg = r_tot / pix_count;
                let g_avg = g_tot / pix_count;
                let b_avg = b_tot / pix_count;

                new[(x, y)].r = r_avg as u8;
                new[(x, y)].g = g_avg as u8;
                new[(x, y)].b = b_avg as u8;
            }
        }
        new
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

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    #[test]
    fn number() {
        let zero = Wrapping(0usize);
        let one = Wrapping(1usize);
        print!("{}", zero - one);
    }
}
