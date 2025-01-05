use crate::constants::{BETA, NIGHT_DURATION};
use crate::game::enemy::components::Enemy;
use bevy::prelude::{Resource, Vec2};
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Resource)]
pub struct EnemySpawner {
    pub enemies: Vec<Enemy>,
}

impl EnemySpawner {
    fn get_spawn_probabilities(&self, day: u32, time: f32) -> Vec<f32> {
        let probabilities: Vec<f32> = self
            .enemies
            .iter()
            .map(|&enemy| {
                if enemy.strength <= day as f32 {
                    // Linearly increase probability before day
                    enemy.strength / day as f32
                } else {
                    // Exponentially decrease probability after day. The decay is less
                    // steep as time progresses, increasing the probability of stronger
                    // enemies over time
                    (-BETA * (1. - time / NIGHT_DURATION) * (enemy.strength - day as f32)).exp()
                }
            })
            .collect();

        let total_weight: f32 = probabilities.iter().sum();
        probabilities.iter().map(|&p| p / total_weight).collect()
    }

    pub fn choose_enemy(&self, day: u32, time: f32) -> Option<&Enemy> {
        // The Probability of not spawning any enemy is 1 / day + 1
        if rand::random::<f32>() < 1. / (day as f32 + 1.) {
            // Function to sample an enemy based on weighted probabilities
            let weights = self.get_spawn_probabilities(day, time);
            println!("Weights: {:?}", weights);

            // Sample an index based on the normalized weights
            let dist = WeightedIndex::new(&weights).unwrap();
            Some(
                self.enemies
                    .get(dist.sample(&mut rand::thread_rng()))
                    .unwrap(),
            )
        } else {
            None
        }
    }
}

impl Default for EnemySpawner {
    fn default() -> Self {
        let enemies = vec![
            Enemy {
                name: "Dartling",
                image: "enemy/dartling.png",
                max_health: 50.,
                health: 50.,
                size: Vec2::new(50., 50.),
                armor: 0.,
                speed: 10.,
                damage: 10.,
                strength: 1.0,
            },
            Enemy {
                name: "Skitterling",
                image: "enemy/skitterling.png",
                max_health: 20.,
                health: 20.,
                size: Vec2::new(30., 30.),
                armor: 0.,
                speed: 25.,
                damage: 5.,
                strength: 1.0,
            },
            Enemy {
                name: "Shellback",
                image: "enemy/shellback.png",
                max_health: 50.,
                health: 50.,
                size: Vec2::new(70., 70.),
                armor: 5.,
                speed: 7.,
                damage: 20.,
                strength: 2.0,
            },
            Enemy {
                name: "Quickstrike",
                image: "enemy/quickstrike.png",
                max_health: 40.,
                health: 40.,
                size: Vec2::new(40., 40.),
                armor: 2.,
                speed: 20.,
                damage: 10.,
                strength: 2.0,
            },
            Enemy {
                name: "Chiton",
                image: "enemy/chiton.png",
                max_health: 60.,
                health: 60.,
                size: Vec2::new(50., 50.),
                armor: 0.,
                speed: 15.,
                damage: 15.,
                strength: 2.0,
            },
            Enemy {
                name: "Thornbiter",
                image: "enemy/thornbiter.png",
                max_health: 40.,
                health: 40.,
                size: Vec2::new(55., 55.),
                armor: 10.,
                speed: 10.,
                damage: 20.,
                strength: 3.0,
            },
            Enemy {
                name: "Blightcrawler",
                image: "enemy/blightcrawler.png",
                max_health: 80.,
                health: 80.,
                size: Vec2::new(40., 40.),
                armor: 2.,
                speed: 20.,
                damage: 25.,
                strength: 4.0,
            },
            Enemy {
                name: "Shellfist",
                image: "enemy/shellfist.png",
                max_health: 120.,
                health: 120.,
                size: Vec2::new(90., 90.),
                armor: 5.,
                speed: 5.,
                damage: 50.,
                strength: 5.0,
            },
            // Enemy {
            //     name: "Carapacebreaker",
            //     image: "enemy/carapacebreaker.png",
            //     max_health: 20.,
            //     health: 20.,
            //     size: Vec2::new(60., 60.),
            //     armor: 5.,
            //     speed: 2.,
            //     damage: 2.,
            //     strength: 6.5,
            // },
            //
            // Enemy {
            //     name: "Shellwarden",
            //     image: "enemy/shellwarden.png",
            //     max_health: 18.,
            //     health: 18.,
            //     size: Vec2::new(55., 55.),
            //     armor: 4.,
            //     speed: 2.2,
            //     damage: 3.,
            //     strength: 5.0,
            // },
            //
            // Enemy {
            //     name: "Hiveborn",
            //     image: "enemy/hiveborn.png",
            //     max_health: 15.,
            //     health: 15.,
            //     size: Vec2::new(55., 55.),
            //     armor: 3.,
            //     speed: 3.,
            //     damage: 3.,
            //     strength: 5.6,
            // },
            //
            // Enemy {
            //     name: "Stinglash",
            //     image: "enemy/stinglash.png",
            //     max_health: 12.,
            //     health: 12.,
            //     size: Vec2::new(40., 40.),
            //     armor: 2.,
            //     speed: 3.,
            //     damage: 3.2,
            //     strength: 4.8,
            // },
            //
            // Enemy {
            //     name: "Beetlestrike",
            //     image: "enemy/beetlestrike.png",
            //     max_health: 40.,
            //     health: 40.,
            //     size: Vec2::new(75., 75.),
            //     armor: 6.,
            //     speed: 1.8,
            //     damage: 4.,
            //     strength: 7.6,
            // },
            //
            // Enemy {
            //     name: "Blightbeetle",
            //     image: "enemy/blightbeetle.png",
            //     max_health: 60.,
            //     health: 60.,
            //     size: Vec2::new(85., 85.),
            //     armor: 7.,
            //     speed: 1.4,
            //     damage: 6.,
            //     strength: 8.5,
            // },
            //
            // Enemy {
            //     name: "Ironclaw",
            //     image: "enemy/ironclaw.png",
            //     max_health: 50.,
            //     health: 50.,
            //     size: Vec2::new(90., 90.),
            //     armor: 8.,
            //     speed: 1.5,
            //     damage: 5.,
            //     strength: 9.8,
            // },
            //
            // Enemy {
            //     name: "Ironcarapace",
            //     image: "enemy/ironcarapace.png",
            //     max_health: 70.,
            //     health: 70.,
            //     size: Vec2::new(100., 100.),
            //     armor: 10.,
            //     speed: 1.,
            //     damage: 8.,
            //     strength: 10.0,
            // },
        ];

        Self { enemies }
    }
}
