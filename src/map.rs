//! Stuff about the game map.

use std::collections::HashMap;

use bevy::prelude::{Color, Entity};
use hex_grid::Coordinate;
use serde::{Deserialize, Serialize};

/// Geographical features of a [Tile].
#[derive(Serialize, Deserialize)]
struct Terrain {
    name: String,
    color: Color,
}

/// Smallest piece of a [Map].
struct Tile {
    id: Entity,
    terrain: Terrain,
    coord: Coordinate,
}

/// The game world.
struct Map {
    tiles: HashMap<Coordinate, Tile>,
}

#[cfg(test)]
mod test_map {
    use super::*;
    use serde_json::Result;
    use std::fs;

    #[test]
    fn test_terrain_loading() -> Result<()> {
        let terrains_json = fs::read_to_string("assets/terrains.json");
        let terrains: Vec<Terrain> = serde_json::from_str(terrains_json.unwrap().as_str())?;
        assert_eq!(terrains[0].name, "Glaciers");
        Ok(())
    }
}
