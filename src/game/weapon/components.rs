use crate::constants::{FOW_SIZE, MAP_SIZE};
use crate::game::enemy::components::Enemy;
use crate::game::map::components::FogOfWar;
use crate::game::resources::{GameSettings, Player, Resources};
use crate::utils::scale_duration;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponName {
    MachineGun,
    AAA,
    Flamethrower,
    Mortar,
    Turret,
}

#[derive(Clone)]
pub struct FireAnimation {
    /// Name of the asset for firing animation
    pub atlas: &'static str,

    /// Scaling factor for the image
    pub scale: Vec3,

    /// Duration of the animation timer
    pub duration: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FireStrategy {
    /// Don't fire
    None,

    /// Fire at the closest enemy
    Closest,

    /// Fire at enemy with the highest `max_health`
    Strongest,

    /// Fire at the enemy with the most surrounding enemies at
    /// a distance given by the explosion's `radius`
    Density,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AAAFireStrategy {
    None,
    All,
    Airborne,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MortarShell {
    None,
    Light,
    Heavy,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: WeaponName,

    /// Name of the asset for sprite
    pub image: &'static str,

    pub dim: Vec2,
    pub rotation_speed: f32,

    /// Entity to shoot towards
    pub lock: Option<Entity>,
    pub price: Resources,

    pub fire_animation: FireAnimation,

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
    /// Penetration also damages structures when in explosion
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
pub struct ExplosionInfo {
    /// Name of the asset for firing animation
    pub atlas: &'static str,

    /// Explosion radius
    pub radius: f32,

    /// Damage inflicted by the explosion
    pub damage: Damage,
}

#[derive(Clone)]
pub enum Detonation {
    /// Damage is applied to a single enemy
    SingleTarget(Damage),

    /// Piercing bullets don't despawn after hitting an enemy
    Piercing(Damage),

    /// Damage is applied to all enemies in radius `f32`
    Explosion(ExplosionInfo),
}

#[derive(Component, Clone)]
pub struct Bullet {
    /// Name of the asset for sprite
    pub image: &'static str,

    /// Dimensions (size) of the sprite
    pub dim: Vec2,

    /// Distance traveled per second
    pub speed: f32,
    pub angle: f32,
    pub detonation: Detonation,

    /// Maximum distance the bullet can travel (despawned after)
    pub max_distance: f32,

    /// Current distance traveled by the bullet
    pub distance: f32,
}

#[derive(Component, Clone)]
pub struct Mine {
    pub image: &'static str,
    pub atlas: &'static str,
    pub dim: Vec2,
    pub explosion: ExplosionInfo,
}

impl Weapon {
    pub fn get_lock<'a>(
        &mut self,
        transform: &Transform,
        enemy_q: &'a Query<(Entity, &Transform, &Enemy), (With<Enemy>, Without<Weapon>)>,
        fow_q: &Query<&Transform, (With<FogOfWar>, Without<Weapon>)>,
        player: &Player,
    ) -> Option<(&'a Transform, &'a Enemy)> {
        // If a target is locked and still exists, return it's current position
        if let Some(entity) = self.lock {
            if let Ok((_, t, e)) = enemy_q.get(entity) {
                return Some((t, e));
            }
        }

        let enemies = enemy_q.iter().filter_map(|(entity, enemy_t, enemy)| {
            // Ignore enemies behind the fow
            if let Some(fow_t) = fow_q.iter().next() {
                if fow_t.translation.y - FOW_SIZE.y * 0.5
                    < enemy_t.translation.y - enemy.dim.y * 0.5
                {
                    return None;
                }
            }

            // Special case => AAA's don't shoot ground units when strategy is Airborne
            if self.name == WeaponName::AAA
                && player.weapons.settings.aaa_fire_strategy == AAAFireStrategy::Airborne
                && !enemy.can_fly
            {
                return None;
            }

            let distance = transform.translation.distance(enemy_t.translation);
            if distance <= self.bullet.max_distance {
                Some((entity, enemy_t, enemy, distance))
            } else {
                None
            }
        });

        if let Some((entity, t, enemy, _)) = match self.fire_strategy {
            FireStrategy::None => None,
            FireStrategy::Closest => {
                enemies.min_by(|(_, _, _, d1), (_, _, _, d2)| d1.partial_cmp(d2).unwrap())
            }
            FireStrategy::Strongest => enemies.max_by(|(_, _, e1, _), (_, _, e2, _)| {
                e1.max_health.partial_cmp(&e2.max_health).unwrap()
            }),
            FireStrategy::Density => enemies.max_by_key(|(_, t1, _, _)| {
                if let Detonation::Explosion(x) = &self.bullet.detonation {
                    enemy_q
                        .iter()
                        .filter(|(_, &t2, _)| t1.translation.distance(t2.translation) < x.radius)
                        .count()
                } else {
                    panic!("Invalid detonation type for density fire strategy.")
                }
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
                    AAAFireStrategy::None => self.fire_strategy = FireStrategy::None,
                    AAAFireStrategy::All => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.detonation = Detonation::SingleTarget(Damage {
                            ground: 5.,
                            air: 5.,
                            penetration: 0.,
                        })
                    }
                    AAAFireStrategy::Airborne => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.detonation = Detonation::SingleTarget(Damage {
                            ground: 0.,
                            air: 20.,
                            penetration: 0.,
                        })
                    }
                };
            }
            WeaponName::Flamethrower => match player.weapons.settings.flamethrower_power {
                0 => self.fire_strategy = FireStrategy::None,
                _ => {
                    let power = player.weapons.settings.flamethrower_power as f32;

                    self.fire_strategy = FireStrategy::Closest;
                    self.fire_animation.scale.x = 1.5 + power * 0.5;
                    if let Some(timer) = self.fire_timer.as_mut() {
                        timer.set_duration(Duration::from_secs_f32(0.6 - power * 0.1));
                    }
                    self.bullet.max_distance = 100. * (1.5 + power * 0.5);
                    self.bullet.detonation = Detonation::Piercing(Damage {
                        ground: power,
                        air: power,
                        penetration: power,
                    });
                    self.fire_cost.gasoline = power;
                }
            },
            WeaponName::Mortar => {
                match player.weapons.settings.mortar_shell {
                    MortarShell::None => self.fire_strategy = FireStrategy::None,
                    MortarShell::Light => {
                        self.fire_strategy = FireStrategy::Density;
                        self.fire_cost = Resources {
                            bullets: 15.,
                            ..default()
                        };
                        self.bullet.detonation = Detonation::Explosion(ExplosionInfo {
                            atlas: "explosion1",
                            radius: 0.15 * MAP_SIZE.y,
                            damage: Damage {
                                ground: 50.,
                                air: 50.,
                                penetration: 0.,
                            },
                        })
                    }
                    MortarShell::Heavy => {
                        self.fire_strategy = FireStrategy::Density;
                        self.fire_cost = Resources {
                            bullets: 30.,
                            ..default()
                        };
                        self.bullet.detonation = Detonation::Explosion(ExplosionInfo {
                            atlas: "explosion1",
                            radius: 0.25 * MAP_SIZE.y,
                            damage: Damage {
                                ground: 75.,
                                air: 50.,
                                penetration: 25.,
                            },
                        })
                    }
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
    pub aaa: Weapon,
    pub flamethrower: Weapon,
    pub mortar: Weapon,
    pub turret: Weapon,
    pub mine: Mine,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::MachineGun => self.machine_gun.clone(),
            WeaponName::AAA => self.aaa.clone(),
            WeaponName::Flamethrower => self.flamethrower.clone(),
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
                image: "weapon/machine-gun.png",
                dim: Vec2::new(70., 70.),
                rotation_speed: 5.,
                lock: None,
                price: Resources {
                    materials: 100.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "single-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                fire_cost: Resources {
                    bullets: 1.,
                    ..default()
                },
                fire_timer: None,
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/bullet.png",
                    dim: Vec2::new(25., 7.),
                    speed: 0.8 * MAP_SIZE.y,
                    angle: 0.,
                    detonation: Detonation::SingleTarget(Damage {
                        ground: 5.,
                        air: 0.,
                        penetration: 0.,
                    }),
                    max_distance: 0.7 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            flamethrower: Weapon {
                name: WeaponName::Flamethrower,
                image: "weapon/flamethrower.png",
                dim: Vec2::new(60., 60.),
                rotation_speed: 5.,
                lock: None,
                price: Resources {
                    materials: 300.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "flame",
                    scale: Vec3::new(3., 1., 1.),
                    duration: 0.02,
                },
                fire_cost: Resources {
                    gasoline: 1.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/invisible-bullet.png",
                    dim: Vec2::new(20., 7.),
                    speed: 1.2 * MAP_SIZE.y,
                    angle: 0.,
                    detonation: Detonation::Piercing(Damage {
                        ground: 1.,
                        air: 1.,
                        penetration: 0.,
                    }),
                    max_distance: 0., // Is set by self.update()
                    distance: 0.,
                },
            },
            aaa: Weapon {
                name: WeaponName::AAA,
                image: "weapon/aaa.png",
                dim: Vec2::new(80., 80.),
                rotation_speed: 5.,
                lock: None,
                price: Resources {
                    materials: 300.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "single-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                fire_cost: Resources {
                    bullets: 5.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/shell.png",
                    dim: Vec2::new(20., 7.),
                    speed: 1.2 * MAP_SIZE.y,
                    angle: 0.,
                    detonation: Detonation::SingleTarget(Damage {
                        ground: 5.,
                        air: 5.,
                        penetration: 0.,
                    }),
                    max_distance: 1.2 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            mortar: Weapon {
                name: WeaponName::Mortar,
                image: "weapon/mortar.png",
                dim: Vec2::new(70., 70.),
                rotation_speed: 3.,
                lock: None,
                price: Resources {
                    materials: 400.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "wide-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                fire_cost: Resources {
                    bullets: 15.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(3., TimerMode::Once)),
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/mortar-bullet.png",
                    dim: Vec2::new(25., 10.),
                    speed: 0.6 * MAP_SIZE.y,
                    angle: 0.,
                    detonation: Detonation::Explosion(ExplosionInfo {
                        atlas: "explosion1",
                        radius: 0.15 * MAP_SIZE.y,
                        damage: Damage {
                            ground: 50.,
                            air: 50.,
                            penetration: 0.,
                        },
                    }),
                    max_distance: 1.8 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            turret: Weapon {
                name: WeaponName::Turret,
                image: "weapon/turret.png",
                dim: Vec2::new(90., 90.),
                rotation_speed: 3.,
                lock: None,
                price: Resources {
                    materials: 1000.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "triple-flash",
                    scale: Vec3::splat(0.6),
                    duration: 0.1,
                },
                fire_cost: Resources {
                    bullets: 30.,
                    ..default()
                },
                fire_timer: Some(Timer::from_seconds(2., TimerMode::Once)),
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/triple-bullet.png",
                    dim: Vec2::new(25., 25.),
                    speed: 0.6 * MAP_SIZE.y,
                    angle: 0.,
                    detonation: Detonation::SingleTarget(Damage {
                        ground: 50.,
                        air: 50.,
                        penetration: 10.,
                    }),
                    max_distance: 0.9 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            mine: Mine {
                image: "weapon/mine.png",
                atlas: "explosion1",
                dim: Vec2::new(30., 20.),
                explosion: ExplosionInfo {
                    atlas: "explosion1",
                    radius: 0.2 * MAP_SIZE.y,
                    damage: Damage {
                        ground: 50.,
                        air: 0.,
                        penetration: 20.,
                    },
                },
            },
        }
    }
}
