//! Open Arena Client Initialization Module

use crate::oacli::ClientConfig;
use bevy::prelude::*;

/// Client Initialization Plugin
///
/// Bevy Plugin for initializing the game client.
struct OAInitPlug {
    cfg: ClientConfig,
}

impl OAInitPlug {
    /// Adds plugins based on application configuration
    ///
    /// NOTE: Current implementation requires a restart to apply these changes.
    pub fn apply_configurable_plugs(&self, _app: &mut AppBuilder) {
        // No Sound Toggle
        if !self.cfg.nosnd {
            _app.add_plugin(bevy::audio::AudioPlugin::default());
        }

        // Performance Diagnostics Toggle
        if self.cfg.diag {
            _app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default());
        }
    }

    /// Adds default bevy plugins to app
    ///
    /// See bevy::prelude::DefaultPlugins for the struct this is shadowing
    pub fn apply_default_plugs(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::transform::TransformPlugin::default())
            .add_plugin(bevy::input::InputPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default())
            .add_plugin(bevy::asset::AssetPlugin::default())
            .add_plugin(bevy::scene::ScenePlugin::default());
    }

    /// Adds default plugins based on features
    ///
    /// See bevy::prelude::DefaultPlugins for the struct this is shadowing
    pub fn apply_feature_plugs(&self, _app: &mut AppBuilder) {
        #[cfg(feature = "bevy_render")]
        _app.add_plugin(bevy_render::RenderPlugin::default());

        #[cfg(feature = "bevy_sprite")]
        _app.add_plugin(bevy_sprite::SpritePlugin::default());

        #[cfg(feature = "bevy_pbr")]
        _app.add_plugin(bevy_pbr::PbrPlugin::default());

        #[cfg(feature = "bevy_ui")]
        _app.add_plugin(bevy_ui::UiPlugin::default());

        #[cfg(feature = "bevy_text")]
        _app.add_plugin(bevy_text::TextPlugin::default());

        #[cfg(feature = "bevy_audio")]
        _app.add_plugin(bevy_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_gilrs")]
        _app.add_plugin(bevy_gilrs::GilrsPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        _app.add_plugin(bevy_gltf::GltfPlugin::default());

        #[cfg(feature = "bevy_winit")]
        _app.add_plugin(bevy_winit::WinitPlugin::default());

        #[cfg(feature = "bevy_wgpu")]
        _app.add_plugin(bevy_wgpu::WgpuPlugin::default());
    }

    /// Builds then adds client initialization plugin to app.
    pub fn apply_init_plugs(&self, app: &mut AppBuilder) {
        app
            // Systems
            .add_startup_system(oasys::say_hello.system())
            .add_startup_system(oasys::version_info.system());
    }
}

impl Plugin for OAInitPlug {
    fn build(&self, app: &mut AppBuilder) {
        // Init engine
        self.apply_configurable_plugs(app);
        self.apply_default_plugs(app);
        self.apply_feature_plugs(app);
        // Add resources
        app.insert_resource(Msaa::default());
        // Start App
        self.apply_init_plugs(app);
    }
}

/// Application Intialization function
///
/// adds the custom OAInitPlug Bevy Plugin and runs the app.
pub fn init() {
    use structopt::StructOpt;

    // Init logger
    oasys::logger_init();

    // Parse CLI Args
    let cfg = ClientConfig::from_args();
    debug!("Parse CLI Args: {:?}", cfg);

    // Parse Config
    // TODO support config files
    let init_plug = OAInitPlug { cfg };

    // Init app
    App::build().add_plugin(init_plug).run();
}

/// Open Arena Initialization Systems
mod oasys {
    use crate::oastatic::*;
    use log::info;

    /// Intializes the env_logger logging runtime
    ///
    /// This function behaves differently in debug / release mode:
    /// - Release: it defaults to the off logging level
    /// - Debug: it defaults to the debug logging level
    pub fn logger_init() {
        use env_logger::Env;

        // Release Mode
        #[cfg(not(debug_assertions))]
        env_logger::Builder::from_env(Env::default().default_filter_or("off")).init();

        // Debug Mode
        #[cfg(debug_assertions)]
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    }

    /// Simple hello-world function
    pub fn say_hello() {
        info!("Welcome to {}!", GAME_CLIENT_NAME);
    }

    /// Version information logging function
    pub fn version_info() {
        info!(
            "{}: version {} \n",
            GAME_CLIENT_NAME_INTERNAL, GAME_CLIENT_VERSION
        );
    }
}
