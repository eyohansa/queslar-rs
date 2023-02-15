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
    max_health: u32,
}

pub fn create_monster(tier: f32, character_multiplier: f32, mob_multiplier: f32, mob_debuff: f32) -> Monster {
    let mut monster = Monster::default();
    let health = 500.0 * (tier + tier.powf(1.7)) / (1.0 + mob_multiplier) / (1.0 + mob_debuff);
    monster.health = health.round();
    let base_damage =
        20.0 * (tier + tier.powf(1.7)) / (1.0 + mob_multiplier) / (1.0 + character_multiplier);
    monster.base_damage = base_damage.round();
    return monster;
}

