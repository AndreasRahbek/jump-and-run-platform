use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ScoreText{
    pub text: String,
    pub score: i32,
}

#[derive(Resource)]
pub struct ScoreTimer(pub Timer);

pub fn increase_score_system(
    time: Res<Time>,
    mut score_timer: ResMut<ScoreTimer>,
    mut scoreText: ResMut<ScoreText>,
) {
    score_timer.0.tick(time.delta());

    if score_timer.0.just_finished() {
        scoreText.score += 1;
    }
}

pub fn setup_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text::new("Score:"),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },

    ));
}

pub fn scoreboard_system(mut score: ResMut<ScoreText>, mut display: Single<&mut Text>) {
    display.0 = format!("Score: {}", score.score);
}


