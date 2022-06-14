use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct IsPointerCaptured(pub bool);

#[derive(Component)]
pub struct NoPointerCapture;

pub fn pointer_capture_detector(
    windows: Res<Windows>,
    mut is_pointer_captured: ResMut<IsPointerCaptured>,
    node_query: Query<(&Node, &GlobalTransform), Without<NoPointerCapture>>,
) {
    is_pointer_captured.0 = windows
        .get_primary()
        .and_then(|window| window.cursor_position())
        .map(|pointer_position| {
            node_query
                .iter()
                .any(|(&Node { size }, &GlobalTransform { translation, .. })| {
                    let node_position = translation.xy();
                    let half_size = 0.5 * size;
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    (min.x..max.x).contains(&pointer_position.x)
                        && (min.y..max.y).contains(&pointer_position.y)
                })
        })
        .unwrap_or(false);
}

pub struct BevyUiPointerCaptureDetectorPlugin;

impl Plugin for BevyUiPointerCaptureDetectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsPointerCaptured>()
            .add_system_to_stage(CoreStage::PreUpdate, pointer_capture_detector);
    }
}
