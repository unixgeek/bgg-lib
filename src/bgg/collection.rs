use crate::bgg::error;
use crate::bgg::error::Error::XmlError;
use serde::{Deserialize, Serialize};

pub(super) fn from_xml(xml: &str) -> error::Result<Vec<Item>> {
    // debug!("XML: {}", xml);
    Ok(serde_xml_rs::from_str::<Items>(xml)
        .map_err(|error| XmlError(format!("Error deserializing xml: {}", error)))?
        .into_inner())
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "objectid")]
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize)]
struct Items {
    #[serde(rename = "item")]
    inner: Vec<Item>,
}

impl Items {
    fn into_inner(self) -> Vec<Item> {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::bgg::collection::Items;
    use std::fs;

    #[test]
    fn test_deserialize() {
        let items: Items =
            serde_xml_rs::from_str(&fs::read_to_string("test/unixgeek.xml").expect("Reading file"))
                .expect("Parsing XML");

        let items = items.into_inner();

        assert_eq!(items.len(), 3);

        assert_eq!(items[0].id, 421);
        assert_eq!(items[0].name, "1830: Railways & Robber Barons");

        assert_eq!(items[1].id, 228660);
        assert_eq!(items[1].name, "Betrayal at Baldur's Gate");

        assert_eq!(items[2].id, 39567);
        assert_eq!(items[2].name, r#"Formula D: The "Shortcut""#);
    }
}
