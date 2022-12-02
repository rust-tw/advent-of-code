pub mod list;

/// 點數的類型
pub type Point = u32;

/// 出拳
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Response {
    /// 石頭 (1pt)
    Rock,
    /// 布 (2pt)
    Paper,
    /// 剪刀 (3pt)
    Scissors,
}

/// 結果
#[derive(Copy, Clone)]
pub enum Outcome {
    /// 贏 (6pt)
    Win,
    /// 平手 (3pt)
    Draw,
    /// 輸 (0pt)
    Lose,
}

impl TryFrom<char> for Response {
    type Error = anyhow::Error;

    /// 讀入決策 char，解析成 Response
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => anyhow::bail!("Invalid strategy: {}", value),
        })
    }
}

impl Response {
    /// 取得這個拳可以拿到的點數
    pub fn point(&self) -> Point {
        match self {
            Response::Rock => 1,
            Response::Paper => 2,
            Response::Scissors => 3,
        }
    }

    /// 假設目前的 self 是對方，
    /// 我們出的拳會得到什麼樣的結果？
    pub fn beats(&self, our: &Response) -> Outcome {
        use Outcome::*;
        use Response::*;

        // 如果出拳相同，肯定是平手。
        if self == our {
            return Draw;
        }

        match (self, our) {
            (Rock, Paper) => Win,
            (Rock, Scissors) => Lose,
            (Paper, Rock) => Lose,
            (Paper, Scissors) => Win,
            (Scissors, Rock) => Win,
            (Scissors, Paper) => Lose,
            _ => unreachable!(),
        }
    }
}

impl Outcome {
    /// 這個結果可以拿到的點數
    pub fn point(&self) -> Point {
        use Outcome::*;

        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}
