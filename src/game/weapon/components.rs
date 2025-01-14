use crate::constants::{EnemyQ, EXPLOSION_Z, MAP_SIZE, SIZE, WEAPONS_PANEL_SIZE};
use crate::game::assets::WorldAssets;
use crate::game::enemy::components::Enemy;
use crate::game::map::components::{AnimationComponent, FogOfWar};
use crate::game::map::utils::is_visible;
use crate::game::resources::{GameSettings, Player, Resources};
use crate::utils::scale_duration;
use bevy::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Wall;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeaponName {
    AAA,
    Artillery,
    Canon,
    Flamethrower,
    MachineGun,
    MissileLauncher,
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
pub enum AirFireStrategy {
    None,
    All,
    Grounded,
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

    /// Target entitiy to point to
    pub target: Option<Entity>,

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
            atlas: "explosion1",
            interval: 0.01,
            radius: 0.,
            damage: default(),
        }
    }
}

#[derive(Clone)]
pub enum Movement {
    /// Bullets impacts at fist enemy hit
    Straight,

    /// Bullet impacts at location
    Location(Vec3),

    /// Bullets impacts on `Entity`
    Homing(Entity),
}

#[derive(Clone)]
pub enum Impact {
    /// Damage is applied to a single enemy
    SingleTarget(Damage),

    /// Piercing bullets don't despawn after hitting an enemy
    Piercing(Damage),

    /// Explodes after colliding with an enemy
    Explosion(Explosion),
}

impl Impact {
    /// Resolve the impact of the bullet on the enemy
    /// Return whether the impact was resolved
    pub fn resolve(
        &self,
        commands: &mut Commands,
        bullet_e: Entity,
        bullet_t: &Transform,
        enemy: Option<(Entity, &mut Enemy)>,
        assets: &Local<WorldAssets>,
    ) -> bool {
        match self {
            Impact::SingleTarget(d) | Impact::Piercing(d) => {
                let (_, enemy) = enemy.expect("No enemy to resolve impact.");

                if (d.ground > 0. && !enemy.can_fly) || (d.air > 0. && enemy.can_fly) {
                    enemy.health -= d.calculate(enemy).min(enemy.health);

                    // Piercing bullets don't despawn on impact
                    if matches!(self, Impact::SingleTarget { .. }) {
                        commands.entity(bullet_e).try_despawn();
                    }

                    return true;
                }
            }
            Impact::Explosion(e) => {
                // If an enemy is passed, check it can trigger the explosion
                // E.g., a mine can collide with a flying enemy, and it shouldn't explode
                if let Some((_, enemy)) = enemy {
                    if (e.damage.ground == 0. && !enemy.can_fly)
                        || (e.damage.air == 0. && enemy.can_fly)
                    {
                        return false;
                    }
                }

                commands.entity(bullet_e).try_despawn();

                let atlas_asset = assets.get_atlas(e.atlas);
                commands.spawn((
                    Sprite {
                        image: atlas_asset.image,
                        texture_atlas: Some(atlas_asset.texture),
                        custom_size: Some(Vec2::splat(2. * e.radius)),
                        ..default()
                    },
                    Transform::from_xyz(
                        bullet_t.translation.x,
                        bullet_t.translation.y,
                        EXPLOSION_Z,
                    ),
                    AnimationComponent {
                        timer: Timer::from_seconds(e.interval, TimerMode::Repeating),
                        last_index: atlas_asset.last_index,
                        explosion: Some(e.clone()),
                    },
                ));

                return true;
            }
        }

        false
    }
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

    /// Impact type (what happens on collision)
    pub impact: Impact,

    /// Current distance traveled by the bullet
    pub distance: f32,

    /// The maximum distance the bullet can travel (despawn after)
    pub max_distance: f32,
}

impl Weapon {
    /// Acquire a target to fire at. If `self.target` is empty, it will
    /// select a new target excluding the entities in `exclusions`.
    pub fn acquire_target(
        &self,
        transform: &Transform,
        enemy_q: &Query<EnemyQ, (With<Enemy>, Without<Weapon>)>,
        fow_q: &Query<&Transform, (With<FogOfWar>, Without<Weapon>)>,
        exclusions: &HashSet<Entity>,
    ) -> Option<Entity> {
        // Return target if it's already acquired, it still exists and it's still visible
        if let Some(enemy_e) = self.target.and_then(|enemy_e| {
            if let Ok((enemy_e, enemy_t, enemy)) = enemy_q.get(enemy_e) {
                if is_visible(&fow_q.get_single().unwrap(), enemy_t, enemy)
                    && !exclusions.contains(&enemy_e)
                {
                    return Some(enemy_e);
                }
            }
            None
        }) {
            return Some(enemy_e);
        }

        let targets: Vec<(EnemyQ, f32)> = enemy_q
            .iter()
            .filter_map(|(enemy_e, enemy_t, enemy)| {
                // Check if the enemy is behind the fog of war
                if !is_visible(fow_q.get_single().unwrap(), enemy_t, enemy) {
                    return None;
                }

                // Don't shoot flying units when the bullet has only ground damage and vice versa
                match &self.bullet.impact {
                    Impact::SingleTarget(d)
                    | Impact::Piercing(d)
                    | Impact::Explosion(Explosion { damage: d, .. }) => {
                        if (d.ground == 0. && !enemy.can_fly) || (d.air == 0. && enemy.can_fly) {
                            return None;
                        }
                    }
                }

                // Check if the enemy is in range
                let distance = transform.translation.distance(enemy_t.translation);
                if distance > self.bullet.max_distance {
                    return None;
                }

                // Remove exclusions
                if exclusions.contains(&enemy_e) {
                    return None;
                }

                Some(((enemy_e, enemy_t, enemy), distance))
            })
            .collect();

        match self.fire_strategy {
            FireStrategy::None => None,
            FireStrategy::Closest => targets
                .iter()
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                .map(|((enemy_e, _, _), _)| *enemy_e),
            FireStrategy::Strongest => targets
                .iter()
                .max_by(|((_, _, e1), _), ((_, _, e2), _)| {
                    e1.max_health.partial_cmp(&e2.max_health).unwrap()
                })
                .map(|((enemy_e, _, _), _)| *enemy_e),
            FireStrategy::Density => {
                if let Impact::Explosion(e) = &self.bullet.impact {
                    targets
                        .iter()
                        .max_by(|((_, t1, _), _), ((_, t2, _), _)| {
                            let density_a = targets
                                .iter()
                                .filter(|((_, t, _), _)| {
                                    t1.translation.distance(t.translation) <= e.radius
                                })
                                .count();

                            let density_b = targets
                                .iter()
                                .filter(|((_, t, _), _)| {
                                    t2.translation.distance(t.translation) <= e.radius
                                })
                                .count();

                            density_a.cmp(&density_b)
                        })
                        .map(|((enemy_e, _, _), _)| *enemy_e)
                } else {
                    panic!("Invalid impact type for FireStrategy::Density, expected Explosion.")
                }
            }
        }
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
            WeaponName::AAA => {
                // Reset the target to avoid one last shot at the wrong enemy
                self.target = None;

                match player.weapons.settings.aaa {
                    AirFireStrategy::None => self.fire_strategy = FireStrategy::None,
                    AirFireStrategy::All => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.impact = Impact::SingleTarget(Damage {
                            ground: 5.,
                            air: 5.,
                            penetration: 0.,
                        })
                    }
                    AirFireStrategy::Airborne => {
                        self.fire_strategy = FireStrategy::Closest;
                        self.bullet.impact = Impact::SingleTarget(Damage {
                            ground: 0.,
                            air: 20.,
                            penetration: 0.,
                        })
                    }
                    _ => unreachable!(),
                };
            }
            WeaponName::Artillery => {
                self.target = None;
                self.fire_strategy = player.weapons.settings.artillery.clone();
            }
            WeaponName::Canon => {
                // Reset the target to avoid one last shot at the wrong enemy
                self.target = None;

                match player.weapons.settings.canon {
                    AirFireStrategy::None => self.fire_strategy = FireStrategy::None,
                    AirFireStrategy::Grounded => {
                        self.fire_strategy = FireStrategy::Closest;
                        if let Impact::Explosion(ref mut explosion) = self.bullet.impact {
                            explosion.damage = Damage {
                                ground: 20.,
                                air: 0.,
                                penetration: 0.,
                            };
                        }
                    }
                    AirFireStrategy::Airborne => {
                        self.fire_strategy = FireStrategy::Closest;
                        if let Impact::Explosion(ref mut explosion) = self.bullet.impact {
                            explosion.damage = Damage {
                                ground: 0.,
                                air: 20.0,
                                penetration: 0.,
                            };
                        }
                    }
                    _ => unreachable!(),
                };
            }
            WeaponName::Flamethrower => match player.weapons.settings.flamethrower {
                0 => self.fire_strategy = FireStrategy::None,
                _ => {
                    let power = player.weapons.settings.flamethrower as f32;

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
            WeaponName::MachineGun => {
                match player.weapons.settings.machine_gun {
                    0 => {
                        self.fire_timer = None;
                        self.fire_strategy = FireStrategy::None;
                    }
                    v => {
                        self.fire_strategy = FireStrategy::Closest;
                        if let Some(ref mut timer) = self.fire_timer {
                            timer.set_duration(Duration::from_secs_f32(1. / v as f32));
                        } else {
                            self.fire_timer =
                                Some(Timer::from_seconds(1. / v as f32, TimerMode::Once))
                        }
                    }
                };
            }
            WeaponName::MissileLauncher => {
                self.n_bullets = player.weapons.settings.missile_launcher;
                if self.n_bullets == 0 {
                    self.fire_strategy = FireStrategy::None;
                } else {
                    self.fire_strategy = FireStrategy::Strongest;
                }
            }
            WeaponName::Mortar => {
                // Reset the target to recalculate the highest density
                self.target = None;

                match player.weapons.settings.mortar {
                    MortarShell::None => self.fire_strategy = FireStrategy::None,
                    MortarShell::Light => {
                        self.fire_strategy = FireStrategy::Density;
                        self.bullet.price = Resources {
                            bullets: 15.,
                            ..default()
                        };
                        self.bullet.impact = Impact::Explosion(Explosion {
                            radius: 0.05 * MAP_SIZE.y,
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
                        self.bullet.impact = Impact::Explosion(Explosion {
                            radius: 0.1 * MAP_SIZE.y,
                            damage: Damage {
                                ground: 75.,
                                air: 75.,
                                penetration: 25.,
                            },
                            ..default()
                        })
                    }
                };
            }
            WeaponName::Turret => (),
        }
    }
}

#[derive(Resource)]
pub struct WeaponManager {
    pub aaa: Weapon,
    pub artillery: Weapon,
    pub canon: Weapon,
    pub flamethrower: Weapon,
    pub machine_gun: Weapon,
    pub mortar: Weapon,
    pub missile_launcher: Weapon,
    pub turret: Weapon,

    pub mine: Bullet,
    pub bomb: Bullet,
    pub nuke: Bullet,
}

impl WeaponManager {
    pub fn get(&self, name: &WeaponName) -> Weapon {
        match name {
            WeaponName::AAA => self.aaa.clone(),
            WeaponName::Artillery => self.artillery.clone(),
            WeaponName::Canon => self.canon.clone(),
            WeaponName::Flamethrower => self.flamethrower.clone(),
            WeaponName::MachineGun => self.machine_gun.clone(),
            WeaponName::Mortar => self.mortar.clone(),
            WeaponName::MissileLauncher => self.missile_launcher.clone(),
            WeaponName::Turret => self.turret.clone(),
        }
    }
}

impl Default for WeaponManager {
    fn default() -> Self {
        Self {
            aaa: Weapon {
                name: WeaponName::AAA,
                image: "weapon/aaa.png",
                dim: Vec2::new(80., 80.),
                rotation_speed: 5.,
                target: None,
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
                    impact: Impact::SingleTarget(Damage {
                        ground: 5.,
                        air: 5.,
                        penetration: 0.,
                    }),
                    max_distance: 0.7 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            artillery: Weapon {
                name: WeaponName::Artillery,
                image: "weapon/artillery.png",
                dim: Vec2::new(80., 80.),
                rotation_speed: 5.,
                target: None,
                price: Resources {
                    materials: 600.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "cone-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(1., TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/bullet.png",
                    dim: Vec2::new(30., 10.),
                    price: Resources {
                        bullets: 25.,
                        ..default()
                    },
                    speed: 0.9 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    impact: Impact::SingleTarget(Damage {
                        ground: 45.,
                        air: 45.,
                        penetration: 40.,
                    }),
                    max_distance: 1. * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            canon: Weapon {
                name: WeaponName::Canon,
                image: "weapon/canon.png",
                dim: Vec2::new(70., 50.),
                rotation_speed: 6.,
                target: None,
                price: Resources {
                    materials: 200.,
                    ..default()
                },
                fire_animation: FireAnimation {
                    atlas: "cone-flash",
                    scale: Vec3::splat(0.5),
                    duration: 0.1,
                },
                n_bullets: 1,
                fire_timer: Some(Timer::from_seconds(2., TimerMode::Once)),
                fire_strategy: FireStrategy::Closest,
                bullet: Bullet {
                    image: "weapon/grenade.png",
                    dim: Vec2::new(25., 10.),
                    price: Resources {
                        bullets: 10.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    impact: Impact::Explosion(Explosion {
                        atlas: "explosion2",
                        radius: 0.08 * MAP_SIZE.y,
                        ..default() // Damage set when updating fire strategy
                    }),
                    max_distance: 0.9 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            flamethrower: Weapon {
                name: WeaponName::Flamethrower,
                image: "weapon/flamethrower.png",
                dim: Vec2::new(60., 60.),
                rotation_speed: 7.,
                target: None,
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
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/invisible-bullet.png",
                    dim: Vec2::new(20., 7.),
                    price: Resources {
                        gasoline: 1.,
                        ..default()
                    },
                    speed: 1.2 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    impact: Impact::Piercing(Damage {
                        ground: 1.,
                        air: 1.,
                        penetration: 0.,
                    }),
                    max_distance: 0., // Is set by self.update()
                    distance: 0.,
                },
            },
            machine_gun: Weapon {
                name: WeaponName::MachineGun,
                image: "weapon/machine-gun.png",
                dim: Vec2::new(70., 70.),
                rotation_speed: 7.,
                target: None,
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
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/bullet.png",
                    dim: Vec2::new(25., 7.),
                    price: Resources {
                        bullets: 1.,
                        ..default()
                    },
                    speed: 0.8 * MAP_SIZE.y,
                    movement: Movement::Straight,
                    impact: Impact::SingleTarget(Damage {
                        ground: 5.,
                        air: 0.,
                        penetration: 0.,
                    }),
                    max_distance: 0.7 * MAP_SIZE.y,
                    distance: 0.,
                },
            },
            missile_launcher: Weapon {
                name: WeaponName::MissileLauncher,
                image: "weapon/missile-launcher.png",
                dim: Vec2::new(90., 90.),
                rotation_speed: 5.,
                target: None,
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
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/grenade.png",
                    dim: Vec2::new(20., 6.),
                    price: Resources {
                        bullets: 15.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Homing(Entity::from_raw(0)), // Set at spawn
                    impact: Impact::Explosion(Explosion {
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
            mortar: Weapon {
                name: WeaponName::Mortar,
                image: "weapon/mortar.png",
                dim: Vec2::new(70., 70.),
                rotation_speed: 5.,
                target: None,
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
                    movement: Movement::Location(Vec3::splat(0.)), // Set at spawn
                    impact: Impact::Explosion(Explosion {
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
                rotation_speed: 5.,
                target: None,
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
                fire_timer: Some(Timer::from_seconds(1., TimerMode::Once)),
                fire_strategy: FireStrategy::None,
                bullet: Bullet {
                    image: "weapon/triple-bullet.png",
                    dim: Vec2::new(25., 25.),
                    price: Resources {
                        bullets: 30.,
                        ..default()
                    },
                    speed: 0.6 * MAP_SIZE.y,
                    movement: Movement::Homing(Entity::from_raw(0)), // Set at spawn
                    impact: Impact::SingleTarget(Damage {
                        ground: 50.,
                        air: 50.,
                        penetration: 10.,
                    }),
                    max_distance: 2. * MAP_SIZE.y,
                    distance: 0.,
                },
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
                impact: Impact::Explosion(Explosion {
                    interval: 0.02,
                    radius: 0.1 * MAP_SIZE.y,
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
            bomb: Bullet {
                image: "weapon/bomb.png",
                dim: Vec2::new(30., 15.),
                price: Resources {
                    bullets: 250.,
                    gasoline: 250.,
                    ..default()
                },
                speed: 0.4 * MAP_SIZE.y,
                movement: Movement::Location(Vec3::splat(0.)), // Set at spawn
                impact: Impact::Explosion(Explosion {
                    interval: 0.05,
                    radius: 0.35 * MAP_SIZE.y,
                    damage: Damage {
                        ground: 80.,
                        air: 80.,
                        penetration: 20.,
                    },
                    ..default()
                }),
                max_distance: f32::MAX,
                distance: 0.,
            },
            nuke: Bullet {
                image: "weapon/nuke.png",
                dim: Vec2::new(25., 10.),
                price: Resources {
                    bullets: 2500.,
                    gasoline: 2500.,
                    materials: 2500.,
                },
                speed: 0.2 * MAP_SIZE.y,
                movement: Movement::Location(Vec3::new(
                    -WEAPONS_PANEL_SIZE.x * 0.5,
                    SIZE.y * 0.5 - MAP_SIZE.y * 0.5,
                    EXPLOSION_Z,
                )),
                impact: Impact::Explosion(Explosion {
                    interval: 0.05,
                    radius: 1.5 * MAP_SIZE.y,
                    damage: Damage {
                        ground: 20_000.,
                        air: 20_000.,
                        penetration: 20_000.,
                    },
                    ..default()
                }),
                max_distance: f32::MAX,
                distance: 0.,
            },
        }
    }
}
