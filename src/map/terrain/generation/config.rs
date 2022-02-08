use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use serde::{Deserialize, Serialize};
use std::fs;

/// Noise configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct NoiseConfig {
    simplex: bool,
    octaves: i32,
    gain: f32,
    lacunarity: f32,
    frequency: f32,
    scale: f32,
}

impl NoiseConfig {
    pub(super) fn noise(&self) -> (FastNoise, f32) {
        let mut noise = FastNoise::seeded(rand::random());
        noise.set_noise_type(if self.simplex {
            NoiseType::SimplexFractal
        } else {
            NoiseType::PerlinFractal
        });
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(self.octaves);
        noise.set_fractal_gain(self.gain);
        noise.set_fractal_lacunarity(self.lacunarity);
        noise.set_frequency(self.frequency);
        (noise, self.scale)
    }
}

/// [Terrain] name and weight pair to distribute.
#[derive(Serialize, Deserialize)]
pub(super) struct TerrainWeightConfig {
    pub(super) name: String,
    pub(super) weight: f32,
}

/// Noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub(super) struct DistributorConfig {
    pub(super) noise: NoiseConfig,
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
