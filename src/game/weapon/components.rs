use crate::constants::MAP_SIZE;
use crate::game::enemy::components::Enemy;
use crate::game::resources::{GameSettings, Player, Resources};
use crate::utils::scale_duration;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponName {
    MachineGun,
    AAA,
    Mortar,
    Turret,
}

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FireStrategy {
    /// Don't fire
    NoFire,

    /// Fire at the closest enemy
    Closest,

    /// Fire at enemy with the highest `max_health`
    Strongest,

    /// Fire at the enemy with the most surrounding enemies at
    /// a distance given by the element of the tuple.
    /// When detonation is explosion, use the same value for radius
    Density(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AAAFireStrategy {
    NoFire,
    All,
    Airborne,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: WeaponName,
    pub image: String,
    pub dim: Vec2,
    pub rotation_speed: f32,

    /// Entity to shoot towards
    pub lock: Option<Entity>,
    pub price: Resources,

    /// Resources required to fire
    pub fire_cost: Resources,

    /// Time between shots (reload time)
    pub fire_timer: Option<Timer>,

    pub fire_strategy: FireStrategy,
    pub bullet: Bullet,
}

#[derive(Clone)]
pub struct Damage {
    /// Damage to ground enemies
    pub ground: f32,

    /// Damage to flying enemies
    pub air: f32,

    /// Armor penetration
    pub penetration: f32,
}

impl Damage {
    /// Calculate the damage inflicted on `enemy`
    pub fn calculate(&self, enemy: &Enemy) -> f32 {
        (if enemy.can_fly { self.air } else { self.ground })
            + (enemy.armor - self.penetration).max(0.)
    }
}

#[derive(Clone)]
pub enum Detonation {
    /// Damage is applied to a single enemy
    SingleTarget,

    /// Damage is applied to all enemies in radius `u32`
    Explosion(u32),
}

#[derive(Component, Clone)]
pub struct Bullet {
    pub image: String,
    pub dim: Vec2,

    /// Distance traveled per second
    pub speed: f32,
    pub angle: f32,
    pub damage: Damage,
    pub detonation: Detonation,

    /// Maximum distance the bullet can travel (despawned after)
    pub max_distance: f32,

    /// Current distance traveled by the bullet
    pub distance: f32,
}

#[derive(Component, Clone)]
pub struct Landmine {
    pub image: String,
    pub dim: Vec2,
    pub damage: Damage,
    pub detonation: Detonation,
}

impl Weapon {
    pub fn get_lock<'a>(
        &mut self,
        transform: &Transform,
        enemy_q: &'a Query<(&Transform, Entity, &Enemy)>,
        player: &Player,
    ) -> Option<(&'a Transform, &'a Enemy)> {
        // If a target is locked and still exists, return it's current position
        if let Some(entity) = self.lock {
            if let Ok((t, _, e)) = enemy_q.get(entity) {
                return Some((t, e));
            }
        }

        let enemies = enemy_q.iter().filter_map(|(enemy_t, entity, enemy)| {
            // Special case => AAA's don't shoot ground units when strategy is Airborne
            if self.name == WeaponName::AAA
                && player.weapons.settings.aaa_fire_strategy == AAAFireStrategy::Airborne
                && !enemy.can_fly
            {
                return None;
            }

            let distance = transform.translation.distance(enemy_t.translation);
            if distance <= self.bullet.max_distance {
                Some((enemy_t, entity, enemy, distance))
            } else {
                None
            }
        });

        if let Some((t, entity, enemy, _)) = match self.fire_strategy {
            FireStrategy::NoFire => None,
            FireStrategy::Closest => {
                enemies.min_by(|(_, _, _, d1), (_, _, _, d2)| d1.partial_cmp(d2).unwrap())
            }
            FireStrategy::Strongest => enemies.max_by(|(_, _, e1, _), (_, _, e2, _)| {
                e1.max_health.partial_cmp(&e2.max_health).unwrap()
            }),
            FireStrategy::Density(r) => enemies.max_by_key(|(t1, _, _, _)| {
                enemy_q
                    .iter()
                    .filter(|(&t2, _, _)| t1.translation.distance(t2.translation) < r as f32)
                    .count()
            }),
        } {
            self.lock = Some(entity);
            return Some((t, enemy));
        }

        None
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
                match player.weapons.settings.sentry_gun_fire_rate {
                    0 => self.fire_timer = None,
                    v => {
                        if let Some(ref mut timer) = self.fire_timer {
                            timer.set_duration(Duration::from_secs_f32(1. / v as f32));
                        } else {
                            self.fire_timer =
                                Some(Timer::from_seconds(1. / v as f32, TimerMode::Once))
                        }
                    }
                };
            }
            WeaponName::AAA => {
                match player.weapons.settings.aaa_fire_strategy {
                    AAAFireStrategy::NoFire => self.fire_strategy = FireStrategy::NoFire,
                    AAAFireStrategy::All => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.damage = Damage {
                            ground: 5.,
                            air: 5.,
                            penetration: 0.,
                        }
                    }
                    AAAFireStrategy::Airborne => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.damage = Damage {
                            ground: 0.,
                            air: 20.,
                            penetration: 0.,
                        }
                    }
                };
            }
            WeaponName::Mortar => (),
            WeaponName::Turret => {
                self.fire_strategy = player.weapons.settings.turret_fire_strategy.clone();
            }
        }
    }
}

#[derive(Resource)]
pub struct WeaponManager {
    pub machine_gun: Weapon,
    pub aaa: Weapon,
    pub mortar: Weapon,
    pub turret: Weapon,
    pub landmine: Landmine,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::MachineGun => self.machine_gun.clone(),
            WeaponName::AAA => self.aaa.clone(),
            WeaponName::Mortar => self.mortar.clone(),
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
                dim: Vec2::new(70., 70.),
                rotation_speed: 5.,
                lock: None,
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
                    dim: Vec2::new(25., 7.),
                    speed: 0.8 * MAP_SIZE.y,
                    angle: 0.,
                    damage: Damage {
                        ground: 5.,
                        air: 0.,
                        penetration: 0.,
                    },
                    detonation: Detonation::SingleTarget,
                    max_distance: 0.7 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            aaa: Weapon {
                name: WeaponName::AAA,
                image: "weapon/aaa.png".to_string(),
                dim: Vec2::new(80., 80.),
                rotation_speed: 5.,
                lock: None,
                price: Resources {
                    materials: 300.,
                    ..default()
                },
                fire_cost: Resources {
                    bullets: 5.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/shell.png".to_string(),
                    dim: Vec2::new(20., 7.),
                    speed: 1.2 * MAP_SIZE.y,
                    angle: 0.,
                    damage: Damage {
                        ground: 5.,
                        air: 5.,
                        penetration: 0.,
                    },
                    detonation: Detonation::SingleTarget,
                    max_distance: 1.2 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            mortar: Weapon {
                name: WeaponName::Mortar,
                image: "weapon/mortar.png".to_string(),
                dim: Vec2::new(70., 70.),
                rotation_speed: 3.,
                lock: None,
                price: Resources {
                    materials: 400.,
                    ..default()
                },
                fire_cost: Resources {
                    bullets: 15.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(3., TimerMode::Once)),
                fire_strategy: FireStrategy::Density((0.03 * MAP_SIZE.y) as u32),
                bullet: Bullet {
                    image: "weapon/mortar-bullet.png".to_string(),
                    dim: Vec2::new(25., 10.),
                    speed: 0.6 * MAP_SIZE.y,
                    angle: 0.,
                    damage: Damage {
                        ground: 50.,
                        air: 0.,
                        penetration: 5.,
                    },
                    detonation: Detonation::Explosion((0.03 * MAP_SIZE.y) as u32),
                    max_distance: 1.8 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            turret: Weapon {
                name: WeaponName::Turret,
                image: "weapon/turret.png".to_string(),
                dim: Vec2::new(90., 90.),
                rotation_speed: 3.,
                lock: None,
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
                    dim: Vec2::new(25., 25.),
                    speed: 0.6 * MAP_SIZE.y,
                    angle: 0.,
                    damage: Damage {
                        ground: 50.,
                        air: 0.,
                        penetration: 10.,
                    },
                    detonation: Detonation::SingleTarget,
                    max_distance: 0.9 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            landmine: Landmine {
                image: "weapon/landmine.png".to_string(),
                dim: Vec2::new(30., 20.),
                damage: Damage {
                    ground: 50.,
                    air: 0.,
                    penetration: 20.,
                },
                detonation: Detonation::Explosion((0.05 * MAP_SIZE.y) as u32),
            },
        }
    }
}
