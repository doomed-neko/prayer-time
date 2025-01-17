use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "fail")]
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoInfo {
    pub lat: f64,
    pub lon: f64,
}
impl GeoInfo {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let url = "http://ip-api.com/json/?fields=192";
        let req = reqwest::get(url).await?;
        let parsed = req.json::<GeoInfo>().await?;
        Ok(parsed)
    }
}
