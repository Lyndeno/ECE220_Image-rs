use byteorder::{ReadBytesExt, WriteBytesExt};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};

#[derive(Clone, Debug, PartialEq)]
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

impl From<Vec<Vec<Pixel>>> for PixelArray {
    fn from(value: Vec<Vec<Pixel>>) -> Self {
        let height = value.len();
        let width = value[0].len();
        let data: Vec<Pixel> = value.into_iter().flatten().collect();
        Self {
            data,
            width,
            height,
        }
    }
}

#[derive(Debug, PartialEq)]
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
            for _ in 0..padding {
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

    pub fn make_blur(self, blur_y: usize, blur_x: usize) -> Self {
        //let mut new = Self::new(self.width, self.height);
        let mut new = vec![vec![Pixel::new(); self.width]; self.height];
        new.par_iter_mut().enumerate().for_each(|(y, row)| {
            row.par_iter_mut().enumerate().for_each(|(x, pixel)| {
                let x_offset = (blur_x - 1) / 2;
                let y_offset = (blur_y - 1) / 2;

                // Use Wrapping() so that if we subtract and go into the negatives we wrap around.
                // Then we only check that we are not exceeding the maximum height/width.
                let h_low_bound = x.saturating_sub(x_offset);
                let mut h_high_bound = x.saturating_add(x_offset);
                if h_high_bound >= self.width {
                    h_high_bound = self.width - 1;
                }
                let v_low_bound = y.saturating_sub(y_offset);
                let mut v_high_bound = y.saturating_add(y_offset);
                if v_high_bound >= self.height {
                    v_high_bound = self.height - 1;
                }

                let mut pix_count = 0;

                let mut r_tot = 0;
                let mut g_tot = 0;
                let mut b_tot = 0;

                for i in h_low_bound..=h_high_bound {
                    for j in v_low_bound..=v_high_bound {
                        r_tot += self[(i, j)].r as usize;
                        g_tot += self[(i, j)].g as usize;
                        b_tot += self[(i, j)].b as usize;

                        pix_count += 1;
                    }
                }

                let r_avg = r_tot / pix_count;
                let g_avg = g_tot / pix_count;
                let b_avg = b_tot / pix_count;

                pixel.r = r_avg as u8;
                pixel.g = g_avg as u8;
                pixel.b = b_avg as u8;
            });
        });
        new.into()
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

    use crate::fileinfo::FileInfo;
    use std::fs::File;
    use std::io::BufReader;

    use super::PixelArray;

    #[test]
    fn number() {
        let zero = Wrapping(0usize);
        let one = Wrapping(1usize);
        print!("{}", zero - one);
    }

    #[test]
    fn test_blurred() {
        let f = File::open("./Cat.bmp").unwrap();
        let mut buf = BufReader::new(f);
        let meta = FileInfo::from_file(&mut buf).unwrap();
        let orig = PixelArray::from_bm(
            &mut buf,
            meta.px_width as usize,
            meta.px_height as usize,
            meta.pix_offset as usize,
            meta.get_padding(),
        )
        .unwrap();
        let f_blur = File::open("./Cat_blurred.bmp").unwrap();
        let mut buf_blur = BufReader::new(f_blur);
        let meta_blur = FileInfo::from_file(&mut buf_blur).unwrap();
        let blur = PixelArray::from_bm(
            &mut buf_blur,
            meta_blur.px_width as usize,
            meta_blur.px_height as usize,
            meta_blur.pix_offset as usize,
            meta_blur.get_padding(),
        )
        .unwrap();

        assert_eq!(blur, orig.make_blur(7, 7))
    }
}
