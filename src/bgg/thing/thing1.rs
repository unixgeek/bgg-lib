//! # Thing1
//!
//! This module parses the response from the `/collection` endpoint.
//!
//! The api documentation states the result of the endpoint are "thing items", hence the naming
//! used here. For example, the response xml has `items` as the root with `item` children elements.
//!
//! The xml is difficult to parse with serde and the resulting data representation is clumsy and
//! confusing, which is why the `thing2` module exists: to simplify the structure.
//!
//! We might need to consider using xpath or DOM parsing in the future.

use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::result;
use std::str::FromStr;

#[derive(Deserialize)]
pub(super) struct Items {
    #[serde(rename = "item")]
    inner: Vec<Item>,
}

impl Items {
    pub(super) fn into_inner(self) -> Vec<Item> {
        self.inner
    }
}

#[derive(Deserialize)]
pub(super) struct Item {
    pub(super) id: u32,
    #[serde(rename = "type")]
    pub(super) thing_type: String,
    #[serde(rename = "name")]
    pub(super) names: Vec<Name>,
    #[serde(rename = "minplayers")]
    pub(super) min_players: ElementWithIntValueAttribute,
    #[serde(rename = "maxplayers")]
    pub(super) max_players: ElementWithIntValueAttribute,
    pub(super) poll: Poll,
    pub(super) statistics: Statistics,
}

// example: <name type="primary" sortindex="1" value="Eclipse: Second Dawn for the Galaxy"/>
#[derive(Deserialize)]
pub(super) struct Name {
    #[serde(rename = "type")]
    pub(super) _type: String,
    pub(super) value: String,
}

// example: <minplayers value="2"/>
#[derive(Deserialize)]
pub(super) struct ElementWithIntValueAttribute {
    pub(super) value: u8,
}

#[derive(Deserialize)]
pub(super) struct ElementWithFloatValueAttribute {
    pub(super) value: f64,
}

// example: <poll name="suggested_numplayers" title="User Suggested Number of Players" totalvotes="328">
#[derive(Deserialize)]
pub(super) struct Poll {
    // This is "total voters" on the website.
    #[serde(rename = "totalvotes")]
    pub(super) voter_count: u16,
    pub(super) results: Vec<Results>,
}

#[derive(Deserialize)]
pub(super) struct Results {
    #[serde(rename = "numplayers")]
    pub(super) player_count: String,
    #[serde(rename = "result", default)]
    pub(super) results_by_category: Vec<Result>,
}

#[derive(Deserialize)]
pub(super) struct Result {
    #[serde(deserialize_with = "category_from_str")]
    pub(super) value: Category,
    #[serde(rename = "numvotes")]
    pub(super) vote_count: u16,
}

#[derive(Deserialize, PartialEq, Debug)]
pub(super) enum Category {
    Best,
    Recommended,
    NotRecommended,
}

pub(super) struct ParseCategoryError;

impl FromStr for Category {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "best" => Ok(Category::Best),
            "recommended" => Ok(Category::Recommended),
            "not recommended" => Ok(Category::NotRecommended),
            _ => Err(ParseCategoryError),
        }
    }
}

fn category_from_str<'de, D: Deserializer<'de>>(d: D) -> result::Result<Category, D::Error> {
    let s: String = Deserialize::deserialize(d)?;

    match Category::from_str(&s) {
        Ok(c) => Ok(c),
        Err(_error) => Err(Error::unknown_variant(
            &s,
            &["Best", "Recommended, Not Recommended"],
        )),
    }
}

#[derive(Deserialize)]
pub(super) struct Statistics {
    pub(super) ratings: Ratings,
}

#[derive(Deserialize)]
pub(super) struct Ratings {
    pub(super) average: ElementWithFloatValueAttribute,
}

#[cfg(test)]
mod tests {
    use crate::bgg::thing::thing1::{Category, Items};
    use std::fs;

    #[test]
    fn test_deserialize() {
        let items: Items = serde_xml_rs::from_str(
            &fs::read_to_string("test/eclipse-transformed.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        let games = items.into_inner();
        assert_eq!(games.len(), 1);

        let game = games.get(0).unwrap();
        assert_eq!(game.id, 246900);
        assert_eq!(game.thing_type, "boardgame");
        assert_eq!(game.names.len(), 9);
        let name = game.names.iter().find(|n| n._type == "primary").unwrap();
        assert_eq!(name.value, "Eclipse: Second Dawn for the Galaxy");
        assert_eq!(game.min_players.value, 2);
        assert_eq!(game.max_players.value, 6);

        let poll = &game.poll;
        assert_eq!(poll.voter_count, 336);
        assert_eq!(poll.results.len(), 7);
        assert_eq!(poll.results[0].player_count, "1");
        assert_eq!(poll.results[0].results_by_category.len(), 3);
        assert_eq!(poll.results[0].player_count, "1");
        assert_eq!(poll.results[0].results_by_category[0].value, Category::Best);
        assert_eq!(poll.results[0].results_by_category[0].vote_count, 1);
        assert_eq!(poll.results[1].player_count, "2");
        assert_eq!(poll.results[1].results_by_category[0].value, Category::Best);
        assert_eq!(poll.results[1].results_by_category[0].vote_count, 16);
        assert_eq!(poll.results[2].player_count, "3");
        assert_eq!(
            poll.results[2].results_by_category[2].value,
            Category::NotRecommended
        );
        assert_eq!(poll.results[2].results_by_category[2].vote_count, 33);

        assert_eq!(game.statistics.ratings.average.value, 8.43349);
    }

    #[test]
    fn test_missing_results() {
        let items: Items = serde_xml_rs::from_str(
            &fs::read_to_string("test/tower-capture-transformed.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        let game = items.into_inner().pop().unwrap();

        assert_eq!(game.poll.results.len(), 1);
        let results = game.poll.results.get(0).unwrap();
        assert_eq!(results.player_count, "2+");
        assert!(results.results_by_category.is_empty())
    }
}
