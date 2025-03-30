#![allow(dead_code)]

mod rgba;
mod matrix;

use rgba::RGBA;
use matrix::Matrix;

// Common bit setup for tiles
const BOTTOM: u8 = 0;
const RIGHT: u8 = 1;
const TOP: u8 = 2;
const LEFT: u8 = 3;

fn save_ppm(mat: & Matrix<RGBA>, filepath: &str) -> Result<(), String> {
    use std::io::Write;

    let arq = std::fs::File::create(filepath).map_err(|e| e.to_string())?;
    let mut buf = std::io::BufWriter::new(arq);

    buf.write_all(format!("P6\n{} {} 255\n", mat.width, mat.height).as_bytes())
        .map_err(|e| e.to_string())?;

    mat.for_each(|(_,_), pix| {
        let r = (pix.r * 255.0) as u8;
        let g = (pix.g * 255.0) as u8;
        let b = (pix.b * 255.0) as u8;
        buf.write_all(&[r,g, b]).map_err(|e| e.to_string()).unwrap();
    });

    buf.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn wang_shader(frag: &mut RGBA, u: f32, v: f32, id: u8) {
    let bot = v <= 0.0 && (v.abs() >= u.abs());
    let rgt = u >= 0.0 && (v.abs() <= u.abs());
    let top = v >= 0.0 && (v.abs() >= u.abs());
    let lft = u <= 0.0 && (v.abs() <= u.abs());

    let coeff = (((id >> BOTTOM) & 1) & bot as u8)
              | (((id >> RIGHT) & 1) & rgt as u8)
              | (((id >> TOP) & 1) & top as u8)
              | (((id >> LEFT) & 1) & lft as u8);

    *frag = RGBA::lerp(&RGBA::red(), &RGBA::white(), coeff as u32 as f32);
    frag.gamma_correct();
}

fn generate_tiles_image(tile_size: usize, filepath: &str) -> Result<(), String> {
    let width: usize = 4 * tile_size;
    let height: usize = 4 * tile_size;

    // Generate an atlas with all the tiles we will need
    let mut tex = Matrix::<RGBA>::new(width, height, &RGBA::black());
    tex.for_each_mut(|(x, y), pixel: &mut RGBA| {
        // To identify where tiles begin and end
        if x % tile_size == 0 || y % tile_size == 0 {
            *pixel = RGBA::black();
            return;
        }
        // Determine which index goes in this tile
        let idx = x / tile_size;
        let idy = y / tile_size;
        let id = idy * 4 + idx;
        // Get coordinates from 0.0 to 1.0
        let u = (x % tile_size) as f32 / tile_size as f32;
        let v = (y % tile_size) as f32 / tile_size as f32;
        // Re-center coordinates and scale
        let u = 2.0 * u - 1.0;
        let v = 1.0 - 2.0 * v;
        wang_shader(pixel, u, v, id as u8);
    });
    save_ppm(&tex, filepath)?;

    Ok(())
}

fn main() -> Result<(), String> {
    // Configurations for grid and tile sizes
    const TILE: usize = 64;
    const WIDTH: usize = 16 * 2;
    const HEIGHT: usize = 9 * 2;

    // So we can see the tiles used
    generate_tiles_image(TILE, "tiles.ppm")?;

    // Generate grid
    let mut grid = Matrix::<u8>::new(WIDTH, HEIGHT, &0u8);
    // First element is completely random
    *grid.at_mut(0,0) = rand::random::<u8>() % 16;
    // First column
    for l in 1..grid.width {
        let rng = rand::random::<u8>() % 16;
        *grid.at_mut(l,0) = (rng & !(1 << LEFT))
                          | (((grid.at(l-1, 0) >> RIGHT) & 1) << LEFT);
    }
    // Go thru other rows
    for k in 1..grid.height {
        // The first element depends only on top element
        let rng = rand::random::<u8>() % 16;
        *grid.at_mut(0,k) = (rng & !(1 << TOP))
                          | (((grid.at(0, k-1) >> BOTTOM) & 1) << TOP);
        // Bulk
        for l in 1..grid.width {
            let rnd = rand::random::<u8>() % 16;
            *grid.at_mut(l,k) = (rnd & !((1 << TOP) | (1 << LEFT)))
                              | (((grid.at(l, k-1) >> BOTTOM) & 1) << TOP)
                              | (((grid.at(l-1, k) >> RIGHT) & 1) << LEFT);
        }
    }

    let mut tex = Matrix::<RGBA>::new(WIDTH * TILE, HEIGHT * TILE, &RGBA::black());
    tex.for_each_mut(|(x,y), pixel: &mut RGBA| {
        if x % TILE == 0 || y % TILE == 0 {
            *pixel = RGBA::black();
            return;
        }
        // Get id from grid
        let id = grid.at(x / TILE, y / TILE);
        // Get coordinates from 0.0 to 1.0
        let u = (x % TILE) as f32 / TILE as f32;
        let v = (y % TILE) as f32 / TILE as f32;
        // Re-center coordinates and scale
        let u = 2.0 * u - 1.0;
        let v = 1.0 - 2.0 * v;
        wang_shader(pixel, u, v, *id);
    });
    save_ppm(&tex, "output.ppm")?;

    Ok(())
}
