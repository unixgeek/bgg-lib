use bgg_lib::options::collections::CollectionOptionsBuilder;

fn main() -> bgg_lib::error::Result<()> {
    let client = bgg_lib::BggClient::new();

    let options = CollectionOptionsBuilder::default("unixgeek").build();

    let items = client.collection(options)?;

    println!("{}", items.len());

    Ok(())
}
