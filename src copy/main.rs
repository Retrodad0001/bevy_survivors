use assets::AssetInfo;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    window::WindowResolution,
};
use bevy_egui::{EguiContexts, EguiPlugin, egui};

mod assets;
mod constants;

fn main() {
    let mut app = App::new();

    // RESOURCES
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))); // Corrected typo in color value comment

    // PLUGINS
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // BUG: (Repeating rendering error in debug console workaround) | wait for upstream Bevy fix
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
            .set(
                // Here we configure the main window
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        // ...
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .set(ImagePlugin::default_nearest()), // Pixel rendering
    );
    app.add_plugins(EguiPlugin);

    // SYSTEMS
    app.add_systems(Startup, (setup_assets, setup_camera, setup_scene).chain());
    app.add_systems(Update, (show_developer_overlay, camera_update));

    app.run();
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image: Handle<Image> = asset_server.load("0x72_DungeonTilesetII_v1.7.png");
    let layout: TextureAtlasLayout = TextureAtlasLayout::new_empty(UVec2::new(100, 100));
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    commands.insert_resource(AssetInfo::new(image, texture_atlas_layout));
}

fn setup_scene(mut commands: Commands, asset_info: Res<AssetInfo>) {
    commands.spawn((Sprite::from_atlas_image(
        asset_info.image.clone(),
        TextureAtlas {
            layout: asset_info.texture_atlas_layout.clone(),
            index: 0, //index of spritesheet
        },
    ),));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d {}, MyCamera {}));
}

fn show_developer_overlay(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

#[derive(Component)]
struct MyCamera;

fn camera_update(mut query_camera: Query<&mut OrthographicProjection, With<MyCamera>>) {
    let mut projection = query_camera.single_mut();
    projection.scale = 0.2;
}

// TODO: Use later
// fn update_follow_camera(
//     mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
//     player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
//     time: Res<Time>,
// ) {
//     let Ok(mut camera) = camera.get_single_mut() else {
//         return;
//     };

//     let Ok(player) = player.get_single() else {
//         return;
//     };

//     let Vec3 { x, y, .. } = player.translation;
//     let direction = Vec3::new(x, y, camera.translation.z);

//     // Applies a smooth effect to camera movement using stable interpolation
//     // between the camera position and the player position on the x and y axes.
//     camera
//         .translation
//         .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
// }
