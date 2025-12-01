use bevy::prelude::*;
use crate::character::Player;
use crate::world_grid::GridConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;

const HIGHSCORE_FILE: &str = "highscores.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighScoreEntry {
    pub name: String,
    pub score: f32,
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<HighScoreEntry>,
}

impl HighScores {
    pub fn is_high_score(&self, score: f32) -> bool {
        if self.scores.len() < 10 {
            return true;
        }
        score > self.scores.last().map(|s| s.score).unwrap_or(0.0)
    }

    pub fn add_score(&mut self, name: String, score: f32) {
        self.scores.push(HighScoreEntry { name, score });
        // Sort descending
        self.scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        // Keep top 10
        if self.scores.len() > 10 {
            self.scores.truncate(10);
        }
    }
}

#[derive(Resource, Default)]
pub struct ScoreText {
    pub text: String,
    pub score: f32,
}

#[derive(Resource, Default)]
pub struct FinalScore(pub f32);

#[derive(Resource, Default)]
pub struct PlayerNameInput(pub String);

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ScoreboardState {
    #[default]
    Hidden,
    EnterName,
    ShowScores,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct DeathScoreDisplay;

#[derive(Resource)]
pub struct ScoreTimer(pub Timer);

pub fn setup_scoreboard(mut commands: Commands, mut score_text: ResMut<ScoreText>) {
    // Load high scores
    let high_scores = load_high_scores().unwrap_or_default();
    commands.insert_resource(high_scores);
    commands.insert_resource(ScoreboardState::Hidden);
    commands.insert_resource(PlayerNameInput("".to_string()));
    commands.insert_resource(FinalScore(0.0));
    
    // Reset score
    score_text.score = 0.0;

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

pub fn scoreboard_system(
    mut score: ResMut<ScoreText>,
    mut display_query: Query<&mut Text, With<ScoreDisplay>>,
    mut query: Query<(Entity, &mut Player)>,
) {
    for(_player_entity, player) in query.iter_mut() {
        if player.is_dead{
            return;
        } else {
            if let Ok(mut text) = display_query.get_single_mut() {
                text.0 = format!("Score: {}", score.score);
            }
        }
    }
}

// System to handle state changes and render UI
pub fn update_death_scoreboard_ui(
    mut commands: Commands,
    state: Res<ScoreboardState>,
    mut current_state: Local<ScoreboardState>,
    high_scores: Res<HighScores>,
    name_input: Res<PlayerNameInput>,
    final_score: Res<FinalScore>,
    ui_query: Query<Entity, With<DeathScoreDisplay>>,
) {
    if *state != *current_state {
        // Cleanup old
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        match *state {
            ScoreboardState::EnterName => {
                spawn_enter_name_ui(&mut commands, final_score.0, &name_input.0);
            },
            ScoreboardState::ShowScores => {
                spawn_high_score_list(&mut commands, &high_scores, final_score.0);
            },
            ScoreboardState::Hidden => {},
        }
        *current_state = *state;
    } 
}

// System to update the text of the input field
pub fn update_input_text(
    name_input: Res<PlayerNameInput>,
    state: Res<ScoreboardState>,
    mut query: Query<&mut Text, (With<DeathScoreDisplay>, Without<ScoreDisplay>)>, // refine query
) {
    if *state == ScoreboardState::EnterName {
        // ...
    }
}

#[derive(Component)]
pub struct NameInputText;

pub fn update_name_input_display(
    name_input: Res<PlayerNameInput>,
    mut query: Query<&mut Text, With<NameInputText>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("Enter Name: {}_", name_input.0);
    }
}

pub fn handle_input_system(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut name_input: ResMut<PlayerNameInput>,
    mut state: ResMut<ScoreboardState>,
    mut high_scores: ResMut<HighScores>,
    final_score: Res<FinalScore>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if *state == ScoreboardState::EnterName {
        if keys.just_pressed(KeyCode::Enter) {
            let name = if name_input.0.trim().is_empty() {
                "Anonymous".to_string()
            } else {
                name_input.0.clone()
            };
            
            high_scores.add_score(name, final_score.0);
            save_high_scores(&high_scores).ok();
            *state = ScoreboardState::ShowScores;
            return;
        }

        if keys.just_pressed(KeyCode::Backspace) {
            name_input.0.pop();
        }

        for ev in key_evr.read() {
            if ev.state == ButtonState::Pressed {
               match &ev.logical_key {
                   bevy::input::keyboard::Key::Character(c) => {
                       let s = c.as_str();
                       if s.chars().count() == 1 {
                            let ch = s.chars().next().unwrap();
                            if !ch.is_control() {
                                name_input.0.push(ch);
                            }
                       }
                   },
                   _ => {}
               }
            }
        }
    } else if *state == ScoreboardState::ShowScores {
         if keys.just_pressed(KeyCode::KeyR) {
             next_game_state.set(GameState::Playing);
         }
    }
}

// Helper functions for UI spawning
fn spawn_enter_name_ui(commands: &mut Commands, score: f32, current_name: &str) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            ..default()
        },
        DeathScoreDisplay
    )).with_children(|parent| {
        parent.spawn((
            Text::new(format!("New High Score: {:.0}!", score)),
            TextFont { font_size: 40.0, ..default() },
            TextColor(Color::WHITE),
        ));
        
        parent.spawn((
            Text::new(format!("Enter Name: {}_", current_name)),
            TextFont { font_size: 40.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)),
            NameInputText,
        ));
    });
}

fn spawn_high_score_list(commands: &mut Commands, high_scores: &HighScores, user_score: f32) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DeathScoreDisplay
    )).with_children(|parent| {
        parent.spawn((
            Text::new("HIGH SCORES"),
            TextFont { font_size: 50.0, ..default() },
            TextColor(Color::srgb(1.0, 0.84, 0.0)),
            Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() },
        ));

        for (i, entry) in high_scores.scores.iter().enumerate() {
            let color = if entry.score == user_score { Color::srgb(0.0, 1.0, 0.0) } else { Color::WHITE };
            parent.spawn((
                Text::new(format!("{}. {} - {:.0}", i + 1, entry.name, entry.score)),
                TextFont { font_size: 30.0, ..default() },
                TextColor(color),
            ));
        }

        parent.spawn((
             Text::new("Press R to Restart"), 
             TextFont { font_size: 20.0, ..default() },
             TextColor(Color::srgb(0.5, 0.5, 0.5)),
             Node { margin: UiRect::top(Val::Px(40.0)), ..default() },
        ));
    });
}

// Load/Save helpers
fn load_high_scores() -> Option<HighScores> {
    if let Ok(data) = fs::read_to_string(HIGHSCORE_FILE) {
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

fn save_high_scores(scores: &HighScores) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(scores)?;
    let mut file = fs::File::create(HIGHSCORE_FILE)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
