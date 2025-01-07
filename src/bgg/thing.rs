use crate::bgg::error;
use crate::bgg::error::Error::XmlError;
use crate::bgg::thing::thing1::{Item, Items};
pub use thing2::Game;

mod thing1;
mod thing2;
mod xslt;

pub(super) fn from_xml(xml: &str) -> error::Result<Vec<Game>> {
    #[cfg(feature = "moar-debug")]
    log::debug!("Things XML: {}", xml);

    let items: Vec<Item> = serde_xml_rs::from_str::<Items>(&xslt::transform(xml)?)
        .map_err(|error| XmlError(format!("Error deserializing xml: {}", error)))?
        .into_inner();

    let mut games = Vec::new();
    for item in items {
        games.push(item.try_into()?);
    }
    Ok(games)
}

#[cfg(test)]
mod tests {
    use crate::bgg::thing::from_xml;
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
        assert_eq!(game.best_player_counts.len(), 4);
        assert_eq!(game.best_player_counts[0], 1);
        assert_eq!(game.best_player_counts[1], 2);
        assert_eq!(game.best_player_counts[2], 3);
        assert_eq!(game.best_player_counts[3], 4);
        assert_eq!(game.rating, 7.894);
    }
}
