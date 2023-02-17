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

pub fn create_monster(tier: f32, cata_params: HashMap<String, String>, character_multiplier: f32, mob_multiplier: f32, mob_debuff: f32) -> Monster {
    let mut monster = Monster::default();
    monster.health = scale_by_tier(tier, mob_multiplier, mob_debuff, 500.0).round();
    monster.base_damage = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.hit = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.dodge = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0).round();
    monster.crit_chance = scale_by_tier(tier, mob_multiplier, mob_debuff, 0.1) / 100.0;
    monster.crit_damage = scale_by_tier(tier, mob_multiplier, mob_debuff, 0.1) /100.0;
    monster.water_damage = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0);
    monster.thunder_damage = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0);
    monster.fire_damage = scale_by_tier(tier, mob_multiplier, mob_debuff, 20.0);
    monster.damage = (monster.base_damage - monster.water_damage - monster.thunder_damage - monster.fire_damage).round();
    monster.max_health = scale_by_tier(tier, mob_multiplier, mob_debuff, 500.0);
    return monster;
}

fn scale_by_tier(tier: f32, mob_multiplier: f32, mob_debuff: f32, value_per_tier: f32) -> f32 {
    return value_per_tier * (tier + tier.powf(1.7)) / (1.0 + mob_multiplier) / (1.0 + mob_debuff);
}
