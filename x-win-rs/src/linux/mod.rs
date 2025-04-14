#![deny(unused_imports)]

mod api;
use crate::common::{api::Api, result::Result};

use api::LinuxAPI;

use self::api::APIGnome;

pub fn init_platform_api() -> impl Api {
  LinuxAPI {}
}

pub fn gnome_install_extension() -> Result<bool> {
  LinuxAPI::install_extension()
}

pub fn gnome_uninstall_extension() -> Result<bool> {
  LinuxAPI::uninstall_extension()
}

pub fn gnome_enable_extension() -> Result<bool> {
  LinuxAPI::enable_extension()
}

pub fn gnome_disable_extension() -> Result<bool> {
  LinuxAPI::disable_extension()
}
