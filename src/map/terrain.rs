pub(crate) mod generation;

use bevy::prelude::Color;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

/// Geographical features of a [Tile].
#[derive(Debug)]
pub(crate) struct Terrain {
    id: usize,
    name: String,
    color: Color,
}

impl Terrain {
    /// Index inside the [Terrains].
    pub fn id(&self) -> usize {
        self.id
    }

    /// Name in English.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Map color.
    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl PartialEq for Terrain {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Terrain {}

/// All the terrains.
pub(crate) struct Terrains {
    indexed: Vec<Terrain>,
    named: HashMap<String, usize>,
}

impl Terrains {
    /// Reads from the assets.
    ///
    /// # Panics
    ///
    /// * Cannot read the file "assets/terrains.json".
    /// * Cannot parse the JSON.
    fn new() -> Self {
        let data: Vec<TerrainData> =
            serde_json::from_str(&fs::read_to_string("assets/terrains.json").unwrap()).unwrap();
        let mut indexed = Vec::new();
        let mut named = HashMap::new();
        for (id, TerrainData { name, color }) in data.into_iter().enumerate() {
            indexed.push(Terrain {
                id,
                name: name.clone(),
                color,
            });
            named.insert(name, id);
        }
        Terrains { indexed, named }
    }

    /// Returns a [Terrain] by id.
    ///
    /// # Panics
    ///
    /// Invalid id.
    pub fn of_id(&self, id: usize) -> &Terrain {
        &self.indexed[id]
    }

    /// Returns a [Terrain] by name.
    ///
    /// # Panics
    ///
    /// A terrain by the given name does not exist.
    pub fn of_name(&self, name: &String) -> &Terrain {
        self.of_id(*self.named.get(name).unwrap())
    }
}

/// Terrain data as it is in the assets.
#[derive(Serialize, Deserialize)]
struct TerrainData {
    name: String,
    color: Color,
}

#[cfg(test)]
mod test_map {
    use super::*;

    #[test]
    fn test_terrain_loading() {
        let terrains = Terrains::new();
        assert_eq!(terrains.of_id(0).name, "Glaciers");
    }
}
