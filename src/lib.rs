use image::{Rgba, RgbaImage};

pub struct Image {
    pub buf: Vec<u8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buf: vec![0; (width * height * 4) as usize],
            width,
            height,
        }
    }

    pub fn from_bytes(buf: Vec<u8>, width: u32, height: u32) -> Self {
        Self { buf, width, height }
    }

    /// Loads an image from the given path. Makes an educated guess about the image format.
    pub fn load_from_file(path: &str) -> Self {
        let img = image::open(path).unwrap();
        Self {
            buf: img.to_rgba8().to_vec(),
            width: img.width(),
            height: img.height(),
        }
    }

    /// Create a new image from a byte slice
    /// Makes an educated guess about the image format. TGA is not supported by this function.
    pub fn load_from_memory(buf: &[u8]) -> Self {
        let img = image::load_from_memory(buf).unwrap();
        Self {
            buf: img.to_rgba8().to_vec(),
            width: img.width(),
            height: img.height(),
        }
    }

    /// Save the image to a file. The image format is derived from the file extension.
    /// Only jpeg, png, ico, pnm, bmp, exr and tiff files are supported.
    #[inline]
    pub fn save_image(&self, path: &str) -> image::ImageResult<()> {
        image::save_buffer(
            path,
            &self.buf,
            self.width,
            self.height,
            image::ColorType::Rgba8,
        )
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline(always)]
    pub fn set(&mut self, x: u32, y: u32, col: (u8, u8, u8, u8)) {
        let width = self.width; // copy to stack for more efficency
        if x >= width || y >= self.height {
            return;
        }

        unsafe {
            // use pointer arithmetic to get rid of bound checking overhead
            let p = self.buf.as_mut_ptr().add((y * width * 4 + x * 4) as _);
            *p = col.0;
            *p.add(1) = col.1;
            *p.add(2) = col.2;
            *p.add(3) = col.3;
        }
    }

    #[inline(always)]
    pub fn at(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        unsafe {
            let p = self.buf.as_ptr().add((y * self.width * 4 + x * 4) as _);
            (*p, *p.add(1), *p.add(2), *p.add(3))
        }
    }

    #[inline(always)]
    pub fn clear(&mut self, col: (u8, u8, u8, u8)) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, col);
            }
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: (u8, u8, u8, u8)) {
        for xx in x..=x + w {
            for yy in y..=y + h {
                self.set(xx, yy, col)
            }
        }
    }
}
