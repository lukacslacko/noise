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
    let w = 512;
    let h = 512;
    let mut image = Image::new_fill(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::new(0.0, 1.0, 0.0, 1.0).to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let perlin = Perlin::new(0, 4.0, 1.7, 0.5, 4);
    let gap = 5;
    for row in 0..(h / gap) {
        for col in 0..(w / gap) {
            let mut x = (col * gap) as f32 / w as f32;
            let mut y = (row * gap) as f32 / h as f32;
            let mut value = perlin.noise([x, y]);
            let eps = 0.0001;
            let v = 10.0;
            for _ in 0..100 {
                if let Some(pixel) = image.pixel_bytes_mut(UVec3::new(
                    (x * w as f32) as u32,
                    (y * h as f32) as u32,
                    0,
                )) {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 255;
                }
                let dx = perlin.noise([x + eps, y]) - value;
                let dy = perlin.noise([x, y + eps]) - value;
                x += dx * v;
                y += dy * v;
                value = perlin.noise([x, y]);
            }
        }
    }

    let image_handle = images.add(image);
    commands.spawn(Sprite::from_image(image_handle));
    commands.spawn((Camera2d,));
}
