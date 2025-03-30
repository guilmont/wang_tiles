#[derive(Debug)]
pub struct Matrix<TP> {
    pub width: usize,
    pub height: usize,
    data: Vec<TP>,
}

impl<TP> Matrix<TP> {
    pub fn new(width: usize, height: usize, value: &TP) -> Matrix<TP>
    where TP: Clone {
        return Matrix { width, height, data: vec![value.clone(); width * height] };
    }

    pub fn at(&self, x: usize, y: usize) -> &TP {
        return &self.data[y * self.width + x];
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut TP {
        return &mut self.data[y * self.width + x];
    }

    pub fn for_each(&self, mut func: impl FnMut((usize, usize), &TP)) {
        for k in 0..self.height {
            for l in 0..self.width {
                func((l, k), &self.at(l, k));
            }
        }
    }

    pub fn for_each_mut(&mut self, mut func: impl FnMut((usize, usize), &mut TP)) {
        for k in 0..self.height {
            for l in 0..self.width {
                func((l, k), &mut self.at_mut(l, k));
            }
        }
    }
}

impl<TP: std::fmt::Display> Matrix<TP> {
    pub fn print(&self) {
        for k in 0..self.height {
            for l in 0..self.width {
                print!("{} ", self.at(l, k));
            }
            println!();
        }
    }

}
