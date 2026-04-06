use crate::*;

// From and Into implementations for converting between image::RgbaImage and Texture<Srgba>
impl From<image::RgbaImage> for Texture<pixel::Srgba, 2> {
    fn from(img: image::RgbaImage) -> Self {
        let dim = [img.width(), img.height()];
        let mut data = Vec::with_capacity((dim[0] * dim[1]) as usize);
        for px in img.pixels() {
            data.push(pixel::Srgba::from(px.0));
        }
        Texture::from_parts(dim, data)
    }
}
impl Into<image::RgbaImage> for Texture<pixel::Srgba, 2> {
    fn into(self) -> image::RgbaImage {
        let [width, height] = self.dim();
        let mut img = image::RgbaImage::new(width, height);

        let mut cur_x = 0;
        let mut cur_y = 0;
        for px in self.data().iter() {
            img.put_pixel(cur_x, cur_y, image::Rgba(px.to_array()));

            cur_x += 1;
            if cur_x >= width {
                cur_x = 0;
                cur_y += 1;
            }
        }
        img
    }
}
impl Texture<pixel::Srgba, 2> {
    pub fn load_srgba<P: AsRef<std::path::Path>>(path: P) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgba8();
        Ok(Self::from(img))
    }

    pub fn load_bytes_srgba(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?.to_rgba8();
        Ok(Self::from(img))
    }

    pub fn save_srgba<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let img: image::RgbaImage = self.clone().into();
        img.save(path)
    }
}

// From and Into implementations for converting between image::RgbaImage and Texture<[UNorm8; 4]>
impl From<image::RgbaImage> for Texture<[pixel::UNorm8; 4], 2> {
    fn from(img: image::RgbaImage) -> Self {
        let dim = [img.width(), img.height()];
        let mut data = Vec::with_capacity((dim[0] * dim[1]) as usize);
        for px in img.pixels() {
            data.push(px.0.map(|v| UNorm8(v)));
        }
        Texture::from_parts(dim, data)
    }
}
impl Into<image::RgbaImage> for Texture<[pixel::UNorm8; 4], 2> {
    fn into(self) -> image::RgbaImage {
        let [width, height] = self.dim();
        let mut img = image::RgbaImage::new(width, height);

        let mut cur_x = 0;
        let mut cur_y = 0;
        for px in self.data().iter() {
            img.put_pixel(cur_x, cur_y, image::Rgba(px.map(|v| v.0)));

            cur_x += 1;
            if cur_x >= width {
                cur_x = 0;
                cur_y += 1;
            }
        }
        img
    }
}
impl Texture<[pixel::UNorm8; 4], 2> {
    pub fn load_linear_rgba<P: AsRef<std::path::Path>>(path: P) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgba8();
        Ok(Self::from(img))
    }

    pub fn load_bytes_linear_rgba(bytes: &[u8]) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?.to_rgba8();
        Ok(Self::from(img))
    }

    pub fn save_linear_rgba<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), image::ImageError> {
        let img: image::RgbaImage = self.clone().into();
        img.save(path)
    }

    pub fn save_srgba<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let data = self
            .data()
            .iter()
            .map(|px| pixel::Srgba::from_linear_rgba(*px))
            .collect::<Vec<_>>();

        let t = Texture::from_parts(self.dim(), data);
        let img: image::RgbaImage = t.into();
        img.save(path)
    }
}
