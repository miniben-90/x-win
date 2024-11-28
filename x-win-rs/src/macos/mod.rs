#![deny(unused_imports)]

mod api;
pub mod permission;

use crate::common::api::Api;
use api::MacosAPI;

pub fn init_platform_api() -> impl Api {
  MacosAPI {}
}
