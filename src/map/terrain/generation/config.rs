use perlin2d::PerlinNoise2D;
use serde::{Deserialize, Serialize};
use std::fs;

/// Perlin noise configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct Noise {
    octaves: i32,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: f64,
}

impl Noise {
    pub(super) fn perlin(&self, amplitude: f64) -> PerlinNoise2D {
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
pub(super) struct TerrainWeightConfig {
    pub(super) name: String,
    pub(super) weight: f64,
}

/// Perlin noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct DistributorConfig {
    pub(super) noise: Noise,
    pub(super) distribution: Vec<TerrainWeightConfig>,
}

/// Configuration of [Terrain] generation.
#[derive(Serialize, Deserialize)]
pub(crate) struct GenerationConfig {
    pub(super) radius: i32,
    pub(super) height: DistributorConfig,
    pub(super) humidity: DistributorConfig,
}

impl GenerationConfig {
    pub(crate) fn new() -> Self {
        serde_json::from_str(&fs::read_to_string("assets/terrain_generation.json").unwrap())
            .unwrap()
    }
}
