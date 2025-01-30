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
pub mod options;
mod request;
mod thing;

pub use crate::collection::Item as CollectionItem;
use crate::options::collections::{CollectionOptions, CollectionOptionsBuilder, Filter};
use crate::request::RequestResult;
pub use crate::thing::Game;
use log::debug;
use ureq::{Agent, AgentBuilder, Response};

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
        let agent = AgentBuilder::new()
            .user_agent(&format!(
                "{} {} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY")
            ))
            .build();
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
    pub fn collection(&self, options: CollectionOptions) -> error::Result<Vec<CollectionItem>> {
        let url = format!(
            "{base}/xmlapi2/collection{params}",
            base = self.url,
            params = options.as_url_params()
        );

        request::do_request(|| {
            let response = request::request_with_all_status_codes(self.agent.get(&url))?;
            log_headers(&response);

            let status_code = response.status();
            match status_code {
                200 => {
                    let xml = response.into_string()?;
                    Ok(RequestResult::Done(collection::from_xml(&xml, &options)))
                }
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })?
    }

    /// Get games.
    ///
    /// Calls `/thing`.
    /// Note that [Self::collection] is limited to the `boardgame` subtype, but this is not.
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
    /// This basically just calls [Self::collection] and [Self::get_games].
    pub fn get_all_games_for_user(
        &self,
        user: &str,
        _include_expansions: bool,
    ) -> error::Result<Vec<Game>> {
        let options = CollectionOptionsBuilder::default(user)
            .own(Filter::Include)
            .build();
        // if !include_expansions {
        //     options_builder = options_builder.exclude_sub_type(ThingType::BoardGameExpansion);
        // };

        let ids = self
            .collection(options)?
            .into_iter()
            .map(|item| item.id)
            .collect::<Vec<u32>>();

        self.get_games(&ids)
    }

    fn get_games_from_api(&self, ids: &[u32]) -> error::Result<Vec<Game>> {
        let ids_as_strings = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>();

        request::do_request(|| {
            let ids_string = ids_as_strings.join(",");
            let response = request::request_with_all_status_codes(self.agent.get(&format!(
                "{base}/xmlapi2/thing?id={ids_string}&stats=1",
                base = self.url
            )))?;

            log_headers(&response);

            let status_code = response.status();
            match response.status() {
                200 => Ok(RequestResult::Done(thing::from_xml(
                    &response.into_string()?,
                )?)),
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })
    }
}

#[cfg(feature = "moar-debug")]
fn log_headers(response: &Response) {
    response.headers_names().iter().for_each(|header| {
        if let Some(value) = response.header(header) {
            debug!("HEADER FOR RESEARCH {}: {}", header, value);
        }
    });
}
#[cfg(not(feature = "moar-debug"))]
fn log_headers(_: &Response) {}

impl Default for BggClient {
    fn default() -> Self {
        Self::new()
    }
}
