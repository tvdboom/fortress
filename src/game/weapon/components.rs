use crate::game::resources::Resources;
use bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WeaponId {
    SentryGun,
}

#[derive(Component)]
pub struct Weapon {
    pub id: WeaponId,
    pub fire_timer: Option<Timer>,
}

impl Weapon {
    pub fn new(id: &WeaponId) -> Self {
        Self {
            id: id.clone(),
            fire_timer: Some(Timer::from_seconds(1., TimerMode::Repeating)),
        }
    }

    pub fn update(&mut self, settings: &WeaponSettings) {
        let params = settings.get_params(&self.id);
        match self.id {
            WeaponId::SentryGun => {
                self.fire_timer = match params.fire_rate {
                    0 => None,
                    v => Some(Timer::from_seconds(1. / v as f32, TimerMode::Repeating)),
                };
            }
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Bullet {
    pub image: String,
    pub size: Vec2,
    pub speed: f32,
    pub angle: f32,
    pub damage: u32,
    pub max_distance: f32, // 0-100 as percentage of map's height
    pub distance: f32,     // Current distance traveled by the bullet
}

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

#[derive(Resource)]
pub struct WeaponSettings {
    pub sentry_gun: WeaponParams,
}

impl WeaponSettings {
    pub fn get_params(&self, id: &WeaponId) -> &WeaponParams {
        match *id {
            WeaponId::SentryGun => &self.sentry_gun,
        }
    }

    pub fn get_params_mut(&mut self, id: &WeaponId) -> &mut WeaponParams {
        match *id {
            WeaponId::SentryGun => &mut self.sentry_gun,
        }
    }
}

impl Default for WeaponSettings {
    fn default() -> Self {
        WeaponSettings {
            sentry_gun: WeaponParams {
                name: "Sentry gun".to_string(),
                image: "weapon/sentry-gun.png".to_string(),
                size: Vec2::new(110., 110.),
                price: Resources {
                    materials: 100,
                    ..default()
                },
                fire_rate: 1,
                max_fire_rate: 5,
                fire_cost: Resources {
                    bullets: 1,
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
            },
        }
    }
}
