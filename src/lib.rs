mod impls;

use autoclip_core::{AutoclipPlugin, PluginRegistrar};
use impls::encrypt;

autoclip_core::export_plugin!("pfngp", AutoclipPluginPfngp);

pub struct AutoclipPluginPfngp;

impl AutoclipPlugin for AutoclipPluginPfngp {
    fn on_clip(&self, contents: &str) -> Option<String> {
        if contents.find("/enc").is_some() {
            return encrypt(contents);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_appends_footer() {
        let plugin = AutoclipPluginPfngp {};
        println!("{:?}", plugin.on_clip("ABCDE/enc fli"));
    }
}
