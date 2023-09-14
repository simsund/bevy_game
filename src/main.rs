use bevy::{prelude::*, ecs::{component, query}};
//use bevy::prelude::ResMut;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (greet_people, player));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("ping".to_string())));
    commands.spawn((Person, Name("pong".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
    
}

fn player(mut commands: Commands, asset_server:Res<AssetServer>) {
    commands.spawn(DirectionalLightBundle {
        directional_light : DirectionalLight {
            color: Color::WHITE,
            shadow_depth_bias: 0.019,
            shadow_normal_bias: 0.6,
            illuminance: 50000., 
            shadows_enabled: true,
        },
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(35.,35.,35.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene : asset_server.load("C:/Users/simon.sundsfjord/OneDrive - Rana Gruber/Skrivebord/kode prosjekter/Rust prosjekter/bevy_game/models/player.gltf#Scene0"),
        ..default()
    });
}

