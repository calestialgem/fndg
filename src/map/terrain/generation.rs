use hex_grid::{Coordinate, Offset, CENTER};
use perlin2d::PerlinNoise2D;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use super::Terrains;

/// Perlin noise configuration.
#[derive(Serialize, Deserialize)]
struct Noise {
    octaves: i32,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: f64,
}

impl Noise {
    fn perlin(&self, amplitude: f64) -> PerlinNoise2D {
        PerlinNoise2D::new(
            self.octaves,
            amplitude,
            self.frequency,
            self.persistence,
            self.lacunarity,
            (self.scale, self.scale),
            0.0,
            rand::random(),
        )
    }
}

/// [Terrain] name and weight pair to distribute.
#[derive(Serialize, Deserialize)]
struct TerrainWeightConfig {
    name: String,
    weight: f64,
}

/// Perlin noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
struct DistributorConfig {
    noise: Noise,
    distribution: Vec<TerrainWeightConfig>,
}

/// Configuration of [Terrain] generation.
#[derive(Serialize, Deserialize)]
struct GenerationConfig {
    radius: u16,
    height: DistributorConfig,
    humidity: DistributorConfig,
}

impl GenerationConfig {
    fn new() -> Self {
        serde_json::from_str(&fs::read_to_string("assets/map_generation.json").unwrap()).unwrap()
    }
}

/// [Terrain] and weight pair to distribute.
struct TerrainWeight {
    terrain: usize,
    weight: f64,
}

impl TerrainWeight {
    const BY_HUMIDITY: usize = usize::MAX;

    fn new(terrains: &Terrains, config: &TerrainWeightConfig) -> Self {
        if config.name == "BY_HUMIDITY" {
            TerrainWeight {
                terrain: Self::BY_HUMIDITY,
                weight: config.weight,
            }
        } else {
            TerrainWeight {
                terrain: terrains.of_name(&config.name).id(),
                weight: config.weight,
            }
        }
    }
}

struct Distributor {
    noise: PerlinNoise2D,
    distribution: Vec<TerrainWeight>,
}

impl Distributor {
    fn new(terrains: &Terrains, config: &DistributorConfig) -> Self {
        let mut weights = Vec::new();
        let mut amplitude = 0.0;
        for weight in config.distribution.iter() {
            weights.push(TerrainWeight::new(terrains, weight));
            amplitude += weight.weight;
        }
        Distributor {
            noise: config.noise.perlin(amplitude),
            distribution: weights,
        }
    }

    fn noise(&self, coord: Coordinate) -> f64 {
        self.noise.get_noise(coord.x as f64, coord.y as f64)
    }

    fn distribute(&self, mut value: f64) -> usize {
        for TerrainWeight { terrain, weight } in self.distribution.iter() {
            value -= weight;
            if value <= 0.0 {
                return *terrain;
            }
        }
        unreachable!("`value` is too big!");
    }
}

/// Generates; a [Map] from Perlin noise.
struct Generator {
    radius: u16,
    height: Distributor,
    humidity: Distributor,
}

impl Generator {
    fn new(terrains: &Terrains, config: &GenerationConfig) -> Self {
        Generator {
            radius: config.radius,
            height: Distributor::new(terrains, &config.height),
            humidity: Distributor::new(terrains, &config.humidity),
        }
    }

    fn generate(&self, terrains: &Terrains) -> HashMap<Coordinate, usize> {
        let coords = CENTER + Offset::fill_hex(self.radius);
        let mut map = HashMap::new();
        for coord in coords {
            map.insert(coord, self.generate_tile(terrains, coord));
        }
        map
    }

    fn generate_tile(&self, terrains: &Terrains, coord: Coordinate) -> usize {
        let from_height = self.height.distribute(self.height.noise(coord));
        if from_height == TerrainWeight::BY_HUMIDITY {
            self.humidity.distribute(self.humidity.noise(coord))
        } else {
            from_height
        }
    }
}
