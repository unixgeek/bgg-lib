use crate::bgg::error;
use crate::bgg::error::Error::XmlError;

const XSLT: &str = env!("GAME_XSLT");

// The game / item xml is difficult to parse. Use xslt to remove things that cause problems and we
// aren't going to use anyway.

pub(super) fn transform(xml: &str) -> error::Result<String> {
    let xml_doc = libxml::parser::Parser::default()
        .parse_string(xml)
        .map_err(|err| XmlError(format!("Error parsing xml with libxml: {}", err)))?;

    match libxslt::parser::parse_bytes(XSLT.to_string().into_bytes(), "") {
        Ok(mut stylesheet) => match stylesheet.transform(&xml_doc, vec![]) {
            Ok(result_doc) => Ok(result_doc.to_string()),
            Err(err) => Err(XmlError(format!(
                "Error transforming xml with xslt: {}",
                err
            ))),
        },
        Err(error) => Err(XmlError(format!("Error parsing xml with xslt: {error}"))),
    }
}

#[cfg(test)]
mod tests {
    use crate::bgg::thing::xslt::transform;
    use std::fs;

    #[test]
    fn test_transform() {
        let xml = fs::read_to_string("test/eclipse.xml").expect("Reading file");
        assert!(xml.contains("suggested_playerage"));

        let result = transform(&xml).expect("Transforming");
        assert!(!result.contains("suggested_playerage"));
    }
}
