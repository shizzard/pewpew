pub mod parameters;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use parameters::*;
use rand::Rng;

use super::*;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WeaponShotParameters {
    fire_rate: f32,
    reload_rate: f32,
    mag_capacity: usize,
    spread_deg: f32,
    projectiles: usize,
    damage: WeaponShotDamage,
    shot_speed: f32,
    shot_size: Vec2,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum WeaponShotDamage {
    Direct(WeaponShotDamageDirect),
    Explosive(WeaponShotDamageExplosive),
}

impl Default for WeaponShotDamage {
    fn default() -> Self {
        Self::Direct(WeaponShotDamageDirect::default())
    }
}

impl WeaponShotDamage {
    pub const fn direct(damage: f32) -> Self {
        Self::Direct(WeaponShotDamageDirect { damage })
    }

    pub const fn explosive(damage: f32, damage_radius: f32) -> Self {
        Self::Explosive(WeaponShotDamageExplosive {
            damage,
            damage_radius,
            damage_degradation: WeaponShotDamageExplosiveDegradation::Linear,
        })
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WeaponShotDamageDirect {
    damage: f32,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WeaponShotDamageExplosive {
    damage: f32,
    damage_radius: f32,
    damage_degradation: WeaponShotDamageExplosiveDegradation,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum WeaponShotDamageExplosiveDegradation {
    #[default]
    Linear,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
enum TimerState {
    #[default]
    InterShot,
    Reload,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub timer: Timer,
    timer_state: TimerState,
    shots_left: usize,
    pub shot_params: WeaponShotParameters,
}

// This is a WIP hack not to pass enemy collider into `try_apply_damage`
use crate::encounter::arena::enemy::ENEMY_ENTITY_HEIGHT;
impl Weapon {
    fn new(shot_params: WeaponShotParameters) -> Self {
        Self {
            timer: Timer::from_seconds(1. / shot_params.fire_rate, TimerMode::Once),
            shots_left: shot_params.mag_capacity,
            shot_params,
            ..default()
        }
    }

    fn sprite_bundle(&self) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(self.shot_params.shot_size),
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        }
    }

    pub fn shotgun() -> Self {
        Self::new(SHOTGUN_SHOT_PARAMS)
    }

    pub fn rifle() -> Self {
        Self::new(RIFLE_SHOT_PARAMS)
    }

    pub fn machine_gun() -> Self {
        Self::new(MACHINE_GUN_SHOT_PARAMS)
    }

    pub fn autocannon() -> Self {
        Self::new(AUTOCANNON_SHOT_PARAMS)
    }

    pub fn cannon() -> Self {
        Self::new(CANNON_SHOT_PARAMS)
    }

    pub fn rocket() -> Self {
        Self::new(ROCKET_SHOT_PARAMS)
    }

    pub fn tick(&mut self, timer: impl AsRef<Time>) -> &mut Self {
        self.timer.tick(timer.as_ref().delta());
        self
    }

    pub fn shoot(
        &mut self,
        spawn_translation: Vec3,
        mut rng: ResMut<GlobalEntropy<WyRand>>,
    ) -> Option<(Vec<(Velocity, Transform)>, SpriteBundle)> {
        match (
            self.shots_left > 0,
            self.timer.finished(),
            &self.timer_state,
        ) {
            // still have shots in mag, and inter-shot timer is finished
            (true, true, TimerState::InterShot) => {
                self.timer.reset();
                self.shots_left -= 1;

                let mut shots = vec![];
                for _ in 0..self.shot_params.projectiles {
                    let mut transform = Transform::from_translation(spawn_translation);
                    let angle_range =
                        (-self.shot_params.spread_deg / 2.0)..(self.shot_params.spread_deg / 2.0);
                    transform.rotate_local_z(rng.gen_range(angle_range).to_radians());
                    shots.push((Velocity::new(self.shot_params.shot_speed), transform));
                }

                Some((shots, self.sprite_bundle()))
            }
            // no shots left, time to reload
            (false, _, TimerState::InterShot) => {
                self.timer_state = TimerState::Reload;
                self.timer
                    .set_duration(Duration::from_secs_f32(1. / self.shot_params.reload_rate));
                self.timer.reset();
                None
            }
            // reload finished, get back firing
            (_, true, TimerState::Reload) => {
                self.timer_state = TimerState::InterShot;
                self.timer
                    .set_duration(Duration::from_secs_f32(1. / self.shot_params.fire_rate));
                self.timer.reset();
                self.shots_left = self.shot_params.mag_capacity;
                None
            }
            // timer not finished yet
            (_, _, _) => None,
        }
    }

    pub fn try_apply_damage(
        &self,
        self_transform: &Transform,
        enemies: &mut Vec<(impl AsMut<Health>, &Transform)>,
    ) -> Option<f32> {
        match &self.shot_params.damage {
            // direct hit damage
            WeaponShotDamage::Direct(direct) => {
                for (enemy_health, enemy_transform) in enemies.into_iter() {
                    if self_transform
                        .translation
                        .distance(enemy_transform.translation)
                        <= ENEMY_ENTITY_HEIGHT / 2.
                    {
                        enemy_health.as_mut().actual -= direct.damage;
                        return Some(direct.damage);
                    }
                }
                None
            }

            // explosive damage
            WeaponShotDamage::Explosive(explosive) => {
                // detect if any enemy is triggering the shot explosive
                if !enemies.iter_mut().any(|(_, enemy_transform)| -> bool {
                    self_transform
                        .translation
                        .distance(enemy_transform.translation)
                        <= ENEMY_ENTITY_HEIGHT / 2.
                }) {
                    return None;
                }
                // calculate and apply damage
                let damage_done =
                    enemies
                        .iter_mut()
                        .fold(0., |acc, (enemy_health, enemy_transform)| -> f32 {
                            let enemy_distance = self_transform
                                .translation
                                .distance(enemy_transform.translation);
                            if enemy_distance <= explosive.damage_radius {
                                let damage = (1. - (enemy_distance / explosive.damage_radius))
                                    * explosive.damage;
                                enemy_health.as_mut().actual -= damage;
                                return acc + damage;
                            }
                            acc
                        });
                Some(damage_done)
            }
        }
    }
}
