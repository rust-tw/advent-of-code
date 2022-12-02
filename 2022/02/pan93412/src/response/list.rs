use std::ops::Deref;

use crate::tokenizer::TheirOurTable;

use super::{Point, Response};

/// 敵方 vs 我方的 [Response] 對決
pub struct ResponseList(pub Vec<(Response, Response)>);

impl ResponseList {
    /// 計算輸入資料的點數
    pub fn point(&self) -> Point {
        self.iter()
            .map(|(their, our)| {
                let our_strategy_point = our.point();
                let beatable_point = their.beats(our).point();

                our_strategy_point + beatable_point
            })
            .sum()
    }
}

impl TryFrom<&TheirOurTable> for ResponseList {
    type Error = anyhow::Error;

    fn try_from(table: &TheirOurTable) -> Result<Self, Self::Error> {
        let data = table
            .iter()
            .map(|(their, our)| {
                let their = Response::try_from(*their)?;
                let our = Response::try_from(*our)?;
                Ok::<_, anyhow::Error>((their, our))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Self(data?))
    }
}

impl Deref for ResponseList {
    type Target = Vec<(Response, Response)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
