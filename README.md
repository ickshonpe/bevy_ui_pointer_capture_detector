# bevy_ui_pointer_capture_detector

A plugin that detects if the mouse pointer is above a Bevy Ui Node.
* supports Bevy 0.9

## Usage

In your ```Cargo.toml``` ```[dependencies]``` section, add 

```toml
bevy_ui_pointer_capture_detector = "0.3"
```

This example draws a square red UI node. Each update it sets the 
color of the background depending on if the mouse pointer is over the red node:

```rust

use bevy::prelude::*;
use bevy_ui_pointer_capture_detector::*;

fn setup(
    mut commands: Commands
) {
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
    is_captured: Res<IsPointerCaptured>,
    mut clear_color: ResMut<ClearColor>,
) {
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
```
#
### Examples

```
cargo run --example example
```