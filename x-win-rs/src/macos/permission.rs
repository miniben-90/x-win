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

/// Return `true` or `false` about screen record permission
/// <div class="warning">important Work only with macOS 10.15+</div>
/// To use this function you need to add `macos_permission` feature
pub fn check_screen_record_permission() -> bool {
  unsafe { CGPreflightScreenCaptureAccess() == YES }
}

/// Ask for screen record permission and return `true` or `false` after confirmation
/// <div class="warning">important Work only with macOS 10.15+</div>
pub fn request_screen_record_permission() -> bool {
  unsafe { CGRequestScreenCaptureAccess() == YES }
}
