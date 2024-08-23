use crate::components::ScoreText;
use crate::consts::ARENA_OFFSET;
use bevy::prelude::*;

pub fn spawn_score(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Px(ARENA_OFFSET),
            width: Val::Percent(30.0),
            height: Val::Percent(20.0),
            border: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(2.0),
                top: Val::Px(0.0),
                bottom: Val::Px(2.0),
            },
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        border_color: BorderColor(Color::WHITE),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn((TextBundle {
            text: Text::from_section(format!("{} | {}", 0, 0).to_string(), TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..Default::default()
            }),
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        }, ScoreText::default()));
    });
}