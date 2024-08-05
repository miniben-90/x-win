#![deny(unused_imports)]

mod api;
use crate::common::api::API;

use api::LinuxAPI;

use self::api::APIGnome;

pub fn init_platform_api() -> impl API {
  LinuxAPI {}
}

pub fn gnome_install_extension() -> bool {
  LinuxAPI::install_extension()
}

pub fn gnome_uninstall_extension() -> bool {
  LinuxAPI::uninstall_extension()
}

pub fn gnome_enable_extension() -> bool {
  LinuxAPI::enable_extension()
}

pub fn gnome_disable_extension() -> bool {
  LinuxAPI::disable_extension()
}
