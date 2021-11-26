use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub title: String,
    pub model: String,
    pub image: String,
}
