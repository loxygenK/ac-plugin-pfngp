mod impls;

use autoclip_core::{AutoclipPlugin, PluginRegistrar};
use impls::{encrypt, decrypt};

autoclip_core::export_plugin!("pfngp", AutoclipPluginPfngp);

pub struct AutoclipPluginPfngp;

impl AutoclipPlugin for AutoclipPluginPfngp {
    fn on_clip(&self, contents: &str) -> Option<String> {
        if contents.find("/enc").is_some() {
            return encrypt(contents);
        }
        if contents.find("-----BEGIN PGP MESSAGE-----").is_some() {
            return decrypt(contents);
        }
        None
    }
}
