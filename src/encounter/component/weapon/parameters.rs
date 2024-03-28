use super::*;

pub const SHOTGUN_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 1.,
    reload_rate: 0.6,
    mag_capacity: 5,
    spread_deg: 30.,
    projectiles: 10,
    damage: WeaponShotDamage::direct(10.),
    shot_speed: 500.,
    shot_size: Vec2::new(2., 2.),
};
pub const RIFLE_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 10.,
    reload_rate: 0.6,
    mag_capacity: 30,
    spread_deg: 5.,
    projectiles: 1,
    damage: WeaponShotDamage::direct(15.),
    shot_speed: 500.,
    shot_size: Vec2::new(2., 6.),
};
pub const MACHINE_GUN_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 15.,
    reload_rate: 0.2,
    mag_capacity: 100,
    spread_deg: 7.,
    projectiles: 1,
    damage: WeaponShotDamage::direct(15.),
    shot_speed: 500.,
    shot_size: Vec2::new(2., 6.),
};
pub const AUTOCANNON_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 3.,
    reload_rate: 0.2,
    mag_capacity: 10,
    spread_deg: 5.,
    projectiles: 1,
    damage: WeaponShotDamage::explosive(50., 100.),
    shot_speed: 400.,
    shot_size: Vec2::new(3., 9.),
};
pub const CANNON_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 1.,
    reload_rate: 0.5,
    mag_capacity: 1,
    spread_deg: 5.,
    projectiles: 1,
    damage: WeaponShotDamage::explosive(150., 150.),
    shot_speed: 400.,
    shot_size: Vec2::new(5., 11.),
};
pub const ROCKET_SHOT_PARAMS: WeaponShotParameters = WeaponShotParameters {
    fire_rate: 1.,
    reload_rate: 0.3,
    mag_capacity: 1,
    spread_deg: 5.,
    projectiles: 1,
    damage: WeaponShotDamage::explosive(200., 200.),
    shot_speed: 200.,
    shot_size: Vec2::new(7., 13.),
};
