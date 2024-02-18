#![deny(unused_imports)]

mod api;
use crate::common::api::API;

use api::LinuxAPI;

pub fn init_platform_api() -> impl API {
  LinuxAPI { }
}
