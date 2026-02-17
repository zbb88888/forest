use bevy::prelude::*;
use crate::states::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
           .add_systems(Update, menu_action.run_if(in_state(GameState::MainMenu)))
           .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct StartButton;

fn setup_menu(mut commands: Commands) {
    // Root node for the menu
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            MenuRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Dark Forest"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Spacing
            parent.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            // Start Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.3, 0.3, 0.3);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.2, 0.2, 0.2);
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
