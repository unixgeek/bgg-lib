//! An XSLT transformer for the XML received from the `/thing` endpoint.
//!
//! The XML is difficult to parse with serde, so we use this to remove things that cause problems,
//! and we aren't going to use anyway.
//!
//! Might need to consider using xpath or DOM parsing in the future.
use crate::error;
use crate::error::Error::XmlError;

const XSLT: &str = env!("GAME_XSLT");

pub(super) fn transform(xml: &str) -> error::Result<String> {
    let xml_doc = libxml::parser::Parser::default()
        .parse_string(xml)
        .map_err(|err| XmlError(format!("Error parsing xml with libxml: {err}")))?;

    match libxslt::parser::parse_bytes(XSLT.to_owned().into_bytes(), "") {
        Ok(mut stylesheet) => match stylesheet.transform(&xml_doc, vec![]) {
            Ok(result_doc) => Ok(result_doc.to_string()),
            Err(err) => Err(XmlError(format!("Error transforming xml with xslt: {err}"))),
        },
        Err(error) => Err(XmlError(format!("Error parsing xml with xslt: {error}"))),
    }
}

#[cfg(test)]
mod tests {
    use crate::thing::xslt::transform;
    use std::fs;

    #[test]
    fn test_transform() {
        let xml = fs::read_to_string("test/eclipse.xml").expect("Reading file");
        assert!(xml.contains("suggested_playerage"));

        let result = transform(&xml).expect("Transforming");
        assert!(!result.contains("suggested_playerage"));
    }

    // #[test]
    // fn transformed_xml() {
    //     let (input, output) = ("test/enormity.xml", "test/enormity-transformed.xml");
    //
    //     fs::write(
    //         output,
    //         transform(&fs::read_to_string(input).expect("Reading file")).expect("Transforming"),
    //     )
    //     .expect("Writing file");
    // }
}
