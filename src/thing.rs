//! The response from the `/thing` endpoint.
//!
//! The XML API documentation states the result of the endpoint are "thing items", hence the naming
//! used in this module and submodules. For example, the response XML has `items` as the root with
//! `item` children elements.
use crate::error;
use crate::error::Error::XmlError;
use crate::thing::thing1::{Item, Items};
pub use thing2::Game;

mod thing1;
mod thing2;
mod xslt;

pub(super) fn from_xml(xml: &str) -> error::Result<Vec<Game>> {
    #[cfg(feature = "moar-debug")]
    log::debug!("Things XML: {}", xml);

    let items: Vec<Item> = serde_xml_rs::from_str::<Items>(&xslt::transform(xml)?)
        .map_err(|error| XmlError(format!("Error deserializing xml: {error}")))?
        .into_inner();

    let mut games = Vec::new();
    for item in items {
        games.push(item.try_into()?);
    }
    Ok(games)
}

#[cfg(test)]
mod tests {
    use crate::thing::from_xml;
    use std::fs;

    #[test]
    fn test_from_xml() {
        let game = from_xml(&fs::read_to_string("test/enormity.xml").expect("Reading file"))
            .expect("Parsing XML");

        assert_eq!(game.len(), 1);
        let game = game.get(0).unwrap();
        assert_eq!(game.id, 430350);
        assert_eq!(game.name, "Enormity");
        assert_eq!(game.min_player_count, 1);
        assert_eq!(game.max_player_count, 4);
        assert_eq!(game.voter_count, 0);
        assert_eq!(game.best_player_counts.len(), 0);
        assert_eq!(game.rating, 7.894);
    }
}
