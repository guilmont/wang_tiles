mod rgba;
mod texture;

use rgba::RGBA;
use texture::Texture;


fn wang_shader(frag: &mut RGBA, u: f32, v: f32, id: u8) {
    let bot = v < 0.0 && (v.abs() > u.abs());
    let rgt = u > 0.0 && (v.abs() < u.abs());
    let top = v > 0.0 && (v.abs() > u.abs());
    let lft = u < 0.0 && (v.abs() < u.abs());

    let coeff = (((id >> 0) & 1u8) & bot as u8)
              | (((id >> 1) & 1u8) & rgt as u8)
              | (((id >> 2) & 1u8) & top as u8)
              | (((id >> 3) & 1u8) & lft as u8);

    *frag = RGBA::lerp(&RGBA::red(), &RGBA::white(), coeff as u32 as f32);
}


fn main() -> Result<(), String> {

    const TILE: usize = 64;
    const WIDTH: usize = 4 * TILE;
    const HEIGHT: usize = 4 * TILE;

    let mut tex = Texture::new(WIDTH, HEIGHT);

    tex.for_each(|(x, y), pixel: &mut RGBA| {
        let idx = x / TILE;
        let idy = y / TILE;
        let id = idy * 4 + idx;
        // Get coordinates from 0.0 to 1.0
        let u = (x % TILE) as f32 / TILE as f32;
        let v = (y % TILE) as f32 / TILE as f32;
        // Re-center coordinates and scale
        let u = 2.0 * u - 1.0;
        let v = 1.0 - 2.0 * v;
        wang_shader(pixel, u, v, id as u8);
    });

    tex.save_ppm("output.ppm")?;

    Ok(())
}
