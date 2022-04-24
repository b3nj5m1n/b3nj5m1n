use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

pub fn get_config(filename: String) -> Config {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    toml::from_str(&contents).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub person: Person,
    pub languages: Languages,
    pub socials: HashMap<String, Social>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub screen_name: String,
    pub full_name: String,
    pub aliases: Vec<String>,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
    pub favorites: HashMap<String, Language>,
    pub learning: HashMap<String, Language>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub display_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Social {
    pub link: String,
    pub username: String,
}
