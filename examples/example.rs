use bevy::prelude::*;
use bevy_ui_pointer_capture_detector::*;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(100.0),
                    bottom: Val::Px(100.0),
                    right: Val::Auto,
                    top: Val::Auto,
                },
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                },
                ..Default::default()
            },
            color: UiColor(Color::RED),
            ..Default::default()
        })
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(200.0),
                    bottom: Val::Px(300.0),
                    right: Val::Auto,
                    top: Val::Auto,
                },
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                },
                ..Default::default()
            },
            color: UiColor(Color::GREEN),
            ..Default::default()
        })
        .insert(NoPointerCapture)
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(100.0),
                    bottom: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(100.0),
                },
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                },
                ..Default::default()
            },
            color: UiColor(Color::BLUE),
            ..Default::default()
        });
}

fn is_pointer_captured(is_captured: Res<IsPointerCaptured>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = if is_captured.0 {
        Color::DARK_GRAY
    } else {
        Color::WHITE
    };
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyUiPointerCaptureDetectorPlugin)
        .add_startup_system(setup)
        .add_system(is_pointer_captured)
        .run();
}
