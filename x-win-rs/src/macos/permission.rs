#![deny(unused_imports)]
#![allow(dead_code)]

use objc2::ffi::{BOOL, YES};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  #[link_name = "CGRequestScreenCaptureAccess"]
  fn CGRequestScreenCaptureAccess() -> BOOL;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  #[link_name = "CGPreflightScreenCaptureAccess"]
  fn CGPreflightScreenCaptureAccess() -> BOOL;
}

pub fn check_screen_record_permission() -> bool {
  unsafe { CGPreflightScreenCaptureAccess() == YES }
}

pub fn request_screen_record_permission() -> bool {
  unsafe { CGRequestScreenCaptureAccess() == YES }
}
