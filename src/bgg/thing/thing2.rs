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
use std::cmp::Ordering;

#[derive(Clone, Deserialize, Serialize)]
pub struct Game {
    pub id: usize,
    pub is_expansion: bool,
    pub name: String,
    pub min_player_count: usize,
    pub max_player_count: usize,
    pub voter_count: usize,
    pub best_player_counts: Vec<usize>,
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

        let mut player_counts = Vec::new();
        for poll_results in best_player_results {
            if let Ok(player_count) = poll_results.player_count.parse::<usize>() {
                // Get the vote count for the "Best" category and ignore the other categories.
                let best_count = poll_results
                    .results_by_category
                    .into_iter()
                    .find(|c| c.value == Category::Best)
                    .map(|c| c.vote_count);

                // Add to the list.
                if let Some(best_count) = best_count {
                    player_counts.push((player_count, best_count));
                    // There should always be vote count?
                } else {
                    return Err(XmlApiError(format!(
                        "No best count for player count: {}",
                        poll_results.player_count
                    )));
                }
            // There may be other variants of numplayers strings we are not aware of.
            } else {
                return Err(XmlApiError(format!(
                    "Could not parse player count: {}",
                    poll_results.player_count
                )));
            }
        }

        // Retain max values for the counts. There are cases where the game is best at more than
        // one player count.
        // .max_by() returns last element if two values are equal. We want them all.
        // let best_player_counts = player_counts
        //     .into_iter()
        //     .max_by(|a, b| a.1.cmp(&b.1))
        //     .map(|(player_count, _best_count)| player_count)
        //     .into_iter()
        //     .collect();

        let mut best_player_counts = Vec::new();
        for (player_count, best_count) in player_counts {
            if let Some((_, max_best_count)) = best_player_counts.last() {
                match best_count.cmp(max_best_count) {
                    Ordering::Equal => best_player_counts.push((player_count, best_count)),
                    Ordering::Greater => {
                        best_player_counts.clear();
                        best_player_counts.push((player_count, best_count));
                    }
                    _ => {}
                }
            } else {
                best_player_counts.push((player_count, best_count));
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
            best_player_counts: best_player_counts
                .into_iter()
                .map(|(player_count, _)| player_count)
                .collect(),
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
        assert_eq!(game.best_player_counts[0], 4);
        assert_eq!(game.rating, 8.44283)
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
