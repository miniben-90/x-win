use core_foundation::base::Boolean;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
  pub fn AXIsProcessTrusted() -> Boolean;
}
