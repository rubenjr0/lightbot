pub enum Endpoint {
    Now,
    Min,
    Max,
    Avg,
}

impl Endpoint {
    pub fn get_endpoint(&self) -> &str {
        match self {
            Endpoint::Now => "https://api.preciodelaluz.org/v1/prices/now?zone=PCB",
            Endpoint::Min => "https://api.preciodelaluz.org/v1/prices/min?zone=PCB",
            Endpoint::Max => "https://api.preciodelaluz.org/v1/prices/max?zone=PCB",
            Endpoint::Avg => "https://api.preciodelaluz.org/v1/prices/avg?zone=PCB",
        }
    }
}
