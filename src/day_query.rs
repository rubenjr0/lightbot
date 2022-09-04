use std::fmt::{Display, Error};

use serde::Deserialize;

use crate::{
    endpoint::Endpoint,
    price_query::{get_price, PriceQuery},
};

#[derive(Debug, Deserialize)]
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
}

impl Display for DayQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_starting_time, min_ending_time) = self.min.hour().ok_or(Error)?;
        let (max_starting_time, max_ending_time) = self.max.hour().ok_or(Error)?;
        write!(f, "Precio más bajo: {:.3} € por kWh, de {:2} a {:2}\nPrecio más alto: {:.3} € por kWh, de {:2} a {:2}\nPrecio medio: {} € por kWh", self.min.price(), min_starting_time, min_ending_time, self.max.price(), max_starting_time, max_ending_time, self.avg.price())
    }
}
