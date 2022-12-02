use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub cf_api_key: String,
    pub cf_zone_id: String,
    pub cf_records: Vec<String>,
}

impl Configs {
    pub fn get() -> Self {
        Figment::new()
            .merge(Env::prefixed("KAKUSHI_"))
            .extract()
            .expect("Error loading configs")
    }

    pub fn url(&self, url: &str) -> String {
        let mut url = url.to_owned();
        if url.starts_with("/") {
            url = url.remove(0).to_string();
        }
        format!("https://api.cloudflare.com/client/v4/zones/{}/{}", self.cf_zone_id, url)
    }
}
