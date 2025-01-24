use std::{env, fs};

fn main() {
    let file = env::args().nth(1).expect("Getting file from args");
    let xml = fs::read_to_string(&file).expect("Reading file");
    let games = bgg_lib::get_games(&xml).expect("Parsing xml");

    games.into_iter().for_each(|game| println!("{game:?}"))
}
