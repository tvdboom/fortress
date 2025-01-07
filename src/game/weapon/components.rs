use crate::game::enemy::components::Enemy;
use crate::game::resources::{GameSettings, Player, Resources};
use crate::utils::scale_duration;
use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponName {
    MachineGun,
    Turret,
}

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;

#[derive(Clone, Eq, PartialEq)]
pub enum FireStrategy {
    NoFire,
    Closest,
    Strongest,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: WeaponName,
    pub image: String,
    pub size: Vec2,
    pub rotation_speed: f32,
    pub price: Resources,
    pub fire_cost: Resources,
    pub fire_timer: Option<Timer>,
    pub fire_strategy: FireStrategy,
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

#[derive(Component, Clone)]
pub struct Landmine {
    pub image: String,
    pub size: Vec2,
    pub sensibility: f32,
    pub damage: f32,
    pub explosion_timer: Timer,
}

impl Weapon {
    pub fn select_target(
        &self,
        transform: &Transform,
        enemy_q: &Query<(&Transform, &Enemy)>,
        map_height: f32,
    ) -> Option<Transform> {
        let enemies = enemy_q.iter().filter_map(|(enemy_t, enemy)| {
            let distance = transform.translation.distance(enemy_t.translation);
            if distance <= map_height / 100. * self.bullet.max_distance {
                Some((enemy_t, enemy, distance))
            } else {
                None
            }
        });

        let enemies = match self.fire_strategy {
            FireStrategy::NoFire => None,
            FireStrategy::Closest => {
                enemies.min_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
            }
            FireStrategy::Strongest => enemies.max_by(|(_, e1, _), (_, e2, _)| {
                e1.max_health.partial_cmp(&e2.max_health).unwrap()
            }),
        };

        enemies.map(|(e, _, _)| e.clone())
    }

    /// Whether the weapon's timer is finished
    pub fn can_fire(&mut self, time: &Time, game_settings: &GameSettings) -> bool {
        if let Some(ref mut timer) = &mut self.fire_timer {
            timer.tick(scale_duration(time.delta(), game_settings.speed));
            if timer.finished() {
                timer.reset(); // Start reload
                return true;
            }
        }
        false
    }

    /// Whether the weapon points at the given angle
    pub fn is_aiming(&self, angle: &f32, transform: &Transform) -> bool {
        // Accept a 0.1 tolerance (in radians)
        (angle - transform.rotation.to_euler(EulerRot::XYZ).2).abs() < 0.1
    }

    /// Update the weapon's settings based on the player and game settings
    pub fn update(&mut self, player: &Player) {
        match self.name {
            WeaponName::MachineGun => {
                self.fire_timer = match player.weapons.settings.sentry_gun_fire_rate {
                    0 => None,
                    v => Some(Timer::from_seconds(1. / v as f32, TimerMode::Once)),
                };
            }
            WeaponName::Turret => {
                self.fire_strategy = player.weapons.settings.turret_fire_strategy.clone();
            }
        }
    }
}

#[derive(Resource)]
pub struct WeaponManager {
    pub machine_gun: Weapon,
    pub turret: Weapon,
    pub landmine: Landmine,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::MachineGun => self.machine_gun.clone(),
            WeaponName::Turret => self.turret.clone(),
        }
    }
}

impl Default for WeaponManager {
    fn default() -> Self {
        Self {
            machine_gun: Weapon {
                name: WeaponName::MachineGun,
                image: "weapon/machine-gun.png".to_string(),
                size: Vec2::new(70., 70.),
                rotation_speed: 5.,
                price: Resources {
                    materials: 100.,
                    ..default()
                },
                fire_cost: Resources {
                    bullets: 1.,
                    ..default()
                },
                fire_timer: None,
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/bullet.png".to_string(),
                    size: Vec2::new(25., 7.),
                    speed: 80.,
                    angle: 0.,
                    damage: 5.,
                    max_distance: 70.,
                    distance: 0.,
                },
            },
            turret: Weapon {
                name: WeaponName::Turret,
                image: "weapon/turret.png".to_string(),
                size: Vec2::new(90., 90.),
                rotation_speed: 3.,
                price: Resources {
                    materials: 1000.,
                    ..default()
                },
                fire_cost: Resources {
                    bullets: 30.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(2., TimerMode::Once)),
                fire_strategy: FireStrategy::NoFire,
                bullet: Bullet {
                    image: "weapon/triple-bullet.png".to_string(),
                    size: Vec2::new(25., 25.),
                    speed: 60.,
                    angle: 0.,
                    damage: 50.,
                    max_distance: 100.,
                    distance: 0.,
                },
            },
            landmine: Landmine {
                image: "weapon/landmine.png".to_string(),
                size: Vec2::new(30., 20.),
                sensibility: 50.,
                damage: 100.,
                explosion_timer: Timer::from_seconds(3., TimerMode::Once),
            },
        }
    }
}
