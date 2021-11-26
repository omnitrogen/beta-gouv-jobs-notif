mod structs;

extern crate atom_syndication;
extern crate base64;
extern crate dotenv_codegen;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use crate::structs::Device;
use atom_syndication::Feed;
use dotenv_codegen::dotenv;
use reqwest::header::AUTHORIZATION;
use std::{error::Error, thread, time};

const PUSH_NOTIFIER_API: &str = "https://api.pushnotifier.de/v2";
const BETA_GOUV_JOBS_FEED: &str = "https://beta.gouv.fr/jobs.xml";
const APP_PACKAGE_NAME: &str = dotenv!("APP_PACKAGE_NAME");
const API_TOKEN: &str = dotenv!("API_TOKEN");
const APP_TOKEN: &str = dotenv!("APP_TOKEN");

fn main() {
    let devices_ids = get_devices();

    match devices_ids {
        Ok(v) => {
            if v.is_empty() {
                println!("The device list is empty!")
            } else {
                let devices_ids = v;
                println!("{:?}", devices_ids);
            }
        }
        Err(e) => println!("error parsing header: {:?}", e),
    }

    let feed = get_atom_feed();

    match feed {
        Ok(v) => {
            let feed = v;
            for (i, entry) in feed.entries.into_iter().enumerate() {
                println!("entry {}: {:?}", i, entry);
            }
            return;
        }
        Err(e) => println!("error parsing feed {:?}", e),
    }

    loop {
        println!("hehe");
        thread::sleep(time::Duration::from_millis(900000));
    }
}

fn get_atom_feed() -> Result<Feed, Box<dyn Error>> {
    let url = BETA_GOUV_JOBS_FEED;

    let res = reqwest::blocking::get(url)?.bytes()?;

    let feed = Feed::read_from(&res[..])?;

    Ok(feed)
}

fn get_devices() -> Result<Vec<String>, reqwest::Error> {
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

    let devices_ids: Vec<String> = devices.into_iter().map(|x| x.id).collect();

    Ok(devices_ids)
}
