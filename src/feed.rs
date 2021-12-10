use atom_syndication::Feed;
use std::error::Error;

pub fn get_atom_feed(beta_gouv_jobs_feed: &str) -> Result<Feed, Box<dyn Error>> {
    let url = beta_gouv_jobs_feed;

    let res = reqwest::blocking::get(url)?.bytes()?;

    let feed = Feed::read_from(&res[..])?;

    Ok(feed)
}
