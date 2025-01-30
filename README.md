# bgg-lib
This crate provides a simple api to get a subset of data from [boardgamegeek.com](https://boardgamegeek.com),
using the [BGG XML API2](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
The data is limited to a small number of fields for my use case.

## Simple Example
```rust
use bgg_lib::{error, BggClient};
fn main() -> error::Result<()>{
    let client = BggClient::new();
    let games = client.get_all_games_for_user("unixgeek", false)?;
    Ok(())
}
```


curl -o full.xml 'https://boardgamegeek.com/xmlapi2/collection?username=techgunter'
curl -o brief.xml 'https://boardgamegeek.com/xmlapi2/collection?username=techgunter&brief=1'

Does own=1 inherently mean prevowned=0?