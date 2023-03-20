use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::Seek;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub id1: u8,
    pub id2: u8,
    pub size_file: u32,
    pub unused1: u16,
    pub unused2: u16,
    pub pix_offset: u32,
    //DIB Header
    pub dib_size: u32,
    pub px_width: u32,
    pub px_height: u32,
    pub cplane: u16,
    pub bit_px: u16,
    pub px_compress: u32,
    pub raw_size: u32,
    pub dpi_h: u32,
    pub dpi_v: u32,
    pub colors: u32,
    pub imp_colors: u32,
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

    pub fn from_file(f: &mut File) -> Result<Self, std::io::Error> {
        let mut i = Self::new();
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

    pub fn write_file(&self, f: &mut File) -> Result<(), std::io::Error> {
        f.rewind()?;

        f.write_u8(self.id1)?;
        f.write_u8(self.id2)?;

        f.write_u32::<NativeEndian>(self.size_file)?;

        f.write_u16::<NativeEndian>(self.unused1)?;
        f.write_u16::<NativeEndian>(self.unused2)?;

        f.write_u32::<NativeEndian>(self.pix_offset)?;

        f.write_u32::<NativeEndian>(self.dib_size)?;
        f.write_u32::<NativeEndian>(self.px_width)?;
        f.write_u32::<NativeEndian>(self.px_height)?;

        f.write_u16::<NativeEndian>(self.cplane)?;
        f.write_u16::<NativeEndian>(self.bit_px)?;

        f.write_u32::<NativeEndian>(self.px_compress)?;
        f.write_u32::<NativeEndian>(self.raw_size)?;

        f.write_u32::<NativeEndian>(self.dpi_h)?;
        f.write_u32::<NativeEndian>(self.dpi_v)?;

        f.write_u32::<NativeEndian>(self.colors)?;
        f.write_u32::<NativeEndian>(self.imp_colors)?;

        Ok(())
    }
}
