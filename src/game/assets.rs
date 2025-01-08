use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AtlasInfo {
    pub image: Handle<Image>,
    pub texture: TextureAtlas,
    pub last_index: usize,
}

pub struct WorldAssets {
    pub images: HashMap<&'static str, Handle<Image>>,
    pub atlas: HashMap<&'static str, AtlasInfo>,
}

impl WorldAssets {
    pub fn get_image(&self, name: &str) -> Handle<Image> {
        self.images[name].clone_weak()
    }

    pub fn get_atlas(&self, name: &str) -> AtlasInfo {
        self.atlas[name].clone()
    }
}

impl FromWorld for WorldAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        let images = HashMap::from([
            // Icons
            ("day", assets.load("icons/day.png")),
            ("night", assets.load("icons/night.png")),
            ("person", assets.load("icons/user.png")),
            ("wall", assets.load("icons/wall.png")),
            ("fence", assets.load("icons/fence.png")),
            ("lightning", assets.load("icons/lightning.png")),
            ("weapon", assets.load("icons/rifle.png")),
            ("bullets", assets.load("icons/bullet.png")),
            ("gasoline", assets.load("icons/gasoline.png")),
            ("materials", assets.load("icons/brick.png")),
            ("spot", assets.load("icons/spot.png")),
            ("hourglass", assets.load("icons/hourglass.png")),
            ("clock", assets.load("icons/clock.png")),
            ("game_over", assets.load("map/game-over.png")),
            // Enemies
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
            // Sprite sheets
            ("flashes", assets.load("weapon/flashes.png")),
            ("explosion1", assets.load("weapon/explosion1.png")),
        ]);

        let mut texture = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let flash1 =
            TextureAtlasLayout::from_grid(UVec2::new(88, 100), 5, 6, Some(UVec2::new(0, 2)), None);
        let explosion1 = TextureAtlasLayout::from_grid(UVec2::new(128, 125), 5, 5, None, None);

        let atlas = HashMap::from([
            (
                "flash1",
                AtlasInfo {
                    image: images["flashes"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(flash1),
                        index: 25,
                    },
                    last_index: 30,
                },
            ),
            (
                "explosion1",
                AtlasInfo {
                    image: images["explosion1"].clone_weak(),
                    texture: TextureAtlas {
                        layout: texture.add(explosion1),
                        index: 1,
                    },
                    last_index: 25,
                },
            ),
        ]);

        Self { images, atlas }
    }
}
