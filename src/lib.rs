use autoclip_core::{AutoclipPlugin, PluginRegistrar};

autoclip_core::export_plugin!("pfngp", AutoclipPluginPfngp);

pub struct AutoclipPluginPfngp;

impl AutoclipPlugin for AutoclipPluginPfngp {
    fn on_clip(&self, contents: &str) -> Option<String> {
        Some(format!("{} ... OK", contents))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_appends_footer() {
        let plugin = AutoclipPluginPfngp {};
        assert_eq!(plugin.on_clip("ABCDE").unwrap(), "ABCDE ... OK");
        assert_eq!(plugin.on_clip("").unwrap(), " ... OK");
    }
}
