use crate::game::resources::{GameSettings, Resources};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum Weapon {
    SentryGun { timer: Option<Timer> },
}

impl PartialEq for Weapon {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SentryGun { .. }, Self::SentryGun { .. }) => true,
        }
    }
}

impl Weapon {
    pub fn sentry_gun(settings: &WeaponSettings, game_settings: &GameSettings) -> Self {
        Self::SentryGun {
            timer: Some(Timer::from_seconds(
                1. / settings.sentry_gun.fire_rate as f32 / game_settings.speed,
                TimerMode::Repeating,
            )),
        }
    }

    pub fn can_fire(&mut self, time: &Res<Time>) -> bool {
        match self {
            Self::SentryGun { timer } => match timer {
                Some(t) => {
                    t.tick(time.delta());
                    t.finished()
                }
                None => false,
            },
        }
    }

    pub fn update(&mut self, weapon_params: &WeaponParams, game_settings: &GameSettings) {
        match self {
            Self::SentryGun { timer } => {
                *timer = match weapon_params.fire_rate {
                    0 => None,
                    v => Some(Timer::from_seconds(
                        1. / v as f32 / game_settings.speed,
                        TimerMode::Repeating,
                    )),
                };
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct Bullet {
    pub image: String,
    pub size: Vec2,
    pub speed: f32,
    pub angle: f32,
    pub damage: u32,
    pub max_distance: f32, // 0-100 as percentage of map's height
    pub distance: f32,     // Current distance traveled by the bullet
}

#[derive(Clone)]
pub struct WeaponParams {
    pub name: String,
    pub image: String,
    pub size: Vec2,
    pub price: Resources,
    pub fire_rate: u32,
    pub max_fire_rate: u32,
    pub fire_cost: Resources,
    pub bullet: Bullet,
}

#[derive(Clone)]
pub struct WeaponSettings {
    pub sentry_gun: WeaponParams,
}

impl WeaponSettings {
    pub fn get(&self, weapon: &Weapon) -> &WeaponParams {
        match weapon {
            Weapon::SentryGun { .. } => &self.sentry_gun,
        }
    }
}

impl Default for WeaponSettings {
    fn default() -> Self {
        Self {
            sentry_gun: {
                WeaponParams {
                    name: "Sentry gun".to_string(),
                    image: "weapon/sentry-gun.png".to_string(),
                    size: Vec2::new(110., 110.),
                    price: Resources {
                        materials: 100.,
                        ..default()
                    },
                    fire_rate: 1,
                    max_fire_rate: 5,
                    fire_cost: Resources {
                        bullets: 1.,
                        ..default()
                    },
                    bullet: Bullet {
                        image: "weapon/bullet.png".to_string(),
                        size: Vec2::new(30., 30.),
                        speed: 60.,
                        angle: 0.,
                        damage: 5,
                        max_distance: 70.,
                        distance: 0.,
                    },
                }
            },
        }
    }
}
