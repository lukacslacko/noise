use std::f32::consts::PI;

pub struct Perlin {
    seed: u64,
    frequency: f32,
    lacunarity: f32,
    persistence: f32,
    octaves: u32,
}

impl Perlin {
    pub fn new(seed: u64, frequency: f32, lacunarity: f32, persistence: f32, octaves: u32) -> Self {
        Perlin {
            seed,
            frequency,
            lacunarity,
            persistence,
            octaves,
        }
    }

    pub fn noise<const N: usize>(&self, pos: [f32; N]) -> f32 {
        let mut total = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = self.frequency;

        for _ in 0..self.octaves {
            total += self.noise_layer(pos.iter().map(|x| x * frequency)) * amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }

        total
    }

    fn randvec(&self, pos: Vec<i32>) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        pos.iter()
            .enumerate()
            .map(|(i, _)| {
                let mut hasher = DefaultHasher::new();
                pos.hash(&mut hasher);
                i.hash(&mut hasher);
                self.seed.hash(&mut hasher);
                let hash = hasher.finish();
                let u1 = (hash >> 32) as f32 / u32::MAX as f32;
                let u2 = (hash as u32) as f32 / u32::MAX as f32;
                let theta = 2.0 * PI * u1;
                let rho = (-2.0 * u2.ln()).sqrt();
                rho * theta.cos()
            })
            .collect()
    }

    fn normalize(pos: Vec<f32>) -> Vec<f32> {
        let length = (pos.iter().map(|x| x * x).sum::<f32>()).sqrt();
        pos.iter().map(|x| x / length).collect()
    }

    fn noise_layer(&self, pos: impl Iterator<Item = f32>) -> f32 {
        let parts = pos
            .map(|x| {
                let floor = x.floor();
                (floor as i32, x - floor)
            })
            .collect::<Vec<_>>();
        let dim = parts.len();
        let contrib = |corner: Vec<f32>| {
            let vec = Self::normalize(
                self.randvec((0..dim).map(|i| parts[i].0 + corner[i] as i32).collect()),
            );
            let dot = (0..dim)
                .map(|i| vec[i] * (parts[i].1 - corner[i]))
                .sum::<f32>();
            let fade = |x| 3.0 * x * x - 2.0 * x * x * x;
            let scale = (0..dim)
                .map(|i| {
                    let fade_val = fade(parts[i].1);
                    let t = if corner[i] == 1.0 {
                        fade_val
                    } else {
                        1.0 - fade_val
                    };
                    t * dot
                })
                .product::<f32>();
            scale * dot
        };
        (0..(1 << dim))
            .map(|i| contrib((0..dim).map(|j| ((i >> j) & 1) as f32).collect::<Vec<_>>()))
            .sum()
    }
}
