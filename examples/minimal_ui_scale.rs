use bevy::prelude::*;
use bevy_ui_pointer_capture_detector::*;

fn setup(
    mut commands: Commands
) {
    commands.insert_resource(UiScale { scale: 3.0 });
    commands
        .spawn(Camera2dBundle::default())
        .commands()
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect { 
                    left: Val::Px(100.0), 
                    bottom: Val::Px(100.0), 
                    right: Val::Auto, 
                    top: Val::Auto 
                },
                size: Size { 
                    width: Val::Px(100.0), 
                    height: Val::Px(100.0) 
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::RED),
            ..Default::default()
        });
}

fn is_pointer_captured(
    capture: Res<UiPointerCaptureStatus>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = if capture.is_captured() {
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