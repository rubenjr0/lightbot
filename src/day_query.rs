use std::fmt::{Display, Error};

use serde::Deserialize;

use crate::{
    endpoint::Endpoint,
    price_query::{get_price, PriceQuery},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DayQuery {
    min: PriceQuery,
    avg: PriceQuery,
    max: PriceQuery,
}

impl DayQuery {
    pub async fn new() -> Result<DayQuery, reqwest::Error> {
        let min = get_price(Endpoint::Min).await?;
        let max = get_price(Endpoint::Max).await?;
        let avg = get_price(Endpoint::Avg).await?;
        Ok(DayQuery { min, avg, max })
    }

    pub fn date(&self) -> &str {
       self.avg.date()
    }
}

impl Display for DayQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = self.avg.date();
        let mnp = self.min.price();
        let mxp = self.max.price();
        let avp = self.avg.price();
        let (mnst, mnet) = self.min.hour().ok_or(Error)?;
        let (mxst, mxet) = self.max.hour().ok_or(Error)?;
        write!(f, "Precio de la luz a {date}\nMínimo: {mnp:.3}€, de {mnst:02} a {mnet:02}\nMáximo: {mxp:.3}€, de {mxst:02} a {mxet:02}\nMedia: {avp:.3}€")
    }
}
