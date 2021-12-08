use log::info;

use scarper::declare_plugin;
use scarper::plugins::Plugin;

#[derive(Debug, Default)]
pub struct PlugGimp;

impl Plugin for PlugGimp {
    fn name(&self) -> &'static str {
        "plug_gimp"
    }

    fn on_plugin_load(&self) {
        info!("plug_gimp loaded");
    }

    fn on_plugin_unload(&self) {
        info!("plug_gimp unloaded");
    }

    fn get_package_version(&self) -> &'static str {
        info!("plug_gimp Get Version");
        "2.10.14"
    }
}

declare_plugin!(PlugGimp, PlugGimp::default);