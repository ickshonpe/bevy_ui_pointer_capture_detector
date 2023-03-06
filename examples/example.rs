use bevy::prelude::*;
use bevy_ui_pointer_capture_detector::*;

#[derive(Component)]
struct CaptureTextMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiScale { scale: 1.5 });
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.,
        color: Color::WHITE,
    };
    commands
        .spawn(Camera2dBundle::default())
        .commands()
        .spawn((
            TextBundle {
                text: Text::from_section("pointer not captured", text_style.clone()),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(20.),
                        top: Val::Px(20.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            CaptureTextMarker,
            NoPointerCapture,
        ))
        .commands()
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(100.),
                    bottom: Val::Px(100.),
                    right: Val::Auto,
                    top: Val::Auto,
                },
                size: Size {
                    width: Val::Px(150.),
                    height: Val::Px(150.),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::RED),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section("captures pointer", text_style.clone())
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        max_size: Size::new(Val::Px(150.), Val::Px(150.)),
                        ..Default::default()
                    }),
            );
        })
        .commands()
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        left: Val::Px(200.),
                        bottom: Val::Px(300.),
                        right: Val::Auto,
                        top: Val::Auto,
                    },
                    size: Size {
                        width: Val::Px(150.),
                        height: Val::Px(150.),
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..Default::default()
            },
            NoPointerCapture,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section("not capturing", text_style.clone())
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        max_size: Size::new(Val::Px(150.), Val::Px(150.)),
                        ..Default::default()
                    }),
                NoPointerCapture,
            ));
        })
        .commands()
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(100.),
                    bottom: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(100.),
                },
                size: Size {
                    width: Val::Px(150.),
                    height: Val::Px(150.),
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section("captures pointer", text_style.clone())
                    .with_text_alignment(TextAlignment::CENTER)
                    .with_style(Style {
                        max_size: Size::new(Val::Px(150.), Val::Px(150.)),
                        ..Default::default()
                    }),
            );
        });
}

fn is_pointer_captured(
    capture_status: Res<UiPointerCaptureStatus>,
    mut text_query: Query<&mut Text, With<CaptureTextMarker>>,
) {
    text_query.single_mut().sections[0].value = if capture_status.is_captured() {
        "pointer captured by UI"
    } else {
        "pointer escaped UI"
    }
    .to_string();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyUiPointerCaptureDetectorPlugin)
        .add_startup_system(setup)
        .add_system(is_pointer_captured)
        .run();
}
