use crate::embed_asset;
use crate::prelude::*;

use bevy_ui_text_input::{TextInputContents, TextInputFilter, TextInputMode, TextInputNode};

const DEFAULT_FONT_PATH: &str = "embedded://assets/fonts/Ithaca/Ithaca-LVB75.ttf";
const TEXT_COLOR: Color = Color::srgb_u8(0xFF, 0xFF, 0xFF);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        embed_asset!(app, "assets/fonts/Ithaca/Ithaca-LVB75.ttf");

        app.init_state::<MenuState>();

        #[cfg(feature = "debug")]
        app.add_systems(Update, log_transitions::<MenuState>);

        app.add_systems(
            OnEnter(MenuState::Main),
            ((load_font, camera_setup), main_enter).chain(),
        )
        .add_systems(Update, button_highlight);
    }
}

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

fn load_font(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(GameFont(asset_server.load(DEFAULT_FONT_PATH)));
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::WindowSize,
            ..OrthographicProjection::default_2d()
        }),
        Transform::IDENTITY,
    ));
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    Main,
}

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

fn main_enter(mut commands: Commands, font: Res<GameFont>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font: font.0.clone(),
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StateScoped(MenuState::Main),
        ))
        .with_children(|builder| {
            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        TextInputContents::default(),
                        TextInputNode {
                            clear_on_submit: false,
                            mode: TextInputMode::SingleLine,
                            focus_on_pointer_down: true,
                            unfocus_on_submit: true,
                            max_chars: Some(16),
                            filter: Some(TextInputFilter::Hex),
                            ..default()
                        },
                        button_text_style.clone(),
                    ));

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            children![(button_text_style, Text::new("Quit"), Pickable::IGNORE),],
                        ))
                        .observe(quit_game_on_click);
                });
        });
}

fn button_highlight(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    // for (interaction, mut background_color, selected) in &mut interaction_query {
    //     *background_color = match (*interaction, selected) {
    //         (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
    //             PRESSED_BUTTON_COLOR.into()
    //         }
    //         (Interaction::Hovered, Some(_)) => style.hovered_pressed_button_color.into(),
    //         (Interaction::Hovered, Option::None) => style.hovered_button_color.into(),
    //         (Interaction::None, Option::None) => style.button_color.into(),
    //     }
    // }
}

fn quit_game_on_click(
    mut click: Trigger<Pointer<Click>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    click.propagate(false);

    if click.button == PointerButton::Primary {
        app_exit_events.write(AppExit::Success);
    }
}
