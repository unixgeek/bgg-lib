use crate::options::{bool_to_param, u16_to_param, u8_to_param};

macro_rules! option_bool {
    ($name:tt) => {
        pub fn $name(mut self, $name: bool) -> Self {
            self.options.$name = $name;
            self
        }
    };
}

macro_rules! option_filter {
    ($name:tt) => {
        pub fn $name(mut self, $name: Filter) -> Self {
            self.options.$name = Some($name);
            self
        }
    };
}

macro_rules! option_u8 {
    ($name:tt) => {
        pub fn $name(mut self, $name: u8) -> Self {
            self.options.$name = Some($name);
            self
        }
    };
}

pub enum Filter {
    Exclude = 0,
    Include = 1,
}

impl Filter {
    pub fn as_str(&self) -> &str {
        match self {
            Filter::Exclude => "0",
            Filter::Include => "1",
        }
    }
}

pub enum ThingType {
    BoardGame,
    BoardGameAccessory,
    BoardGameExpansion,
    RPGIssue,
    RPGItem,
    VideoGame,
}

impl ThingType {
    pub fn as_str(&self) -> &str {
        match self {
            ThingType::BoardGame => "boardgame",
            ThingType::BoardGameAccessory => "boardgameaccessory",
            ThingType::BoardGameExpansion => "boardgameexpansion",
            ThingType::RPGIssue => "rpgissue",
            ThingType::RPGItem => "rpgitem",
            ThingType::VideoGame => "videogame",
        }
    }
}

pub struct CollectionOptions {
    pub user: String,
    pub version: bool,
    pub sub_type: ThingType,
    pub exclude_sub_type: Option<ThingType>,
    pub ids: Option<Vec<u32>>,
    pub brief: bool,
    pub stats: bool,
    // From here to max_plays, it is just filters?
    pub own: Option<Filter>,
    pub rated: Option<Filter>,
    pub played: Option<Filter>,
    pub comment: Option<Filter>,
    pub trade: Option<Filter>,
    pub want: Option<Filter>,
    pub wishlist: Option<Filter>,
    pub wishlist_priority: Option<u8>,
    pub pre_ordered: Option<Filter>,
    pub want_to_play: Option<Filter>,
    pub want_to_buy: Option<Filter>,
    pub previously_owned: Option<Filter>,
    pub has_parts: Option<Filter>,
    pub want_parts: Option<Filter>,
    pub min_rating: Option<u8>,
    pub rating: Option<u8>,
    pub min_bgg_rating: Option<u8>,
    pub bgg_rating: Option<u8>,
    pub min_plays: Option<u16>,
    pub max_plays: Option<u16>,
    pub collection_id: Option<u32>,
    // modified_since
}

impl CollectionOptions {
    pub fn as_url_params(&self) -> String {
        let mut params = String::from("?username=");
        params.push_str(&self.user);

        params.push_str("&subtype=");
        params.push_str(self.sub_type.as_str());

        if let Some(exclude_sub_type) = &self.exclude_sub_type {
            params.push_str("&excludesubtype=");
            params.push_str(exclude_sub_type.as_str());
        }

        if let Some(ids) = &self.ids {
            params.push_str("&id=");
            params.push_str(
                &ids.iter()
                    .map(|id| id.to_owned().to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }

        params.push_str(&bool_to_param("version", self.version));
        params.push_str(&bool_to_param("brief", self.brief));
        params.push_str(&bool_to_param("stats", self.stats));

        params.push_str(&filter_to_param("own", &self.own.as_ref()));
        params.push_str(&filter_to_param("rated", &self.rated.as_ref()));
        params.push_str(&filter_to_param("played", &self.played.as_ref()));
        params.push_str(&filter_to_param("comment", &self.comment.as_ref()));
        params.push_str(&filter_to_param("trade", &self.trade.as_ref()));
        params.push_str(&filter_to_param("want", &self.want.as_ref()));
        params.push_str(&filter_to_param("wishlist", &self.wishlist.as_ref()));

        params.push_str(&u8_to_param("wishlistpriority", self.wishlist_priority));

        params.push_str(&filter_to_param("preordered", &self.pre_ordered.as_ref()));
        params.push_str(&filter_to_param("wanttoplay", &self.want_to_play.as_ref()));
        params.push_str(&filter_to_param("wanttobuy", &self.want_to_buy.as_ref()));
        params.push_str(&filter_to_param(
            "prevowned",
            &self.previously_owned.as_ref(),
        ));
        params.push_str(&filter_to_param("hasparts", &self.has_parts.as_ref()));
        params.push_str(&filter_to_param("wantparts", &self.want_parts.as_ref()));

        params.push_str(&u8_to_param("minrating", self.min_rating));
        params.push_str(&u8_to_param("rating", self.rating));
        params.push_str(&u8_to_param("minbggrating", self.min_bgg_rating));
        params.push_str(&u8_to_param("bggrating", self.bgg_rating));

        params.push_str(&u16_to_param("minplays", self.min_plays));
        params.push_str(&u16_to_param("maxplays", self.max_plays));

        if let Some(collection_id) = &self.collection_id {
            params.push_str("&collid=");
            params.push_str(&collection_id.to_string());
        }

        params
    }
}

pub struct CollectionOptionsBuilder {
    options: CollectionOptions,
}

impl CollectionOptionsBuilder {
    pub fn default(user: &str) -> Self {
        let options = CollectionOptions {
            user: user.to_owned(),
            version: false,
            sub_type: ThingType::BoardGame,
            exclude_sub_type: None,
            ids: None,
            brief: true,
            stats: false,
            own: None,
            rated: None,
            played: None,
            comment: None,
            trade: None,
            want: None,
            wishlist: None,
            wishlist_priority: None,
            pre_ordered: None,
            want_to_play: None,
            want_to_buy: None,
            previously_owned: None,
            has_parts: None,
            want_parts: None,
            min_rating: None,
            rating: None,
            min_bgg_rating: None,
            bgg_rating: None,
            min_plays: None,
            max_plays: None,
            collection_id: None,
        };
        Self { options }
    }
    pub fn sub_type(mut self, sub_type: ThingType) -> Self {
        self.options.sub_type = sub_type;
        self
    }

    pub fn exclude_sub_type(mut self, exclude_sub_type: ThingType) -> Self {
        self.options.exclude_sub_type = Some(exclude_sub_type);
        self
    }

    pub fn ids(mut self, ids: Option<Vec<u32>>) -> Self {
        self.options.ids = ids;
        self
    }

    option_bool!(version);
    option_bool!(brief);
    option_bool!(stats);
    option_filter!(own);
    option_filter!(rated);
    option_filter!(played);
    option_filter!(comment);
    option_filter!(trade);
    option_filter!(want);
    option_filter!(wishlist);
    option_u8!(wishlist_priority);
    option_filter!(pre_ordered);
    option_filter!(want_to_play);
    option_filter!(want_to_buy);
    option_filter!(previously_owned);
    option_filter!(has_parts);
    option_filter!(want_parts);
    option_u8!(min_rating);
    option_u8!(rating);
    option_u8!(min_bgg_rating);
    option_u8!(bgg_rating);

    pub fn max_plays(mut self, max_plays: u16) -> Self {
        self.options.max_plays = Some(max_plays);
        self
    }

    pub fn min_plays(mut self, min_plays: u16) -> Self {
        self.options.min_plays = Some(min_plays);
        self
    }

    pub fn collection_id(mut self, collection_id: u32) -> Self {
        self.options.collection_id = Some(collection_id);
        self
    }

    pub fn build(self) -> CollectionOptions {
        self.options
    }
}

fn filter_to_param(name: &str, filter: &Option<&Filter>) -> String {
    filter
        .map(|f| format!("&{name}={}", f.as_str()))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::options::collections::{CollectionOptionsBuilder, Filter, ThingType};

    #[test]
    fn test_collection_options_default_url_params() {
        let url_params = CollectionOptionsBuilder::default("unixgeek")
            .build()
            .as_url_params();

        let expected = "?username=unixgeek&subtype=boardgame&brief=1";
        assert_eq!(url_params, expected);
    }

    #[test]
    fn test_collection_options_all_url_params() {
        // let options =
        let url_params = CollectionOptionsBuilder::default("unixgeek")
            .exclude_sub_type(ThingType::BoardGameExpansion)
            .version(true)
            .ids(Some(vec![1, 2, 3]))
            .brief(false)
            .stats(true)
            .own(Filter::Include)
            .rated(Filter::Exclude)
            .played(Filter::Include)
            .comment(Filter::Exclude)
            .trade(Filter::Include)
            .want(Filter::Exclude)
            .wishlist(Filter::Include)
            .wishlist_priority(4)
            .pre_ordered(Filter::Exclude)
            .want_to_play(Filter::Include)
            .want_to_buy(Filter::Exclude)
            .previously_owned(Filter::Include)
            .has_parts(Filter::Exclude)
            .want_parts(Filter::Include)
            .min_rating(6)
            .rating(8)
            .min_bgg_rating(4)
            .bgg_rating(6)
            .min_plays(2)
            .max_plays(4)
            .collection_id(100)
            .build()
            .as_url_params();

        let expected = "?username=unixgeek&subtype=boardgame&excludesubtype=boardgameexpansion&id=1,2,3\
            &version=1&stats=1&own=1&rated=0&played=1&comment=0&trade=1&want=0&wishlist=1&wishlistpriority=4\
            &preordered=0&wanttoplay=1&wanttobuy=0&prevowned=1&hasparts=0&wantparts=1&minrating=6&rating=8\
            &minbggrating=4&bggrating=6&minplays=2&maxplays=4&collid=100";

        assert_eq!(url_params, expected);
    }
}

/*
let o = Options {
username: "techgunter",
Options::Default()
}

let mut o = Options::default();
o.username = "techgunter";

builder.include_owned().exclude_played()
 */
