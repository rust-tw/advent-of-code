use std::ops::Deref;

use crate::{response::list::ResponseList, tokenizer::TheirOurTable};

use super::{Response, ResponseStrategyExt, Strategy};

/// 敵方之 [Response]，及我方之 [Strategy] 之對映
pub struct ResponseStrategyMap(pub Vec<(Response, Strategy)>);

impl ResponseStrategyMap {
    /// 將輸入依照第二題題意尋找最優解答。
    pub fn find_best_solution(&self) -> ResponseList {
        ResponseList(
            self.iter()
                .map(|(their, our_strategy)| {
                    let our_response = their.find_best_response(*our_strategy);

                    (*their, our_response)
                })
                .collect(),
        )
    }
}
impl TryFrom<&TheirOurTable> for ResponseStrategyMap {
    type Error = anyhow::Error;

    fn try_from(table: &TheirOurTable) -> Result<Self, Self::Error> {
        let data = table
            .iter()
            .map(|(their, our)| {
                let their = Response::try_from(*their)?;
                let our = Strategy::try_from(*our)?;
                Ok::<_, anyhow::Error>((their, our))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Self(data?))
    }
}

impl Deref for ResponseStrategyMap {
    type Target = Vec<(Response, Strategy)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
