pub struct Tower {
    levels: Vec<u8>,
    height: usize,
}

const BUF_LENGTH: usize = 1000;

impl Tower {
    pub fn new() -> Self {
        Tower {
            levels: vec![0; BUF_LENGTH],
            height: 0,
        }
    }

    pub fn get_level(&self, y: usize) -> u8 {
        self.levels[y % BUF_LENGTH]
    }

    pub fn test_xy(&self, x: usize, y: usize) -> bool {
        self.levels[y % BUF_LENGTH] & (1 << x) != 0
    }

    pub fn set_grids(&mut self, grids: &Vec<(i64, i64)>) {
        for &(x, y) in grids {
            let adj_y = (y as usize) % BUF_LENGTH;
            self.levels[adj_y] = self.levels[adj_y] | (1 << x);
            self.height = self.height.max(y as usize + 1);
        }

        for i in 1..=8 {
            self.levels[(self.height + i) % BUF_LENGTH] = 0;
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
