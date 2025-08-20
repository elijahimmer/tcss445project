use crate::embed_asset;
use crate::prelude::*;

use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::state::state::FreelyMutableState;
use bevy_ui_text_input::{TextInputContents, TextInputMode, TextInputNode};

const DEFAULT_FONT_PATH: &str = "embedded://assets/fonts/Ithaca/Ithaca-LVB75.ttf";
const TITLE_PATH: &str = "embedded://assets/title.png";
const TEXT_COLOR: Color = Color::srgb_u8(0xFF, 0xFF, 0xFF);
const TEXT_INPUT_COLOR: Color = Color::srgb_u8(0x33, 0x55, 0x33);
const BUTTON_COLOR: Color = Color::srgb_u8(0x33, 0x55, 0x77);
const HOVERED_BUTTON_COLOR: Color = Color::srgb_u8(0x77, 0x55, 0x33);
const PRESSED_BUTTON_COLOR: Color = Color::srgb_u8(0x00, 0x00, 0x00);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        embed_asset!(app, "assets/fonts/Ithaca/Ithaca-LVB75.ttf");
        embed_asset!(app, "assets/title.png");

        app.init_state::<MenuState>();
        app.add_plugins(bevy_ui_text_input::TextInputPlugin);

        #[cfg(feature = "debug")]
        app.add_systems(Update, log_transitions::<MenuState>);

        app.add_systems(Startup, (load_font, camera_setup))
            .add_systems(
                Update,
                change_state(MenuState::Main).run_if(in_state(MenuState::Loading)),
            )
            .add_systems(OnEnter(MenuState::Main), main_enter)
            .add_systems(OnEnter(MenuState::Breed), breed_enter)
            .add_systems(OnEnter(MenuState::Search), search_enter)
            .add_systems(Update, button_highlight);
    }
}

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

pub fn change_state<State: FreelyMutableState + Clone>(
    state: State,
) -> ScheduleConfigs<ScheduleSystem> {
    (move |mut next_state: ResMut<NextState<State>>| next_state.set(state.clone())).into_configs()
}

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
    Loading,
    Main,
    Breed,
    Search,
}

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct MotherPokemonInput;

#[derive(Component)]
struct MotherPokemonInfo;

#[derive(Component)]
struct OtherPokemonInput;

#[derive(Component)]
struct OtherPokemonInfo;

#[derive(Component)]
struct ResultLabel;

fn main_enter(mut commands: Commands, font: Res<GameFont>, asset_server: ResMut<AssetServer>) {
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
                        button_text_style.clone(),
                        ImageNode {
                            image: asset_server.load(TITLE_PATH),
                            ..default()
                        },
                        Pickable::IGNORE,
                    ));

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(
                                button_text_style.clone(),
                                Text::new("Breed"),
                                Pickable::IGNORE
                            ),],
                        ))
                        .observe(change_state_on_click(
                            PointerButton::Primary,
                            MenuState::Breed,
                        ));

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(
                                button_text_style.clone(),
                                Text::new("Search"),
                                Pickable::IGNORE
                            ),],
                        ))
                        .observe(change_state_on_click(
                            PointerButton::Primary,
                            MenuState::Search,
                        ));

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(
                                button_text_style.clone(),
                                Text::new("Quit"),
                                Pickable::IGNORE
                            ),],
                        ))
                        .observe(quit_game_on_click);
                });
        });
}

fn breed_enter(mut commands: Commands, font: Res<GameFont>) {
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
            StateScoped(MenuState::Breed),
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
                        button_text_style.clone(),
                        Text::new("Mother Pokemon"),
                        Pickable::IGNORE,
                    ));
                    builder.spawn((
                        Node {
                            width: Val::Px(500.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        TextInputContents::default(),
                        BackgroundColor(TEXT_INPUT_COLOR),
                        TextInputNode {
                            clear_on_submit: false,
                            mode: TextInputMode::SingleLine,
                            focus_on_pointer_down: true,
                            unfocus_on_submit: true,
                            max_chars: Some(32),
                            ..default()
                        },
                        MotherPokemonInput,
                        button_text_style.clone(),
                    ));
                    builder.spawn((
                        button_text_style.clone(),
                        Text::new(""),
                        Pickable::IGNORE,
                        MotherPokemonInfo,
                    ));

                    builder.spawn((
                        button_text_style.clone(),
                        Text::new("Other Pokemon"),
                        Pickable::IGNORE,
                    ));
                    builder.spawn((
                        Node {
                            width: Val::Px(500.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        TextInputContents::default(),
                        BackgroundColor(TEXT_INPUT_COLOR),
                        TextInputNode {
                            clear_on_submit: false,
                            mode: TextInputMode::SingleLine,
                            focus_on_pointer_down: true,
                            unfocus_on_submit: true,
                            max_chars: Some(32),
                            ..default()
                        },
                        OtherPokemonInput,
                        button_text_style.clone(),
                    ));
                    builder.spawn((
                        button_text_style.clone(),
                        Text::new(""),
                        Pickable::IGNORE,
                        OtherPokemonInfo,
                    ));

                    builder.spawn((
                        button_text_style.clone(),
                        Text::new("Result:"),
                        Pickable::IGNORE,
                    ));
                    builder.spawn((
                        button_text_style.clone(),
                        Text::new(""),
                        Pickable::IGNORE,
                        ResultLabel,
                    ));

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(
                                button_text_style.clone(),
                                Text::new("Submit"),
                                Pickable::IGNORE
                            ),],
                        ))
                        .observe(breed_submit_button);

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(button_text_style, Text::new("Back"), Pickable::IGNORE),],
                        ))
                        .observe(change_state_on_click(
                            PointerButton::Primary,
                            MenuState::Main,
                        ));
                });
        });
}
fn search_enter(mut commands: Commands, font: Res<GameFont>) {
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
            StateScoped(MenuState::Search),
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
                        button_text_style.clone(),
                        Text::new("Pokemon"),
                        Pickable::IGNORE,
                    ));
                    builder.spawn((
                        Node {
                            width: Val::Px(500.0),
                            height: Val::Px(60.0),
                            ..default()
                        },
                        TextInputContents::default(),
                        BackgroundColor(TEXT_INPUT_COLOR),
                        TextInputNode {
                            clear_on_submit: false,
                            mode: TextInputMode::SingleLine,
                            focus_on_pointer_down: true,
                            unfocus_on_submit: true,
                            max_chars: Some(32),
                            ..default()
                        },
                        MotherPokemonInput,
                        button_text_style.clone(),
                    ));

                    builder.spawn((
                        button_text_style.clone(),
                        Text::new("Result:"),
                        Pickable::IGNORE,
                    ));
                    builder.spawn((
                        button_text_style.clone(),
                        Text::new(""),
                        Pickable::IGNORE,
                        ResultLabel,
                    ));
                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(
                                button_text_style.clone(),
                                Text::new("Submit"),
                                Pickable::IGNORE
                            ),],
                        ))
                        .observe(search_submit_button);

                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            children![(button_text_style, Text::new("Back"), Pickable::IGNORE),],
                        ))
                        .observe(change_state_on_click(
                            PointerButton::Primary,
                            MenuState::Main,
                        ));
                });
        });
}

fn button_highlight(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Option::None) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::None, Option::None) => BUTTON_COLOR.into(),
        }
    }
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

fn breed_submit_button(
    mut click: Trigger<Pointer<Click>>,
    mother: Query<&TextInputContents, With<MotherPokemonInput>>,
    other: Query<&TextInputContents, With<OtherPokemonInput>>,
    mut mother_info: Query<
        &mut Text,
        (
            With<MotherPokemonInfo>,
            Without<ResultLabel>,
            Without<OtherPokemonInfo>,
        ),
    >,
    mut other_info: Query<
        &mut Text,
        (
            With<OtherPokemonInfo>,
            Without<ResultLabel>,
            Without<MotherPokemonInfo>,
        ),
    >,
    mut result: Query<&mut Text, With<ResultLabel>>,
    db: NonSend<Database>,
) {
    click.propagate(false);

    if click.button == PointerButton::Primary {
        let mother = mother.single().unwrap().get();
        let other = other.single().unwrap().get();

        let mother_groups = get_groups(&db, mother);
        let other_groups = get_groups(&db, other);

        let mut result = result.single_mut().unwrap();

        let mut mother_info = mother_info.single_mut().unwrap();

        mother_info.0 = if !exists(&db, mother) {
            "Not Found".into()
        } else {
            let str = if mother_groups.len() > 0 {
                mother_groups.join(", ")
            } else {
                "None".into()
            };
            format!("Egg Groups: {}", str)
        };

        let mut other_info = other_info.single_mut().unwrap();

        other_info.0 = if !exists(&db, other) {
            "Not Found".into()
        } else {
            let str = if other_groups.len() > 0 {
                other_groups.join(", ")
            } else {
                "None".into()
            };
            format!("Egg Groups: {}", str)
        };

        let any_overlap = mother_groups
            .iter()
            .any(|g| other_groups.iter().any(|y| y == g));

        if !any_overlap {
            result.0 = "Bad Match!".into();
        } else {
            let pokemon = if mother == "Ditto" { other } else { mother };

            let egg_moves = get_egg_moves(&db, pokemon);

            let egg_moves = if egg_moves.len() > 0 {
                egg_moves.join(", ")
            } else {
                "None".into()
            };

            result.0 = format!("{pokemon}\nEgg Moves: {egg_moves}");
        };
    }
}

fn search_submit_button(
    mut click: Trigger<Pointer<Click>>,
    mother: Query<&TextInputContents, With<MotherPokemonInput>>,
    mut result: Query<&mut Text, With<ResultLabel>>,
    db: NonSend<Database>,
) {
    click.propagate(false);

    if click.button == PointerButton::Primary {
        let mother = mother.single().unwrap().get();

        let mut result = result.single_mut().unwrap();

        result.0 = if !exists(&db, mother) {
            "Not Found".into()
        } else {
            let compatible = get_pokemon_compatible(&db, mother);

            format!(
                "Breedable: {}",
                if compatible.len() > 0 {
                    compatible.join(", ")
                } else {
                    "None".into()
                }
            )
        };
    }
}
fn exists(db: &Database, name: &str) -> bool {
    let query = r#"
            SELECT COUNT(*)
                FROM pokemon
                WHERE pokemon.name = :name
                COLLATE NOCASE
        "#;
    let mut query = db.connection.prepare_cached(query).unwrap();

    query.query_one((name,), |a| a.get::<_, u32>(0)).unwrap() > 0
}

fn get_groups(db: &Database, name: &str) -> Vec<String> {
    let query = r#"
            SELECT egg_group.name
                FROM pokemon
                    JOIN pokemon_egg_group ON pokemon.pokemon_id = pokemon_egg_group.pokemon_id
                    JOIN egg_group ON pokemon_egg_group.egg_group_id = egg_group.egg_group_id
                WHERE pokemon.name = :pokemon_name
                COLLATE NOCASE
        "#;
    let mut query = db.connection.prepare_cached(query).unwrap();

    query
        .query_map((name,), |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn get_egg_moves(db: &Database, name: &str) -> Vec<String> {
    let query = r#"
        SELECT move.name
            FROM pokemon
                JOIN pokemon_move ON pokemon.pokemon_id = pokemon_move.pokemon_id
                JOIN move ON pokemon_move.move_id = move.move_id
            WHERE pokemon.name = :pokemon_name
              AND pokemon_move.method = 'egg'
            COLLATE NOCASE
    "#;
    let mut query = db.connection.prepare_cached(query).unwrap();

    query
        .query_map((name,), |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn get_pokemon_compatible(db: &Database, name: &str) -> Vec<String> {
    let query = r#"
        SELECT pokemon.name
            FROM pokemon
                JOIN pokemon_egg_group ON pokemon.pokemon_id = pokemon_egg_group.pokemon_id
                JOIN egg_group ON pokemon_egg_group.egg_group_id = egg_group.egg_group_id
                JOIN (
                    SELECT eg.egg_group_id AS id
                        FROM pokemon as pk
                            JOIN pokemon_egg_group peg ON pk.pokemon_id = peg.pokemon_id
                            JOIN egg_group eg ON peg.egg_group_id = eg.egg_group_id
                        WHERE pk.name = :pokemon_name
                        COLLATE NOCASE
                ) AS breedable ON egg_group.egg_group_id = breedable.id
            GROUP BY pokemon.pokemon_id
    "#;
    let mut query = db.connection.prepare_cached(query).unwrap();

    query
        .query_map((name,), |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn change_state_on_click<State: FreelyMutableState + Clone>(
    click: PointerButton,
    state: State,
) -> impl Fn(Trigger<Pointer<Click>>, ResMut<NextState<State>>) {
    move |mut event, mut next_state| {
        if event.button != click {
            return;
        }

        next_state.set(state.clone());
        event.propagate(false);
    }
}
