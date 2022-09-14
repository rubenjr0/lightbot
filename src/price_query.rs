use std::fmt::{Display, Error};

use serde::Deserialize;

use crate::endpoint::Endpoint;

pub async fn get_price(endpoint: Endpoint) -> Result<PriceQuery, reqwest::Error> {
    let res = reqwest::get(endpoint.get_endpoint()).await?;
    let query = res.json::<PriceQuery>().await?;
    Ok(query)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PriceQuery {
    date: String,
    price: f32,
    is_cheap: Option<bool>,
    is_under_avg: Option<bool>,
    hour: Option<String>,
}

impl PriceQuery {
    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn price(&self) -> f32 {
        self.price / 1000.0
    }

    pub fn hour(&self) -> Option<(u8, u8)> {
        let hours: Vec<u8> = self
            .hour
            .as_ref()?
            .split('-')
            .map(|h| h.parse().unwrap())
            .collect();
        Some((hours[0], hours[1]))
    }
}

impl Display for PriceQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_cheap = self.is_cheap.ok_or(Error)?;
        let is_under_avg = self.is_under_avg.ok_or(std::fmt::Error)?;
        write!(
            f,
            "{:.3}€, {}",
            self.price(),
            if is_cheap {
                "barata"
            } else if is_under_avg {
                "precio medio"
            } else {
                "cara"
            }
        )
    }
}
