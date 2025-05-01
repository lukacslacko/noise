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
        &Srgba::new(0.5, 0.5, 0.5, 1.0).to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let perlin = Perlin::new(0, 4.0, 2.0, 0.5, 4);
    for row in 0..h {
        for col in 0..w {
            let x = col as f32 / h as f32;
            let y = row as f32 / h as f32;
            let noise_value = 1.0 + perlin.noise([x, y]);
            let value = (noise_value*50.0).clamp(0.0, 255.0) as u8;
            // println!("x: {}, y: {}, noise_value: {}, value: {}", x, y, noise_value, value);
            let pixel = image.pixel_bytes_mut(UVec3::new(col, row, 0)).unwrap();
            pixel[0] = value as u8;
            pixel[1] = value as u8;
            pixel[2] = value as u8;
        }
    }
    let image_handle = images.add(image);
    commands.spawn(Sprite::from_image(image_handle));
    commands.spawn((Camera2d,));
}
