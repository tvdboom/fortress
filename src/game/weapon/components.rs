use crate::constants::{EnemyQ, SpriteQ, MAP_SIZE};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::utils::{EnemyOperations, EnemySelection};
use crate::game::map::components::FogOfWar;
use crate::game::resources::{GameSettings, Player, Resources};
use crate::utils::scale_duration;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;
use bevy::reflect::List;

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
    MissileLauncher,
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
    /// Name of the weapon
    pub name: WeaponName,

    /// Name of the asset for sprite
    pub image: &'static str,

    /// Dimensions (size) of the sprite
    pub dim: Vec2,

    /// Rotation speed in radians per second
    pub rotation_speed: f32,

    /// Price to buy the weapon
    pub price: Resources,

    /// Animation to play when firing
    pub fire_animation: FireAnimation,

    /// Number of bullets fired per shot
    pub n_bullets: u32,

    /// Target entities to point to (length is either zero or `n_bullets`)
    pub target: Vec<Entity>,

    /// Time between shots (reload time)
    pub fire_timer: Option<Timer>,

    /// Strategy to select a target
    pub fire_strategy: FireStrategy,

    /// Bullet fired by the weapon
    pub bullet: Bullet,
}

#[derive(Clone)]
pub struct Damage {
    /// Damage to ground enemies
    pub ground: f32,

    /// Damage to flying enemies
    pub air: f32,

    /// Armor penetration. Also damages structures if in explosion
    pub penetration: f32,
}

impl Default for Damage {
    fn default() -> Self {
        Self {
            ground: 0.,
            air: 0.,
            penetration: 0.,
        }
    }
}

impl Damage {
    /// Calculate the damage inflicted on `enemy`
    pub fn calculate(&self, enemy: &Enemy) -> f32 {
        (if enemy.can_fly { self.air } else { self.ground })
            + (enemy.armor - self.penetration).max(0.)
    }
}

#[derive(Clone)]
pub struct Explosion {
    /// Name of the asset for firing animation
    pub atlas: &'static str,

    /// Interval between frames (in seconds)
    pub interval: f32,

    /// Explosion radius
    pub radius: f32,

    /// Damage inflicted by the explosion
    pub damage: Damage,
}

impl Default for Explosion {
    fn default() -> Self {
        Self {
            atlas: "explosion",
            interval: 0.01,
            radius: 0.,
            damage: default(),
        }
    }
}

#[derive(Clone)]
pub enum Movement {
    /// Bullets flies straight at `angle`
    Straight,

    /// Bullets homes in on `Entity`
    Homing(Entity),
}

#[derive(Clone)]
pub enum Impact {
    /// Damage is applied to a single enemy
    SingleTarget(Damage),

    /// Piercing bullets don't despawn after hitting an enemy
    Piercing(Damage),

    /// Explodes after colliding with an enemy
    OnHitExplosion(Explosion),

    /// Explodes after reaching `max_distance`
    OnLocationExplosion(Explosion),

    /// Explodes after reaching a specific target
    OnTargetExplosion(Explosion),
}

#[derive(Component, Clone)]
pub struct Bullet {
    /// Name of the asset for sprite
    pub image: &'static str,

    /// Dimensions (size) of the sprite
    pub dim: Vec2,

    /// Cost per bullet
    pub price: Resources,

    /// Distance traveled per second
    pub speed: f32,

    /// Movement type
    pub movement: Movement,

    /// Angle (in radians) the bullet points to
    pub angle: f32,

    /// Impact type (what happens on collision)
    pub impact: Impact,

    /// Current distance traveled by the bullet
    pub distance: f32,

    /// The maximum distance the bullet can travel (despawn after)
    pub max_distance: f32,
}

impl Weapon {

    /// Acquire the targets to fire at. If `self.target` is empty, it will
    /// select new targets based on the `fire_strategy` and `fire_range`.
    pub fn acquire_targets(
        &mut self,
        transform: &Transform,
        enemy_q: &Query<EnemyQ, (With<Enemy>, Without<Weapon>)>,
        fow_q: &Query<SpriteQ, (With<FogOfWar>, Without<Weapon>)>,
        player: &Player,
    ) -> Option<Vec<(EnemyQ)>> {
        if self.target.is_empty() {
            return enemy_q.filter(|(enemy_e, _, _)| self.target.contains(enemy_e)).collect();
        }

        let enemies = enemy_q.iter().filter_map(|enemy_q| {
            // Check if the enemy is behind the fog of war
            if !enemy_q.is_visible(fow_q.get_single()) {
                return None
            }

            // Special case => AAA's don't shoot ground units when strategy is Airborne
            if self.name == WeaponName::AAA
                && player.weapons.settings.aaa_fire_strategy == AAAFireStrategy::Airborne
                && !enemy_q.2.can_fly
            {
                return None;
            }

            // Check if the enemy is in range
            let distance = transform.translation.distance(enemy_q.1.translation);
            if distance > self.bullet.max_distance {
                return None;
            }

            Some((enemy_q, distance))
        });

        let mut targets = match self.fire_strategy {
            FireStrategy::None => vec![],
            FireStrategy::Closest => enemies.sort_closest(),
            FireStrategy::Strongest => enemies.sort_strongest(),
            FireStrategy::Density => enemies.sort_densest(&self.bullet.impact),
        }.take(..self.n_bullets).collect();

        // Assign new targets
        if !targets.is_empty() {
            self.target = targets.map(|(e, _, _)| e).collect();
        }

        targets
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
                        self.bullet.impact = Impact::SingleTarget(Damage {
                            ground: 5.,
                            air: 5.,
                            penetration: 0.,
                        })
                    }
                    AAAFireStrategy::Airborne => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.impact = Impact::SingleTarget(Damage {
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
                    self.bullet.impact = Impact::Piercing(Damage {
                        ground: power,
                        air: power,
                        penetration: power,
                    });
                    self.bullet.price.gasoline = power;
                }
            },
            WeaponName::Mortar => {
                match player.weapons.settings.mortar_shell {
                    MortarShell::None => self.fire_strategy = FireStrategy::None,
                    MortarShell::Light => {
                        self.fire_strategy = FireStrategy::Density;
                        self.bullet.price = Resources {
                            bullets: 15.,
                            ..default()
                        };
                        self.bullet.impact = Impact::OnLocationExplosion(Explosion {
                            radius: 0.15 * MAP_SIZE.y,
                            damage: Damage {
                                ground: 50.,
                                air: 50.,
                                penetration: 0.,
                            },
                            ..default()
                        })
                    }
                    MortarShell::Heavy => {
                        self.fire_strategy = FireStrategy::Density;
                        self.bullet.price = Resources {
                            bullets: 30.,
                            ..default()
                        };
                        self.bullet.impact = Impact::OnLocationExplosion(Explosion {
                            radius: 0.25 * MAP_SIZE.y,
                            damage: Damage {
                                ground: 75.,
                                air: 50.,
                                penetration: 25.,
                            },
                            ..default()
                        })
                    }
                };
            }
            WeaponName::Turret => {
                self.fire_strategy = player.weapons.settings.turret_fire_strategy.clone();
            }
            WeaponName::MissileLauncher => {
                self.fire_strategy = FireStrategy::Closest;
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
    pub missile_launcher: Weapon,
    pub bomb: Bullet,
    pub mine: Bullet,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::MachineGun => self.machine_gun.clone(),
            WeaponName::AAA => self.aaa.clone(),
            WeaponName::Flamethrower => self.flamethrower.clone(),
            WeaponName::Mortar => self.mortar.clone(),
            WeaponName::Turret => self.turret.clone(),
            WeaponName::MissileLauncher => self.missile_launcher.clone(),
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
                target: vec![],
                price: Resources {
                    materials: 100.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "single-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: None,
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/bullet.png",
                    dim: Vec2::new(25., 7.),
                    price: Resources {
                        bullets: 1.,
                        ..default()
                    },
                    speed: 0.8 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    angle: 0.,
                    impact: Impact::SingleTarget(Damage {
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
                target: vec![],
                price: Resources {
                    materials: 300.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "flame",
                    scale: Vec3::new(3., 1., 1.),
                    duration: 0.02,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/invisible-bullet.png",
                    dim: Vec2::new(20., 7.),
                    price: Resources {
                        gasoline: 1.,
                        ..default()
                    },
                    speed: 1.2 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    angle: 0.,
                    impact: Impact::Piercing(Damage {
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
                target: vec![],
                price: Resources {
                    materials: 300.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "single-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(0.5, TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/shell.png",
                    dim: Vec2::new(20., 7.),
                    price: Resources {
                        bullets: 5.,
                        ..default()
                    },
                    speed: 1.2 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    angle: 0.,
                    impact: Impact::SingleTarget(Damage {
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
                target: vec![],
                price: Resources {
                    materials: 400.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "wide-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(3., TimerMode::Once)),
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/grenade.png",
                    dim: Vec2::new(25., 10.),
                    price: Resources {
                        bullets: 15.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    angle: 0.,
                    impact: Impact::OnLocationExplosion(Explosion {
                        radius: 0.15 * MAP_SIZE.y,
                        damage: Damage {
                            ground: 50.,
                            air: 50.,
                            penetration: 0.,
                        },
                        ..default()
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
                target: vec![],
                price: Resources {
                    materials: 1000.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "triple-flash",
                    scale: Vec3::splat(0.6),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(2., TimerMode::Once)),
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/triple-bullet.png",
                    dim: Vec2::new(25., 25.),
                    price: Resources {
                        bullets: 30.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    angle: 0.,
                    impact: Impact::SingleTarget(Damage {
                        ground: 50.,
                        air: 50.,
                        penetration: 10.,
                    }),
                    max_distance: 0.9 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            missile_launcher: Weapon {
                name: WeaponName::MissileLauncher,
                image: "weapon/missile-launcher.png",
                dim: Vec2::new(90., 90.),
                rotation_speed: 3.,
                target: vec![],
                price: Resources {
                    materials: 1200.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "wide-flash",
                    scale: Vec3::splat(0.7),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(3., TimerMode::Once)),
                fire_strategy: FireStrategy::Strongest,
                bullet: Bullet {
                    image: "weapon/grenade.png",
                    dim: Vec2::new(20., 10.),
                    price: Resources {
                        bullets: 15.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Homing(Entity::from_raw(0)), // Set by spawn_bullet
                    angle: 0.,
                    impact: Impact::OnTargetExplosion(Explosion {
                        radius: 0.1 * MAP_SIZE.y,
                        damage: Damage {
                            ground: 30.,
                            air: 30.,
                            penetration: 5.,
                        },
                        ..default()
                    }),
                    max_distance: 1.8 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            bomb: Bullet {
                image: "weapon/bomb.png",
                dim: Vec2::new(30., 20.),
                price: Resources {
                    bullets: 250.,
                    gasoline: 250.,
                    ..default()
                },
                speed: 0.4 * MAP_SIZE.y,
                movement: Movement::Straight,
                angle: -PI * 0.5,
                impact: Impact::OnLocationExplosion(Explosion {
                    interval: 0.05,
                    radius: 0.7 * MAP_SIZE.y,
                    damage: Damage {
                        ground: 80.,
                        air: 80.,
                        penetration: 20.,
                    },
                    ..default()
                }),
                max_distance: 0., // Set during spawn
                distance: 0.,
            },
            mine: Bullet {
                image: "weapon/mine.png",
                dim: Vec2::new(30., 20.),
                price: Resources {
                    bullets: 25.,
                    gasoline: 25.,
                    ..default()
                },
                speed: 0.,
                movement: Movement::Straight,
                angle: 0.,
                impact: Impact::OnHitExplosion(Explosion {
                    interval: 0.05,
                    radius: 0.2 * MAP_SIZE.y,
                    damage: Damage {
                        ground: 50.,
                        air: 0.,
                        penetration: 20.,
                    },
                    ..default()
                }),
                max_distance: f32::MAX,
                distance: 0.,
            },
        }
    }
}
