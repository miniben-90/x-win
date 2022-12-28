#![deny(unused_imports)]

mod api;

use crate::common::api::API;
use api::WindowsAPI;

pub fn init_platform_api() -> impl API {
  WindowsAPI { }
}