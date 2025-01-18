pub use crate::bgg::collection::Item as CollectionItem;
use crate::bgg::request::RequestResult;
pub use crate::bgg::thing::Game;
use crate::{PKG_NAME, PKG_VERSION};
use log::debug;
use ureq::{Agent, AgentBuilder, Response};

mod collection;
pub mod error;
mod request;
mod thing;

// bgg says max is 20.
const MAX_IDS: usize = 20;

pub struct BggClient {
    agent: Agent,
}

impl BggClient {
    pub fn new() -> Self {
        let agent = AgentBuilder::new()
            .user_agent(&format!("{} {}", PKG_NAME, PKG_VERSION))
            .build();
        Self { agent }
    }

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
            "https://boardgamegeek.com/xmlapi2/collection?username={user}&own=1&brief=1&subtype=boardgame{exclude_param}"
        );

        request::do_request(|| {
            let response = request::request_with_all_status_codes(self.agent.get(&url))?;
            log_headers(&response);

            let status_code = response.status();
            match status_code {
                200 => {
                    let xml = response.into_string()?;
                    Ok(RequestResult::Done(collection::from_xml(&xml)))
                }
                _ => Ok(RequestResult::NotDone(status_code)),
            }
        })?
    }

    pub fn get_games(&self, ids: &[usize]) -> error::Result<Vec<Game>> {
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

        request::do_request(|| {
            let ids_string = ids_as_strings.join(",");
            let response = request::request_with_all_status_codes(self.agent.get(&format!(
                "https://boardgamegeek.com/xmlapi2/thing?id={ids_string}&stats=1"
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
