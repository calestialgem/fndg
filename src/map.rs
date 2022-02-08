//! Stuff about the game map.

use bevy::prelude::{Commands, Component, Entity};
use hex_grid::{Coordinate, Offset, CENTER};
use perlin2d::PerlinNoise2D;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use self::terrain::{Terrain, Terrains};

mod terrain;

/// Smallest piece of a [Map].
#[derive(Component)]
struct Tile {
    terrain: usize,
    coord: Coordinate,
}

impl Tile {
    /// Creates a new tile.
    fn new(terrain: usize, coord: Coordinate) -> Self {
        Tile {
            terrain: terrain,
            coord: coord,
        }
    }

    /// Returns this tile's terrain from the given terrains. This must be the
    /// same terrains that was used in the generation of the tile.
    fn terrain<'a>(&self, terrains: &'a Terrains) -> &'a Terrain {
        terrains.of_id(self.terrain)
    }

    /// Coordinates of the hexagon.
    fn coord(&self) -> Coordinate {
        self.coord
    }
}

/// The game world.
#[derive(Default)]
struct Map {
    tiles: HashMap<Coordinate, Entity>,
}
