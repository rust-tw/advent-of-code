#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub face: i32,
    pub right: i32,
    pub down: i32,
    pub left: i32,
    pub up: i32,
    pub hidden: i32,
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            face: 0,
            right: 1,
            down: 2,
            left: 3,
            up: 4,
            hidden: 5,
        }
    }
}

impl Cube {
    pub fn get_face(&self) -> i32 {
        self.face
    }

    pub fn get_surround(&self) -> [usize; 4] {
        [
            self.right as usize,
            self.down as usize,
            self.left as usize,
            self.up as usize,
        ]
    }

    pub fn turn(&self, dir: i32) -> Self {
        let dir = dir.rem_euclid(4);
        match dir {
            0 => self.turn_right(),
            1 => self.turn_down(),
            2 => self.turn_left(),
            3 => self.turn_up(),
            _ => panic!("Impossible."),
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            right: self.face,
            face: self.left,
            left: self.hidden,
            hidden: self.right,
            ..*self
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            left: self.face,
            face: self.right,
            right: self.hidden,
            hidden: self.left,
            ..*self
        }
    }

    fn turn_up(&self) -> Self {
        Self {
            up: self.face,
            face: self.down,
            down: self.hidden,
            hidden: self.up,
            ..*self
        }
    }

    fn turn_down(&self) -> Self {
        Self {
            down: self.face,
            face: self.up,
            up: self.hidden,
            hidden: self.down,
            ..*self
        }
    }
}
