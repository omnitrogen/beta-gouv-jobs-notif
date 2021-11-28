mod database;
mod feed;
mod notifier;
mod structs;

extern crate dotenv_codegen;
extern crate log;

use dotenv_codegen::dotenv;
use log::info;
use std::{thread, time};

const API_TOKEN: &str = dotenv!("API_TOKEN");
const APP_PACKAGE_NAME: &str = dotenv!("APP_PACKAGE_NAME");
const APP_TOKEN: &str = dotenv!("APP_TOKEN");
const BETA_GOUV_JOBS_FEED: &str = "https://beta.gouv.fr/jobs.xml";
const DB_NAME: &str = "job_entries.db";
const PUSH_NOTIFIER_API: &str = "https://api.pushnotifier.de/v2";

fn main() {
    // init env logger
    env_logger::init();

    // create `job_entries.db` database if it doesn't exist, as well as the `jobs` table
    let conn = database::init_db(DB_NAME).expect("Could not initiate the database.");

    // create PushNotifier API client
    let client = notifier::create_client(APP_PACKAGE_NAME, API_TOKEN, APP_TOKEN)
        .expect("Error during API client creation.");

    // fetch a list of devices ids that you added on your PushNotifier account
    let devices_ids = notifier::get_devices(&client, PUSH_NOTIFIER_API)
        .expect("Error fetching PushNotifier devices");

    // on the first run, the database is empty
    let mut first_run = database::is_table_empty(&conn)
        .expect("Can't determine whether the table is empty or not.");

    // executes every 15 minutes
    loop {
        // fetch the beta.gouv atom feed
        let feed =
            feed::get_atom_feed(BETA_GOUV_JOBS_FEED).expect("Unable to get beta.gouv Atom feed.");

        // iterate over entries starting with the most recent one
        for entry in feed.entries.into_iter() {
            if database::exists(&conn, &entry)
                .expect("Error checking if the entry exists in database.")
            {
                // if the most recent one is already in the database, do nothing
                break;
            } else {
                // else insert the latest entry in the database
                database::insert(&conn, &entry).expect("Couldn't insert new entry in database.");
                info!("Inserting new entry '{}' to database.", entry.id);
                if !first_run {
                    // push notification if it isn't the first run
                    // (we do not want a notification for every entry on the first run)
                    notifier::push_notification(&client, PUSH_NOTIFIER_API, &entry, &devices_ids)
                        .expect("Couldn't push the notification.");
                    info!("Pushing notification '{}'.", entry.id);
                }
            }
        }
        first_run = false;
        thread::sleep(time::Duration::from_millis(1000 * 60 * 15));
    }
}
