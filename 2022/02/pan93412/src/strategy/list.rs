use std::ops::Deref;

use crate::tokenizer::TheirOurTable;

use super::{Response, Strategy};

/// 敵方之 [Response]，及我方之 [Strategy] 之對映
pub struct ResponseStrategyMap(pub Vec<(Response, Strategy)>);

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
