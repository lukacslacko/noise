use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use perlin::Perlin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}

fn update() {}

fn init(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let w = 1024;
    let h = 1024;
    let mut image = Image::new_fill(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::new(0.0, 0.0, 0.0, 1.0).to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let perlin = Perlin::new(123, 2.0, 1.64, 0.7, 6);

    let mut values = vec![vec![0.0; w as usize]; h as usize];
    for row in 0..h as usize {
        for col in 0..w as usize {
            values[row][col] = perlin.noise([col as f32 / w as f32, row as f32 / h as f32]);
        }
    }
    let min_value = values
        .iter()
        .flat_map(|v| v.iter())
        .cloned()
        .fold(f32::INFINITY, f32::min);
    let max_value = values
        .iter()
        .flat_map(|v| v.iter())
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);
    for row in 0..h {
        for col in 0..w {
            let pixel = image.pixel_bytes_mut(UVec3::new(col, row, 0)).unwrap();
            let value = values[row as usize][col as usize];
            let value = ((value - min_value) / (max_value - min_value)).clamp(0.0, 1.0);
            let value_north = values[row.saturating_sub(1) as usize][col as usize];
            let value_north = ((value_north - min_value) / (max_value - min_value)).clamp(0.0, 1.0);
            let shadow = 20.0 * (value - value_north) + 1.0;
            let lerp = |a: i32, b: i32, t: f32, s: f32| {
                (shadow * s * (a as f32 + t * (b as f32 - a as f32))).clamp(0.0, 255.0) as u8
            };
            let rgb = |v, s| (lerp(0, 100, v, s), lerp(150, 50, v, s), lerp(0, 50, v, s));
            let (r, g, b) = rgb(value, 1.0);
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }
    }

    let gap = 20;
    for row in 0..(h / gap) {
        for col in 0..(w / gap) {
            let mut x = (col * gap) as f32 / w as f32;
            let mut y = (row * gap) as f32 / h as f32;
            let mut value = perlin.noise([x, y]);
            let eps = 0.0001;
            for _ in 0..500 {
                if let Some(pixel) = image.pixel_bytes_mut(UVec3::new(
                    (x * w as f32) as u32,
                    (y * h as f32) as u32,
                    0,
                )) {
                    pixel[0] = 30;
                    pixel[1] = 30;
                    pixel[2] = 255;
                }
                let dx = perlin.noise([x + eps, y]) - value;
                let dy = perlin.noise([x, y + eps]) - value;
                let v = 2.0 * w as f32 * (dx * dx + dy * dy).sqrt();
                x += -dx / v;
                y += -dy / v;
                value = perlin.noise([x, y]);
            }
        }
    }

    let image_handle = images.add(image);
    commands.spawn(Sprite::from_image(image_handle));
    commands.spawn((Camera2d,));
}
