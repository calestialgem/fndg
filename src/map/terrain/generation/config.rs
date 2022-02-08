use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use serde::{Deserialize, Serialize};

/// Noise configuration.
#[derive(Serialize, Deserialize)]
pub struct NoiseConfig {
    pub simplex: bool,
    pub octaves: i32,
    pub gain: f32,
    pub lacunarity: f32,
    pub frequency: f32,
    pub scale: f32,
}

impl NoiseConfig {
    pub fn noise(&self) -> FastNoise {
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
        noise
    }
}

/// [Terrain] name and weight pair to distribute.
#[derive(Serialize, Deserialize)]
pub struct TerrainWeightConfig {
    pub name: String,
    pub weight: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Distribution(pub Vec<TerrainWeightConfig>);

/// Noise to [Terrain] distributor configuration.
#[derive(Serialize, Deserialize)]
pub struct DistributorConfig {
    pub noise: NoiseConfig,
    pub distribution: Distribution,
}
