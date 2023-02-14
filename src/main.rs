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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize)]
pub struct Skills {
    battling: u16,
    hunting: u16,
    mining: u16,
    woodcutting: u16,
    stonecarving: u16,
    crafting: u16,
    enchanting: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Boosts {
    critChance: u16,
    critDamage: u16,
    multistrike: u16,
    healing: u16,
    hunting_boost: u16,
    mining_boost: u16,
    stonecarving_boost: u16,
    catacombs_elemental: u16,
    catacombs_health: u16,
    catacombs_damage: u16,
    catacombs_hit: u16,
    catacombs_dodge: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Partner {
    hunting: u16,
    mining: u16,
    woodcutting: u16,
    stonecarving: u16,
    intelligence: u16,
    speed: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatacombBanner {
    mobs: u16,
    reward_multiplier: u16,
    mob_multiplier: u16,
    character_multiplier: u16,
    water_resistance: u16,
    thunder_resistance: u16,
    fire_resistance: u16,
    melee_resistance: u16,
    ranged_resistance: u16,
    elemental_conversion: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatacombCharacter {
    health: u16,
    damage: u16,
    hit: u16,
    dodge: u16,
    crit_damage: u16,
    crit_chance: u16,
    water_damage: u16,
    thunder_damage: u16,
    fire_damage: u16
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