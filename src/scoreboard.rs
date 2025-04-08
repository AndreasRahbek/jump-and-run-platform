use bevy::prelude::*;
use crate::character::Player;
use crate::world_grid::GridConfig;

#[derive(Resource, Default)]
pub struct ScoreText{
    pub text: String,
    pub score: f32,
}

#[derive(Component)]
pub struct ScoreDisplay;
#[derive(Component)]
pub struct DeathScoreDisplay;

#[derive(Resource)]
pub struct ScoreTimer(pub Timer);

pub fn increase_score_system(
    time: Res<Time>,
    mut score_timer: ResMut<ScoreTimer>,
    mut score_text: ResMut<ScoreText>,
    mut grid_config: ResMut<GridConfig>

) {
    score_timer.0.tick(time.delta());

    if score_timer.0.just_finished() {
        score_text.score  += (2.0f32.powf(grid_config.scroll_speed / 30f32)).round();
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
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
        ScoreDisplay
    ));
}

pub fn scoreboard_system(
    mut score: ResMut<ScoreText>,
    mut display: Single<(&mut Text), With<ScoreDisplay>>,
    mut query: Query<(Entity, &mut Player)>,
) {
    for(_player_entity, player) in query.iter_mut() {
        if player.is_dead{
            return;
        } else {
            display.0 = format!("Score: {}", score.score);
        }
    }
}

pub fn show_death_scoreboard(
    mut commands: &mut Commands,
    mut query: &mut Query<(Entity, &mut Player)>,
    mut score_text: Single<Entity>,
    mut score: ResMut<ScoreText>
){
    commands.spawn((
        Text::new("Noob dead. Final score:"),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(1., 0., 0.)),
        Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        DeathScoreDisplay
    ));

    for(_player_entity, mut player) in query.iter_mut() {
        player.final_score = score.score;
    }
    commands.entity(*score_text).despawn();
}


