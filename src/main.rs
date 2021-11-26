mod structs;

extern crate base64;
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use crate::structs::Device;
use dotenv_codegen::dotenv;
use reqwest::header::AUTHORIZATION;
use std::{thread, time};

const PUSH_NOTIFIER_API: &str = "https://api.pushnotifier.de/v2";
const APP_PACKAGE_NAME: &str = dotenv!("APP_PACKAGE_NAME");
const API_TOKEN: &str = dotenv!("API_TOKEN");
const APP_TOKEN: &str = dotenv!("APP_TOKEN");

fn main() {
    let devices = get_devices();

    loop {
        println!("hehe");
        thread::sleep(time::Duration::from_millis(900000));
    }
}

fn get_devices() -> Result<Vec<Device>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let url = format!("{}/devices", PUSH_NOTIFIER_API);

    let auth_payload = format!("{}:{}", APP_PACKAGE_NAME, API_TOKEN);

    let auth_payload_as_bytes = auth_payload.as_bytes();

    let res = client
        .get(url)
        .header(
            AUTHORIZATION,
            format!("Basic {}", base64::encode(auth_payload_as_bytes)),
        )
        .header("X-AppToken", APP_TOKEN)
        .send()?;

    let devices: Vec<Device> = res.json()?;

    for device in &devices {
        println!("{}: {} / {}", device.id, device.title, device.model);
    }

    Ok(devices)
}
