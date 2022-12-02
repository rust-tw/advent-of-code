//! 根據輸入策略計算最優解答

pub mod list;

use crate::response::Response;

/// 我們的策略
#[derive(Clone, Copy)]
pub enum Strategy {
    /// 嘗試輸掉遊戲
    DoLose,
    /// 嘗試平局
    DoDraw,
    /// 嘗試贏得遊戲
    DoWin,
}

impl TryFrom<char> for Strategy {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'X' => Self::DoLose,
            'Y' => Self::DoDraw,
            'Z' => Self::DoWin,
            _ => anyhow::bail!("Invalid strategy: {}", value),
        })
    }
}

pub trait ResponseStrategyExt: Clone {
    /// 取得可以使本值贏得遊戲（我們輸）的 [Response]。
    fn lose_game(&self) -> Self;

    /// 取得可以使本值平局的 [Response]。
    fn draw_game(&self) -> Self {
        self.clone()
    }

    /// 取得可以使本值輸掉遊戲（我們贏）的 [Response]。
    fn win_game(&self) -> Self;

    /// 根據策略 [Strategy] 尋找最優回擊。
    fn find_best_response(&self, strategy: Strategy) -> Self {
        use Strategy::*;

        match strategy {
            DoLose => self.lose_game(),
            DoDraw => self.draw_game(),
            DoWin => self.win_game(),
        }
    }
}

impl ResponseStrategyExt for Response {
    fn lose_game(&self) -> Self {
        use Response::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn win_game(&self) -> Self {
        use Response::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}
