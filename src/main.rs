use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use reqwest::Result;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    api: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    gold: f32,
    credits: u16,
    relics: u32,
    meat: f32,
    iron: f32,
    wood: f32,
    stone: f32
}

#[derive(Serialize, Deserialize)]
pub struct House {
    chairs: u8,
    stove: u8,
    sink: u8,
    basket: u8,
    table: u8,
    couch: u8,
    carpet: u8,
    candlestick: u8,
    pitchfork: u8,
    shed: u8,
    fountain: u8,
    tools: u8,
    bed: u8,
    closet: u8,
    nightstand: u8,
    window: u8,
    barrel: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    player: Player,
    currency: Currency
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = load_settings_from_file();
    let player_data = fetch_api(settings.api).await?;
    let player: Character = serde_json::from_str(&player_data).unwrap();
    println!("{:?}", player);
    Ok(())
}

pub fn load_settings_from_file() -> Settings {
    let file = File::open("./settings.json").expect("Should be able to read file.");
    let reader = BufReader::new(file);
    let settings: Settings = serde_json::from_reader(reader).expect("Should be able to parse settings.");

    return settings;
}

pub async fn fetch_api(api: String) -> Result<String> {
    let url = format!("https://queslar.com/api/player/full/{}", api);
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}