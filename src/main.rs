use bevy::prelude::*;

// Set colors.
const BALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
//const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
//const BACKGROUND_COLOR: Color = Color::rgb(0.7, 0.9, 1.3);

const BALL_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_startup_system(setup)
       .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_ball(&mut commands);
}

fn spawn_ball(commands: &mut Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: BALL_COLOR,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: BALL_SIZE,
            ..default()
        },
        ..default()
    });
}
