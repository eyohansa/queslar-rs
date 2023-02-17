use std::collections::HashMap;

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
