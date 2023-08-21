use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/Fox.glb#Scene0"),
        transform: Transform::from_scale(Vec3::splat(0.1)),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 10.0, 30.0)
            .looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.7, 0.4)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_systems(Startup, setup)
        .run();
}
