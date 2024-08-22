use bevy::prelude::*;
use rand::Rng;

const PADDLE_SPEED: f32 = 200.0;
const PADDLE_HEIGHT: f32 = 150.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HALF_HEIGHT: f32 = PADDLE_HEIGHT / 2.0;
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 100.0;
const BALL_SPEED_MAX: f32 = 500.0;
const BALL_SPEED_INC: f32 = 50.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (
        spawn_camera,
        spawn_players,
        spawn_ball,
    ));
    app.add_systems(Update, (
        exit_on_esc,
        move_puddle,
        move_ball,
        collide_ball,
    ));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    speed: f32,
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(700.0, 500.0)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    }, Paddle {
        speed: PADDLE_SPEED,
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
    }));
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    }, Paddle {
        speed: PADDLE_SPEED,
        move_up: KeyCode::ArrowUp,
        move_down: KeyCode::ArrowDown,
    }));
}

fn move_puddle(
    mut paddles: Query<(&Paddle, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (paddle, mut transform) in paddles.iter_mut() {
        let mut direction = 0.0;
        if input.pressed(paddle.move_up) {
            direction += 1.0;
        }
        if input.pressed(paddle.move_down) {
            direction -= 1.0;
        }
        transform.translation.y += direction * paddle.speed * time.delta_seconds();
        transform.translation.y = transform.translation.y.clamp(-250.0 + PADDLE_HALF_HEIGHT, 250.0 - PADDLE_HALF_HEIGHT);
    }
}

fn exit_on_esc(mut app_exit_events: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

#[derive(Component)]
struct Ball {
    direction: Vec2,
    speed: f32,
}

fn spawn_ball(mut commands: Commands) {
    let dir_x = if rand::thread_rng().gen_bool(0.5) {
        1.0
    } else {
        -1.0
    };
    let dir_y = rand::thread_rng().gen_range(-10..=10) as f32 / 10.0;
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    }, Ball {
        direction: Vec2::new(dir_x, dir_y).normalize(),
        speed: BALL_SPEED,
    }));
}

fn move_ball(mut balls: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in balls.iter_mut() {
        transform.translation += ball.direction.extend(0.0) * ball.speed * time.delta_seconds();
    }
}

fn collide_ball(
    mut balls: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    paddles: Query<(&Paddle, &Transform)>,
) {
    for (mut ball, mut ball_transform) in balls.iter_mut() {
        if ball_transform.translation.y > 250.0 - BALL_SIZE / 2.0 {
            ball_transform.translation.y = 250.0 - BALL_SIZE / 2.0;
            ball.direction.y = -ball.direction.y;
        } else if ball_transform.translation.y < -250.0 + BALL_SIZE / 2.0 {
            ball_transform.translation.y = -250.0 + BALL_SIZE / 2.0;
            ball.direction.y = -ball.direction.y;
        }
        for (_, paddle_transform) in paddles.iter() {
            let ball_translation = ball_transform.translation;
            let paddle_translation = paddle_transform.translation;
            if ball_translation.x - BALL_SIZE / 2.0 < paddle_translation.x + PADDLE_WIDTH / 2.0
                && ball_translation.x + BALL_SIZE / 2.0 > paddle_translation.x - PADDLE_WIDTH / 2.0
                && ball_translation.y - BALL_SIZE / 2.0 < paddle_translation.y + PADDLE_HALF_HEIGHT
                && ball_translation.y + BALL_SIZE / 2.0 > paddle_translation.y - PADDLE_HALF_HEIGHT
            {
                ball.direction.x = -ball.direction.x;
                ball.speed += BALL_SPEED_INC;
                ball.speed = ball.speed.min(BALL_SPEED_MAX);
            }
        }
    }
}