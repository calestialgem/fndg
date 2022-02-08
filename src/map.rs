//! Stuff about the game map.

mod terrain;

use self::terrain::{
    generation::{GenerationConfig, Generator},
    Terrain, Terrains,
};
use bevy::{
    math::Vec3,
    prelude::{Bundle, Commands, Component, Entity, Res, ResMut, Transform},
    sprite::SpriteBundle,
};
use hex_grid::{Coordinate, HexSize, PixelOrientation};
use std::collections::HashMap;

/// Location in the [Map].
#[derive(Component)]
struct Location {
    coord: Coordinate,
}

/// Smallest piece of a [Map].
#[derive(Component)]
struct Tile {
    terrain: usize,
}

impl Tile {
    const SIZE: HexSize = HexSize::from_regular_height(1.0);
    const PIXEL_ORIENTATION: PixelOrientation = PixelOrientation {
        right_increasing: true,
        up_increasing: true,
    };

    /// Returns this tile's terrain from the given terrains. This must be the
    /// same terrains that was used in the generation of the tile.
    fn terrain<'a>(&self, terrains: &'a Terrains) -> &'a Terrain {
        terrains.of_id(self.terrain)
    }
}

#[derive(Bundle)]
struct TileBundle {
    #[bundle]
    sprite: SpriteBundle,
    location: Location,
    tile: Tile,
}

impl TileBundle {
    fn new(coord: Coordinate, terrain: usize, terrains: &Terrains) -> Self {
        fn to_screen_coord(coord: Coordinate) -> Vec3 {
            let pixel = coord.to_pixel(Tile::SIZE, Tile::PIXEL_ORIENTATION);
            Vec3::new(pixel.0, pixel.1, 0.0)
        }
        TileBundle {
            sprite: SpriteBundle {
                sprite: bevy::sprite::Sprite {
                    color: *terrains.of_id(terrain).color(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: to_screen_coord(coord),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            location: Location { coord },
            tile: Tile { terrain },
        }
    }
}

/// The game world.
#[derive(Default)]
struct Map {
    tiles: HashMap<Coordinate, Entity>,
}

fn generate_map(mut map: ResMut<Map>, terrains: Res<Terrains>, mut commands: Commands) {
    let terrain_map = Generator::new(&terrains, &GenerationConfig::new()).generate();
    for (coord, terrain) in terrain_map.into_iter() {
        let tile = commands
            .spawn_bundle(TileBundle::new(coord, terrain, &terrains))
            .id();
        map.tiles.insert(coord, tile);
    }
}
