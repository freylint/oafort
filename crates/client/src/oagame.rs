//! OAFort Gameplay Module

use bevy::prelude::*;

/// Bevy plugin containing logic for OAF's gameplay
pub struct OAGameplayPlug;

impl Plugin for OAGameplayPlug {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(gp_bootstrap::spawn_cam_main.system());
    }
}

/// Gameplay bootstrapping module
mod gp_bootstrap {
    use super::*;
    /// Spawn the main camera
    pub fn spawn_cam_main(commands: Commands) {}
}
