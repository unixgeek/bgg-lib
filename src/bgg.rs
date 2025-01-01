pub use crate::bgg::collection::Item;
use crate::bgg::request::{do_request, RequestResult};
pub use crate::bgg::thing::Game;
use crate::{PKG_NAME, PKG_VERSION};
use log::debug;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

mod collection;
pub mod error;
mod request;
mod thing;

// bgg says max is 20.
const MAX_IDS: usize = 20;

pub struct BggClient {
    client: Client,
}

impl BggClient {
    pub fn new() -> Self {
        let client = ClientBuilder::default()
            .user_agent(format!("{} {}", PKG_NAME, PKG_VERSION))
            .build()
            .unwrap();
        Self { client }
    }

    pub fn get_collection(&self, user: &str) -> error::Result<Vec<Item>> {
        let url = format!(
            "https://boardgamegeek.com/xmlapi2/collection?username={user}&own=1&brief=1&subtype=boardgame&excludesubtype=boardgameexpansion"
        );

        do_request(|| {
            let response = self.client.get(&url).send()?;
            log_headers(response.headers());

            let status_code = response.status();
            match status_code {
                StatusCode::OK => {
                    log_headers(response.headers());
                    let xml = response.text()?;
                    Ok(RequestResult::Done(collection::from_xml(&xml)))
                }
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })?
    }

    pub fn get_games(&mut self, ids: &[usize]) -> error::Result<Vec<Game>> {
        let mut games = Vec::new();
        let total = ids.len();
        let mut count = 0;

        for chunk in ids.chunks(MAX_IDS) {
            count += chunk.len();
            debug!("Getting games ({} / {})", count, total);
            games.extend(self.get_games_from_api(chunk)?);
        }

        Ok(games)
    }

    fn get_games_from_api(&self, ids: &[usize]) -> error::Result<Vec<Game>> {
        let ids_as_strings = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>();

        do_request(|| {
            let ids_string = ids_as_strings.join(",");
            let response = self
                .client
                .get(format!(
                    "https://boardgamegeek.com/xmlapi2/thing?id={ids_string}&stats=1`"
                ))
                .send()?;

            log_headers(response.headers());

            let status_code = response.status();
            match response.status() {
                StatusCode::OK => Ok(RequestResult::Done(thing::from_xml(&response.text()?)?)),
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })
    }
}

fn log_headers(headers: &HeaderMap) {
    headers.iter().for_each(|header| {
        debug!(
            "HEADER FOR RESEARCH {}: {}",
            header.0,
            header.1.to_str().unwrap()
        );
    });
}

impl Default for BggClient {
    fn default() -> Self {
        Self::new()
    }
}
