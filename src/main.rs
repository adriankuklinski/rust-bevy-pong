use bevy::prelude::*;

const BALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 100.0, 0.0);

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .insert_resource(ClearColor(Color::rgb(0.25, 0.23, 0.35)))
       .add_startup_system(setup)
       .add_system(ball_movement_system)
       .add_system(paddle_movement_system)
       .run();
}

#[derive(Component)]
struct Ball {
    velocity: Vec2 
}

#[derive(Component)]
struct Paddle;

impl Paddle {
    const SPEED: f32 = 20.0;
}


#[derive(Component)]
enum Player {
    Left,
    Right
}

impl Player {
    fn start_position(&self) -> Vec2 {
        let x_position = match self {
            Player::Left => -300.0, 
            Player::Right => 300.0, 
        };

        return Vec2::new(x_position, 0.0);
    }

    fn movement_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            Player::Left => (KeyCode::W, KeyCode::S),
            Player::Right => (KeyCode::Up, KeyCode::Down),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::Right
    }
}

impl Default for Ball {
    fn default() -> Self {
        const DEFAULT_VELOCITY: f32 = 400.0;

        Self { 
            velocity: Vec2::new(DEFAULT_VELOCITY, 0.0)
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_ball(&mut commands);
    spawn_paddle(&mut commands, Player::Right);
    spawn_paddle(&mut commands, Player::Left);
}

fn spawn_ball(commands: &mut Commands) {
    commands.spawn((SpriteBundle {
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
    },
    Ball::default()
    ));
}

fn spawn_paddle(commands: &mut Commands, player: Player) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        transform: Transform {
            translation: player.start_position().extend(0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        ..default()
    },
    Paddle,
    player
    ));
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in &mut query.iter_mut() {
        transform.translation += ball.velocity.extend(0.0) * time.delta_seconds();
    }
}

fn paddle_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Paddle, &Player, &mut Transform)>) {
    for (_paddle, player, mut transform) in &mut query.iter_mut() {
        let (up_keycode, down_keycode) = player.movement_keys();

        if keyboard_input.pressed(up_keycode) {
            transform.translation += Vec2::new(0.0, Paddle::SPEED).extend(0.0);
        }
        
        if keyboard_input.pressed(down_keycode) {
            transform.translation += Vec2::new(0.0, -Paddle::SPEED).extend(0.0);
        }
    } 
}
