//! This crate provides a simple api to get a subset of data from [boardgamegeek.com](https://boardgamegeek.com),
//! using the [BGG XML API2](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
//! The data is limited to a small number of fields for my use case.
//!
//! # Basic Usage
//! ```no_run
//! # use bgg_lib::{error, BggClient};
//! # fn main() -> error::Result<()>{
//!     let client = BggClient::new();
//!     let games = client.get_all_games_for_user("unixgeek", false)?;
//!     # Ok(())
//! # }
//! ```
mod collection;
pub mod error;
mod request;
mod thing;

pub use crate::collection::Item as CollectionItem;
use crate::request::RequestResult;
pub use crate::thing::Game;
use log::debug;
use ureq::http::{HeaderMap, HeaderValue, StatusCode};
use ureq::Agent;

// bgg says max is 20.
const MAX_IDS: u8 = 20;

/// Client for the [BGG XML API2](https://boardgamegeek.com/wiki/page/BGG_XML_API2)
pub struct BggClient {
    agent: Agent,
    url: String,
}

impl BggClient {
    /// Creates a [BggClient] with the specified URL as the base.
    ///
    /// [Self::default] uses <https://boardgamegeek.com>. This might be useful if using one of the
    /// other available apis or local testing.
    /// Other apis:
    /// * <https://rpggeek.com>
    /// * <https://videogamegeek.com>
    pub fn from_url(url: &str) -> Self {
        let agent = Agent::config_builder()
            .http_status_as_error(false)
            .user_agent(format!(
                "{} {} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY")
            ))
            .build()
            .into();
        Self {
            agent,
            url: url.to_owned(),
        }
    }

    /// Creates a [BggClient].
    pub fn new() -> Self {
        Self::from_url("https://boardgamegeek.com")
    }

    /// Get a user's collection.
    ///
    /// Calls `/collection` with `brief=1` and `subtype=boardgame`.
    pub fn get_collection(
        &self,
        user: &str,
        include_expansions: bool,
    ) -> error::Result<Vec<CollectionItem>> {
        let exclude_param = if include_expansions {
            ""
        } else {
            "&excludesubtype=boardgameexpansion"
        };

        let url = format!(
            "{base}/xmlapi2/collection?username={user}&own=1&brief=1&subtype=boardgame{exclude_param}",
            base = self.url
        );

        request::do_request(|| {
            let mut response = self.agent.get(&url).call()?;
            log_headers(response.headers());

            let status_code = response.status();
            match status_code {
                StatusCode::OK => {
                    let xml = response.body_mut().read_to_string()?;
                    Ok(RequestResult::Done(collection::from_xml(&xml)))
                }
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })?
    }

    /// Get games.
    ///
    /// Calls `/thing`.
    /// Note that [Self::get_collection] is limited to the `boardgame` subtype, but this is not.
    /// `Thing`s that are not boardgames have not been tested.
    pub fn get_games(&self, ids: &[u32]) -> error::Result<Vec<Game>> {
        let mut games = Vec::new();
        let total = ids.len();
        let mut count = 0;

        for chunk in ids.chunks(MAX_IDS as usize) {
            count += chunk.len();
            debug!("Getting games ({} / {})", count, total);
            games.extend(self.get_games_from_api(chunk)?);
        }

        Ok(games)
    }

    /// Get all games for a user.
    ///
    /// This basically just calls [Self::get_collection] and [Self::get_games].
    pub fn get_all_games_for_user(
        &self,
        user: &str,
        include_expansions: bool,
    ) -> error::Result<Vec<Game>> {
        let ids = self
            .get_collection(user, include_expansions)?
            .into_iter()
            .map(|item| item.id)
            .collect::<Vec<u32>>();

        self.get_games(&ids)
    }

    fn get_games_from_api(&self, ids: &[u32]) -> error::Result<Vec<Game>> {
        let ids_as_strings = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>();

        request::do_request(|| {
            let ids_string = ids_as_strings.join(",");
            let mut response = self
                .agent
                .get(&format!(
                    "{base}/xmlapi2/thing?id={ids_string}&stats=1",
                    base = self.url
                ))
                .call()?;

            log_headers(response.headers());

            let status_code = response.status();
            match response.status() {
                StatusCode::OK => Ok(RequestResult::Done(thing::from_xml(
                    &response.body_mut().read_to_string()?,
                )?)),
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })
    }
}

#[cfg(feature = "moar-debug")]
fn log_headers(headers: &HeaderMap<HeaderValue>) {
    headers.iter().for_each(|(name, value)| {
        debug!(
            "HEADER FOR RESEARCH {}: {}",
            name,
            value.to_str().unwrap_or("not a string")
        );
    });
}

#[cfg(not(feature = "moar-debug"))]
fn log_headers(_: &HeaderMap<HeaderValue>) {}

impl Default for BggClient {
    fn default() -> Self {
        Self::new()
    }
}
