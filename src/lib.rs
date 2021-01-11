use autoclip_core::{AutoclipPlugin, PluginRegistrar};

autoclip_core::export_plugin!("pfngp", AutoclipPluginPfngp);

pub struct AutoclipPluginPfngp;

impl AutoclipPlugin for AutoclipPluginPfngp {
    fn on_clip(&self, contents: &str) -> Option<String> {
        Some(format!("{} ... OK", contents))
    }
}
