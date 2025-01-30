use crate::options::bool_to_param;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ThingType {
    BoardGame,
    BoardGameExpansion,
    BoardGameAccessory,
    VideoGame,
    RPGItem,
    RPGIssue,
}

#[allow(dead_code)]
pub struct ThingOptions {
    thing_types: Vec<ThingType>,
    versions: bool,
    videos: bool,
    stats: bool,
    marketplace: bool,
    comments: bool, // todo Seems like it needs page and pagesize also need ratingcomments, which are mutually exclusive? use enum for mutually exclusive thing?
}

#[allow(dead_code)]
impl ThingOptions {
    pub(super) fn into_url_params(self) -> String {
        let mut params = String::new();

        let thing_types = self
            .thing_types
            .into_iter()
            .map(|t| match t {
                ThingType::BoardGame => "boardgame",
                ThingType::BoardGameExpansion => "boardgameexpansion",
                ThingType::BoardGameAccessory => "boardgameaccessory",
                ThingType::VideoGame => "videogame",
                ThingType::RPGItem => "rpgitem",
                ThingType::RPGIssue => "rpgissue",
            })
            .collect::<Vec<&str>>()
            .join(",");

        params.push_str("&thingtype=");
        params.push_str(&thing_types);
        params.push_str(&bool_to_param("versions", self.versions)); // todo Use a trait? then just .into() or as_whatever()?
        params.push_str(&bool_to_param("videos", self.videos));
        params.push_str(&bool_to_param("stats", self.stats));
        params.push_str(&bool_to_param("marketplace", self.marketplace));
        params.push_str(&bool_to_param("comments", self.comments));

        params
    }
}

#[allow(dead_code)]
struct ThingOptionsBuilder {
    options: ThingOptions,
}

#[allow(dead_code)]
impl Default for ThingOptionsBuilder {
    fn default() -> Self {
        Self {
            options: ThingOptions {
                thing_types: vec![ThingType::BoardGame],
                versions: false,
                videos: false,
                stats: false,
                marketplace: false,
                comments: false,
            },
        }
    }
}

#[allow(dead_code)]
impl ThingOptionsBuilder {
    pub fn thing_type(mut self, thing_type: Vec<ThingType>) -> Self {
        self.options.thing_types = thing_type;
        self
    }

    pub fn versions(mut self, versions: bool) -> Self {
        self.options.versions = versions;
        self
    }

    pub fn videos(mut self, videos: bool) -> Self {
        self.options.videos = videos;
        self
    }

    pub fn stats(mut self, stats: bool) -> Self {
        self.options.stats = stats;
        self
    }

    pub fn marketplace(mut self, marketplace: bool) -> Self {
        self.options.marketplace = marketplace;
        self
    }

    pub fn comments(mut self, comments: bool) -> Self {
        self.options.comments = comments;
        self
    }

    pub fn build(self) -> ThingOptions {
        self.options
    }
}

#[cfg(test)]
mod tests {
    use crate::options::thing::{ThingOptionsBuilder, ThingType};

    #[test]
    fn test_thing_options_builder() {
        let options = ThingOptionsBuilder::default()
            .versions(true)
            .videos(false)
            .stats(true)
            .marketplace(false)
            .comments(true)
            .thing_type(vec![ThingType::BoardGame, ThingType::VideoGame])
            .build();

        assert_eq!(options.versions, true);
        assert_eq!(options.videos, false);
        assert_eq!(options.stats, true);
        assert_eq!(options.marketplace, false);
        assert_eq!(options.comments, true);
        assert_eq!(
            options.thing_types,
            vec![ThingType::BoardGame, ThingType::VideoGame]
        );
        assert_eq!(
            options.into_url_params(),
            "&thingtype=boardgame,videogame&versions=1&stats=1&comments=1"
        )
    }
}
