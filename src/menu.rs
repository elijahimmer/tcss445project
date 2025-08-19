use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();

        #[cfg(feature = "debug")]
        app.add_systems(Update, log_transitions::<MenuState>);

        app.add_systems(Update, button_highlight)
            .add_systems(OnEnter(MenuState::Main), main_enter);
    }
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

fn main_enter(mut commands: Commands) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

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
                    builder
                        .spawn((
                            Button,
                            button_node.clone(),
                            children![(Text::new("Quit"), Pickable::IGNORE),],
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
