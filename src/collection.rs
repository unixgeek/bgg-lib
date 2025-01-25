//! The response from the `/collection` endpoint.
use crate::error;
use crate::error::Error::{InvalidUserError, XmlApiError, XmlError};
use serde::{Deserialize, Serialize};

pub(super) fn from_xml(xml: &str) -> error::Result<Vec<Item>> {
    #[cfg(feature = "moar-debug")]
    log::debug!("Collection XML: {}", xml);

    if xml.contains("<errors>") {
        let errors = serde_xml::<ErrorResponses>(xml)?.inner;

        if errors.len() == 1 && errors[0].message == "Invalid username specified" {
            Err(InvalidUserError)
        } else {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join(", ");
            Err(XmlApiError(message))
        }
    } else {
        Ok(serde_xml::<Items>(xml)?.inner)
    }
}

fn serde_xml<'a, T: Deserialize<'a>>(xml: &str) -> error::Result<T> {
    serde_xml_rs::from_str::<T>(xml)
        .map_err(|error| XmlError(format!("Error deserializing xml: {}", error)))
}

/// Represents a user's collection.
#[derive(Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "objectid")]
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
struct Items {
    #[serde(rename = "item", default)]
    inner: Vec<Item>,
}

#[derive(Deserialize)]
#[serde(rename = "error")]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize)]
struct ErrorResponses {
    #[serde(rename = "error")]
    inner: Vec<ErrorResponse>,
}

#[cfg(test)]
mod tests {
    use crate::collection::{from_xml, ErrorResponses, Items};
    use crate::error::Error::{InvalidUserError, XmlApiError};
    use std::fs;

    #[test]
    fn test_deserialize() {
        let items: Items =
            serde_xml_rs::from_str(&fs::read_to_string("test/unixgeek.xml").expect("Reading file"))
                .expect("Parsing XML");

        let items = items.inner;

        assert_eq!(items.len(), 3);

        assert_eq!(items[0].id, 421);
        assert_eq!(items[0].name, "1830: Railways & Robber Barons");

        assert_eq!(items[1].id, 228660);
        assert_eq!(items[1].name, "Betrayal at Baldur's Gate");

        assert_eq!(items[2].id, 39567);
        assert_eq!(items[2].name, r#"Formula D: The "Shortcut""#);
    }

    #[test]
    fn test_deserialize_empty_collection() {
        let items: Items = serde_xml_rs::from_str(
            &fs::read_to_string("test/empty-collection.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        assert_eq!(items.inner.len(), 0);
    }

    #[test]
    fn test_deserialize_error_response() {
        let error_responses: ErrorResponses = serde_xml_rs::from_str(
            &fs::read_to_string("test/invalid-username.xml").expect("Reading file"),
        )
        .expect("Parsing XML");

        assert_eq!(error_responses.inner.len(), 1);
        assert_eq!(
            error_responses.inner[0].message,
            "Invalid username specified"
        );
    }

    #[test]
    fn test_from_xml_invalid_user_error() {
        let result =
            from_xml(&fs::read_to_string("test/invalid-username.xml").expect("Reading file"));

        assert!(result.is_err());
        assert!(matches!(result, Err(InvalidUserError)));
    }

    #[test]
    fn test_from_xml_unknown_error() {
        let result =
            from_xml(&fs::read_to_string("test/unknown-errors.xml").expect("Reading file"));

        assert!(result.is_err());
        assert!(matches!(result, Err(XmlApiError(_))));
        assert_eq!(
            result.err().unwrap().to_string(),
            "I don't like your name, I don't like your face, I don't like you".to_owned()
        );
    }
}
