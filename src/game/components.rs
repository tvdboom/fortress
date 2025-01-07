use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct Images {
    pub day: Handle<Image>,
    pub night: Handle<Image>,
    pub person: Handle<Image>,
    pub wall: Handle<Image>,
    pub fence: Handle<Image>,
    pub lightning: Handle<Image>,
    pub weapon: Handle<Image>,
    pub bullets: Handle<Image>,
    pub gasoline: Handle<Image>,
    pub materials: Handle<Image>,
    pub spot: Handle<Image>,
    pub hourglass: Handle<Image>,
    pub clock: Handle<Image>,
    pub explosion1: Handle<Image>,
    pub explosion2: Handle<Image>,
    pub explosion3: Handle<Image>,
    pub game_over: Handle<Image>,
    pub enemies: HashMap<&'static str, Handle<Image>>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource_mut::<AssetServer>().unwrap();

        Self {
            day: assets.load("icons/day.png"),
            night: assets.load("icons/night.png"),
            person: assets.load("icons/user.png"),
            wall: assets.load("icons/wall.png"),
            fence: assets.load("icons/fence.png"),
            lightning: assets.load("icons/lightning.png"),
            weapon: assets.load("icons/rifle.png"),
            bullets: assets.load("icons/bullet.png"),
            gasoline: assets.load("icons/gasoline.png"),
            materials: assets.load("icons/brick.png"),
            spot: assets.load("icons/spot.png"),
            hourglass: assets.load("icons/hourglass.png"),
            clock: assets.load("icons/clock.png"),
            explosion1: assets.load("weapon/explosion1.png"),
            explosion2: assets.load("weapon/explosion2.png"),
            explosion3: assets.load("weapon/explosion3.png"),
            game_over: assets.load("map/game-over.png"),
            enemies: HashMap::from([
                ("Dartling", assets.load("enemy/dartling.png")),
                ("Skitterling", assets.load("enemy/skitterling.png")),
                ("Shellback", assets.load("enemy/shellback.png")),
                ("Quickstrike", assets.load("enemy/quickstrike.png")),
                ("Chiton", assets.load("enemy/chiton.png")),
                ("Thornbiter", assets.load("enemy/thornbiter.png")),
                ("Needler", assets.load("enemy/needler.png")),
                ("Blightcraw", assets.load("enemy/blightcraw.png")),
                ("Shellfist", assets.load("enemy/shellfist.png")),
                ("Shellwarden", assets.load("enemy/shellwarden.png")),
                ("Hiveborn", assets.load("enemy/hiveborn.png")),
                ("Crawler", assets.load("enemy/crawler.png")),
                ("Carapacebreaker", assets.load("enemy/carapacebreaker.png")),
                ("Dartmite", assets.load("enemy/dartmite.png")),
                ("Nestling", assets.load("enemy/nestling.png")),
                ("Gargantula", assets.load("enemy/gargantula.png")),
                ("Ironclaw", assets.load("enemy/ironclaw.png")),
                ("Ironcarapace", assets.load("enemy/ironcarapace.png")),
            ]),
        }
    }
}
