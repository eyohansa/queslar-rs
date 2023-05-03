use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub gold: f32,
    pub credits: u16,
    pub relics: u32,
    pub meat: f32,
    pub iron: f32,
    pub wood: f32,
    pub stone: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct House {
    pub chairs: u8,
    pub stove: u8,
    pub sink: u8,
    pub basket: u8,
    pub table: u8,
    pub couch: u8,
    pub carpet: u8,
    pub candlestick: u8,
    pub pitchfork: u8,
    pub shed: u8,
    pub fountain: u8,
    pub tools: u8,
    pub bed: u8,
    pub closet: u8,
    pub nightstand: u8,
    pub window: u8,
    pub barrel: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    pub battling: u16,
    pub hunting: u16,
    pub mining: u16,
    pub woodcutting: u16,
    pub stonecarving: u16,
    pub crafting: u16,
    pub enchanting: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boosts {
    #[serde(rename = "critChance")]
    pub crit_chance: u16,

    #[serde(rename = "critDamage")]
    pub crit_damage: u16,

    pub multistrike: u16,
    pub healing: u16,
    pub hunting_boost: u16,
    pub mining_boost: u16,
    pub stonecarving_boost: u16,
    pub catacombs_elemental: u16,
    pub catacombs_health: u16,
    pub catacombs_damage: u16,
    pub catacombs_hit: u16,
    pub catacombs_dodge: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Partner {
    pub hunting: u16,
    pub mining: u16,
    pub woodcutting: u16,
    pub stonecarving: u16,
    pub intelligence: u16,
    pub speed: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatacombBanner {
    pub mobs: f32,
    pub reward_multiplier: f32,
    pub mob_multiplier: f32,
    pub character_multiplier: f32,
    pub water_resistance: f32,
    pub thunder_resistance: f32,
    pub fire_resistance: f32,
    pub melee_resistance: f32,
    pub ranged_resistance: f32,
    pub elemental_conversion: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatacombCharacter {
    pub health: u16,
    pub damage: u16,
    pub hit: u16,
    pub dodge: u16,
    pub crit_damage: u16,
    pub crit_chance: u16,
    pub water_damage: u16,
    pub thunder_damage: u16,
    pub fire_damage: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub player: Player,
    pub currency: Currency,
    pub house: House,
    pub skills: Skills,
    pub boosts: Boosts,
    pub partners: Vec<Partner>,

    #[serde(rename = "catacombBanner")]
    pub catacomb_banner: CatacombBanner,

    #[serde(rename = "catacombCharacter")]
    pub catacomb_character: CatacombCharacter,
}

pub struct Guardian {
    pub health: f32,
    pub damage: f32,
    pub hit: f32,
    pub dodge: f32,
    pub crit_damage: f32,
    pub crit_chance: f32,
}

