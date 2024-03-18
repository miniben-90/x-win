#![deny(unused_imports)]

mod api;
use crate::common::api::API;

use api::LinuxAPI;

use self::api::APIGnome;

pub fn init_platform_api() -> impl API {
  LinuxAPI { }
}

pub fn gnome_install_extension() -> () {
  LinuxAPI::install_extension()
}

pub fn gnome_uninstall_extension() -> () {
  LinuxAPI::uninstall_extension()
}