//! # Thing2
//!
//! This module maps the resulting structure from the `thing1` module to an easier to use structure.
//!
//! The api documentation states the result of the endpoint are "thing items", so maybe this should
//! be called `Thing` or `Item`, but we only care about board games, so we use `Game`.

use crate::bgg;
use crate::bgg::error::Error::XmlApiError;
use crate::bgg::thing::thing1::{Category, Item, Results};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: u32,
    pub is_expansion: bool,
    pub name: String,
    pub min_player_count: u16,
    pub max_player_count: u16,
    pub voter_count: u16,
    pub best_player_counts: Vec<u16>,
    pub rating: f64,
}

impl TryFrom<Item> for Game {
    type Error = bgg::error::Error;

    fn try_from(item: Item) -> Result<Self, bgg::error::Error> {
        let voter_count = item.poll.voter_count;

        let best_player_results: Vec<Results> = item
            .poll
            .results
            .into_iter()
            // A <results> element can have an attribute, numplayers, with a non-integer value, i.e. <results numplayers="6+">.
            // I don't think these are of any value, so simply ignore them.
            .filter(|poll_results| !poll_results.player_count.contains("+"))
            .collect();

        /*
            Looks like this on the website.
            1   0.5%   (1)  5.2%  (11) 94.3% (198)  210
            2   6.1%  (16) 63.4% (166) 30.5%  (80)  262
            3  20.5%  (55) 67.2% (180) 12.3%  (33)  268
            4  71.7% (205) 26.2%  (75)  2.1%   (6)  286
            5  25.4%  (63) 61.7% (153) 12.9%  (32)  248
            6  58.4% (149) 31.4%  (80) 10.2%  (26)  255
            6+  2.5%   (4)  5.5%   (9) 92.0% (150)  163
            Total voters 336
        */
        let mut best_player_counts = Vec::new();
        for poll_results in best_player_results {
            if let Ok(player_count) = poll_results.player_count.parse::<u16>() {
                // Get the total vote count.
                let total_count = poll_results
                    .results_by_category
                    .iter()
                    .fold(0, |acc, c| acc + c.vote_count);

                // Get the vote count for the "Best" category and ignore the other categories.
                let best_count = poll_results
                    .results_by_category
                    .into_iter()
                    .find(|c| c.value == Category::Best)
                    .map_or(0, |c| c.vote_count);

                let percentage = (best_count as f64) / (total_count as f64) * 100.0;

                // Based on observation. Not sure if this is the actual algorithm.
                if percentage > 50.0 {
                    best_player_counts.push(player_count);
                }
            // There may be other variants of numplayers strings we are not aware of.
            } else {
                return Err(XmlApiError(format!(
                    "Could not parse player count: {}",
                    poll_results.player_count
                )));
            }
        }

        let name = if let Some(name) = item.names.into_iter().find(|n| n._type == "primary") {
            name.value
        } else {
            return Err(XmlApiError("No primary name found".to_owned()));
        };

        Ok(Self {
            id: item.id,
            is_expansion: item.thing_type == "boardgameexpansion",
            name,
            min_player_count: item.min_players.value,
            max_player_count: item.max_players.value,
            voter_count,
            best_player_counts,
            rating: item.statistics.ratings.average.value,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bgg::thing::thing1::Items;
    use crate::bgg::thing::thing2::Game;
    use std::fs;

    #[test]
    fn test_try_from() {
        let items: Items = serde_xml_rs::from_str(
            &fs::read_to_string("test/eclipse-transformed.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        let game = Game::try_from(items.into_inner().pop().unwrap()).unwrap();
        assert_eq!(game.id, 246900);
        assert!(!game.is_expansion);
        assert_eq!(game.name, "Eclipse: Second Dawn for the Galaxy");
        assert_eq!(game.min_player_count, 2);
        assert_eq!(game.max_player_count, 6);
        assert_eq!(game.best_player_counts.len(), 2);
        assert_eq!(game.best_player_counts[0], 4);
        assert_eq!(game.best_player_counts[1], 6);
        assert_eq!(game.rating, 8.43349)
    }

    #[test]
    fn test_try_from_expansion() {
        let items: Items = serde_xml_rs::from_str(
            &fs::read_to_string("test/fire-and-ice-transformed.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        let game = Game::try_from(items.into_inner().pop().unwrap()).unwrap();
        assert_eq!(game.id, 161317);
        assert!(game.is_expansion);
    }
}
