extern crate atom_syndication;
extern crate base64;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use crate::structs::Device;
use atom_syndication::Entry;
use reqwest::{header, Result};
use serde_json::json;

pub fn create_client(
    app_package_name: &str,
    api_token: &str,
    app_token: &'static str,
) -> Result<reqwest::blocking::Client> {
    let auth_payload = format!("{}:{}", app_package_name, api_token);

    let auth_payload_as_bytes = auth_payload.as_bytes();

    let mut headers = header::HeaderMap::new();

    let auth_header = format!("Basic {}", base64::encode(auth_payload_as_bytes));

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth_header).unwrap(),
    );

    headers.insert("X-AppToken", header::HeaderValue::from_static(app_token));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}

pub fn get_devices(
    client: &reqwest::blocking::Client,
    push_notifier_api: &str,
) -> Result<Vec<String>> {
    let url = format!("{}/devices", push_notifier_api);

    let res = client.get(url).send()?;

    let devices: Vec<Device> = res.json()?;

    let devices_ids: Vec<String> = devices.into_iter().map(|x| x.id).collect();

    Ok(devices_ids)
}

pub fn push_notification(
    client: &reqwest::blocking::Client,
    push_notifier_api: &str,
    entry: &Entry,
    devices_ids: &[String],
) -> Result<()> {
    let url = format!("{}/notifications/notification", push_notifier_api);

    let payload = json!({
        "devices": devices_ids,
        "content": "There is a new offer available at beta.gouv! Check it out!",
        "url": entry.id,
        "silent": false
    });

    client.put(url).json(&payload).send()?;

    Ok(())
}
