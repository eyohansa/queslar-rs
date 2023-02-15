use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    gold: f32,
    credits: u16,
    relics: u32,
    meat: f32,
    iron: f32,
    wood: f32,
    stone: f32,
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
    barrel: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    battling: u16,
    hunting: u16,
    mining: u16,
    woodcutting: u16,
    stonecarving: u16,
    crafting: u16,
    enchanting: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boosts {
    #[serde(rename = "critChance")]
    crit_chance: u16,

    #[serde(rename = "critDamage")]
    crit_damage: u16,

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
    speed: u16,
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
    elemental_conversion: u16,
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
    fire_damage: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    player: Player,
    currency: Currency,
    house: House,
    skills: Skills,
    boosts: Boosts,
    partners: Vec<Partner>,

    #[serde(rename = "catacombBanner")]
    catacomb_banner: CatacombBanner,

    #[serde(rename = "catacombCharacter")]
    catacomb_character: CatacombCharacter,
}
