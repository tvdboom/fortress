use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::prelude::{FromWorld, World};
use std::collections::HashMap;

pub struct Images {
    pub bug: Handle<Image>,
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
    pub game_over: Handle<Image>,
    pub enemies: HashMap<&'static str, Handle<Image>>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bug: asset_server.load("icons/bug.png"),
            day: asset_server.load("icons/day.png"),
            night: asset_server.load("icons/night.png"),
            person: asset_server.load("icons/user.png"),
            wall: asset_server.load("icons/wall.png"),
            fence: asset_server.load("icons/fence.png"),
            lightning: asset_server.load("icons/lightning.png"),
            weapon: asset_server.load("icons/rifle.png"),
            bullets: asset_server.load("icons/bullet.png"),
            gasoline: asset_server.load("icons/gasoline.png"),
            materials: asset_server.load("icons/brick.png"),
            hourglass: asset_server.load("icons/hourglass.png"),
            clock: asset_server.load("icons/clock.png"),
            spot: asset_server.load("icons/spot.png"),
            game_over: asset_server.load("map/game-over.png"),
            enemies: HashMap::from([
                ("Dartling", asset_server.load("enemy/dartling.png")),
                ("Skitterling", asset_server.load("enemy/skitterling.png")),
                ("Shellback", asset_server.load("enemy/shellback.png")),
                ("Quickstrike", asset_server.load("enemy/quickstrike.png")),
                ("Chiton", asset_server.load("enemy/chiton.png")),
                ("Thornbiter", asset_server.load("enemy/thornbiter.png")),
                ("Needler", asset_server.load("enemy/needler.png")),
                ("Blightcraw", asset_server.load("enemy/blightcraw.png")),
                ("Shellfist", asset_server.load("enemy/shellfist.png")),
                ("Shellwarden", asset_server.load("enemy/shellwarden.png")),
                ("Hiveborn", asset_server.load("enemy/hiveborn.png")),
                ("Crawler", asset_server.load("enemy/crawler.png")),
                (
                    "Carapacebreaker",
                    asset_server.load("enemy/carapacebreaker.png"),
                ),
                ("Dartmite", asset_server.load("enemy/dartmite.png")),
                ("Nestling", asset_server.load("enemy/nestling.png")),
                ("Gargantula", asset_server.load("enemy/gargantula.png")),
                ("Ironclaw", asset_server.load("enemy/ironclaw.png")),
                ("Ironcarapace", asset_server.load("enemy/ironcarapace.png")),
            ]),
        }
    }
}
