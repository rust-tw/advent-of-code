use super::tower::Tower;

#[derive(Debug)]
pub struct Rock {
    grids: Vec<(i64, i64)>,
}

impl Rock {
    pub fn new(rock_type: usize, y: i64) -> Self {
        let grids = match rock_type % 5 {
            0 => vec![(2, y), (3, y), (4, y), (5, y)],
            1 => vec![(2, y + 1), (3, y + 2), (4, y + 1), (3, y)],
            2 => vec![(2, y), (3, y), (4, y), (4, y + 2), (4, y + 1)],
            3 => vec![(2, y + 3), (2, y + 2), (2, y + 1), (2, y)],
            4 => vec![(2, y + 1), (3, y + 1), (2, y), (3, y)],
            _ => panic!("Why?"),
        };
        Self { grids }
    }

    pub fn move_right(&mut self, tower: &Tower) {
        let is_valid = self
            .grids
            .iter()
            .map(|(x, y)| (x + 1, *y))
            .all(|(x, y)| x < 7 && !tower.test_xy(x as usize, y as usize));

        if is_valid {
            self.grids.iter_mut().for_each(|(x, _)| *x += 1);
        }
    }

    pub fn move_left(&mut self, tower: &Tower) {
        let is_valid = self
            .grids
            .iter()
            .map(|(x, y)| (x - 1, *y))
            .all(|(x, y)| x >= 0 && !tower.test_xy(x as usize, y as usize));

        if is_valid {
            self.grids.iter_mut().for_each(|(x, _)| *x -= 1);
        }
    }

    pub fn move_down(&mut self, tower: &mut Tower) -> bool {
        let is_valid = self
            .grids
            .iter()
            .map(|(x, y)| (*x, y - 1))
            .all(|(x, y)| y >= 0 && !tower.test_xy(x as usize, y as usize));
        if is_valid {
            self.grids.iter_mut().for_each(|(_, y)| *y -= 1);
        } else {
            tower.set_grids(&self.grids);
        }
        is_valid
    }

    #[allow(dead_code)]
    pub fn draw(&self, tower: &Tower) {
        let max_y = self.grids.iter().map(|(_, y)| *y).max().unwrap();
        for y in ((max_y - 8)..=max_y).rev() {
            let mut s = String::from("|");
            for x in 0..7 {
                if self
                    .grids
                    .iter()
                    .any(|&(grid_x, grid_y)| x == grid_x && y == grid_y)
                {
                    s.push('@');
                } else if tower.test_xy(x as usize, y as usize) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('|');
            println!("{}", s);
        }
        println!("+-------+\n");
    }
}
