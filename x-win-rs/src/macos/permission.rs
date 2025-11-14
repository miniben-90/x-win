#![deny(unused_imports)]
#![allow(dead_code)]

use objc2_core_graphics::{CGPreflightScreenCaptureAccess, CGRequestScreenCaptureAccess};

/// Return `true` or `false` about screen record permission
/// <div class="warning">important Work only with macOS 10.15+</div>
/// To use this function you need to add `macos_permission` feature
pub fn check_screen_record_permission() -> bool {
  CGPreflightScreenCaptureAccess()
}

/// Ask for screen record permission and return `true` or `false` after confirmation
/// <div class="warning">important Work only with macOS 10.15+</div>
pub fn request_screen_record_permission() -> bool {
  CGRequestScreenCaptureAccess()
}
