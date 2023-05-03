use std::collections::HashMap;
use rand;

use crate::queslar::{CatacombBanner};

#[derive(Default, Debug)]
pub struct Monster {
    water_dist: f32,
    fire_dist: f32,
    thunder_dist: f32,
    mob_multiplier: f32,
    mob_scaling: f32,
    health: f32,
    base_damage: f32,
    damage: f32,
    hit: f32,
    dodge: f32,
    crit_chance: f32,
    crit_damage: f32,
    water_damage: f32,
    fire_damage: f32,
    thunder_damage: f32,
    water_resistance: f32,
    fire_resistance: f32,
    thunder_resistance: f32,
    max_health: f32,
}

pub fn create_monster(tier: f32, cata_params: HashMap<String, String>, mob_multiplier: f32, mob_debuff: f32) -> Monster {
    let mut monster = Monster::default();
    
    let ele_dist_str = cata_params["Elemental distribution"].clone();
    let ele_dist = ele_dist_str.split("-");
    for mut ele in ele_dist {
        ele = ele.trim();
        if ele.ends_with('W') {    
            monster.water_dist = ele.trim_end_matches("% W").parse::<f32>().unwrap() / 100.0;
        } else if ele.ends_with('T') {
            monster.thunder_dist = ele.trim_end_matches("% T").parse::<f32>().unwrap() / 100.0;
        } else if ele.ends_with('F') {
            monster.fire_dist = ele.trim_end_matches("% F").parse::<f32>().unwrap() / 100.0;
        }
    }

    monster.water_resistance = cata_params["Water resistance"].trim().trim_end_matches("%").parse::<f32>().unwrap() / 100.0;
    monster.thunder_resistance = cata_params["Thunder resistance"].trim().trim_end_matches("%").parse::<f32>().unwrap() / 100.0;
    monster.fire_resistance = cata_params["Fire resistance"].trim().trim_end_matches("%").parse::<f32>().unwrap() / 100.0;
    
    monster.health = increment_by_tier(tier, mob_multiplier, mob_debuff, 500.0).round();
    monster.base_damage = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.hit = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.dodge = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.crit_chance = increment_by_tier(tier, mob_multiplier, mob_debuff, 0.1).round() / 100.0;
    monster.crit_damage = increment_by_tier(tier, mob_multiplier, mob_debuff, 0.1).round() /100.0;
    monster.water_damage = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round() * monster.water_dist;
    monster.thunder_damage = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round() * monster.thunder_dist;
    monster.fire_damage = increment_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round() * monster.fire_dist;
    monster.damage = (monster.base_damage - monster.water_damage - monster.thunder_damage - monster.fire_damage).round();
    monster.max_health = increment_by_tier(tier, mob_multiplier, mob_debuff, 500.0);
    return monster;
}

fn increment_by_tier(tier: f32, mob_multiplier: f32, mob_debuff: f32, value_per_tier: f32) -> f32 {
    return value_per_tier * (tier + tier.powf(1.7)) / (1.0 + mob_multiplier) / (1.0 + mob_debuff);
}

pub struct Fighter {
    pub hit: f32,
    pub dodge: f32,
    pub health: f32,
    pub max_health: f32
}

impl Fighter {
    pub fn init(&mut self) {
        self.health = self.max_health;
    }
}

pub fn simulate_battle(mut fighter: Fighter, mut monster: Monster, replays: Vec<i32>, innerRounds: i32, rounds: i32, tier: f32) {
    let fighter_hit: f32 = fighter.hit / (fighter.hit + monster.dodge);
    let monster_hit = monster.hit / (monster.hit + fighter.dodge);
    let mut rounds_survived = 0;
    let mut health_pool = 0;

    for y in 0..replays.len() {
        fighter.init();

        for i in 0..rounds {
            monster.health = monster.max_health;

            while monster.health > 0.0 && fighter.health > 0.0 {
                rounds_survived += 1;

                for x in 0..innerRounds {
                    let fighter_rand = rand::thread_rng();
                    let monster_rand = rand::thread_rng();


                }
            }  
        }
    }
}

struct Character {
    health: f32,
    damage: f32,
    hit: f32,
    dodge: f32,
    crit_damage: f32,
    crit_chance: f32,
    water_damage: f32,
    thunder_damage: f32,
    fire_damage: f32,
    water_resistance: f32,
    thunder_resistance: f32,
    fire_resistance: f32,
    melee: f32,
    ranged: f32,
    elemental_conversion: f32,
    max_health: f32,
    lifesteal: f32,
    multi_mob: f32,
    mob_skip: f32,
    speed: f32,
    mobs_debuff: f32
}

struct Tome {
    mobs: f32,
    reward_multi: f32,
    mob_debuff: f32,
    char_multi: f32,
    water_resistance: f32,
    thunder_resistance: f32,
    fire_resistance: f32,
    melee_resistance: f32,
    ranged_resistance: f32,
    elemental_conversion: f32,
    multi_mob: f32,
    mob_skip: f32,
    lifesteal: f32,
    action_speed: f32,
}

impl Character {
    pub fn new(
        &mut self, 
        level: f32, 
        relic_boost: f32, 
        banner: &CatacombBanner, 
        tome: &Tome, 
        guardian: &Guardian, 
        blacksmith: f32, 
        hp: f32, 
        damage: f32, 
        hit: f32, 
        dodge: f32, 
        mob: f32, 
        multi: f32
    ) -> Character {
        let mut char_multi: f32 = banner.character_multiplier.clone() + tome.char_multi.clone():

    }
}