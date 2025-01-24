use crate::bgg::thing::Game;
use crate::bgg::{error, thing};

pub mod bgg;

pub fn get_games(xml: &str) -> error::Result<Vec<Game>> {
    thing::from_xml(xml)
}
