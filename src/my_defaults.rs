use bevy::app::PluginGroupBuilder;
use bevy::app::PluginGroup;
use bevy::scene::ScenePlugin;
use bevy::asset::AssetPlugin;
use bevy::window::WindowPlugin;
use bevy::input::InputPlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::transform::TransformPlugin;
use bevy::core::CorePlugin;
use bevy::log::LogPlugin;

pub struct MyDefaultPlugins;

impl PluginGroup for MyDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LogPlugin::default());
        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(DiagnosticsPlugin::default());
        group.add(InputPlugin::default());
        group.add(WindowPlugin::default());
        group.add(AssetPlugin::default());
        group.add(ScenePlugin::default());

        #[cfg(feature = "bevy_render")]
        group.add(bevy_render::RenderPlugin::default());

        #[cfg(feature = "bevy_sprite")]
        group.add(bevy_sprite::SpritePlugin::default());

        #[cfg(feature = "bevy_pbr")]
        group.add(bevy_pbr::PbrPlugin::default());

        #[cfg(feature = "bevy_ui")]
        group.add(bevy_ui::UiPlugin::default());

        #[cfg(feature = "bevy_text")]
        group.add(bevy_text::TextPlugin::default());

        #[cfg(feature = "bevy_audio")]
        group.add(bevy_audio::AudioPlugin::default());

        // #[cfg(feature = "bevy_gilrs")]
        // group.add(bevy_gilrs::GilrsPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        group.add(bevy_gltf::GltfPlugin::default());

        #[cfg(feature = "bevy_winit")]
        group.add(bevy_winit::WinitPlugin::default());

        #[cfg(feature = "bevy_wgpu")]
        group.add(bevy_wgpu::WgpuPlugin::default());
    }
}