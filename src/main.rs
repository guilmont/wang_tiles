mod rgba;
mod texture;

use rgba::RGBA;
use texture::Texture;


fn circle_shader(frag: &mut RGBA, u: f32, v: f32) {
    *frag = RGBA::lerp(&RGBA::red(), &RGBA::white(), ((u*u + v*v) < 0.5 * 0.5) as u32 as f32);

}


fn main() -> Result<(), String> {

    const WIDTH: usize = 512;
    const HEIGHT: usize = 256;
    const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

    let mut tex = Texture::new(WIDTH, HEIGHT);

    tex.for_each(|(x, y), pixel: &mut RGBA| {
        let v = 2.0 * y as f32 / HEIGHT as f32 - 1.0;
        let u = (2.0 * x as f32 / WIDTH as f32  - 1.0) * ASPECT_RATIO;
        circle_shader(pixel, u, v);
    });

    tex.save_ppm("output.ppm")?;

    Ok(())
}
