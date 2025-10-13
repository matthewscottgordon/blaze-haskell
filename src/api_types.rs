use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub ruleset: Value,
    pub map: String,
    pub timeout: i32,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coordinates>,
    pub hazards: Vec<Coordinates>,
    pub snakes: Vec<Battlesnake>,
}

#[derive(Serialize, Deserialize)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Coordinates>,
    pub latency: String,
    pub head: Coordinates,
    pub length: u8,
    pub shout: String,
    pub squad: String,
    pub customizations: Value,
}
