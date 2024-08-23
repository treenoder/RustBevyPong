use crate::components::{Ball, GameState, Paddle, PaddleSide, ScoreText};
use crate::consts::*;
use crate::events::GameEvent;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


pub fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(ARENA_WIDTH, ARENA_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-ARENA_WIDTH * PADDLE_OFFSET, 0.0, 0.0)),
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
        transform: Transform::from_translation(Vec3::new(ARENA_WIDTH * PADDLE_OFFSET, 0.0, 0.0)),
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

pub fn move_puddle(
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
        transform.translation.y = transform.translation.y.clamp(-ARENA_HEIGHT + PADDLE_HALF_HEIGHT, ARENA_HALF_HEIGHT - PADDLE_HALF_HEIGHT);
    }
}

pub fn start(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    if input.pressed(KeyCode::Space) {
        game_state.winner = None;
    }
}

pub fn exit_on_esc(mut app_exit_events: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    }, Ball::default()));
}

pub fn move_ball(
    mut balls: Query<(&mut Ball, &mut Transform)>,
    game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    let (mut ball, mut transform) = balls.iter_mut().next().expect("No ball found");
    if ball.is_out {
        transform.translation = Vec3::ZERO;
        *ball = Ball::default();
        return;
    }
    if game_state.winner.is_some() {
        return;
    }
    transform.translation += ball.direction.extend(0.0) * ball.speed * time.delta_seconds();
}

pub fn collide_ball(
    mut balls: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    paddles: Query<(&Paddle, &Transform)>,
    mut events: EventWriter<GameEvent>,
) {
    for (mut ball, mut ball_transform) in balls.iter_mut() {
        if ball.is_out {
            continue;
        }
        // reflect walls
        if ball_transform.translation.y > ARENA_HALF_HEIGHT - BALL_SIZE / 2.0 {
            ball_transform.translation.y = ARENA_HALF_HEIGHT - BALL_SIZE / 2.0;
            ball.direction.y = -ball.direction.y;
        } else if ball_transform.translation.y < -ARENA_HALF_HEIGHT + BALL_SIZE / 2.0 {
            ball_transform.translation.y = -ARENA_HALF_HEIGHT + BALL_SIZE / 2.0;
            ball.direction.y = -ball.direction.y;
        }
        // reflect paddles
        for (_, paddle_transform) in paddles.iter() {
            let ball_translation = ball_transform.translation;
            let paddle_translation = paddle_transform.translation;
            if ball_translation.x - BALL_HALF_SIZE < paddle_translation.x + PADDLE_HALF_WIDTH
                && ball_translation.x + BALL_HALF_SIZE > paddle_translation.x - PADDLE_HALF_WIDTH
                && ball_translation.y - BALL_HALF_SIZE < paddle_translation.y + PADDLE_HALF_HEIGHT
                && ball_translation.y + BALL_HALF_SIZE > paddle_translation.y - PADDLE_HALF_HEIGHT
            {
                ball.direction.x = -ball.direction.x;
                ball.speed += BALL_SPEED_INC;
                ball.speed = ball.speed.min(BALL_SPEED_MAX);
            }
        }
        // out of bounds
        if ball_transform.translation.x > ARENA_HALF_WIDTH + BALL_SIZE / 2.0 {
            ball.is_out = true;
            events.send(GameEvent::GainPoint(PaddleSide::Left));
        } else if ball_transform.translation.x < -ARENA_HALF_WIDTH - BALL_SIZE / 2.0 {
            ball.is_out = true;
            events.send(GameEvent::GainPoint(PaddleSide::Right));
        }
    }
}

pub fn score(
    mut events: EventReader<GameEvent>,
    mut score_text: Query<(&mut Text, &ScoreText)>,
    mut game_state: ResMut<GameState>,
) {
    for event in events.read() {
        let GameEvent::GainPoint(side) = event;
        game_state.winner = Some(*side);
        match side {
            PaddleSide::Left => {
                game_state.left_score += 1;
            }
            PaddleSide::Right => {
                game_state.right_score += 1;
            }
        }
        for (mut text, _) in score_text.iter_mut() {
            text.sections[0].value = format!("{} | {}", game_state.left_score, game_state.right_score);
        }
    }
}