#![deny(unused_imports)]

mod application_services;
mod api;

use crate::common::api::API;
use api::MacosAPI;

pub fn init_platform_api() -> impl API {
  MacosAPI { }
}