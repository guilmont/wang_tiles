#![allow(dead_code)]

mod rgba;
mod texture;

use rgba::RGBA;
use texture::Texture;

// Common bit setup for tiles
const BOTTOM: u8 = 0;
const RIGHT: u8 = 1;
const TOP: u8 = 2;
const LEFT: u8 = 3;


struct Matrix {
    cols: usize,
    rows: usize,
    data: Vec<u8>,
}

impl Matrix {
    fn new(num_cols: usize, num_rows: usize) -> Matrix {
        let mut data = Vec::<u8>:: new();
        data.resize(num_cols * num_rows, 0u8);
        return Matrix {
            cols: num_cols,
            rows: num_rows,
            data
        };
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        return &mut self.data[y * self.cols + x];
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        return self.data[y * self.cols + x];
    }

    fn print(&self) {
        for k in 0..self.rows {
            for l in 0..self.cols {
                print!("{:04b} ", self.at(l, k));
            }
            println!();
        }
    }
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
    let mut tex = Texture::new(width, height);
    tex.for_each(|(x, y), pixel: &mut RGBA| {
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
    tex.save_ppm(filepath)?;

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
    let mut grid = Matrix::new(WIDTH, HEIGHT);
    // First element is completely random
    *grid.at_mut(0,0) = rand::random::<u8>() % 16;
    // First column
    for l in 1..grid.cols {
        let rng = rand::random::<u8>() % 16;
        *grid.at_mut(l,0) = (rng & !(1 << LEFT))
                          | (((grid.at(l-1, 0) >> RIGHT) & 1) << LEFT);
    }
    // Go thru other rows
    for k in 1..grid.rows {
        // The first element depends only on top element
        let rng = rand::random::<u8>() % 16;
        *grid.at_mut(0,k) = (rng & !(1 << TOP))
                          | (((grid.at(0, k-1) >> BOTTOM) & 1) << TOP);
        // Bulk
        for l in 1..grid.cols {
            let rnd = rand::random::<u8>() % 16;
            *grid.at_mut(l,k) = (rnd & !((1 << TOP) | (1 << LEFT)))
                              | (((grid.at(l, k-1) >> BOTTOM) & 1) << TOP)
                              | (((grid.at(l-1, k) >> RIGHT) & 1) << LEFT);
        }
    }

    let mut tex = Texture::new(grid.cols * TILE, grid.rows * TILE);
    tex.for_each(|(x,y), pixel: &mut RGBA| {
        // if x % TILE == 0 || y % TILE == 0 {
        //     *pixel = RGBA::black();
        //     return;
        // }
        // Get id from grid
        let id = grid.at(x / TILE, y / TILE);
        // Get coordinates from 0.0 to 1.0
        let u = (x % TILE) as f32 / TILE as f32;
        let v = (y % TILE) as f32 / TILE as f32;
        // Re-center coordinates and scale
        let u = 2.0 * u - 1.0;
        let v = 1.0 - 2.0 * v;
        wang_shader(pixel, u, v, id);
    });
    tex.save_ppm("output.ppm")?;

    Ok(())
}
