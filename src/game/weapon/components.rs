use crate::game::resources::{GameSettings, Player, Resources};
use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponName {
    SentryGun,
}

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: WeaponName,
    pub image: String,
    pub size: Vec2,
    pub rotation_speed: f32,
    pub price: Resources,
    pub max_fire_rate: u32,
    pub fire_cost: Resources,
    pub fire_timer: Option<Timer>,
    pub bullet: Bullet,
}

#[derive(Component, Clone)]
pub struct Bullet {
    pub image: String,
    pub size: Vec2,
    pub speed: f32,
    pub angle: f32,
    pub damage: f32,
    pub max_distance: f32, // 0-100 as percentage of map's height
    pub distance: f32,     // Current distance traveled by the bullet
}

impl Weapon {
    pub fn can_fire(&mut self, time: &Res<Time>) -> bool {
        if let Some(ref mut timer) = &mut self.fire_timer {
            timer.tick(time.delta());
            return timer.finished();
        }
        false
    }

    pub fn rotate(
        &mut self,
        angle: &f32,
        transform: &mut Transform,
        game_settings: &GameSettings,
        time: &Time,
    ) -> bool {
        // Accept a 0.1 tolerance (in radians)
        if (angle - transform.rotation.to_euler(EulerRot::XYZ).2).abs() >= 0.1 {
            transform.rotation = transform.rotation.slerp(
                Quat::from_rotation_z(*angle),
                self.rotation_speed * game_settings.speed * time.delta_secs(),
            );
            return false;
        }
        true
    }

    pub fn update(&mut self, player: &Player, game_settings: &GameSettings) {
        match self.name {
            WeaponName::SentryGun => {
                self.fire_timer = match player.weapons.settings.sentry_gun_fire_rate {
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

#[derive(Resource)]
pub struct WeaponManager {
    pub sentry_gun: Weapon,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::SentryGun => self.sentry_gun.clone(),
        }
    }
}

impl Default for WeaponManager {
    fn default() -> Self {
        Self {
            sentry_gun: Weapon {
                name: WeaponName::SentryGun,
                image: "weapon/sentry-gun.png".to_string(),
                size: Vec2::new(110., 110.),
                rotation_speed: 5.,
                price: Resources {
                    materials: 100.,
                    ..default()
                },
                max_fire_rate: 5,
                fire_cost: Resources {
                    bullets: 1.,
                    ..default()
                },
                fire_timer: None,
                bullet: Bullet {
                    image: "weapon/bullet.png".to_string(),
                    size: Vec2::new(30., 5.),
                    speed: 60.,
                    angle: 0.,
                    damage: 5.,
                    max_distance: 70.,
                    distance: 0.,
                },
            },
        }
    }
}
