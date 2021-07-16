use super::wall::Wall;
use super::Pong;
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::ecs::system::{Commands, Res};
use bevy::math::{Rect, Size};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Default, Debug, Inspectable)]
pub struct Score {
    pub left: usize,
    pub right: usize,
}

impl Display for Score {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.left, self.right)
    }
}

pub struct ScoreBoard;

pub fn spawn_score_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(50.0)),
                // center
                margin: Rect {
                    top: Val::Px(2.0 * Wall::THICKNESS),
                    ..Rect::all(Val::Auto)
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0 : 0".to_string(),
                    style: TextStyle {
                        font_size: 60.0,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        ..Default::default()
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        })
        .insert(ScoreBoard)
        .insert(Name::new("Score Board"))
        .insert(Pong);
}

pub fn update_score_board(score: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("{}", score.deref());
}
