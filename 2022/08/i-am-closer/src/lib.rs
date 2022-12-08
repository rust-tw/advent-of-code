pub fn challenge_08(lines: Vec<&str>) -> (usize, i32) {
    let mut grids = Grids::create(lines);

    let part1 = (0..grids.height)
        .map(|y| {
            (0..grids.width)
                .map(|x| grids.is_visable(x, y))
                .sum::<usize>()
        })
        .sum();

    let part2 = grids
        .compute_scenic_score()
        .into_iter()
        .map(|row| {
            row.into_iter().max().unwrap()
        })
        .max()
        .unwrap();

    (part1, part2)
}

struct Grids {
    width: usize,
    height: usize,
    values: Vec<Vec<i8>>,
    max_up: Vec<Vec<Option<i8>>>,
    max_down: Vec<Vec<Option<i8>>>,
    max_left: Vec<Vec<Option<i8>>>,
    max_right: Vec<Vec<Option<i8>>>,
}

impl Grids {
    fn create(lines: Vec<&str>) -> Self {
        let values = lines.into_iter()
        .map(|s| {
            s.as_bytes().into_iter().map(|&c| (c - ('0' as u8)) as i8).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        let width = values[0].len();
        let height = values.len();

        let mut max_up = vec![vec![None; width]; height];
        for i in 0..width {
            max_up[0][i] = Some(-1i8);
        }

        let mut max_down = vec![vec![None; width]; height];
        for i in 0..width {
            max_down[height - 1][i] = Some(-1i8);
        }

        let mut max_right = vec![vec![None; width]; height];
        for i in 0..height {
            max_right[i][width - 1] = Some(-1i8);
        }

        let mut max_left = vec![vec![None; width]; height];
        for i in 0..height {
            max_left[i][0] = Some(-1i8);
        }

        Self {
            width,
            height,
            values,
            max_up,
            max_down,
            max_left,
            max_right,
        }
    }

    /*
      利用 DP 的方式找到 (x, y) 點的上、下、左、右方向最高的樹有多高。
      想法：假設想找 (x, y) 點往上 (y 往負方向走) 最高的樹的高度，
            只要比對 (x, y - 1) 記錄的最高的數的高度，再跟 (x, y - 1) 本身比對取高值即可。
     */

    fn get_max_generic<F>(
        x: usize,
        y: usize,
        next_xy: &F,
        values: &Vec<Vec<i8>>,
        maxes: &mut Vec<Vec<Option<i8>>>,
    ) -> i8
    where
        F: Fn(usize, usize) -> (usize, usize)
    {
        if let None = maxes[y][x] {
            // next_xy() 可以計算往「想要看的方向」看過去的下一點為何。
            let (next_x, next_y) = next_xy(x, y);
            // 取得 next_xy(x, y) 記錄的最高的數的高度
            let cand = Self::get_max_generic(next_x, next_y, next_xy, values, maxes);
            // 跟 next_xy(x, y) 本身的高度比對取高值，即為 (x, y) 本身往「想看的方向」看去最高的高度
            maxes[y][x] = Some(std::cmp::max(cand, values[next_y][next_x]));
        }
        maxes[y][x].unwrap()
    }
    fn get_up_max(&mut self, x: usize, y: usize) -> i8 {
        Self::get_max_generic(x, y, &(|x, y| (x, y - 1)), &self.values, &mut self.max_up)
    }

    fn get_down_max(&mut self, x: usize, y: usize) -> i8 {
        Self::get_max_generic(x, y, &(|x, y| (x, y + 1)), &self.values, &mut self.max_down)
    }

    fn get_left_max(&mut self, x: usize, y: usize) -> i8 {
        Self::get_max_generic(x, y, &(|x, y| (x - 1, y)), &self.values, &mut self.max_left)
    }

    fn get_right_max(&mut self, x: usize, y: usize) -> i8 {
        Self::get_max_generic(x, y, &(|x, y| (x + 1, y)), &self.values, &mut self.max_right)
    }

    fn is_visable(&mut self, x: usize, y: usize) -> usize {
        let value = self.values[y][x];
        if value > self.get_up_max(x, y) || value > self.get_down_max(x, y) ||
            value > self.get_left_max(x, y) || value > self.get_right_max(x, y) {
            1
        } else {
            0
        }
    }

    /*
      針對每個方向的一整行，逐一計算離「最近那顆」「高度 >= 自己高度」的樹，
      距離有多少。
      想法：利用一個長度為 10 的 array (`last_seen`) ，記錄上次看到高度為 n
            (n = 0~9) 的樹是在什麼位置（-1 代表沒有出現過）。這樣走到每個點時，
            就可以檢查 `last_seen` index >= 自身高度中、哪一個是離自身最接近
            的，即為這個方向中可以看到的樹的數量。
     */

    fn compute_scenic_score(&self) -> Vec<Vec<i32>> {
        let mut count_left = vec![vec![-1; self.width]; self.height];
        for y in 0..self.height {
            let mut last_seen = [-1; 10];   // 高度為 index 的樹，最後一次出現的位置
            for x in 0..self.width {
                let value = self.values[y][x] as usize;
                // 找出高度 >= 自己的樹中、最接近自己的位置
                let stopper_idx =
                    last_seen[value..]  // 只取 index >= 自己高度的部份
                    .iter()
                    .cloned()
                    .filter(|i| *i != -1)   // 去掉 -1，表示從未出現過
                    .max()  // 行進方向是 x 遞增，因此最大的就表示最接近的
                    .unwrap_or(0);  // None 表示統統 filter 掉了，表示沒有阻礙

                count_left[y][x] = (x as i32) - stopper_idx; // 可看到的數量為自身所在的 index - stopper index
                last_seen[value] = x as i32;    // 更新自身高度最後出現的位置
            }
        }

        let mut count_right = vec![vec![-1; self.width]; self.height];
        for y in 0..self.height {
            let mut last_seen = [-1; 10];
            for x in (0..self.width).rev() {    // 倒著數
                let value = self.values[y][x] as usize;
                let stopper_idx = last_seen[value..]
                    .iter()
                    .cloned()
                    .filter(|i| *i != -1)
                    .min()  // 要注意現在行進方向是反的，因此要取 min
                    .unwrap_or((self.width as i32) - 1);
                count_right[y][x] = stopper_idx - (x as i32);
                last_seen[value] = x as i32;
            }
        }

        let mut count_up = vec![vec![-1; self.width]; self.height];
        for x in 0..self.width {
            let mut last_seen = [-1; 10];
            for y in 0..self.height {
                let value = self.values[y][x] as usize;
                let stopper_idx = last_seen[value..]
                    .iter()
                    .cloned()
                    .filter(|i| *i != -1)
                    .max()
                    .unwrap_or(0);
                count_up[y][x] = (y as i32) - stopper_idx;
                last_seen[value] = y as i32;
            }
        }

        let mut count_down = vec![vec![-1; self.width]; self.height];
        for x in 0..self.width {
            let mut last_seen = [-1; 10];
            for y in (0..self.height).rev() {
                let value = self.values[y][x] as usize;
                let stopper_idx = last_seen[value..]
                    .iter()
                    .cloned()
                    .filter(|i| *i != -1)
                    .min()
                    .unwrap_or((self.width as i32) - 1);
                count_down[y][x] = stopper_idx - (y as i32);
                last_seen[value] = y as i32;
            }
        }

        (0..self.height)
            .map(|y| {
                (0..self.width)
                .map(|x|{
                    count_left[y][x] * count_right[y][x] * count_up[y][x] * count_down[y][x]
                }).collect()
            })
            .collect()
    }
}



