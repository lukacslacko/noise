use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::{image, prelude::*};
use perlin::Perlin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}

#[derive(Resource)]
struct State {
    precomputed: Vec<Vec<f32>>,
    frame: i32,
}

fn update(
    image_handle: Res<ImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<State>,
) {
    let image = images.get_mut(&image_handle.0).unwrap();
    let w = image.width() as usize;
    let h = image.height() as usize;
    for row in 0..h {
        for col in 0..w {
            let pixel = image
                .pixel_bytes_mut(UVec3::new(col as u32, row as u32, 0))
                .unwrap();
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
        }
    }
    println!("Generating image... frame {}", state.frame);
    state.frame += 1;
    let noise_fn = |col: f32, row: f32| {
        let ci = (col as i32).max(0).min(w as i32 - 2);
        let ri = (row as i32).max(0).min(h as i32 - 2);
        let cf = col - ci as f32;
        let rf = row - ri as f32;
        let precomputed = &state.precomputed;
        cf * rf * precomputed[ri as usize][ci as usize]
            + (1.0 - cf) * rf * precomputed[ri as usize][ci as usize + 1]
            + cf * (1.0 - rf) * precomputed[ri as usize + 1][ci as usize]
            + (1.0 - cf) * (1.0 - rf) * precomputed[ri as usize + 1][ci as usize + 1]
    };

    let gap = 8;
    for row_idx in 0..h / gap {
        for col_idx in 0..w / gap {
            let mut col = (gap * col_idx) as f32 / w as f32;
            let mut row = (gap * row_idx) as f32 / h as f32;
            let mut points = 0;
            while points < 10000 && col < 1.0 && row < 1.0 && col > 0.0 && row > 0.0 {
                let step = 0.0001;
                let noise =
                    (0.05 * state.frame as f32) + 6.0 * noise_fn(col * w as f32, row * h as f32);
                row += noise.sin() * step;
                col += noise.cos() * step;
                let x = (col * w as f32) as u32;
                let y = (row * h as f32) as u32;
                if x < w as u32 && y < h as u32 && x > 0 && y > 0 {
                    let pixel = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
                    pixel[0] = pixel[0].saturating_add(2);
                    pixel[1] = pixel[1].saturating_add(2);
                    pixel[2] = pixel[2].saturating_add(2);
                }
                points += 1;
            }
        }
    }
}

#[derive(Resource)]
pub struct ImageHandle(pub Handle<Image>);

fn init(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let w = 512;
    let h = 512;
    let image = Image::new_fill(
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
    let perlin = Perlin::new(0, 4.0, 1.7, 0.4, 5);

    let mut state = State {
        precomputed: vec![vec![0.0; w as usize]; h as usize],
        frame: 0,
    };
    println!("Precomputing noise...");
    for row in 0..h {
        for col in 0..w {
            let noise = perlin.noise([row as f32 / w as f32, col as f32 / h as f32]);
            state.precomputed[row as usize][col as usize] = noise;
        }
    }
    commands.insert_resource(state);
    let image_handle = images.add(image);
    commands.insert_resource(ImageHandle(image_handle.clone()));
    commands.spawn(Sprite::from_image(image_handle));
    commands.spawn((Camera2d,));
}
