use crate::rgba::RGBA;

#[derive(Debug)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    data: Vec<RGBA>,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Texture {
        let mut data = Vec::<RGBA>::new();
        data.resize_with((width * height) as usize, || RGBA::black());
        return Texture { width, height, data };
    }

    pub fn at(&mut self, x: usize, y: usize) -> &mut RGBA {
        return &mut self.data[y * self.width + x];
    }

    pub fn save_ppm(&self, filepath: &str) -> Result<(), String> {
        use std::io::Write;

        let arq = std::fs::File::create(filepath).map_err(|e| e.to_string())?;
        let mut buf = std::io::BufWriter::new(arq);

        buf.write_all(format!("P6\n{} {} 255\n", self.width, self.height).as_bytes())
            .map_err(|e| e.to_string())?;

        for c in self.data.iter() {
            let r = (c.r * 255.0) as u8;
            let g = (c.g * 255.0) as u8;
            let b = (c.b * 255.0) as u8;
            buf.write_all(&[r,g, b]).map_err(|e| e.to_string())?;
        }

        buf.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn for_each(&mut self, mut func: impl FnMut((usize, usize), &mut RGBA)) {
        for k in 0..self.height {
            for l in 0..self.width {
                func((l, k), &mut self.at(l, k));
            }
        }

    }
}
